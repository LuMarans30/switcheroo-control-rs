// SPDX-License-Identifier: GPL-3.0-or-later

use std::os::fd::RawFd;

#[repr(C)]
struct DrmAmdgpuInfo {
    return_pointer: u64,
    return_size: u32,
    query: u32,
    _reserved: [u32; 16],
}

#[repr(C)]
#[derive(Default)]
struct DrmAmdgpuInfoDevice {
    device_id: u32,
    chip_rev: u32,
    external_rev: u32,
    pci_rev: u32,
    family: u32,
    num_shader_engines: u32,
    num_shader_arrays_per_engine: u32,
    gpu_counter_freq: u32,
    max_engine_clock: u64,
    max_memory_clock: u64,
    cu_active_number: u32,
    cu_ao_mask: u32,
    cu_bitmap: [u32; 16],
    enabled_rb_pipes_mask: u32,
    num_rb_pipes: u32,
    num_hw_gfx_contexts: u32,
    _pad: u32,
    ids_flags: u64,
}

nix::ioctl_write_ptr!(ioctl_amdgpu_info, b'd', 0x45, DrmAmdgpuInfo);

const AMDGPU_INFO_DEV_INFO: u32 = 0x16;
const AMDGPU_IDS_FLAGS_FUSION: u64 = 1 << 0;

pub fn probe_fd(fd: RawFd) -> Result<bool, nix::Error> {
    let mut device_info = DrmAmdgpuInfoDevice::default();

    let request = DrmAmdgpuInfo {
        return_pointer: &raw mut device_info as u64,
        #[allow(clippy::cast_possible_truncation)]
        return_size: std::mem::size_of::<DrmAmdgpuInfoDevice>() as u32,
        query: AMDGPU_INFO_DEV_INFO,
        _reserved: [0; 16],
    };

    unsafe { ioctl_amdgpu_info(fd, &raw const request)? };

    Ok(device_info.ids_flags & AMDGPU_IDS_FLAGS_FUSION == 0)
}
