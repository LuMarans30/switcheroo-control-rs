# switcheroo-daemon

A background system service that monitors GPU information and exposes it over D-Bus. 
It can be queried by the [`switcherooctl`](https://crates.io/crates/switcherooctl) crate or the [original `switcherooctl` client](https://gitlab.freedesktop.org/hadess/switcheroo-control).

## Usage

```bash
# Run the daemon as root in the background
sudo -b switcheroo-daemon

# Replace an already running instance of `switcheroo-daemon` (or the original `switcheroo-control` service)
sudo switcheroo-daemon --replace
```

## Docs
Please see the [project repository README](https://github.com/LuMarans30/switcheroo-control-rs) for more information.