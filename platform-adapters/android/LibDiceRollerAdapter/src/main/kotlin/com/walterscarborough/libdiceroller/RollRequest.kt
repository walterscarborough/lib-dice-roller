package com.walterscarborough.libdiceroller

import protobuf_roll.RollRequest.ProtobufRollRequest

data class RollRequest(val diceSize: Int, val numberOfRolls: Int) {

    fun toProtobuf(): ProtobufRollRequest {
        return ProtobufRollRequest
            .newBuilder()
            .setDiceSize(this.diceSize)
            .setNumberOfRolls(this.numberOfRolls)
            .build()
    }

    companion object {
        fun fromProtobuf(protobufRollRequest: ProtobufRollRequest): RollRequest {
            return RollRequest(
                diceSize = protobufRollRequest.diceSize,
                numberOfRolls = protobufRollRequest.numberOfRolls,
            )
        }
    }
}
