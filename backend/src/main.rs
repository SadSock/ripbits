use axum::{
    extract::Query,
    http::header,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

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

#[derive(Debug, Deserialize)]
struct ConvertParams {
    value: String,
    format: String,
}

async fn convert_bits(Query(params): Query<ConvertParams>) -> Json<BitConversionResponse> {
    let value = match params.format.as_str() {
        "hex" => {
            let clean_hex = params.value.trim_start_matches("0x").trim_start_matches("0X");
            u32::from_str_radix(clean_hex, 16).unwrap_or(0)
        }
        "uint32" => params.value.parse::<u32>().unwrap_or(0),
        "int32" => params.value.parse::<i32>().unwrap_or(0) as u32,
        "float32" => {
            if let Ok(f) = params.value.parse::<f32>() {
                f.to_bits()
            } else {
                0
            }
        }
        _ => 0,
    };
    
    Json(convert_value_to_response(value))
}

async fn convert_bits_post(Json(req): Json<BitConversionRequest>) -> Json<BitConversionResponse> {
    Json(convert_value_to_response(req.value))
}

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/convert", get(convert_bits))
        .route("/convert", post(convert_bits_post))
        .layer(
            CorsLayer::permissive()  // 允许所有来源、方法和头部
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Backend server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await.unwrap();
}