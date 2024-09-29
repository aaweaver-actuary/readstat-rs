#[derive(Debug, Clone)]
pub enum SasDtype {
    SasCharacter,
    SasInteger,
    SasFloat,
    SasFixedWidth,
    SasDate,
    SasTime,
    SasDatetime,
}
