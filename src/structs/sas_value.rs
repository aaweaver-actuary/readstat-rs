#[derive(Debug, Clone)]
pub struct SasValue {
    pub value: String,
    pub index: u32,
    pub offset: u32,
    pub width: u32,
    pub dtype: SasDtype,
}
