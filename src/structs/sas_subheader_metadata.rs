use crate::enums::SasDtype;
pub struct SasSubheaderMetadata {
    pub column_offset: u32,
    pub column_width: u32,
    pub column_dtype: SasDtype,
}
