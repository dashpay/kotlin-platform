/**
 * Copyright (c) 2020-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
package org.dashj.platform.sdk.platform

import com.google.common.base.Preconditions
import io.grpc.StatusRuntimeException
import org.bitcoinj.core.ECKey
import org.bitcoinj.core.NetworkParameters
import org.bitcoinj.core.Sha256Hash
import org.dashj.platform.dapiclient.DapiClient
import org.dashj.platform.dapiclient.model.DocumentQuery
import org.dashj.platform.dpp.document.Document
import org.dashj.platform.dpp.errors.DriveErrorMetadata
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identity.Identity
import org.dashj.platform.dpp.identity.IdentityPublicKey
import org.dashj.platform.dpp.voting.Contenders
import org.dashj.platform.dpp.voting.ContestedDocumentResourceVotePoll
import org.dashj.platform.dpp.voting.ResourceVote
import org.dashj.platform.dpp.voting.ResourceVoteChoice
import org.dashj.platform.dpp.voting.Vote
import org.dashj.platform.dpp.voting.VotePoll
import org.dashj.platform.sdk.callbacks.Signer
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import java.util.concurrent.TimeUnit

class Documents(val platform: Platform) {

    companion object {
        const val DOCUMENT_LIMIT = DapiClient.DEFAULT_LIMIT
        private val log: Logger = LoggerFactory.getLogger(Documents::class.java)

        fun votingPeriod(params: NetworkParameters): Long = if (params.id == NetworkParameters.ID_MAINNET) TimeUnit.DAYS.toMillis(14) else TimeUnit.MINUTES.toMillis(90)

    }

    fun broadcast(identity: Identity, privateKey: ECKey, create: List<Document>?, replace: List<Document>? = null, delete: List<Document>? = null) {
        val transitionMap = hashMapOf<String, List<Document>?>()
        if (create != null) {
            transitionMap["create"] = create
        }
        if (replace != null) {
            transitionMap["replace"] = replace
        }
        if (delete != null) {
            transitionMap["delete"] = delete
        }

        TODO() // should we finish this function or remove it
    }

    fun create(typeLocator: String, userId: Identifier, opts: MutableMap<String, Any?>): Document {
        val dpp = platform.dpp

        val appNames = platform.apps.keys

        val (appName, fieldType) = getAppnameAndType(typeLocator, appNames)

        if (!platform.apps.containsKey(appName)) {
            throw Exception("Cannot find contractId for $appName")
        }

        val dataContract = platform.contracts.get(platform.apps[appName]!!.contractId)

        return dpp.document.create(
            dataContract!!,
            userId,
            fieldType,
            opts
        )
    }

    /**
     * Takes a document name in the form of "appname.document_type" and returns
     * "appname" and "document_type"
     * @param typeLocator String
     * @param appNames MutableSet<String>
     * @return Pair<String, String>
     */
    private fun getAppnameAndType(
        typeLocator: String,
        appNames: MutableSet<String>
    ): Pair<String, String> {
        val appName: String
        val fieldType: String
        if (typeLocator.contains('.')) {
            val split = typeLocator.split('.')
            appName = split[0]
            fieldType = split[1]
        } else {
            appName = appNames.first()
            fieldType = typeLocator
        }
        return Pair(appName, fieldType)
    }

    /**
     * Fetches all results that match the query this allows limit to be greater than 100
     * and will return more than 100 results
     */
    fun getAll(
        typeLocator: String,
        documentQuery: DocumentQuery
    ): List<Document> {
        val query = documentQuery.clone()
        val limit = query.limit
        var total = 0
        if (limit > 100) {
            query.limit = 100
        }
        val documents = ArrayList<Document>()
        var documentList: List<Document>
        var requests = 0

        do {
            try {
                documentList =
                    platform.documents.get(
                        typeLocator,
                        query
                    )
                requests += 1
                if (documentList.isNotEmpty()) {
                    when {
                        limit == -1 -> documents.addAll(documentList)
                        total + documentList.size > limit -> {
                            total - limit
                            for (i in 0 until limit - total) {
                                documents.add(documentList[i])
                            }
                        }
                        else -> documents.addAll(documentList)
                    }
                    query.startAt = documentList.last().id
                }
                total += documentList.size
            } catch (e: Exception) {
                log.warn("Exception $e")
                throw e
            }
        } while ((requests == 0 || documentList.size >= DOCUMENT_LIMIT))

        return documents
    }

    fun get(typeLocator: String, opts: DocumentQuery): List<Document> {
        val appNames = platform.apps.keys

        val (appName, fieldType) = getAppnameAndType(typeLocator, appNames)

        if (!platform.apps.containsKey(appName)) {
            throw Exception("No app named $appName specified.")
        }
        val appDefinition = platform.apps[appName]
        if (appDefinition == null || appDefinition.contractId.toBuffer().isEmpty()) {
            throw Exception("Missing contract ID for $appName")
        }

        val contractId = appDefinition.contractId

        return get(contractId, fieldType, opts)
    }

    fun get(
        dataContractId: Identifier,
        documentType: String,
        opts: DocumentQuery,
    ): List<Document> {
        try {
            return platform.stateRepository.fetchDocuments(dataContractId, documentType, opts)
        } catch (e: StatusRuntimeException) {
            log.error(
                "Document query: unable to get documents of $dataContractId: " +
                    "${DriveErrorMetadata(e.trailers.toString())}  with $opts",
                e
            )
            throw e
        } catch (e: Exception) {
            log.error("Document query: unable to get documents of $dataContractId with $opts", e)
            throw e
        }
    }

    fun deserialize(bytes: ByteArray, dataContractId: Identifier, documentType: String): Document {
        return platform.client.deserializeDocument(
            bytes,
            dataContractId,
            documentType
        )
    }

    fun getVoteContenders(
        dataContractId: Identifier,
        documentType: String,
        indexName: String,
        indexes: List<String>,
    ): Contenders {
        Preconditions.checkArgument(documentType.isNotEmpty())
        Preconditions.checkArgument(indexName.isNotEmpty())
        Preconditions.checkArgument(indexes.isNotEmpty())
        return platform.client.getVoteContenders(
            dataContractId,
            documentType,
            indexName,
            indexes
        )
    }

    fun broadcastVote(
        resourceVoteChoice: ResourceVoteChoice,
        dataContractId: Identifier,
        documentType: String,
        indexName: String,
        indexValues: List<String>,
        voterProTxHash: Sha256Hash,
        identityPublicKey: IdentityPublicKey,
        signerCallback: Signer
    ): Vote {
        val vote = Vote(
            ResourceVote(
                resourceVoteChoice,
                ContestedDocumentResourceVotePoll(dataContractId, documentType, indexName, indexValues)
            )
        )
        return platform.client.broadcastVote(vote, voterProTxHash, identityPublicKey, signerCallback)
    }

    fun getVotePolls(
        dataContractId: Identifier,
        documentType: String,
        startTime: Long,
        startTimeIncluded: Boolean = true,
        endTime:Long,
        endTimeIncluded: Boolean = true,
        limit: Int = DOCUMENT_LIMIT,
        orderAscending: Boolean = true
    ): List<VotePoll> {
        val votePollsGroupedByTimestamp = platform.client.getVotePolls(
            startTime,
            startTimeIncluded,
            endTime,
            endTimeIncluded,
            limit,
            orderAscending
        )

        val result = arrayListOf<VotePoll>()
        votePollsGroupedByTimestamp.list.forEach { votePollGroup ->
            votePollGroup.second.forEach { votePoll ->
                when (votePoll) {
                    is ContestedDocumentResourceVotePoll -> {
                        if (votePoll.dataContractId == dataContractId && votePoll.documentTypeName == documentType) {
                            result.add(votePoll)
                        }
                    }
                    // add other VotePoll types here
                }
            }
        }
        return result
    }

    fun getAllVotePolls(
        dataContractId: Identifier,
        documentType: String,
        startTime: Long,
        startTimeIncluded: Boolean = true,
        endTime:Long,
        endTimeIncluded: Boolean = true,
        orderAscending: Boolean = true
    ): List<VotePoll> {
        var count = 0
        var currentStartTime = startTime
        var currentStartTimeIncluded = startTimeIncluded
        val results = arrayListOf<VotePoll>()
        do {
            val batch = getVotePolls(
                dataContractId,
                documentType,
                currentStartTime,
                currentStartTimeIncluded,
                endTime,
                endTimeIncluded, DOCUMENT_LIMIT, orderAscending)
            count = batch.size
            val lastVotePoll = batch.last()
            currentStartTime = when (lastVotePoll) {
                is ContestedDocumentResourceVotePoll -> {
                    val voteContenders = getVoteContenders(
                        lastVotePoll.dataContractId,
                        lastVotePoll.documentTypeName,
                        lastVotePoll.indexName,
                        lastVotePoll.indexValues
                    )
                    val createdAt = voteContenders.map.minOf { contenders ->
                        val document = deserialize(
                            contenders.value.serializedDocument!!,
                            lastVotePoll.dataContractId,
                            lastVotePoll.documentTypeName
                        )
                        document.createdAt ?: 0
                    }
                    createdAt + votingPeriod(platform.params)
                }
                else -> -1L
            }
            currentStartTimeIncluded = true
            results.addAll(
                batch.filter {
                    when (it) {
                        is ContestedDocumentResourceVotePoll -> {
                            it.dataContractId == dataContractId && it.documentTypeName == documentType
                        }
                        else -> false
                    }
                }
            )
        } while (count == 100 && currentStartTime != -1L)
        return results
    }

    fun getVoteFromMasternode(proTxHash: Sha256Hash, dataContractId: Identifier, documentType: String, indexName: String, indexes: List<String>) {
        platform.client.getLastVoteFromMasternode(proTxHash, dataContractId, documentType, indexName, indexes)
    }
}
