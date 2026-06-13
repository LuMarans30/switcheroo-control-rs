use std::fmt;

use serde::{Deserialize, Serialize};
use zbus::zvariant::{OwnedValue, Type, Value};

/// Represents an environment variable key-value pair required to offload rendering to a specific GPU
#[derive(Debug, Clone, Serialize, Deserialize, Type, Value, OwnedValue, PartialEq)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

impl EnvVar {
    pub fn apply(&self, cmd: &mut std::process::Command) {
        cmd.env(&self.key, &self.value);
    }
}

impl fmt::Display for EnvVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}
