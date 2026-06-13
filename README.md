## Switcheroo Rust Port

A Rust port of the original [switcheroo-control daemon and CLI tool](https://gitlab.freedesktop.org/hadess/switcheroo-control) by [Bastien Nocera](https://gitlab.freedesktop.org/hadess).

From a [switcheroo-control man page](https://linuxcommandlibrary.com/man/switcheroo-control):

> switcherooctl is the command-line interface for switcheroo-control, a daemon that manages hybrid graphics on Linux laptops with multiple GPUs. It provides a simple way to list available graphics adapters and launch applications on a specific GPU.
> 
> On hybrid graphics systems with both integrated (power-efficient) and discrete (high-performance) GPUs, applications default to the integrated GPU. Using switcherooctl, you can run specific applications on the discrete GPU for better graphics performance.

This port uses the [`udev` crate](https://github.com/Smithay/udev-rs) for GPU monitoring, combined with the [`nix` crate](https://github.com/nix-rust/nix) for low-level ioctl driver queries (amdgpu, i915, nouveau, and xe drivers) to determine whether a card is discrete or integrated.

It reimplements the standard `net.hadess.SwitcherooControl` interface using the [`zbus` crate](https://github.com/z-galaxy/zbus). 

`switcherooctl` provides a user-friendly CLI via the [`clap` crate](https://github.com/clap-rs/clap), including implicit subcommands which are also present in the original project. The daemon likewise uses `clap` for its own `--replace` flag.

Also, just like the original project, it includes a regex-based cleanup module to prettify GPU information before it is published over D-Bus

### Workspace Members

| Crate               | Description                                                                |
| ------------------- | -------------------------------------------------------------------------- |
| `switcheroo-daemon` | Background system service that monitors hardware and exposes it over D-Bus |
| `switcherooctl`     | CLI utility used to list GPUs and launch binaries using a specific GPU     |
| `switcheroo-common` | Shared types, serialization/deserialization logic and D-Bus definitions    |

### Architecture

```mermaid
flowchart TB
    User((User))

    subgraph Userspace [Userspace Context]
        CLI(switcherooctl)
        Daemon(switcheroo-daemon)
        DBus{{D-Bus System Bus}}
        TargetApp[[Target Application]]
    end

    subgraph OS_Kernel [OS & Kernel Interface]
        subgraph UdevSub [udev Subsystem]
            Udev[udev crate]
            LibUdev[libudev C FFI]
        end

        subgraph IOCTL_Sub [DRM Subsystem]
            DRM_Nodes[(/dev/dri/render*)]
            Drivers[\amdgpu, i915, xe, nouveau, nvidia/]
        end
    end

    User -->|Runs| CLI
    
    CLI -->|Queries GPUs| DBus
    DBus <-->|net.hadess.SwitcherooControl| Daemon
    DBus -->|GPU info & env| CLI
    
    CLI -.->|Spawns with env vars| TargetApp
    
    Daemon -->|Queries GPUs & watch events| Udev
    Udev -->|GPUs info & events| Daemon
    
    Daemon -->|switcheroo-discrete-gpu tag check| LibUdev
    LibUdev -->|is_discrete| Daemon
    
    Daemon -->|IOCTL probe| DRM_Nodes
    DRM_Nodes -->|VRAM / LMEM status| Daemon
    
    DRM_Nodes --- Drivers
    TargetApp -.->|Render offload| Drivers
```

### Usage

Ensure you have the `libudev` library installed where it can be found by `pkg-config`. 

Example:

```bash
sudo apt-get install libudev-dev    # Debian-based Linux distributions
sudo zypper install systemd-devel   # openSUSE/SLE
```

Compile a release build using [Cargo](https://rustup.rs/):

```bash
cargo build --release
```

To test the daemon locally, stop the existing system service first to avoid D-Bus conflicts:

```bash
sudo systemctl stop switcheroo-control
sudo ./target/release/switcheroo-daemon
```

> [!TIP]
> The daemon supports a `--replace` flag to replace an already running instance. 
> Without it, the daemon will exit immediately if another instance holds the bus name.

You can now use the CLI tool:

```bash
./target/release/switcherooctl help
```

```bash
Usage: switcherooctl [OPTIONS] [COMMAND]

Commands:
  list    List the known GPUs
  launch  Launch a command on a specific GPU
  help    Print this message or the help of the given subcommand(s)

Options:
  -g, --gpu <GPU>
  -h, --help       Print help
```

### Example

Launch the glmark2's refract benchmark on a discrete GPU:

```bash
./target/release/switcherooctl launch --gpu 1 glmark2 -b refract
```

For simplicity, you can also omit subcommands:

```bash
./target/release/switcherooctl            # "switcherooctl list"
./target/release/switcherooctl <program>  # "switcherooctl launch <program>" 
```

### License

The project is licensed under `GPL-3.0-or-later` to match upstream.
