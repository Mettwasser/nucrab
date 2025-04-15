use std::error::Error;

pub mod package_root;
pub mod package_type;

pub trait FromNuonValue: Sized {
    type Error: Error;
    fn from_nuon_value(value: nu_protocol::Value) -> Result<Self, Self::Error>;
}
