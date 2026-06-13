# switcherooctl

A CLI utility used to list GPUs and launch binaries using a specific GPU.
It can use the [`switcheroo-daemon`](https://crates.io/crates/switcheroo-daemon) crate or the [original `switcheroo-control` service](https://gitlab.freedesktop.org/hadess/switcheroo-control) as its backend.

## Usage

```bash
# List available GPUs
switcherooctl list

# Launch a binary on a specific GPU
switcherooctl launch --gpu <gpu_id> <command>
```

## Docs
Please see the [project repository README](https://github.com/LuMarans30/switcheroo-control-rs) for more information.