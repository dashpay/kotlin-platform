package org.dashj.platform

import org.bitcoinj.core.Base58
import java.util.Scanner
import org.dashj.platform.sdk.example;

object PlatformExplorer {

    init {
        System.loadLibrary("sdklib")
    };


    @JvmStatic
    fun main(args: Array<String>) {
        val identifier = example.getDocument()
        println(Base58.encode(identifier._0._0))

        println("Platform Explorer")
        println("-----------------")
        println("Main Menu")
        println("1. Identities")

        println("Enter an identity id:")
        val scanner = Scanner(System.`in`)
        val idString = scanner.nextLine()

        println(" > $idString")


    }
}