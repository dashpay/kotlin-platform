/**
 * Copyright (c) 2025-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
package dashj.org.platform

import org.bitcoinj.core.Address
import org.bitcoinj.core.DumpedPrivateKey
import org.bitcoinj.crypto.MnemonicCode
import org.bitcoinj.wallet.authentication.AuthenticationGroupExtension
import java.util.Scanner
import org.dashj.platform.dpp.toHex
import org.dashj.platform.sdk.Client
import org.dashj.platform.sdk.client.ClientOptions
import org.dashj.platform.sdk.client.WalletOptions

class DisplayIdentityKeys {
    companion object {
        lateinit var client: Client

        @JvmStatic
        fun main(args: Array<String>) {
            if (args.isEmpty()) {
                println("Usage: DisplayIdentityKeys network")
                return
            }
            println("Enter a recovery phrase: ")
            val scanner = Scanner(System.`in`)
            val phrase = scanner.nextLine()

            MnemonicCode().check(phrase.split(" ", "\n"))

            val recoveryPhrase = if (phrase == "default") { DefaultIdentity(args[0]).seed } else phrase
            client = Client(ClientOptions(network = args[0], walletOptions = WalletOptions(recoveryPhrase)))
            client.platform.useValidNodes()
            displayKeys()
        }

        private fun displayKeys() {
            val authGroup = client.wallet!!.getKeyChainExtension(AuthenticationGroupExtension.EXTENSION_ID) as AuthenticationGroupExtension
            val firstKey = authGroup.identityKeyChain.getKey(0)
            println(firstKey)
            println("privateKey: ${firstKey.privKeyBytes.toHex()}")
            println("privateKey: ${DumpedPrivateKey(client.params,firstKey.privKeyBytes, true)}")
            println("pubkey: ${firstKey.pubKey.toHex()}")
            println("pubkeyhash: ${firstKey.pubKeyHash.toHex()}")
            println("address: ${Address.fromKey(client.params, firstKey)}")
        }
    }
}
