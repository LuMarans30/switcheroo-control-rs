// SPDX-License-Identifier: GPL-3.0-or-later

use std::os::fd::RawFd;

#[repr(C)]
struct DrmI915GetParam {
    param: i32,
    value: *mut i32,
}

// DRM_IOCTL_BASE is 'd' (0x64)
// DRM_I915_GETPARAM is 0x06
// 0x40 (DRM_COMMAND_BASE) + 0x06 = 0x46
nix::ioctl_readwrite!(ioctl_i915_getparam, b'd', 0x46, DrmI915GetParam);

const I915_PARAM_HAS_LMEM: i32 = 55;

pub fn probe_fd(fd: RawFd) -> Result<bool, nix::Error> {
    let mut value = 0i32;
    let mut request = DrmI915GetParam {
        param: I915_PARAM_HAS_LMEM,
        value: &raw mut value,
    };

    unsafe { ioctl_i915_getparam(fd, &raw mut request)? };

    Ok(value > 0)
}
