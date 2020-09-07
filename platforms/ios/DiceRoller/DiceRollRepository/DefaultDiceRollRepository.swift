import Foundation

struct DefaultDiceRollRepository {
    func rollDice(rollRequest: RollRequest, completion: (Error?, RollResponse?) -> Void) {

        let protobufRollRequest = rollRequest.toProtobuf()
        var serializedProtobufRollRequest: Data = try! protobufRollRequest.serializedData()
        let serializedProtobufRollRequestLength = UInt(serializedProtobufRollRequest.count)

        serializedProtobufRollRequest.withUnsafeMutableBytes({ (bytes: UnsafeMutablePointer<Int8>) -> Void in
            let ffiArrayBufferRollRequest = FfiArrayBuffer(data: bytes, len: serializedProtobufRollRequestLength)

            let ffiArrayBufferRollResponse = ffi_roll_dice(ffiArrayBufferRollRequest)

            let rollResponseBufferPointer = UnsafeMutableBufferPointer(
                start: ffiArrayBufferRollResponse.data,
                count: Int(ffiArrayBufferRollResponse.len)
            )
            let rollResponseProtobufData = Data(buffer: rollResponseBufferPointer)
            let rollResponseProtobuf = try! ProtobufRoll_ProtobufRollResponse(serializedData: rollResponseProtobufData)

            ffi_roll_dice_free(ffiArrayBufferRollResponse)

            let rollResponse = RollResponse.fromProtobuf(protobufRollResponse: rollResponseProtobuf)
            completion(nil, rollResponse)
        })
    }
}
