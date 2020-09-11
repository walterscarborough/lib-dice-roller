package com.walterscarborough.libdiceroller

import protobuf_roll.RollResponse.ProtobufRollResponse

data class RollResponse(val diceValues: List<UInt>) {
    fun toProtobuf(): ProtobufRollResponse {
        return ProtobufRollResponse
            .newBuilder()
            .addAllDiceValues(this.diceValues.map(UInt::toInt))
            .build()
    }

    companion object {
        fun fromProtobuf(protobufRollResponse: ProtobufRollResponse): RollResponse {
            return RollResponse(
                diceValues = protobufRollResponse.diceValuesList.map(Int::toUInt)
            )
        }
    }
}
