// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::fmt::{self};
use zbus::zvariant::{DeserializeDict, OwnedValue, SerializeDict, Type, Value};

#[derive(Debug, Clone, SerializeDict, DeserializeDict, Type, Value, OwnedValue)]
#[zvariant(signature = "dict")]
pub struct GpuDevice {
    #[zvariant(rename = "Name")]
    pub name: String,
    #[zvariant(rename = "Default")]
    pub is_default: bool,
    #[zvariant(rename = "Discrete")]
    pub is_discrete: bool,
    #[zvariant(rename = "Environment")]
    pub environment: Vec<EnvVar>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Value, OwnedValue)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

impl EnvVar {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

impl fmt::Display for EnvVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

impl GpuDevice {
    pub fn apply_env(&self, cmd: &mut std::process::Command) {
        for EnvVar { key, value } in &self.environment {
            cmd.env(key, value);
        }
    }

    pub fn format_list_entry(&self, idx: usize) -> String {
        let mut s = String::new();
        writeln!(s, "Device: {}", idx).unwrap();
        writeln!(s, "  Name:        {}", self.name).unwrap();
        writeln!(
            s,
            "  Default:     {}",
            if self.is_default { "yes" } else { "no" }
        )
        .unwrap();
        writeln!(
            s,
            "  Discrete:    {}",
            if self.is_discrete { "yes" } else { "no" }
        )
        .unwrap();

        if !self.environment.is_empty() {
            let env_str = self
                .environment
                .iter()
                .map(EnvVar::to_string)
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(s, "  Environment: {}", env_str).unwrap();
        }

        s
    }
}

impl fmt::Display for GpuDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  Name:        {}\n  Default:     {}\n  Discrete:    {}",
            self.name,
            if self.is_default { "yes" } else { "no" },
            if self.is_discrete { "yes" } else { "no" },
        )?;

        if !self.environment.is_empty() {
            let env_str = self
                .environment
                .iter()
                .map(EnvVar::to_string)
                .collect::<Vec<_>>()
                .join(" ");
            write!(f, "\n  Environment: {}", env_str)?;
        }

        Ok(())
    }
}
