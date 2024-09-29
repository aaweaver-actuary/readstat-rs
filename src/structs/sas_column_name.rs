use crate::enums::SasDtype;
use derive_builder::Builder;
#[derive(Debug, Builder, Clone)]
pub struct SasColumnName {
    pub name: String,
    pub index: u32,
    pub offset: u32,
    pub width: u32,
    pub dtype: SasDtype,
}
