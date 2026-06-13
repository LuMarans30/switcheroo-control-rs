// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{Parser, Subcommand};
use color_eyre::{Result, eyre::eyre};
use switcheroo_common::GpuDevice;

mod client;
use client::SwitcherooProxy;
use zbus::proxy::ProxyDefault;

use std::os::unix::process::CommandExt;
use std::process::{Command, exit};

#[derive(Parser)]
#[command(version, about = "Switcheroo CLI utility")]
struct Cli {
    #[arg(short = 'g', long = "gpu")]
    gpu: Option<u32>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List the known GPUs
    List,
    /// Launch a command on a specific GPU
    #[command(trailing_var_arg = true, allow_hyphen_values = true)]
    Launch {
        #[arg(short = 'g', long = "gpu")]
        gpu: Option<u32>,
        #[arg(required = true)]
        args: Vec<String>,
    },
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let connection = zbus::Connection::system().await?;
    let gpus = fetch_gpu_devices(&connection).await?;

    match cli.command.unwrap_or(Commands::List) {
        Commands::List => list_gpus(&gpus),
        Commands::Launch { gpu, args } => launch_on_gpu(&gpus, gpu.or(cli.gpu), &args)?,
        Commands::External(args) => launch_on_gpu(&gpus, cli.gpu, &args)?,
    }

    Ok(())
}

/// Connects to the switcheroo proxy and fetches devices, handling daemon errors gracefully
async fn fetch_gpu_devices(connection: &zbus::Connection) -> Result<Vec<GpuDevice>> {
    let proxy = SwitcherooProxy::new(connection).await?;

    proxy
        .gpus()
        .await
        .map_err(|e| match zbus::fdo::Error::from(e) {
            zbus::fdo::Error::ServiceUnknown(_) => {
                eyre!("The switcheroo daemon is not running or installed.")
            }
            zbus::fdo::Error::NoReply(_) | zbus::fdo::Error::Timeout(_) => {
                let service_name = SwitcherooProxy::DESTINATION.unwrap_or("switcheroo");
                eyre!("Service '{service_name}' is registered but failed to respond.")
            }
            err => err.into(),
        })
}

/// Lists all the available GPUs
fn list_gpus(gpus: &[GpuDevice]) {
    for (idx, gpu) in gpus.iter().enumerate() {
        println!("Device: {idx}\n{gpu}\n");
    }
}

/// Launches a program using the specified GPU
fn launch_on_gpu(gpus: &[GpuDevice], gpu: Option<u32>, args: &[String]) -> Result<()> {
    if gpus.is_empty() {
        return Err(eyre!("No GPUs found on the system."));
    }

    let gpu_idx = gpu.map_or_else(
        || gpus.iter().position(|g| g.discrete).unwrap_or(0),
        |id| id as usize,
    );

    let target_gpu = gpus
        .get(gpu_idx)
        .ok_or_else(|| eyre!("GPU index {} not found", gpu_idx))?;

    let mut cmd = Command::new(&args[0]);
    if args.len() > 1 {
        cmd.args(&args[1..]);
    }

    target_gpu.apply_env(&mut cmd);

    let err = cmd.exec();
    eprintln!("switcherooctl: failed to execute '{}': {}", args[0], err);
    exit(1)
}
