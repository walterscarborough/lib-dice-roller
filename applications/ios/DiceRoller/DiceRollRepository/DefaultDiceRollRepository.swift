import Foundation

struct DefaultDiceRollRepository {
    func rollDice(rollRequest: RollRequest, completion: (Error?, RollResponse?) -> Void) {

        let protobufRollRequest = rollRequest.toProtobuf()
        var serializedProtobufRollRequest: Data = try! protobufRollRequest.serializedData()
        let serializedProtobufRollRequestLength = serializedProtobufRollRequest.count

        serializedProtobufRollRequest.withUnsafeMutableBytes({ (bytes: UnsafeMutableRawBufferPointer) -> Void in

            let unsafeBufferPointer = bytes.bindMemory(to: Int8.self)
            let unsafePointer = unsafeBufferPointer.baseAddress!

            let ffiArrayBufferRollRequest = FfiArrayBuffer(data: unsafePointer, len: UInt(serializedProtobufRollRequestLength))

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
