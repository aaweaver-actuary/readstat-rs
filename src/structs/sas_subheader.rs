use derive_builder::Builder;

#[derive(Debug, Builder)]
pub struct SasSubheader {
    pub subheader: String,
    pub subheader_length: u32,
    pub subheader_offset: u32,
}
