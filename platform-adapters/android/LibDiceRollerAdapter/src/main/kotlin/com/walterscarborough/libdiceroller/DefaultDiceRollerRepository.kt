package com.walterscarborough.libdiceroller

import protobuf_roll.RollResponse.ProtobufRollResponse

class DefaultDiceRollerRepository {
    external fun roll_dice(input: ByteArray?): ByteArray

    init {
        System.loadLibrary("dice_roller_jni")
    }

    fun rollDice() {
        val rollRequest = RollRequest(6, 2)
        val rollRequestProtobuf = rollRequest.toProtobuf()
        val rollRequestByteArray = rollRequestProtobuf.toByteArray()

        val rollResponseByteArray = roll_dice(rollRequestByteArray)

        val rollResponseProtobuf = ProtobufRollResponse.parseFrom(rollResponseByteArray)
        val rollResponse = RollResponse.fromProtobuf(rollResponseProtobuf)

        println("*** all done! ${rollResponse}")
    }
}
