use crate::roll_generated::protobuf_roll::ProtobufRollRequest;
use prost::Message;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RollRequest {
    pub dice_size: u32,
    pub number_of_rolls: u32,
}

impl RollRequest {
    pub fn to_protobuf(&self) -> Vec<u8> {
        let protobuf_roll_request = ProtobufRollRequest {
            dice_size: self.dice_size,
            number_of_rolls: self.number_of_rolls,
        };

        let mut buf: Vec<u8> = vec![];
        protobuf_roll_request
            .encode(&mut buf)
            .expect("should be able to serialize to protobuf");

        buf
    }

    pub fn from_protobuf(data: Vec<u8>) -> RollRequest {
        let protobuf_roll_request = ProtobufRollRequest::decode(data.as_slice())
            .expect("should be able to deserialize from protobuf");

        RollRequest {
            dice_size: protobuf_roll_request.dice_size,
            number_of_rolls: protobuf_roll_request.number_of_rolls,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::roll::roll_request::RollRequest;

    #[test]
    fn roll_request_should_be_able_to_convert_to_and_from_protobuf() {
        let original_roll_request = RollRequest {
            dice_size: 6,
            number_of_rolls: 123,
        };

        let protobuf_roll_request = original_roll_request.to_protobuf();

        let deserialized_roll_request = RollRequest::from_protobuf(protobuf_roll_request);

        assert_eq!(original_roll_request, deserialized_roll_request);
    }
}
