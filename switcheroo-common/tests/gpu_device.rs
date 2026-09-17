// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

use switcheroo_common::{EnvVar, GpuDevice};
use zbus::zvariant::{Type, Value};

fn sample() -> GpuDevice {
    GpuDevice {
        name: "NVIDIA GeForce RTX 3080".into(),
        default: false,
        discrete: true,
        environment: vec![
            EnvVar {
                key: "__NV_PRIME_RENDER_OFFLOAD".into(),
                value: "1".into(),
            },
            EnvVar {
                key: "__VK_LAYER_NV_optimus".into(),
                value: "NVIDIA_only".into(),
            },
        ],
    }
}

fn value_from(fields: impl IntoIterator<Item = (&'static str, Value<'static>)>) -> Value<'static> {
    Value::from(fields.into_iter().collect::<HashMap<_, _>>())
}

#[test]
fn signature_is_a_dict_of_variants() {
    assert_eq!(GpuDevice::SIGNATURE.to_string(), "a{sv}");
}

#[test]
fn gpu_list_signature_is_an_array_of_dicts() {
    assert_eq!(<Vec<GpuDevice>>::SIGNATURE.to_string(), "aa{sv}");
}

#[test]
fn value_roundtrip_preserves_all_fields() {
    let gpu = sample();
    let value = Value::from(gpu.clone());
    assert_eq!(GpuDevice::try_from(value).unwrap(), gpu);
}

#[test]
fn value_roundtrip_without_environment() {
    let gpu = GpuDevice {
        environment: vec![],
        ..sample()
    };
    let value = Value::from(gpu.clone());
    assert_eq!(GpuDevice::try_from(value).unwrap(), gpu);
}

#[test]
fn missing_name_field_is_an_error() {
    let value = value_from([
        ("Default", Value::from(true)),
        ("Discrete", Value::from(false)),
    ]);
    let err = GpuDevice::try_from(value).unwrap_err();
    assert!(
        err.to_string().contains("Missing field 'Name'"),
        "unexpected error: {err}"
    );
}

#[test]
fn odd_environment_list_drops_trailing_entry() {
    let value = value_from([
        ("Name", Value::from("GPU")),
        ("Default", Value::from(false)),
        ("Discrete", Value::from(true)),
        (
            "Environment",
            Value::from(vec![
                "KEY".to_string(),
                "value".to_string(),
                "DANGLING".to_string(),
            ]),
        ),
    ]);
    let gpu = GpuDevice::try_from(value).unwrap();
    assert_eq!(
        gpu.environment,
        vec![EnvVar {
            key: "KEY".into(),
            value: "value".into(),
        }]
    );
}

#[test]
fn display_formats_all_fields() {
    assert_eq!(
        sample().to_string(),
        "  Name:        NVIDIA GeForce RTX 3080\n  Default:     no\n  Discrete:    yes\n  Environment: __NV_PRIME_RENDER_OFFLOAD=1 __VK_LAYER_NV_optimus=NVIDIA_only"
    );
}

#[test]
fn display_omits_empty_environment() {
    let gpu = GpuDevice {
        environment: vec![],
        ..sample()
    };
    assert_eq!(
        gpu.to_string(),
        "  Name:        NVIDIA GeForce RTX 3080\n  Default:     no\n  Discrete:    yes\n"
    );
}

#[test]
fn apply_env_sets_environment_variables() {
    let mut cmd = std::process::Command::new("true");
    sample().apply_env(&mut cmd);

    let envs: HashMap<String, Option<String>> = cmd
        .get_envs()
        .map(|(k, v)| {
            (
                k.to_string_lossy().into_owned(),
                v.map(|v| v.to_string_lossy().into_owned()),
            )
        })
        .collect();

    assert_eq!(
        envs.get("__NV_PRIME_RENDER_OFFLOAD"),
        Some(&Some("1".to_string()))
    );
    assert_eq!(
        envs.get("__VK_LAYER_NV_optimus"),
        Some(&Some("NVIDIA_only".to_string()))
    );
}
