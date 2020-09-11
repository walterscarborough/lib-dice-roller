import XCTest
@testable import LibDiceRoller

class DefaultDiceRollRepositoryTests: XCTestCase {
    func testDefaultDiceRepositoryReturnsRollResponse() throws {
        let defaultDiceRepository = DefaultDiceRollRepository()
        let rollRequest = RollRequest(diceSize: 6, numberOfRolls: 102)
        let rollDiceCompletionExpectation = XCTestExpectation(description: "rollDice returns result")

        defaultDiceRepository.rollDice(rollRequest: rollRequest) { maybeError, maybeRollresponse in
            XCTAssertNil(maybeError)
            XCTAssertEqual(maybeRollresponse!.diceValues.count, 102)

            rollDiceCompletionExpectation.fulfill()
        }

        wait(for: [rollDiceCompletionExpectation], timeout: 5.0)
    }
}
