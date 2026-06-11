// SPDX-License-Identifier: GPL-3.0-or-later

use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

mod amdgpu;
mod nouveau;
mod xe;

/// Determines whether the GPU at `devnode` (a DRM render node such as
/// `/dev/dri/renderD128`) driven by `driver` is a discrete (dedicated) GPU.
pub fn probe(devnode: &str, driver: &str) -> bool {
    match driver {
        "nvidia" | "radeon" => return true,
        "i915" => return false,
        _ => {}
    }

    let file = match OpenOptions::new().read(true).write(true).open(devnode) {
        Ok(f) => f,
        Err(_) => return false,
    };
    let fd = file.as_raw_fd();

    match driver {
        "amdgpu" => amdgpu::probe_fd(fd).unwrap_or(false),
        "xe" => xe::probe_fd(fd).unwrap_or(false),
        "nouveau" => nouveau::probe_fd(fd).unwrap_or(false),
        _ => false,
    }
}
