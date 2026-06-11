# Switcheroo Rust Port (WIP)

A WIP Rust port of the original [switcheroo-control daemon and CLI tool](https://gitlab.freedesktop.org/hadess/switcheroo-control) by [Bastien Nocera](https://gitlab.freedesktop.org/hadess).

From a [switcheroo-control man page](https://linuxcommandlibrary.com/man/switcheroo-control):
> switcherooctl is the command-line interface for switcheroo-control, a daemon that manages hybrid graphics on Linux laptops with multiple GPUs. It provides a simple way to list available graphics adapters and launch applications on a specific GPU.
> 
> On hybrid graphics systems with both integrated (power-efficient) and discrete (high-performance) GPUs, applications default to the integrated GPU. Using switcherooctl, you can run specific applications on the discrete GPU for better graphics performance.
> 
> The underlying daemon interfaces with the kernel's vga_switcheroo subsystem and provides a D-Bus API that desktop environments like GNOME and KDE use for GUI-based GPU selection.
> 
> Setting the environment variable DRI_PRIME=1 achieves a similar effect for individual applications.

## Crates

| Crate               | Description                                                                         |
| ------------------- | ----------------------------------------------------------------------------------- |
| `switcheroo-daemon` | Reads GPU hardware and exposes available GPUs over the D-Bus system bus             |
| `switcherooctl`     | A CLI tool to list GPUs and launch applications using a specific GPU                |
| `switcheroo-common` | A shared library containing common types and D-Bus definitions for both components. |

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

### Example

Launch glmark2 on a discrete GPU:
```bash
./target/release/switcherooctl launch --gpu 1 glmark2 -b refract
```

## License

The project is licensed under `GPL-3.0-or-later` to match upstream.