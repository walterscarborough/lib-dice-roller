public struct RollRequest: Equatable {
    public let diceSize: UInt32
    public let numberOfRolls: UInt32

    public init(diceSize: UInt32, numberOfRolls: UInt32) {
        self.diceSize = diceSize
        self.numberOfRolls = numberOfRolls
    }

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
