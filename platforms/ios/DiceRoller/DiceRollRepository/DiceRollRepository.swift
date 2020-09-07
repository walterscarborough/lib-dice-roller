protocol DiceRollRepository {
    func rollDice(rollRequest: RollRequest) -> RollResponse
}
