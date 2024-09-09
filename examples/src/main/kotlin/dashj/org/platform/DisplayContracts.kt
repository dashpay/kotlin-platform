/**
 * Copyright (c) 2020-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
package dashj.org.platform

import org.dashj.platform.sdk.Client
import org.dashj.platform.sdk.client.ClientOptions
import org.json.JSONObject

class DisplayContracts {
    companion object {
        lateinit var sdk: Client

        @JvmStatic
        fun main(args: Array<String>) {
            if (args.isEmpty()) {
                println("Usage: DisplayContracts network")
                return
            }
            sdk = Client(ClientOptions(network = args[0]))
            sdk.platform.useValidNodes()
            getContracts()
        }

        private fun getContracts() {
            val platform = sdk.platform

            for (app in platform.apps) {
                try {
                    val contract = platform.contracts.get(app.value.contractId)

                    println("app: ${app.key} DataContract: ${contract!!.id}")
                    println(JSONObject(contract.toJSON()).toString(2))
                } catch (e: Exception) {
                    println("\nError retrieving results for app:${app.key}")
                    println(e.message)
                }
            }
        }
    }
}
