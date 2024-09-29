use crate::enums::SasDtype;
use crate::structs::sas_column_name::SasColumnName;
use crate::structs::sas_value::SasValue;
use derive_builder::Builder;
use std::collections::HashMap;

#[derive(Debug, Clone, Builder)]
pub struct SasHeaderMetadata {
    pub page_count: u32,
    pub page_size: u32,
    pub column_count: u32,
    pub row_count: u64,
    pub column_names: Vec<SasColumnName>,
    pub column_widths: Vec<u32>,
    pub column_offsets: Vec<u32>,
    pub column_dtypes: Vec<SasDtype>,
    pub column_formats: Vec<String>,
    pub column_informat: Vec<String>,
    pub column_labels: Vec<String>,
    pub column_missing_values: Vec<String>,
    pub column_text: Vec<String>,
    pub column_values: Vec<Vec<SasValue>>,
    pub column_formats_map: HashMap<String, String>,
    pub column_informat_map: HashMap<String, String>,
    pub column_labels_map: HashMap<String, String>,
    pub column_missing_values_map: HashMap<String, String>,
    pub column_text_map: HashMap<String, String>,
    pub column_values_map: HashMap<String, Vec<SasValue>>,
    pub column_name_map: HashMap<String, usize>,
    pub column_width_map: HashMap<String, u32>,
    pub column_offset_map: HashMap<String, u32>,
    pub column_dtype_map: HashMap<String, SasDtype>,
    pub column_format_map: HashMap<String, String>,
    pub column_label_map: HashMap<String, String>,
    pub column_missing_value_map: HashMap<String, String>,
    pub column_name_index_map: HashMap<usize, String>,
    pub column_width_index_map: HashMap<usize, u32>,
    pub column_offset_index_map: HashMap<usize, u32>,
    pub column_dtype_index_map: HashMap<usize, SasDtype>,
    pub column_format_index_map: HashMap<usize, String>,
    pub column_informat_index_map: HashMap<usize, String>,
    pub column_label_index_map: HashMap<usize, String>,
    pub column_missing_value_index_map: HashMap<usize, String>,
}
