use std::{collections::HashMap, fmt};

use zbus::zvariant::{OwnedValue, Type, Value};

use crate::env_var::EnvVar;

#[derive(Debug, Clone, OwnedValue, PartialEq)]
pub struct GpuDevice {
    pub name: String,
    pub default: bool,
    pub discrete: bool,
    pub environment: Vec<EnvVar>,
}

impl GpuDevice {
    pub fn apply_env(&self, cmd: &mut std::process::Command) {
        for env in &self.environment {
            env.apply(cmd);
        }
    }
}

impl Type for GpuDevice {
    fn signature() -> zbus::zvariant::Signature<'static> {
        zbus::zvariant::Signature::from_static_str("a{sv}").unwrap()
    }
}

impl From<GpuDevice> for Value<'_> {
    fn from(gpu: GpuDevice) -> Self {
        let mut fields = HashMap::new();
        fields.insert("Name", Value::from(gpu.name));
        fields.insert("Default", Value::from(gpu.default));
        fields.insert("Discrete", Value::from(gpu.discrete));

        // Flatten the Vec<EnvVar> into a sequential Vec<String> to match the C daemon's "as" type
        let mut env_flat = Vec::with_capacity(gpu.environment.len() * 2);
        for env in gpu.environment {
            env_flat.push(env.key);
            env_flat.push(env.value);
        }
        fields.insert("Environment", Value::from(env_flat));

        Value::from(fields)
    }
}

impl TryFrom<Value<'_>> for GpuDevice {
    type Error = zbus::zvariant::Error;

    fn try_from(value: Value<'_>) -> zbus::zvariant::Result<Self> {
        let dict = zbus::zvariant::Dict::try_from(value)?;

        let name: String = dict
            .get(&"Name".to_string())?
            .ok_or_else(|| zbus::zvariant::Error::Message("Missing field 'Name'".to_string()))?;

        let default: bool = dict
            .get(&"Default".to_string())?
            .ok_or_else(|| zbus::zvariant::Error::Message("Missing field 'Default'".to_string()))?;

        let discrete: bool = dict.get(&"Discrete".to_string())?.ok_or_else(|| {
            zbus::zvariant::Error::Message("Missing field 'Discrete'".to_string())
        })?;

        let mut environment = Vec::new();

        if let Some(env_value) = dict.get::<&str, &Value>(&"Environment")?
            && let Value::Array(arr) = env_value
        {
            let vals: Vec<String> = arr
                .iter()
                .filter_map(|v| String::try_from(v).ok())
                .collect();

            for chunk in vals.chunks_exact(2) {
                environment.push(EnvVar {
                    key: chunk[0].clone(),
                    value: chunk[1].clone(),
                });
            }
        }

        Ok(GpuDevice {
            name,
            default,
            discrete,
            environment,
        })
    }
}

impl fmt::Display for GpuDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Name:        {}", self.name)?;
        writeln!(f, "  Default:     {}", YesNo(self.default))?;
        writeln!(f, "  Discrete:    {}", YesNo(self.discrete))?;

        if !self.environment.is_empty() {
            let env_str = self
                .environment
                .iter()
                .map(EnvVar::to_string)
                .collect::<Vec<_>>()
                .join(" ");
            write!(f, "  Environment: {env_str}")?;
        }

        Ok(())
    }
}

struct YesNo(bool);

impl fmt::Display for YesNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.0 { "yes" } else { "no" })
    }
}
