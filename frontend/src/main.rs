use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitConversionResponse {
    pub hex: String,
    pub uint32: String,
    pub int32: String,
    pub float32: String,
    pub bits: Vec<bool>,
}

// 动态获取API地址，支持IP访问
fn get_api_base() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();
        let location = window.location();
        let hostname = location.hostname().unwrap();
        format!("http://{}:3000", hostname)
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        "http://localhost:3000".to_string()
    }
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        dioxus::launch(App);
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        dioxus::launch(App);
    }
}

#[component]
fn App() -> Element {
    let mut data = use_signal(|| BitConversionResponse {
        hex: "0x00000000".to_string(),
        uint32: "0".to_string(),
        int32: "0".to_string(),
        float32: "0".to_string(),
        bits: vec![false; 32],
    });
    
    let mut hex_input = use_signal(|| String::new());
    let mut uint32_input = use_signal(|| String::new());
    let mut int32_input = use_signal(|| String::new());
    let mut f32_input = use_signal(|| String::new());
    
    let convert_value = move |value: String, format: String| {
        spawn(async move {
            let api_base = get_api_base();
            let url = format!("{}/convert?value={}&format={}", api_base, value, format);
            
            if let Ok(response) = reqwest::get(&url).await {
                if let Ok(result) = response.json::<BitConversionResponse>().await {
                    data.set(result);
                }
            }
        });
    };
    
    let toggle_bit = move |index: usize| {
        let mut current_data = data.read().clone();
        current_data.bits[index] = !current_data.bits[index];
        
        let mut value: u32 = 0;
        for (i, &bit) in current_data.bits.iter().enumerate() {
            if bit {
                value |= 1 << (31 - i);
            }
        }
        
        spawn(async move {
            let api_base = get_api_base();
            let url = format!("{}/convert?value={}&format=uint32", api_base, value);
            
            if let Ok(response) = reqwest::get(&url).await {
                if let Ok(result) = response.json::<BitConversionResponse>().await {
                    data.set(result);
                }
            }
        });
    };

    rsx! {
        div {
            style: "padding: 20px; font-family: Arial, sans-serif;",
            
            h1 { "RipBits - 位操作工具" }
            
            div {
                style: "display: flex; gap: 10px; margin-bottom: 10px; width: 800px;",
                
                for i in 0..32 {
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 5px;",
                        
                        div {
                            style: {
                                let base_color = if i == 0 { 
                                    "background-color: pink; border: 2px solid red;"
                                } else if i >= 1 && i <= 8 { 
                                    "background-color: lightgreen; border: 2px solid green;"
                                } else { 
                                    "background-color: lightblue; border: 2px solid blue;"
                                };
                                format!("width: 16px; height: 16px; {} cursor: pointer; display: flex; align-items: center; justify-content: center; font-weight: bold; font-size: 16px;", base_color)
                            },
                            onclick: move |_| toggle_bit(i),
                            if data.read().bits[i] { "×" } else { "" }
                        }
                        
                        div {
                            style: "font-size: 12px; text-align: center;",
                            "{31 - i}"
                        }
                    }
                }
            }
            
            div {
                style: "display: flex; flex-direction: column; gap: 10px; width: 800px;",
                
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    label { 
                        style: "width: 50px; font-size: 14px; text-align: left;",
                        "hex:" 
                    }
                    input {
                        r#type: "text",
                        value: if hex_input.read().is_empty() { data.read().hex.clone() } else { hex_input.read().clone() },
                        style: "font-size: 16px; text-align: left; width: 730px; padding-left: 10px;",
                        oninput: move |event| {
                            let input_value = event.value();
                            hex_input.set(input_value.clone());
                            convert_value(input_value, "hex".to_string());
                        }
                    }
                }
                
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    label { 
                        style: "width: 50px; font-size: 14px; text-align: left;",
                        "u32:" 
                    }
                    input {
                        r#type: "text",
                        value: if uint32_input.read().is_empty() { data.read().uint32.clone() } else { uint32_input.read().clone() },
                        style: "font-size: 16px; text-align: left; width: 730px; padding-left: 10px;",
                        oninput: move |event| {
                            let input_value = event.value();
                            uint32_input.set(input_value.clone());
                            convert_value(input_value, "uint32".to_string());
                        }
                    }
                }
                
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    label { 
                        style: "width: 50px; font-size: 14px; text-align: left;",
                        "i32:" 
                    }
                    input {
                        r#type: "text",
                        value: if int32_input.read().is_empty() { data.read().int32.clone() } else { int32_input.read().clone() },
                        style: "font-size: 16px; text-align: left; width: 730px; padding-left: 10px;",
                        oninput: move |event| {
                            let input_value = event.value();
                            int32_input.set(input_value.clone());
                            convert_value(input_value, "int32".to_string());
                        }
                    }
                }
                
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    label { 
                        style: "width: 50px; font-size: 14px; text-align: left;",
                        "f32:" 
                    }
                    input {
                        r#type: "text",
                        value: if f32_input.read().is_empty() { data.read().float32.clone() } else { f32_input.read().clone() },
                        style: "font-size: 16px; text-align: left; width: 730px; padding-left: 10px;",
                        oninput: move |event| {
                            let input_value = event.value();
                            f32_input.set(input_value.clone());
                            convert_value(input_value, "float32".to_string());
                        }
                    }
                }
            }
        }
    }
}