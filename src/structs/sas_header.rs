#[derive(Debug, Clone)]
pub struct SasHeader {
    pub header: String,
    pub header_length: u32,
    pub header_offset: u32,
}
