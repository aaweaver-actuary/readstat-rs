use crate::enums::SasDtype;
use crate::structs::SasColumnLabel;
use crate::structs::SasColumnName;
use derive_builder::Builder;

#[derive(Debug, Builder)]
pub struct SasColumn {
    name: SasColumnName,
    format: SasDtype,
    label: SasColumnLabel,
    column_index: u32,
    column_offset: u32,
    column_width: u32,
}
