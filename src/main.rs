use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut checkboxes = use_signal(|| vec![false; 32]);
    let mut hex_input = use_signal(|| String::new());
    let mut uint32_input = use_signal(|| String::new());
    let mut int32_input = use_signal(|| String::new());
    let mut f32_input = use_signal(|| String::new());
    
    let bits = checkboxes.read();
    let mut value: u32 = 0;
    for (i, &checked) in bits.iter().enumerate() {
        if checked {
            value |= 1 << (31 - i);
        }
    }
    
    let hex_value = format!("0x{:08X}", value);
    let uint32_value = format!("{}", value);
    let int32_value = format!("{}", value as i32);
    let f32_value = format!("{}", f32::from_bits(value));
    
    let mut update_from_value = move |new_value: u32| {
        checkboxes.with_mut(|boxes| {
            for i in 0..32 {
                boxes[i] = (new_value & (1 << (31 - i))) != 0;
            }
        });
    };

    rsx! {
        div {
            style: "padding: 20px; font-family: Arial, sans-serif;",
            
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
                            onclick: move |_| {
                                checkboxes.with_mut(|boxes| {
                                    boxes[i] = !boxes[i];
                                });
                            },
                            if checkboxes.read()[i] { "×" } else { "" }
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
                        value: if hex_input.read().is_empty() { hex_value } else { hex_input.read().clone() },
                        style: "font-size: 16px; text-align: left; width: 730px; padding-left: 10px;",
                        oninput: move |event| {
                            let input_value = event.value();
                            hex_input.set(input_value.clone());
                            if let Ok(parsed) = u32::from_str_radix(&input_value.trim_start_matches("0x").trim_start_matches("0X"), 16) {
                                update_from_value(parsed);
                            }
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
                        value: if uint32_input.read().is_empty() { uint32_value } else { uint32_input.read().clone() },
                        style: "font-size: 16px; text-align: left; width: 730px; padding-left: 10px;",
                        oninput: move |event| {
                            let input_value = event.value();
                            uint32_input.set(input_value.clone());
                            if let Ok(parsed) = input_value.parse::<u32>() {
                                update_from_value(parsed);
                            }
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
                        value: if int32_input.read().is_empty() { int32_value } else { int32_input.read().clone() },
                        style: "font-size: 16px; text-align: left; width: 730px; padding-left: 10px;",
                        oninput: move |event| {
                            let input_value = event.value();
                            int32_input.set(input_value.clone());
                            if let Ok(parsed) = input_value.parse::<i32>() {
                                update_from_value(parsed as u32);
                            }
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
                        value: if f32_input.read().is_empty() { f32_value } else { f32_input.read().clone() },
                        style: "font-size: 16px; text-align: left; width: 730px; padding-left: 10px;",
                        oninput: move |event| {
                            let input_value = event.value();
                            f32_input.set(input_value.clone());
                            if let Ok(parsed) = input_value.parse::<f32>() {
                                update_from_value(parsed.to_bits());
                            }
                        }
                    }
                }
            }
        }
    }
}