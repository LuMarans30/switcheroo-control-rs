// SPDX-License-Identifier: GPL-3.0-or-later

use std::{ffi::CString, sync::LazyLock};

use switcheroo_common::EnvVar;
use udev::{AsRawWithContext, ffi::udev_device_has_tag};

use crate::{GpuDevice, info_cleanup::info_cleanup};

/// Extracts a property string from a udev Device
pub fn get_property<'a>(dev: &'a udev::Device, key: &str) -> Option<&'a str> {
    dev.property_value(key).and_then(|s| s.to_str())
}

/// Extracts the GPU's name
pub fn get_card_name(parent: &udev::Device) -> String {
    let vendor = get_property(parent, "SWITCHEROO_CONTROL_VENDOR_NAME")
        .or_else(|| get_property(parent, "ID_VENDOR_FROM_DATABASE"));

    let product = get_property(parent, "SWITCHEROO_CONTROL_PRODUCT_NAME")
        .or_else(|| get_property(parent, "ID_MODEL_FROM_DATABASE"));

    let raw_name = match (vendor, product) {
        (Some(v), Some(p)) => format!("{v} {p}"),
        (Some(v), None) => v.to_string(),
        (None, Some(p)) => p.to_string(),
        (None, None) => "Unknown Graphics Controller".to_string(),
    };

    info_cleanup(&raw_name)
}

/// Generates the correct environment variables for the driver
pub fn get_card_env(dev: &udev::Device, parent: &udev::Device) -> Vec<EnvVar> {
    let mut env = Vec::new();
    let driver = parent.driver().and_then(|s| s.to_str()).unwrap_or("");

    if driver == "nvidia" {
        env.push(EnvVar {
            key: "__GLX_VENDOR_LIBRARY_NAME".into(),
            value: "nvidia".into(),
        });
        env.push(EnvVar {
            key: "__NV_PRIME_RENDER_OFFLOAD".into(),
            value: "1".into(),
        });
        env.push(EnvVar {
            key: "__VK_LAYER_NV_optimus".into(),
            value: "NVIDIA_only".into(),
        });
    } else {
        // For Mesa drivers (AMD/Intel/Nouveau), use DRI_PRIME with the PCI path
        if let Some(id_path) = get_property(dev, "ID_PATH_TAG") {
            env.push(EnvVar {
                key: "DRI_PRIME".into(),
                value: id_path.into(),
            });
        }
    }

    let vk_driver = match driver {
        "amdgpu" | "radeon" => Some("*radeon*"),
        "i915" | "xe" => Some("*intel*"),
        "nvidia" => Some("*nvidia*"),
        _ => None,
    };

    if let Some(vk) = vk_driver {
        env.push(EnvVar {
            key: "VK_LOADER_DRIVERS_SELECT".into(),
            value: vk.into(),
        });
    }

    env
}

/// Scans the system for all DRM cards and builds the cache
pub fn scan_drm_cards() -> Vec<GpuDevice> {
    let mut cards = Vec::new();

    let Ok(mut enumerator) = udev::Enumerator::new() else {
        return cards;
    };

    if enumerator.match_subsystem("drm").is_err() {
        return cards;
    }

    let Ok(devices) = enumerator.scan_devices() else {
        return cards;
    };

    for device in devices {
        let devnode = device.devnode().and_then(|n| n.to_str()).unwrap_or("");
        if !devnode.starts_with("/dev/dri/render") {
            continue;
        }

        let Some(parent) = device.parent() else {
            continue;
        };

        let env = get_card_env(&device, &parent);

        if env.is_empty() {
            continue;
        }

        let default = parent
            .attribute_value("boot_vga")
            .and_then(|s| s.to_str())
            .is_some_and(|s| s.trim() == "1");

        // Extract the driver name to pass to our custom direct ioctl probes
        let driver = parent.driver().and_then(|s| s.to_str()).unwrap_or("");
        let discrete = get_card_is_discrete(&device)
            || crate::helpers::probe(devnode, driver).unwrap_or_else(|e| {
                log::warn!("Failed to probe discrete status for {driver} at {devnode}: {e}");
                false
            });

        cards.push(GpuDevice {
            name: get_card_name(&parent),
            default,
            discrete,
            environment: env,
        });
    }

    // Fallback
    if !cards.is_empty() && (cards.len() == 1 || !cards.iter().any(|c| c.default)) {
        cards[0].default = true;
    }

    cards.sort_by_key(|b| std::cmp::Reverse(b.default));

    cards
}

static DISCRETE_TAG: LazyLock<CString> =
    LazyLock::new(|| CString::new("switcheroo-discrete-gpu").unwrap());

/// Determines whether a given `udev` device represents a discrete GPU
pub fn get_card_is_discrete(dev: &udev::Device) -> bool {
    // Direct FFI is much faster than parsing TAGS property (1.5 µs vs 65 µs)
    unsafe { udev_device_has_tag(dev.as_raw(), DISCRETE_TAG.as_ptr()) == 1 }
}
