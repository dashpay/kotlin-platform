/**
 * Copyright (c) 2024-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
package dashj.org.platform

import org.bitcoinj.core.Sha256Hash
import org.dashj.platform.dapiclient.model.DocumentQuery
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.sdk.Client
import org.dashj.platform.sdk.client.ClientOptions
import org.dashj.platform.wallet.IdentityVerify
import org.dashj.platform.wallet.IdentityVerifyDocument

class VerifyIdentityDocuments {
    companion object {
        lateinit var sdk: Client

        @JvmStatic
        fun main(args: Array<String>) {
            sdk = Client(ClientOptions(network = args[0]))
            sdk.platform.useValidNodes()
            getDocuments()
        }

        private fun getDocuments() {
            val platform = sdk.platform
            val identityVerify = IdentityVerify(platform)

            var lastItem = Identifier.from(Sha256Hash.ZERO_HASH)
            var documents: List<IdentityVerifyDocument>
            var requests = 0
            val limit = 100
            var queryOpts = DocumentQuery.Builder().limit(limit).build()

            do {
                println(queryOpts.toJSON())

                try {
                    documents = identityVerify.get(queryOpts)

                    requests += 1

                    for (doc in documents) {
                        println(
                            "normalizedLabel: %-20s".format(doc.normalizedLabel) +
                                " (domain: " + doc.normalizedParentDomainName +
                                ") Identity: " + doc.ownerId +
                                " link: " + doc.url
                        )
                    }

                    lastItem = Identifier.from(documents.last().id)
                    if (documents.size == 100) {
                        queryOpts = DocumentQuery.Builder().startAfter(lastItem).limit(100).build()
                    }
                } catch (e: Exception) {
                    println("\nError retrieving results (startAt =  $lastItem)")
                    println(e.message)
                    return
                }
            } while (requests >= 0 && documents.size == limit)
        }
    }
}
