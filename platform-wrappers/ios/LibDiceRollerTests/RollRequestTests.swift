import XCTest
@testable import LibDiceRoller

class RollRequestTests: XCTestCase {
    func testRollRequestShouldConvertToAndFromProtobuf() {
        let rollRequest = RollRequest(diceSize: 6, numberOfRolls: 102)

        let serializedRollRequest = rollRequest.toProtobuf()

        let deserializedRollRequest = RollRequest.fromProtobuf(protobufRollRequest: serializedRollRequest)

        XCTAssertEqual(rollRequest, deserializedRollRequest)
    }
}
