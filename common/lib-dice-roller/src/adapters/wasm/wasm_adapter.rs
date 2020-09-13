use wasm_bindgen::prelude::*;

use crate::roll::roll_dice::roll_dice;
use crate::roll::roll_request::RollRequest;

#[wasm_bindgen]
pub fn wasm_roll_dice(input: &JsValue) -> Result<js_sys::Array, JsValue> {
    let rust_input = convert_js_array_to_protobuf(input);

    let roll_request = RollRequest::from_protobuf(rust_input);
    let roll_response = roll_dice(roll_request);
    let roll_response_protobuf = roll_response.to_protobuf();

    let output = convert_protobuf_to_js_array(roll_response_protobuf);

    Ok(output)
}

pub fn convert_protobuf_to_js_array(protobuf_data: Vec<u8>) -> js_sys::Array {
    let output = js_sys::Array::new();

    for x in protobuf_data {
        let js_value = JsValue::from(x);
        output.push(&js_value);
    }

    output
}

pub fn convert_js_array_to_protobuf(js_array_data: &JsValue) -> Vec<u8> {
    let input_iterator = js_sys::try_iter(js_array_data)
        .expect("should be able to iterate through input data")
        .unwrap();

    let mut rust_data: Vec<u8> = vec![];

    for x in input_iterator {
        let x = x.unwrap();

        if x.as_f64().is_some() {
            let x = x
                .as_f64()
                .expect("should be able to convert from JS number to Rust number")
                as u8;
            rust_data.push(x);
        }
    }

    rust_data
}
