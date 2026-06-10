// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{Parser, Subcommand};
use color_eyre::{Result, eyre::eyre};
use switcheroo_common::GpuDevice;

mod client;
use client::SwitcherooProxy;

use std::os::unix::process::CommandExt;
use std::process::Command;
use which::which;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List the known GPUs
    List,
    /// Launch a command on a specific GPU
    #[command(trailing_var_arg = true, allow_hyphen_values = true)]
    Launch {
        /// The GPU to launch on
        #[arg(short = 'g', long = "gpu")]
        gpu: Option<u32>,
        /// Command and its args to launch
        #[arg(required = true)]
        args: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    validate_args(&cli.command)?;

    let connection = zbus::Connection::system().await?;
    let proxy = SwitcherooProxy::new(&connection).await?;

    let gpus: Vec<GpuDevice> = proxy.gpus().await?;

    match &cli.command {
        Commands::List => cmd_list(&gpus),
        Commands::Launch { gpu, args } => cmd_launch(&gpus, *gpu, args),
    }
}

/// Lists all the available GPUs
fn cmd_list(gpus: &[GpuDevice]) -> Result<()> {
    for (idx, gpu) in gpus.iter().enumerate() {
        println!("Device: {}\n{}\n", idx, gpu);
    }
    Ok(())
}

/// Launches a program using the specified GPU
fn cmd_launch(gpus: &[GpuDevice], gpu: Option<u32>, args: &[String]) -> Result<()> {
    let gpu_idx = match gpu {
        Some(id) => id as usize,
        None => {
            if gpus.is_empty() {
                return Err(eyre!("No GPUs found on the system."));
            }
            gpus.iter().position(|g| g.discrete).unwrap_or(0)
        }
    };

    let target_gpu = gpus
        .get(gpu_idx)
        .ok_or_else(|| eyre!("GPU index {} not found", gpu_idx))?;

    let mut cmd = Command::new(&args[0]);
    if args.len() > 1 {
        cmd.args(&args[1..]);
    }

    target_gpu.apply_env(&mut cmd);

    let err = cmd.exec();
    Err(eyre!("Failed to execute process: {}", err))
}

fn validate_args(command: &Commands) -> Result<()> {
    if let Commands::Launch { args, .. } = command {
        which(&args[0]).map_err(|_| eyre!("Command not found in PATH: '{}'", args[0]))?;
    }
    Ok(())
}
