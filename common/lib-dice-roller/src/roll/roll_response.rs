use crate::roll_generated::protobuf_roll::ProtobufRollResponse;
use prost::Message;

#[derive(PartialEq, Debug, Clone)]
pub struct RollResponse {
    pub dice_values: Vec<u32>,
}

impl RollResponse {
    pub fn to_protobuf(&self) -> Vec<u8> {
        let protobuf_roll_response = ProtobufRollResponse {
            dice_values: self.dice_values.clone(),
        };

        let mut buf: Vec<u8> = vec![];
        protobuf_roll_response
            .encode(&mut buf)
            .expect("should be able to serialize to protobuf");

        buf
    }

    pub fn from_protobuf(data: Vec<u8>) -> RollResponse {
        let protobuf_roll_response = ProtobufRollResponse::decode(data.as_slice())
            .expect("should be able to deserialize from protobuf");

        RollResponse {
            dice_values: protobuf_roll_response.dice_values,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::roll::roll_response::RollResponse;

    #[test]
    fn roll_response_should_be_able_to_convert_to_and_from_flatbuffers() {
        let original_roll_response = RollResponse {
            dice_values: vec![1, 2, 3],
        };

        let protobuf_roll_response = original_roll_response.to_protobuf();

        let deserialized_roll_response = RollResponse::from_protobuf(protobuf_roll_response);

        assert_eq!(original_roll_response, deserialized_roll_response);
    }
}
