use std::str::FromStr;

use super::FromNuonValue;
use super::package_type::PackageType;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum PackageRootDestructureError {
    #[error("Couldn't find key in value: {_0}")]
    KeyNotFound(&'static str),

    #[error("Couldn't convert to {to_type} at {key}")]
    InvalidConversion {
        key: &'static str,
        to_type: &'static str,
    },

    ShellError(#[from] ShellError),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageRoot {
    pub name: String,
    pub r#type: PackageType,
    pub version: String,
    pub description: String,
    pub license: String,
}

impl FromNuonValue for PackageRoot {
    type Error = PackageRootDestructureError;
    fn from_nuon_value(value: nu_protocol::Value) -> Result<Self, Self::Error> {
        let name = get_key_string!(value, "name");
        let r#type = PackageType::from_str(&get_key_string!(value, "type")).map_err(|_| {
            PackageRootDestructureError::InvalidConversion {
                key: "type",
                to_type: "PackageType",
            }
        })?;
        let version = get_key_string!(value, "version");
        let description = get_key_string!(value, "description");
        let license = get_key_string!(value, "license");

        Ok(Self {
            name,
            r#type,
            version,
            description,
            license,
        })
    }
}

macro_rules! get_key_string {
    ($value:expr, $key:literal) => {
        $value
            .get_data_by_key($key)
            .ok_or(PackageRootDestructureError::KeyNotFound($key))?
            .into_string()?
    };
}

use get_key_string;
use nu_protocol::ShellError;
