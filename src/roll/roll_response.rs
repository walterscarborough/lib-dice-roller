extern crate flatbuffers;

use crate::roll::roll_response::roll_response_generated::{FlatbufferRollResponse, FlatbufferRollResponseArgs, get_root_as_flatbuffer_roll_response};

#[allow(dead_code, unused_imports)]
#[path = "./../../flatbuffer_generated/roll_response_generated.rs"]
mod roll_response_generated;

#[derive(PartialEq, Debug)]
pub struct RollResponse {
    pub dice_values: Vec<i32>
}

impl RollResponse {
    pub fn to_flatbuffer(&self) -> Vec<u8> {
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);

        let flatbuffer_dice_values = builder.create_vector(&self.dice_values);
        let flatbuffer_roll_response = FlatbufferRollResponse::create(&mut builder, &FlatbufferRollResponseArgs{dice_values: Some(flatbuffer_dice_values)});
        builder.finish(flatbuffer_roll_response, None);
        let serialized_flatbuffer_roll_response = builder.finished_data();

        serialized_flatbuffer_roll_response.to_vec()
    }

    pub fn from_flatbuffer(flatbuffer_data: Vec<u8>) -> RollResponse {
        let flatbuffer_roll_response = get_root_as_flatbuffer_roll_response(flatbuffer_data.as_slice());

        let mut dice_values: Vec<i32> = vec![];

        for dice_value in flatbuffer_roll_response.dice_values().unwrap().iter() {
            dice_values.push(
                dice_value
            );
        }

        RollResponse {
            dice_values
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::roll::roll_response::RollResponse;

    #[test]
    fn roll_response_should_be_able_to_convert_to_and_from_flatbuffers() {
        let original_roll_response = RollResponse {
            dice_values: vec![1, 2 , 3]
        };

        let flatbuffer_roll_response = original_roll_response.to_flatbuffer();

        let deserialized_roll_response = RollResponse::from_flatbuffer(flatbuffer_roll_response);

        assert_eq!(original_roll_response, deserialized_roll_response);
    }
}
