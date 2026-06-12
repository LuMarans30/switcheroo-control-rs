// SPDX-License-Identifier: GPL-3.0-or-later

use zbus::{connection::Builder, fdo::RequestNameFlags, interface};

use std::{process::exit, sync::Arc};
use tokio::{
    io::unix::AsyncFd,
    signal::unix::{SignalKind, signal},
    sync::{RwLock, mpsc},
};

use crate::detection::scan_drm_cards;

mod detection;
mod helpers;
mod info_cleanup;

use switcheroo_common::GpuDevice;

struct SwitcherooServer {
    gpus_cache: Arc<RwLock<Vec<GpuDevice>>>,
}

#[interface(name = "net.hadess.SwitcherooControl")]
impl SwitcherooServer {
    #[zbus(property)]
    async fn has_dual_gpu(&self) -> bool {
        self.gpus_cache.read().await.len() >= 2
    }

    #[zbus(property, name = "NumGPUs")]
    async fn num_gpus(&self) -> u32 {
        self.gpus_cache.read().await.len() as u32
    }

    #[zbus(property, name = "GPUs")]
    async fn gpus(&self) -> Vec<GpuDevice> {
        self.gpus_cache.read().await.clone()
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let gpus_cache = Arc::new(RwLock::new(Vec::new()));
    let server = SwitcherooServer {
        gpus_cache: gpus_cache.clone(),
    };

    let connection = match Builder::system()?
        .serve_at("/net/hadess/SwitcherooControl", server)?
        .build()
        .await
    {
        Ok(conn) => conn,
        Err(zbus::Error::NameTaken) => {
            eprintln!("Switcheroo daemon is already running");
            exit(0)
        }
        Err(e) => return Err(e.into()),
    };

    // Set AllowReplacement flag so the daemon can be hot-swapped
    connection
        .request_name_with_flags(
            "net.hadess.SwitcherooControl",
            RequestNameFlags::AllowReplacement | RequestNameFlags::ReplaceExisting,
        )
        .await?;

    // Initial hardware scan
    update_gpu_cache(&gpus_cache).await;
    println!("Switcheroo Daemon running...");

    // Monitor cards
    let (tx, rx) = mpsc::channel::<()>(16);
    let udev_handle = tokio::spawn(run_udev_monitor(tx));
    let event_handle = tokio::spawn(handle_hardware_events(rx, gpus_cache, connection));

    // Keep daemon alive until shutdown signal
    wait_for_shutdown().await?;
    println!("Received termination signal. Shutting down...");

    udev_handle.abort();
    event_handle.abort();

    let _ = udev_handle.await;
    let _ = event_handle.await;

    println!("Switcheroo Daemon stopped.");
    Ok(())
}

/// Scans DRM cards and updates the shared cache if changes occurred.
/// Returns `true` if the cache was modified.
async fn update_gpu_cache(cache_lock: &RwLock<Vec<GpuDevice>>) -> bool {
    let new_cards = match tokio::task::spawn_blocking(scan_drm_cards).await {
        Ok(cards) => cards,
        Err(e) => {
            eprintln!("scan_drm_cards panicked: {e}");
            return false;
        }
    };

    let mut cache = cache_lock.write().await;
    let has_changed = *cache != new_cards;

    if has_changed {
        *cache = new_cards;
    }

    has_changed
}

/// Notifies D-Bus clients that GPU properties have changed
async fn emit_gpu_signals(connection: &zbus::Connection) -> zbus::Result<()> {
    let object_server = connection.object_server();
    let iface_ref = object_server
        .interface::<_, SwitcherooServer>("/net/hadess/SwitcherooControl")
        .await?;

    let ctxt = iface_ref.signal_context();
    let server = iface_ref.get().await;

    let _ = server.g_p_us_changed(ctxt).await;
    let _ = server.num_g_p_us_changed(ctxt).await;
    let _ = server.has_dual_gpu_changed(ctxt).await;
    Ok(())
}

/// Event loop that processes signals from the udev monitor thread
async fn handle_hardware_events(
    mut rx: mpsc::Receiver<()>,
    cache: Arc<RwLock<Vec<GpuDevice>>>,
    connection: zbus::Connection,
) {
    while rx.recv().await.is_some() {
        if update_gpu_cache(&cache).await {
            if let Err(e) = emit_gpu_signals(&connection).await {
                eprintln!("Failed to emit D-Bus signals: {e}");
            } else {
                println!("Hardware event processed. GPUs updated.");
            }
        }
    }
    println!("GPU sync task shut down.");
}

/// Non-blocking worker thread dedicated to watching Linux udev events
async fn run_udev_monitor(tx: mpsc::Sender<()>) -> color_eyre::Result<()> {
    let monitor = udev::MonitorBuilder::new()?
        .match_subsystem("drm")?
        .listen()?;

    let mut async_fd = AsyncFd::new(monitor)?;

    loop {
        let mut guard = async_fd.readable_mut().await?;
        guard.clear_ready();

        for _event in guard.get_inner_mut().iter() {
            if tx.send(()).await.is_err() {
                return Ok(());
            }
        }
    }
}

/// Blocks until a system termination signal is received
async fn wait_for_shutdown() -> color_eyre::Result<()> {
    let mut term = signal(SignalKind::terminate())?;
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {},
        _ = term.recv() => {},
    }
    Ok(())
}
