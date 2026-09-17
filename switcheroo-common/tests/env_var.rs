// SPDX-License-Identifier: GPL-3.0-or-later

use switcheroo_common::EnvVar;

#[test]
fn display_formats_as_key_value_pair() {
    let env = EnvVar {
        key: "DRI_PRIME".into(),
        value: "pci-0000_01_00_0".into(),
    };
    assert_eq!(env.to_string(), "DRI_PRIME=pci-0000_01_00_0");
}

#[test]
fn converts_to_str_tuple() {
    let env = EnvVar {
        key: "DRI_PRIME".into(),
        value: "1".into(),
    };
    let (key, value): (&str, &str) = (&env).into();
    assert_eq!((key, value), ("DRI_PRIME", "1"));
}
