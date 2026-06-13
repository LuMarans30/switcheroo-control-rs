// SPDX-License-Identifier: GPL-3.0-or-later

use std::os::fd::RawFd;

#[repr(C)]
struct NvifIoctlV0 {
    object: u64,
    owner: u8,
    route: u8,
    version: u8,
    kind: u8,
    pad: u32,
}

#[repr(C)]
struct NvifIoctlNewV0 {
    handle: u64,
    object: u64,
    oclass: u16,
    route: u8,
    version: u8,
    pad: u32,
    token: u64,
}

#[repr(C)]
struct NvDeviceV0 {
    device: u64,
}

#[repr(C)]
struct NouveauInitArgs {
    ioctl: NvifIoctlV0,
    new: NvifIoctlNewV0,
    dev: NvDeviceV0,
}

#[repr(C)]
struct NvifIoctlMthdV0 {
    method: u16,
    version: u8,
    pad: [u8; 5],
}

#[repr(C)]
#[derive(Default)]
struct NvDeviceInfoV0 {
    version: u8,
    platform: u8,
    chipset: u16,
    revision: u8,
    class: u8,
    pad06: [u8; 2],
    stream: [u32; 3],
    pad14: u32,
    memory: u64,
    pad30: [u8; 16],
}

#[repr(C)]
struct NouveauQueryArgs {
    ioctl: NvifIoctlV0,
    mthd: NvifIoctlMthdV0,
    info: NvDeviceInfoV0,
}

const _: () = {
    use std::mem::size_of;
    assert!(size_of::<NvifIoctlV0>() == 16);
    assert!(size_of::<NvifIoctlNewV0>() == 32);
    assert!(size_of::<NvDeviceV0>() == 8);
    assert!(size_of::<NouveauInitArgs>() == 56);
    assert!(size_of::<NvifIoctlMthdV0>() == 8);
    assert!(size_of::<NvDeviceInfoV0>() == 48);
    assert!(size_of::<NouveauQueryArgs>() == 72);
};

nix::ioctl_write_ptr!(ioctl_nouveau_init, b'd', 0x47, NouveauInitArgs);
nix::ioctl_readwrite!(ioctl_nouveau_query, b'd', 0x47, NouveauQueryArgs);

const NVIF_IOCTL_V0_NEW: u8 = 0x08;
const NVIF_IOCTL_V0_MTHD: u8 = 0x06;
const NVIF_IOCTL_V0_OWNER_ANY: u8 = 0x00;
const NVIF_IOCTL_V0_ROUTE_NVIF: u8 = 0xff;
const NV_DEVICE: u16 = 0x0080;
const NV_DEVICE_V0_INFO: u16 = 0x0000;

const NV_DEVICE_INFO_V0_IGP: u8 = 0x00;
const NV_DEVICE_INFO_V0_SOC: u8 = 0x04;

pub fn probe_fd(fd: RawFd) -> Result<bool, nix::Error> {
    let mut token = 0u8;
    let object = std::ptr::addr_of_mut!(token) as u64;

    let init_args = NouveauInitArgs {
        ioctl: NvifIoctlV0 {
            object: 0,
            owner: NVIF_IOCTL_V0_OWNER_ANY,
            route: 0x00,
            version: 0,
            kind: NVIF_IOCTL_V0_NEW,
            pad: 0,
        },
        new: NvifIoctlNewV0 {
            handle: 0,
            object,
            oclass: NV_DEVICE,
            route: NVIF_IOCTL_V0_ROUTE_NVIF,
            version: 0,
            pad: 0,
            token: object,
        },
        dev: NvDeviceV0 { device: !0u64 },
    };

    unsafe { ioctl_nouveau_init(fd, &raw const init_args)? };

    let mut query_args = NouveauQueryArgs {
        ioctl: NvifIoctlV0 {
            object,
            owner: NVIF_IOCTL_V0_OWNER_ANY,
            route: 0x00,
            version: 0,
            kind: NVIF_IOCTL_V0_MTHD,
            pad: 0,
        },
        mthd: NvifIoctlMthdV0 {
            method: NV_DEVICE_V0_INFO,
            version: 0,
            pad: [0; 5],
        },
        info: NvDeviceInfoV0::default(),
    };

    unsafe { ioctl_nouveau_query(fd, &raw mut query_args)? };

    Ok(!matches!(
        query_args.info.platform,
        NV_DEVICE_INFO_V0_IGP | NV_DEVICE_INFO_V0_SOC
    ))
}
