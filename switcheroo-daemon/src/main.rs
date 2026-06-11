// SPDX-License-Identifier: GPL-3.0-or-later

use zbus::{connection::Builder, interface};

use std::sync::Arc;
use tokio::sync::RwLock;

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
        let cache = self.gpus_cache.read().await;
        cache.len() >= 2
    }

    #[zbus(property, name = "NumGPUs")]
    async fn num_gpus(&self) -> u32 {
        let cache = self.gpus_cache.read().await;
        cache.len() as u32
    }

    #[zbus(property, name = "GPUs")]
    async fn gpus(&self) -> Vec<GpuDevice> {
        self.gpus_cache.read().await.clone()
    }
}

async fn sync_gpu_state(
    cache_ref: &Arc<RwLock<Vec<GpuDevice>>>,
    connection: &zbus::Connection,
    emit_signals: bool,
) {
    let new_cards = match tokio::task::spawn_blocking(scan_drm_cards).await {
        Ok(cards) => cards,
        Err(e) => {
            eprintln!("scan_drm_cards panicked: {}", e);
            return;
        }
    };

    let mut cache = cache_ref.write().await;

    if *cache == new_cards {
        return;
    }

    *cache = new_cards;

    if emit_signals {
        let object_server = connection.object_server();
        if let Ok(iface_ref) = object_server
            .interface::<_, SwitcherooServer>("/net/hadess/SwitcherooControl")
            .await
        {
            let ctxt = iface_ref.signal_context();
            let server = iface_ref.get().await;

            let _ = server.g_p_us_changed(ctxt).await;
            let _ = server.num_g_p_us_changed(ctxt).await;
            let _ = server.has_dual_gpu_changed(ctxt).await;

            println!("Hardware event processed. GPUs updated.");
        }
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let gpus_cache = Arc::new(RwLock::new(Vec::new()));

    let server = SwitcherooServer {
        gpus_cache: gpus_cache.clone(),
    };

    let connection = Builder::system()?
        .name("net.hadess.SwitcherooControl")?
        .serve_at("/net/hadess/SwitcherooControl", server)?
        .build()
        .await?;

    sync_gpu_state(&gpus_cache, &connection, false).await;
    println!("Switcheroo Daemon running...");

    let cache_clone = gpus_cache.clone();
    let conn_clone = connection.clone();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(2);

    tokio::task::spawn_blocking(move || {
        let builder = udev::MonitorBuilder::new().expect("Failed to create udev builder");
        let builder = builder.match_subsystem("drm").expect("Failed to match drm");
        let monitor = builder.listen().expect("Failed to listen to udev");

        for _event in monitor.iter() {
            if tx.blocking_send(()).is_err() {
                break;
            }
        }
    });

    tokio::spawn(async move {
        while let Some(()) = rx.recv().await {
            sync_gpu_state(&cache_clone, &conn_clone, true).await;
        }
    });

    std::future::pending::<()>().await;
    Ok(())
}
