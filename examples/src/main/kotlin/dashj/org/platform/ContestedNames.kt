/**
 * Copyright (c) 2024-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
package dashj.org.platform

import com.google.common.base.Stopwatch
import org.dashj.platform.dapiclient.SystemIds
import org.dashj.platform.dpp.voting.ContestedDocumentResourceVotePoll
import org.dashj.platform.sdk.Client
import org.dashj.platform.sdk.client.ClientOptions
import org.dashj.platform.sdk.platform.Documents
import java.util.concurrent.TimeUnit

/**
 * ContestedNames - Display all of the currently contested usernames
 */
class ContestedNames {
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
            val watch = Stopwatch.createStarted()

            val votePolls = platform.documents.getAllVotePolls(
                SystemIds.dpnsDataContractId,
                "domain",
                System.currentTimeMillis(),
                true,
                System.currentTimeMillis() + Documents.votingPeriod(platform.params),
                true,
                orderAscending = true
            )

            println("${votePolls.size} results returned in ${watch.elapsed(TimeUnit.MILLISECONDS)}")
            val names = arrayListOf<String>()
            votePolls.forEach {
                when(it) {
                    is ContestedDocumentResourceVotePoll -> {
                        println(it.indexValues[1])
                        names.add(it.indexValues[1])
                    }
                    else -> println("unknown vote poll type")
                }
            }
            val watch2 = Stopwatch.createStarted()
            val voteContenders = names.map {
                platform.names.getVoteContenders(it)
            }
            println("get vote contenders ${voteContenders.size} results returned in ${watch2.elapsed(TimeUnit.MILLISECONDS)}")
        }
    }
}