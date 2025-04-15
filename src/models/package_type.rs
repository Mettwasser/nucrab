use derive_more::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromStr)]
pub enum PackageType {
    Module,
}
