// SPDX-License-Identifier: GPL-3.0-or-later

use std::os::fd::RawFd;

#[repr(C)]
#[derive(Default)]
struct DrmXeDeviceQuery {
    extensions: u64,
    query: u32,
    size: u32,
    data: u64,
    reserved: [u64; 2],
}

nix::ioctl_readwrite!(ioctl_xe_device_query, b'd', 0x40, DrmXeDeviceQuery);

const DRM_XE_DEVICE_QUERY_CONFIG: u32 = 2;
const DRM_XE_QUERY_CONFIG_FLAG_HAS_VRAM: u64 = 1 << 0;
const CONFIG_FLAGS_OFFSET: usize = 16;

pub fn probe_fd(fd: RawFd) -> Result<bool, nix::Error> {
    let mut query = DrmXeDeviceQuery {
        query: DRM_XE_DEVICE_QUERY_CONFIG,
        ..Default::default()
    };

    unsafe { ioctl_xe_device_query(fd, &mut query)? };

    let size = query.size as usize;
    if size < CONFIG_FLAGS_OFFSET + std::mem::size_of::<u64>() {
        return Ok(false);
    }

    let mut buf = vec![0u8; size];
    query.data = buf.as_mut_ptr() as u64;

    unsafe { ioctl_xe_device_query(fd, &mut query)? };

    let flags = u64::from_ne_bytes(
        buf[CONFIG_FLAGS_OFFSET..CONFIG_FLAGS_OFFSET + 8]
            .try_into()
            .unwrap_or([0; 8]),
    );

    Ok(flags & DRM_XE_QUERY_CONFIG_FLAG_HAS_VRAM != 0)
}
