// SPDX-License-Identifier: GPL-3.0-or-later

use color_eyre::Result;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

mod amdgpu;
mod i915;
mod nouveau;
mod xe;

/// Determines whether the GPU at `devnode` is a discrete GPU (has dedicated VRAM)
pub fn probe(devnode: &str, driver: &str) -> Result<bool> {
    // The proprietary NVIDIA driver lacks standard DRM ioctls
    // Also NVIDIA does not make integrated x86 GPUs
    if driver == "nvidia" {
        return Ok(true);
    }

    let file = OpenOptions::new().read(true).write(true).open(devnode)?;
    let fd = file.as_raw_fd();

    let is_discrete = match driver {
        "amdgpu" => amdgpu::probe_fd(fd)?,
        "xe" => xe::probe_fd(fd)?,
        "nouveau" => nouveau::probe_fd(fd)?,
        "i915" => i915::probe_fd(fd)?,
        _ => false,
    };

    Ok(is_discrete)
}
