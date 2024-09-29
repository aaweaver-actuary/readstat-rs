#[derive(Debug, Clone)]
pub struct SasError {
    pub error: String,
    pub error_offset: u32,
}
