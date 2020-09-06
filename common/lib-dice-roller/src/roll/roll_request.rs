extern crate flatbuffers;
use crate::roll::roll_request::roll_request_generated::{
    get_root_as_flatbuffer_roll_request, FlatbufferRollRequest, FlatbufferRollRequestArgs,
};

#[allow(dead_code, unused_imports)]
#[path = "./../../flatbuffer_generated/roll_request_generated.rs"]
mod roll_request_generated;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RollRequest {
    pub dice_size: i32,
    pub number_of_rolls: i32,
}

impl RollRequest {
    pub fn to_flatbuffer(&self) -> Vec<u8> {
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
        let flatbuffer_roll_request = FlatbufferRollRequest::create(
            &mut builder,
            &FlatbufferRollRequestArgs {
                dice_size: self.dice_size,
                number_of_rolls: self.number_of_rolls,
            },
        );
        builder.finish(flatbuffer_roll_request, None);
        let serialized_flatbuffer_roll_request = builder.finished_data();

        serialized_flatbuffer_roll_request.to_vec()
    }

    pub fn from_flatbuffer(flatbuffer_data: Vec<u8>) -> RollRequest {
        let flatbuffer_roll_request =
            get_root_as_flatbuffer_roll_request(flatbuffer_data.as_slice());

        RollRequest {
            dice_size: flatbuffer_roll_request.dice_size(),
            number_of_rolls: flatbuffer_roll_request.number_of_rolls(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::roll::roll_request::RollRequest;

    #[test]
    fn roll_request_should_be_able_to_convert_to_and_from_flatbuffers() {
        let original_roll_request = RollRequest {
            dice_size: 6,
            number_of_rolls: 1,
        };

        let flatbuffer_roll_request = original_roll_request.to_flatbuffer();

        let deserialized_roll_request = RollRequest::from_flatbuffer(flatbuffer_roll_request);

        assert_eq!(original_roll_request, deserialized_roll_request);
    }
}
