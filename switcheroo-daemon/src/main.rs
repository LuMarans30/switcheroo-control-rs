// SPDX-License-Identifier: GPL-3.0-or-later

use color_eyre::{Result, eyre::eyre};
use zbus::{
    Connection,
    connection::Builder,
    fdo::{DBusProxy, Properties, RequestNameFlags, RequestNameReply},
    interface,
    object_server::Interface,
    zvariant::Value,
};

use clap::Parser;
use std::{collections::HashMap, process::exit, sync::Arc, time::Duration};
use tokio::{
    io::unix::AsyncFd,
    signal::unix::{SignalKind, signal},
    sync::{
        RwLock,
        mpsc::{self, Sender},
    },
};

use crate::detection::scan_drm_cards;

use futures_lite::stream::StreamExt;

mod detection;
mod helpers;
mod info_cleanup;

use switcheroo_common::GpuDevice;

const DBUS_NAME: &str = "net.hadess.SwitcherooControl";
const DBUS_PATH: &str = "/net/hadess/SwitcherooControl";

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
    #[allow(clippy::cast_possible_truncation)]
    async fn num_gpus(&self) -> u32 {
        self.gpus_cache.read().await.len() as u32
    }

    #[zbus(property, name = "GPUs")]
    async fn gpus(&self) -> Vec<GpuDevice> {
        self.gpus_cache.read().await.clone()
    }
}

#[derive(Parser)]
#[command(version, about = "Switcheroo D-Bus daemon")]
struct Cli {
    /// Replace an already running instance of the daemon
    #[arg(short, long)]
    replace: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let Cli { replace } = Cli::parse();

    let gpus_cache = Arc::new(RwLock::new(Vec::new()));
    let server = SwitcherooServer {
        gpus_cache: gpus_cache.clone(),
    };

    let connection = Builder::system()?
        .serve_at(DBUS_PATH, server)?
        .build()
        .await?;

    let mut name_flags = RequestNameFlags::DoNotQueue | RequestNameFlags::AllowReplacement;
    if replace {
        name_flags |= RequestNameFlags::ReplaceExisting;
    }

    // Enforce a single deamon instance
    let reply = connection
        .request_name_with_flags(DBUS_NAME, name_flags)
        .await?;

    if matches!(reply, RequestNameReply::InQueue | RequestNameReply::Exists) {
        eprintln!("Switcheroo daemon is already running (name taken). Exiting gracefully.");
        exit(0);
    }

    // Initial hardware scan
    update_gpu_cache(&gpus_cache).await;
    println!("Switcheroo Daemon running...");

    // Monitor cards
    let (tx, rx) = mpsc::channel::<()>(16);
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);

    let udev_handle = tokio::spawn(run_udev_monitor(tx));
    let event_handle = tokio::spawn(handle_hardware_events(rx, gpus_cache, connection.clone()));
    let replacement_handle = tokio::spawn(handle_replacement(connection.clone(), shutdown_tx));

    // Keep daemon alive until shutdown signal or replacement
    let shut_reason = tokio::select! {
        _ = wait_for_shutdown() => {
            "Received termination signal"
        }
        _ = shutdown_rx.recv() => {
            "Replaced by another instance"
        }
    };

    println!("{shut_reason}. Shutting down...");

    udev_handle.abort();
    event_handle.abort();
    replacement_handle.abort();

    let _ = udev_handle.await;
    let _ = event_handle.await;
    let _ = replacement_handle.await;

    println!("Switcheroo Daemon stopped.");
    Ok(())
}

/// Handle replacement by another instance safely
async fn handle_replacement(connection: Connection, shutdown: Sender<()>) -> Result<()> {
    let dbus_proxy = DBusProxy::new(&connection).await?;
    let mut owner_changes = dbus_proxy.receive_name_owner_changed().await?;
    let unique_name = connection
        .unique_name()
        .ok_or_else(|| eyre!("Connection has no unique name"))?
        .to_owned();

    while let Some(signal) = owner_changes.next().await {
        let Ok(args) = signal.args() else { continue };

        if args.name().as_str() != DBUS_NAME {
            continue;
        }

        if let Some(new_owner) = args.new_owner().as_deref()
            && new_owner != unique_name.as_str()
        {
            let _ = shutdown.send(()).await;
            break;
        }
    }

    Ok(())
}

/// Scans DRM cards and updates the shared cache if changes occurred
async fn update_gpu_cache(cache_lock: &RwLock<Vec<GpuDevice>>) -> Option<Vec<GpuDevice>> {
    let new_cards = match tokio::task::spawn_blocking(scan_drm_cards).await {
        Ok(cards) => cards,
        Err(e) => {
            eprintln!("scan_drm_cards panicked: {e}");
            return None;
        }
    };

    let mut cache = cache_lock.write().await;

    (*cache != new_cards).then(|| {
        cache.clone_from(&new_cards);
        new_cards
    })
}

/// Emits a `PropertiesChanged` signal to notify D-Bus clients of GPU updates
async fn emit_gpu_signal(connection: &Connection, gpus: Vec<GpuDevice>) -> Result<()> {
    let object_server = connection.object_server();
    let iface_ref = object_server
        .interface::<_, SwitcherooServer>(DBUS_PATH)
        .await?;

    let emitter = iface_ref.signal_context();

    let num_gpus: u32 = gpus.len().try_into()?;
    let has_dual_gpu = num_gpus >= 2;

    Properties::properties_changed(
        emitter,
        SwitcherooServer::name(),
        &HashMap::from([
            ("GPUs", &Value::from(gpus)),
            ("NumGPUs", &Value::from(num_gpus)),
            ("HasDualGpu", &Value::from(has_dual_gpu)),
        ]),
        &[],
    )
    .await?;

    Ok(())
}

/// Event loop that processes signals from the udev monitor thread with debouncing
async fn handle_hardware_events(
    mut rx: mpsc::Receiver<()>,
    cache: Arc<RwLock<Vec<GpuDevice>>>,
    connection: Connection,
) {
    while rx.recv().await.is_some() {
        // Debounce
        tokio::time::sleep(Duration::from_millis(50)).await;
        while rx.try_recv().is_ok() {}

        if let Some(new_gpus) = update_gpu_cache(&cache).await {
            if let Err(e) = emit_gpu_signal(&connection, new_gpus).await {
                eprintln!("Failed to emit D-Bus signal: {e}");
            } else {
                println!("Hardware event processed. GPUs updated.");
            }
        }
    }
    println!("GPU sync task shut down.");
}

/// Non-blocking worker thread dedicated to watching Linux udev events
async fn run_udev_monitor(tx: mpsc::Sender<()>) -> Result<()> {
    let monitor = udev::MonitorBuilder::new()?
        .match_subsystem("drm")?
        .listen()?;

    let mut async_fd = AsyncFd::new(monitor)?;

    loop {
        let mut guard = async_fd.readable_mut().await?;

        let mut event_count = 0;
        for _event in guard.get_inner_mut().iter() {
            event_count += 1;
        }
        guard.clear_ready();

        drop(guard);

        if event_count > 0 {
            let _ = tx.send(()).await;
        }
    }
}

/// Blocks until a system termination signal is received
async fn wait_for_shutdown() -> Result<()> {
    let mut term = signal(SignalKind::terminate())?;
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {},
        _ = term.recv() => {},
    }
    Ok(())
}
