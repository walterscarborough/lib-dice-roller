import XCTest
@testable import LibDiceRoller

class RollResponseTests: XCTestCase {
    func testRollResponseShouldConvertToAndFromProtobuf() {
        let rollResponse = RollResponse(diceValues: [1, 2, 3])

        let serializedRollResponse = rollResponse.toProtobuf()

        let deserializedRollResponse = RollResponse.fromProtobuf(protobufRollResponse: serializedRollResponse)

        XCTAssertEqual(rollResponse, deserializedRollResponse)
    }
}
