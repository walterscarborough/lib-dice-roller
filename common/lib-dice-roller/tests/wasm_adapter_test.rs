use wasm_bindgen_test::*;

use dice_roller::roll::roll_request::RollRequest;
use dice_roller::roll::roll_response::RollResponse;
use dice_roller::wasm::wasm_adapter::{convert_protobuf_to_js_array, wasm_roll_dice, convert_js_array_to_protobuf};

#[wasm_bindgen_test]
fn wasm_roll_dice_should_succeed() {

    let roll_request = RollRequest {
        dice_size: 6,
        number_of_rolls: 123,
    };

    let roll_request_protobuf = roll_request.to_protobuf();
    let js_roll_request_protobuf = convert_protobuf_to_js_array(roll_request_protobuf);

    let js_roll_response_protobuf = wasm_roll_dice(&js_roll_request_protobuf).expect("should be able to run wasm_roll_dice");

    let roll_response_protobuf = convert_js_array_to_protobuf(&js_roll_response_protobuf);
    let response_protobuf = RollResponse::from_protobuf(roll_response_protobuf);

    assert_eq!(response_protobuf.dice_values.len(), 123);
}
