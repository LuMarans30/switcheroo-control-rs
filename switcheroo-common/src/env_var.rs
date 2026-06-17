// SPDX-License-Identifier: GPL-3.0-or-later

use std::fmt;

use serde::{Deserialize, Serialize};
use zbus::zvariant::{OwnedValue, Type, Value};

/// Represents an environment variable key-value pair required to offload rendering to a specific GPU
#[derive(Debug, Clone, Serialize, Deserialize, Type, Value, OwnedValue, PartialEq)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

impl fmt::Display for EnvVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

impl<'a> From<&'a EnvVar> for (&'a str, &'a str) {
    fn from(env_var: &'a EnvVar) -> Self {
        (env_var.key.as_str(), env_var.value.as_str())
    }
}
