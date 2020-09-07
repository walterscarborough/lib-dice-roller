struct RollRequest {
    let diceSize: UInt32
    let numberOfRolls: UInt32

    func toProtobuf() -> ProtobufRoll_ProtobufRollRequest {
        let protobufRollRequest = ProtobufRoll_ProtobufRollRequest.with {
            $0.diceSize = self.diceSize
            $0.numberOfRolls = self.numberOfRolls
        }

        return protobufRollRequest
    }

    static func fromProtobuf(protobufRollRequest: ProtobufRoll_ProtobufRollRequest) -> RollRequest {
        return RollRequest(
            diceSize: protobufRollRequest.diceSize,
            numberOfRolls: protobufRollRequest.numberOfRolls
        )
    }
}
