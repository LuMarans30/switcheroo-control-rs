// SPDX-License-Identifier: GPL-3.0-or-later

use std::fmt::Write;
use std::fmt::{self};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zbus::zvariant::{DeserializeDict, OwnedValue, SerializeDict, Type, Value};

#[derive(Debug, Clone, Serialize, Deserialize, Type, Value, OwnedValue)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, SerializeDict, DeserializeDict, Type, OwnedValue)]
#[zvariant(signature = "dict")]
pub struct GpuDevice {
    pub name: String,
    pub default: bool,
    pub discrete: bool,
    pub environment: Vec<EnvVar>,
}

impl From<GpuDevice> for Value<'_> {
    fn from(gpu: GpuDevice) -> Self {
        let mut fields = HashMap::new();
        fields.insert("Name", Value::from(gpu.name));
        fields.insert("Default", Value::from(gpu.default));
        fields.insert("Discrete", Value::from(gpu.discrete));
        fields.insert("Environment", Value::from(gpu.environment));
        Value::from(fields)
    }
}

impl TryFrom<Value<'_>> for GpuDevice {
    type Error = zbus::zvariant::Error;

    fn try_from(value: Value<'_>) -> zbus::zvariant::Result<Self> {
        Self::try_from(value.try_to_owned()?)
    }
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
            if self.default { "yes" } else { "no" }
        )
        .unwrap();
        writeln!(
            s,
            "  Discrete:    {}",
            if self.discrete { "yes" } else { "no" }
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
            if self.default { "yes" } else { "no" },
            if self.discrete { "yes" } else { "no" },
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
