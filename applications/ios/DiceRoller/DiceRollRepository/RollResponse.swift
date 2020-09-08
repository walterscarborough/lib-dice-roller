struct RollResponse {
    let diceValues: [UInt32]

    func toProtobuf() -> ProtobufRoll_ProtobufRollResponse {
        let protobufRollResponse = ProtobufRoll_ProtobufRollResponse.with {
            $0.diceValues = diceValues
        }

        return protobufRollResponse
    }

    static func fromProtobuf(protobufRollResponse: ProtobufRoll_ProtobufRollResponse) -> RollResponse {
        return RollResponse(diceValues: protobufRollResponse.diceValues)
    }
}
