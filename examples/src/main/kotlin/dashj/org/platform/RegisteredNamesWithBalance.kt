/**
 * Copyright (c) 2025-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
package dashj.org.platform

import org.bitcoinj.core.Coin
import org.bitcoinj.core.Sha256Hash
import org.dashj.platform.dapiclient.model.DocumentQuery
import org.dashj.platform.dpp.document.Document
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.sdk.Client
import org.dashj.platform.sdk.client.ClientOptions
import org.dashj.platform.sdk.platform.Documents
import org.dashj.platform.sdk.platform.DomainDocument

class RegisteredNamesWithBalance {
    companion object {
        lateinit var sdk: Client

        @JvmStatic
        fun main(args: Array<String>) {
            sdk = Client(ClientOptions(network = args[0]))
            sdk.platform.useValidNodes()
            getDocumentsWithIdentityBalance()
        }

        private fun getDocumentsWithIdentityBalance() {
            val platform = sdk.platform

            var lastItem = Identifier.from(Sha256Hash.ZERO_HASH)
            var documents: List<Document>
            val allDocuments = arrayListOf<Document>()
            var requests = 0
            val limit = Documents.DOCUMENT_LIMIT
            var queryOpts = DocumentQuery.Builder().limit(limit).orderBy("normalizedLabel").build()

            do {
                println(queryOpts.toJSON())

                try {
                    documents = platform.documents.get("dpns.domain", queryOpts)
                    allDocuments.addAll(documents)
                    requests += 1

                    if (documents.isEmpty()) {
                        break
                    }

                    lastItem = documents.last().id
                    if (documents.size == limit) {
                        queryOpts = DocumentQuery.Builder()
                            .startAt(lastItem)
                            .orderBy("normalizedLabel")
                            .limit(limit)
                            .build()
                    }
                } catch (e: Exception) {
                    println("\nError retrieving results (startAt =  $lastItem)")
                    println(e.message)
                    return
                }
            } while (requests >= 0 && documents.size == limit)

            val nameBalanceMap = hashMapOf<String, Long>()
            allDocuments.forEachIndexed { i, document ->
                val nameDocument = DomainDocument(document)
                val balance = try {
                    platform.client.getIdentityBalance(nameDocument.dashUniqueIdentityId!!)
                } catch (e: Exception) {
                    println("Failed to fetch balance for ${nameDocument.label}: ${e.message}")
                    null
                } ?: return@forEachIndexed
                nameBalanceMap[nameDocument.normalizedLabel] = balance
            }
            println("username count: ${allDocuments.size}")
            allDocuments.forEachIndexed { i, document ->
                val nameDocument = DomainDocument(document)
                val balance = nameBalanceMap[nameDocument.normalizedLabel]
                println("$i.  ${nameDocument.label}: ${Coin.valueOf((balance ?: -1) / 1000).toFriendlyString()}")
            }
        }
    }
}
