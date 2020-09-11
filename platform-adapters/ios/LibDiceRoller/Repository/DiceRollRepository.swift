public protocol DiceRollRepository {
    func rollDice(rollRequest: RollRequest, completion: (Error?, RollResponse?) -> Void)
}
