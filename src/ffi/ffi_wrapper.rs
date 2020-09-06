use std::os::raw::c_char;
use std::ffi::{CString};
use crate::roll::roll_dice::roll_dice;
use crate::roll::roll_request::RollRequest;
use std::ptr::{slice_from_raw_parts_mut};

#[repr(C)]
pub struct FfiArrayBuffer {
    data: *mut c_char,
    len: usize
}

#[no_mangle]
pub extern "C" fn ffi_roll_dice(roll_request_ffi_array_buffer_ptr: *mut FfiArrayBuffer) -> *mut FfiArrayBuffer {
    let roll_request_flatbuffer = unsafe {
        assert!(!roll_request_ffi_array_buffer_ptr.is_null());

        slice_from_raw_parts_mut((*roll_request_ffi_array_buffer_ptr).data as *mut u8, (*roll_request_ffi_array_buffer_ptr).len)
    };

    let roll_request_flatbuffer = unsafe {
        (*roll_request_flatbuffer).to_vec()
    };

    let roll_request = RollRequest::from_flatbuffer(roll_request_flatbuffer);

    let roll_response= roll_dice(roll_request);

    let roll_response_flatbuffer = roll_response.to_flatbuffer();

    convert_flatbuffer_to_ffi_array_buffer_ptr(roll_response_flatbuffer)
}

#[no_mangle]
pub extern "C" fn ffi_roll_dice_free(roll_response_ffi_array_buffer_ptr: *mut FfiArrayBuffer) {

    unsafe {
        assert!(!roll_response_ffi_array_buffer_ptr.is_null(), "roll_response_ffi_array_buffer_ptr should not be null!");
        assert!(!(*roll_response_ffi_array_buffer_ptr).data.is_null(), "roll_response_ffi_array_buffer_ptr data char array should not be null!");

        CString::from_raw((*roll_response_ffi_array_buffer_ptr).data);
        let boxed = Box::from_raw(roll_response_ffi_array_buffer_ptr);

        drop(boxed);
    }
}

fn convert_flatbuffer_to_ffi_array_buffer_ptr(flatbuffer: Vec<u8>) -> *mut FfiArrayBuffer {
    unsafe {
        let c_roll_response_flatbuffer = CString::from_vec_unchecked(flatbuffer);
        let c_roll_response_flatbuffer_len = c_roll_response_flatbuffer.to_bytes().len();
        let c_roll_response_flatbuffer_ptr = CString::into_raw(c_roll_response_flatbuffer);

        let ffi_array_buffer = FfiArrayBuffer {
            data: c_roll_response_flatbuffer_ptr,
            len: c_roll_response_flatbuffer_len
        };

        let boxed_ffi_array_buffer = Box::new(ffi_array_buffer);

        Box::into_raw(boxed_ffi_array_buffer)
    }
}

#[cfg(test)]
mod tests {
    use crate::roll::roll_request::RollRequest;
    use crate::ffi::ffi_wrapper::{ffi_roll_dice, convert_flatbuffer_to_ffi_array_buffer_ptr, ffi_roll_dice_free};
    use crate::roll::roll_response::RollResponse;
    use std::ptr::{slice_from_raw_parts_mut};

    #[test]
    fn test_ffi_roll_dice() {
        let roll_request = RollRequest {
            dice_size: 6,
            number_of_rolls: 123
        };

        let roll_request_flatbuffer = roll_request.to_flatbuffer();
        let flatbuffer_to_ffi_array_buffer_ptr = convert_flatbuffer_to_ffi_array_buffer_ptr(roll_request_flatbuffer);

        let roll_response_ffi_array_buffer_ptr = ffi_roll_dice(flatbuffer_to_ffi_array_buffer_ptr);

        let roll_response_flatbuffer = unsafe {
            slice_from_raw_parts_mut((*roll_response_ffi_array_buffer_ptr).data as *mut u8, (*roll_response_ffi_array_buffer_ptr).len)
        };

        let roll_response_flatbuffer = unsafe {
            (*roll_response_flatbuffer).to_vec()
        };

        let roll_response = RollResponse::from_flatbuffer(roll_response_flatbuffer);

        assert_eq!(roll_response.dice_values.len(), 123);
    }

    #[test]
    fn test_ffi_roll_dice_free() {
        let roll_response = RollResponse {
            dice_values: vec![1, 2, 3]
        };

        let roll_response_flatbuffer = roll_response.to_flatbuffer();

        let ffi_array_buffer_ptr = convert_flatbuffer_to_ffi_array_buffer_ptr(roll_response_flatbuffer);

        ffi_roll_dice_free(ffi_array_buffer_ptr);
    }
}
