package com.walterscarborough.libdicerollerapp

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import com.walterscarborough.libdiceroller.DefaultDiceRollerRepository
import kotlinx.android.synthetic.main.activity_main.*

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        // Example of a call to a native method
        sample_text.text = "sample text"

        val foo =
            DefaultDiceRollerRepository()

        foo.rollDice()
    }
}
