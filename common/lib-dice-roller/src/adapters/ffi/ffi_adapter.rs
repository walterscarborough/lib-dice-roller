use crate::roll::roll_dice::roll_dice;
use crate::roll::roll_request::RollRequest;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::slice_from_raw_parts_mut;

#[repr(C)]
pub struct FfiArrayBuffer {
    data: *mut c_char,
    len: usize,
}

#[no_mangle]
pub extern "C" fn ffi_roll_dice(roll_request_ffi_array_buffer: FfiArrayBuffer) -> FfiArrayBuffer {
    let roll_request_protobuf = convert_ffi_array_buffer_to_protobuf(roll_request_ffi_array_buffer);

    let roll_request = RollRequest::from_protobuf(roll_request_protobuf);

    let roll_response = roll_dice(roll_request);

    let roll_response_protobuf = roll_response.to_protobuf();

    convert_protobuf_to_ffi_array_buffer(roll_response_protobuf)
}

#[no_mangle]
pub extern "C" fn ffi_roll_dice_free(roll_response_ffi_array_buffer: FfiArrayBuffer) {
    unsafe {
        assert!(
            !roll_response_ffi_array_buffer.data.is_null(),
            "roll_response_ffi_array_buffer_ptr data char array should not be null!"
        );

        let c_string = CString::from_raw(roll_response_ffi_array_buffer.data);

        drop(c_string);
    }
}

fn convert_protobuf_to_ffi_array_buffer(protobuf: Vec<u8>) -> FfiArrayBuffer {
    unsafe {
        let c_roll_response_flatbuffer = CString::from_vec_unchecked(protobuf);
        let c_roll_response_flatbuffer_len = c_roll_response_flatbuffer.to_bytes().len();
        let c_roll_response_flatbuffer_ptr = CString::into_raw(c_roll_response_flatbuffer);

        FfiArrayBuffer {
            data: c_roll_response_flatbuffer_ptr,
            len: c_roll_response_flatbuffer_len,
        }
    }
}

fn convert_ffi_array_buffer_to_protobuf(ffi_array_buffer: FfiArrayBuffer) -> Vec<u8> {
    let protobuf = unsafe {
        assert!(!ffi_array_buffer.data.is_null());

        let protobuf_data =
            slice_from_raw_parts_mut(ffi_array_buffer.data as *mut u8, ffi_array_buffer.len);

        (*protobuf_data).to_vec()
    };

    protobuf
}

#[cfg(test)]
mod tests {
    use crate::adapters::ffi::ffi_adapter::{
        convert_ffi_array_buffer_to_protobuf, convert_protobuf_to_ffi_array_buffer, ffi_roll_dice,
        ffi_roll_dice_free,
    };
    use crate::roll::roll_request::RollRequest;
    use crate::roll::roll_response::RollResponse;

    #[test]
    fn test_ffi_roll_dice() {
        let roll_request = RollRequest {
            dice_size: 6,
            number_of_rolls: 123,
        };

        let roll_request_protobuf = roll_request.to_protobuf();
        let roll_request_ffi_array_buffer =
            convert_protobuf_to_ffi_array_buffer(roll_request_protobuf);

        let roll_response_ffi_array_buffer = ffi_roll_dice(roll_request_ffi_array_buffer);

        let roll_response_protobuf =
            convert_ffi_array_buffer_to_protobuf(roll_response_ffi_array_buffer);

        let roll_response = RollResponse::from_protobuf(roll_response_protobuf);

        assert_eq!(roll_response.dice_values.len(), 123);
    }

    #[test]
    fn test_ffi_roll_dice_free() {
        let roll_response = RollResponse {
            dice_values: vec![1, 2, 3],
        };

        let roll_response_protobuf = roll_response.to_protobuf();

        let ffi_array_buffer = convert_protobuf_to_ffi_array_buffer(roll_response_protobuf);

        ffi_roll_dice_free(ffi_array_buffer);
    }
}
