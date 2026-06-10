# Switcheroo Rust Port

A WIP Rust port of the original [switcheroo-control daemon and CLI tool](https://gitlab.freedesktop.org/hadess/switcheroo-control) by [Bastien Nocera](https://gitlab.freedesktop.org/hadess).

This project implements the `net.hadess.SwitcherooControl` D-Bus interface, allowing DEs like KDE Plasma to offload applications to a specific GPU in multi-GPU systems.

## Features

- Daemon (`switcheroo-daemon`): Reads GPU hardware and exposes available GPUs over the D-Bus system bus
- Client CLI (`switcherooctl`): A CLI tool to list GPUs and launch applications using a specific GPU

## Usage

Compile a release build using [Cargo](https://rustup.rs/):
```bash
cargo build --release
```

Start the daemon as root:
```bash
sudo -b ./target/release/switcheroo-daemon &
```

You can now use the CLI tool:
```bash
./target/release/switcherooctl help
```

```bash
Usage: switcherooctl <COMMAND>

Commands:
  list    List the known GPUs
  launch  Launch a command on a specific GPU
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## License

The project is licensed under `GPL-3.0-or-later` to match upstream.