// SPDX-License-Identifier: GPL-3.0-or-later

use switcheroo_common::EnvVar;

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
        (Some(v), Some(p)) => format!("{} {}", v, p),
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
/// Scans the system for all DRM cards and builds our cache
pub fn scan_drm_cards() -> Vec<GpuDevice> {
    let mut cards = Vec::new();

    let mut enumerator = match udev::Enumerator::new() {
        Ok(e) => e,
        Err(_) => return cards,
    };

    if enumerator.match_subsystem("drm").is_err() {
        return cards;
    }

    let devices = match enumerator.scan_devices() {
        Ok(iter) => iter,
        Err(_) => return cards,
    };

    for device in devices {
        let devnode = device.devnode().and_then(|n| n.to_str()).unwrap_or("");
        if !devnode.starts_with("/dev/dri/render") {
            continue;
        }

        let parent = match device.parent() {
            Some(p) => p,
            None => continue,
        };

        let env = get_card_env(&device, &parent);

        let default = parent
            .attribute_value("boot_vga")
            .and_then(|s| s.to_str())
            .map(|s| s.trim() == "1")
            .unwrap_or(false);

        let discrete = device
            .property_value("TAGS")
            .and_then(|s| s.to_str())
            .map(|tags| tags.contains(":switcheroo-discrete-gpu:"))
            .unwrap_or(false);

        cards.push(GpuDevice {
            name: get_card_name(&parent),
            default,
            discrete,
            environment: env,
        });
    }

    // Single GPU fallback
    if cards.len() == 1 {
        cards[0].default = true;
    }

    cards.sort_by_key(|b| std::cmp::Reverse(b.default));

    cards
}
