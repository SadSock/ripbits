use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitConversionRequest {
    pub value: u32,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitConversionResponse {
    pub hex: String,
    pub uint32: String,
    pub int32: String,
    pub float32: String,
    pub bits: Vec<bool>,
}

pub fn convert_value_to_response(value: u32) -> BitConversionResponse {
    let mut bits = vec![false; 32];
    for i in 0..32 {
        bits[i] = (value & (1 << (31 - i))) != 0;
    }
    
    BitConversionResponse {
        hex: format!("0x{:08X}", value),
        uint32: format!("{}", value),
        int32: format!("{}", value as i32),
        float32: format!("{}", f32::from_bits(value)),
        bits,
    }
}