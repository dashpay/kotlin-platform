/**
 * Copyright (c) 2020-present, Dash Core Team
 *
 * This source code is licensed under the MIT license found in the
 * COPYING file in the root directory of this source tree.
 */

package org.dashj.platform.dapiclient

import com.google.common.base.Preconditions
import com.google.common.base.Stopwatch
import io.grpc.Status
import io.grpc.StatusRuntimeException
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import org.bitcoinj.core.BloomFilter
import org.bitcoinj.core.Sha256Hash
import org.bitcoinj.evolution.SimplifiedMasternodeListManager
import org.bitcoinj.params.DevNetParams
import org.bitcoinj.quorums.LLMQParameters
import org.dash.platform.dapi.v0.CoreOuterClass
import org.dashj.platform.dapiclient.errors.NotFoundException
import org.dashj.platform.dapiclient.errors.ResponseException
import org.dashj.platform.dapiclient.grpc.BroadcastTransactionMethod
import org.dashj.platform.dapiclient.grpc.DefaultShouldRetryCallback
import org.dashj.platform.dapiclient.grpc.GetBlockMethod
import org.dashj.platform.dapiclient.grpc.GetEstimatedTransactionFeeMethod
import org.dashj.platform.dapiclient.grpc.GetMasternodeStatusMethod
import org.dashj.platform.dapiclient.grpc.GetTransactionMethod
import org.dashj.platform.dapiclient.grpc.GrpcMethod
import org.dashj.platform.dapiclient.grpc.GrpcMethodShouldRetryCallback
import org.dashj.platform.dapiclient.grpc.SubscribeToTransactionsWithProofs
import org.dashj.platform.dapiclient.grpc.SubscribeToTransactionsWithProofsMethod
import org.dashj.platform.dapiclient.model.DocumentQuery
import org.dashj.platform.dapiclient.model.GetTransactionResponse
import org.dashj.platform.dapiclient.model.JsonRPCRequest
import org.dashj.platform.dapiclient.model.MasternodeStatus
import org.dashj.platform.dapiclient.provider.DAPIAddress
import org.dashj.platform.dapiclient.provider.DAPIAddressListProvider
import org.dashj.platform.dapiclient.provider.DAPIGrpcMasternode
import org.dashj.platform.dapiclient.provider.ListDAPIAddressProvider
import org.dashj.platform.dapiclient.provider.SimplifiedMasternodeListDAPIAddressProvider
import org.dashj.platform.dapiclient.rest.DapiService
import org.dashj.platform.dpp.DashPlatformProtocol
import org.dashj.platform.dpp.contract.DataContract
import org.dashj.platform.dpp.document.Document
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identity.Identity
import org.dashj.platform.dpp.identity.IdentityPublicKey
import org.dashj.platform.dpp.statetransition.StateTransition
import org.dashj.platform.dpp.toBase58
import org.dashj.platform.dpp.toHex
import org.dashj.platform.dpp.toTimestampMillis
import org.dashj.platform.dpp.util.Converters
import org.dashj.platform.dpp.voting.Contenders
import org.dashj.platform.dpp.voting.ContestedResources
import org.dashj.platform.dpp.voting.Vote
import org.dashj.platform.dpp.voting.VotePollsGroupedByTimestamp
import org.dashj.platform.sdk.PlatformValue
import org.dashj.platform.dpp.voting.ResourceVotesByIdentity
import org.dashj.platform.sdk.SWIGTYPE_p_DashSdk
import org.dashj.platform.sdk.Start
import org.dashj.platform.sdk.callbacks.ContextProvider
import org.dashj.platform.sdk.callbacks.Signer
import org.dashj.platform.sdk.dashsdk
import org.slf4j.LoggerFactory
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import java.math.BigInteger
import java.util.concurrent.TimeUnit

class DapiClient(
    var dapiAddressListProvider: DAPIAddressListProvider,
    val dpp: DashPlatformProtocol,
    val useContextProvider: Boolean,
    val isTestnet: Boolean,
    private var timeOut: Long = DEFAULT_TIMEOUT,
    private var retries: Int = DEFAULT_RETRY_COUNT,
    private var banBaseTime: Int = DEFAULT_BASE_BAN_TIME,
    private var waitForNodes: Int = DEFAULT_WAIT_FOR_NODES,
) {

    // gRPC properties
    var lastUsedAddress: DAPIAddress? = null

    // jRPC Properties
    private lateinit var retrofit: Retrofit
    private lateinit var dapiService: DapiService
    private val debugOkHttpClient: OkHttpClient
    private val debugJrpc = true
    private var initializedJRPC = false
    private val defaultShouldRetryCallback = DefaultShouldRetryCallback()

    // used for reporting
    private var successfulCalls: Long = 0
    private var failedCalls: Long = 0
    private var totalCalls: Long = 0
    private var retriedCalls: Long = 0
    private val stopWatch = Stopwatch.createStarted()

    // proofs
    private val fullVerification: Boolean
    private lateinit var masternodeListManager: SimplifiedMasternodeListManager

    // Constants
    companion object {
        private val logger = LoggerFactory.getLogger(DapiClient::class.java.name)

        const val BLOCK_HASH_LENGTH = 64 // length of a hex string of a hash

        const val DEFAULT_BASE_BAN_TIME = 60 * 1000 // 1 minute

        const val DEFAULT_RETRY_COUNT = 10
        const val USE_DEFAULT_RETRY_COUNT = -1
        const val DEFAULT_TIMEOUT = 5000L // normally a timeout is 5 seconds longer than this
        const val DEFAULT_BROADCAST_TIMEOUT = 80000L
        const val DEFAULT_WAIT_FOR_NODES = 5
        const val DEFAULT_HTTP_TIMEOUT = 10L
        const val REQUIRED_SUCCESS_RATE = 0.50 // 50%
        const val DEFAULT_LIMIT = 100
    }

    var contextProvider = object : ContextProvider() {
        override fun getQuorumPublicKey(
            quorumType: Int,
            quorumHashBytes: ByteArray?,
            coreChainLockedHeight: Int
        ): ByteArray? {
            val quorumHash = Sha256Hash.wrap(quorumHashBytes)
            var quorumPublicKey: ByteArray? = null
            logger.info("searching for quorum: $quorumType, $quorumHash, $coreChainLockedHeight")
            masternodeListManager.getQuorumListAtTip(
                LLMQParameters.LLMQType.fromValue(
                    quorumType
                )
            ).forEachQuorum(true) {
                if (it.llmqType.value == quorumType && it.quorumHash == quorumHash) {
                    quorumPublicKey = it.quorumPublicKey.serialize(false)
                }
            }
            logger.info("searching for quorum: result: ${quorumPublicKey?.toHex()}")
            return quorumPublicKey
        }

        override fun getDataContract(identifier: org.dashj.platform.sdk.Identifier?): ByteArray {
            return byteArrayOf(0)
        }
    }

    val contextProviderFunction: Long
        get() = if (useContextProvider) contextProvider.quorumPublicKeyCallback else 0L

    val contextProviderContext: Long
        get() = if (useContextProvider) contextProvider.nativeContext else 0L

    var rustSdk: SWIGTYPE_p_DashSdk

    init {
        val loggingInterceptor = HttpLoggingInterceptor { msg: String? -> logger.info(msg) }
        loggingInterceptor.level = HttpLoggingInterceptor.Level.BODY

        debugOkHttpClient = OkHttpClient.Builder()
            .addInterceptor(loggingInterceptor)
            .connectTimeout(DEFAULT_HTTP_TIMEOUT, TimeUnit.SECONDS)
            .readTimeout(DEFAULT_HTTP_TIMEOUT, TimeUnit.SECONDS)
            .writeTimeout(DEFAULT_HTTP_TIMEOUT, TimeUnit.SECONDS)
            .build()

        if (banBaseTime != DEFAULT_BASE_BAN_TIME) {
            this.dapiAddressListProvider.setBanBaseTime(banBaseTime)
        }

        fullVerification = try {
            true
        } catch (e: RuntimeException) {
            false
        }

        rustSdk = dashsdk.platformMobileSdkCreateDashSdkWithContext(
            contextProviderContext,
            BigInteger.valueOf(contextProviderFunction),
            BigInteger.ZERO,
            isTestnet,
            timeOut * 2,
            timeOut,
            retries.toLong()
        )
    }

    constructor(
        masternodeAddress: String,
        dpp: DashPlatformProtocol,
        useContextProvider: Boolean,
        isTestnet: Boolean,
        timeOut: Long = DEFAULT_TIMEOUT,
        retries: Int = DEFAULT_RETRY_COUNT,
        banBaseTime: Int = DEFAULT_BASE_BAN_TIME,
        waitForNodes: Int = DEFAULT_WAIT_FOR_NODES
    ) :
            this(
                listOf(masternodeAddress),
                dpp,
                useContextProvider,
                isTestnet,
                timeOut,
                retries,
                banBaseTime,
                waitForNodes
            )

    constructor(
        addresses: List<String>,
        dpp: DashPlatformProtocol,
        useContextProvider: Boolean,
        isTestnet: Boolean,
        timeOut: Long = DEFAULT_TIMEOUT,
        retries: Int = DEFAULT_RETRY_COUNT,
        banBaseTime: Int = DEFAULT_BASE_BAN_TIME,
        waitForNodes: Int = DEFAULT_WAIT_FOR_NODES
    ) :
            this(
                ListDAPIAddressProvider.fromList(addresses, banBaseTime),
                dpp,
                useContextProvider,
                isTestnet,
                timeOut,
                retries,
                banBaseTime,
                waitForNodes
            )

    /* Platform gRPC methods */

    /**
     * Broadcast State Transition to machine
     *
     * @param stateTransition
     * @param statusCheck Whether to call getStatus on a node before broadcastStateTransition to avoid usign a bad node
     * @param retryCallback Determines if the broadcast shoudl be tried again after a failure
     */
    fun broadcastStateTransition(
        stateTransition: StateTransition,
        statusCheck: Boolean = false,
        //retryCallback: GrpcMethodShouldRetryCallback = DefaultShouldRetryCallback()
    ) {
        logger.info("broadcastStateTransition(${stateTransition.toJSON()})")
        //val method = BroadcastStateTransitionMethod(stateTransition)
        //grpcRequest(method, statusCheck = statusCheck, retryCallback = retryCallback)
    }

//    fun broadcastStateTransitionInternal(
//        stateTransition: StateTransition,
//        statusCheck: Boolean = false,
//        retryCallback: BroadcastShouldRetryCallback = DefaultBroadcastRetryCallback()
//    ):
//        BroadcastStateTransitionMethod {
//        logger.info("broadcastStateTransitionInternal(${stateTransition.toJSON()})")
//        val method = BroadcastStateTransitionMethod(stateTransition)
//        grpcRequest(
//            method,
//            statusCheck = statusCheck,
//            retryCallback = object : DefaultShouldRetryCallback() {
//                override fun shouldRetry(grpcMethod: GrpcMethod, e: StatusRuntimeException): Boolean {
//                    return retryCallback.shouldRetry(grpcMethod, e)
//                }
//            }
//        )
//        return method
//    }

    /**
     * Wait for state transition result
     * @param hash
     * @param prove Whether to return the proof
     */
//    fun waitForStateTransitionResult(hash: ByteArray, prove: Boolean): WaitForStateTransitionResult {
//        val method = WaitForStateTransitionResultMethod(hash, prove)
//        logger.info(method.toString())
//        val result = grpcRequest(method) as PlatformOuterClass.WaitForStateTransitionResultResponse
//
//        return if (result.hasError()) {
//            WaitForStateTransitionResult(
//                StateTransitionBroadcastException(result.error),
//                ResponseMetadata(result.metadata)
//            )
//        } else {
//            WaitForStateTransitionResult(Proof(result.proof), ResponseMetadata(result.metadata))
//        }
//    }

//    val threadPoolService = Executors.newCachedThreadPool()
//
//    inner class WaitForStateSubmissionCallable(
//        val signedStateTransition: StateTransitionIdentitySigned,
//        val prove: Boolean
//    ) :
//        Callable<WaitForStateTransitionResult> {
//        override fun call(): WaitForStateTransitionResult {
//            return try {
//                waitForStateTransitionResult(signedStateTransition.hashOnce(), prove)
//            } catch (e: StatusRuntimeException) {
//                if (e.status.code == Status.CANCELLED.code) {
//                    logger.error("waitForStateTransitionResult: canceled due to broadcastStateTransition exception")
//                } else {
//                    logger.error("waitForStateTransitionResult exception: $e")
//                }
//                WaitForStateTransitionResult(
//                    StateTransitionBroadcastException(e.status.code.value(), e.message ?: "", ByteArray(0)),
//                    ResponseMetadata(0, 0)
//                )
//            }
//        }
//    }
//
//    fun broadcastStateTransitionAndWait(
//        signedStateTransition: StateTransitionIdentitySigned,
//        retriesLeft: Int = USE_DEFAULT_RETRY_COUNT,
//        statusCheck: Boolean = false,
//        retryCallback: BroadcastShouldRetryCallback = DefaultBroadcastRetryCallback(),
//        verifyProof: VerifyProof = DefaultVerifyProof(signedStateTransition)
//    ) {
//        val retryAttemptsLeft = if (retriesLeft == USE_DEFAULT_RETRY_COUNT) {
//            retries // set in constructor
//        } else {
//            retriesLeft // if called recursively
//        }
//
//        val watch = Stopwatch.createStarted()
//        val futuresList = arrayListOf<Future<WaitForStateTransitionResult>>()
//
//        val futureWithProof = threadPoolService.submit(WaitForStateSubmissionCallable(signedStateTransition, true))
//        futuresList.add(futureWithProof)
//
//        for (i in 0 until waitForNodes - 1)
//            futuresList.add(threadPoolService.submit(WaitForStateSubmissionCallable(signedStateTransition, false)))
//
//        var broadcast: BroadcastStateTransitionMethod?
//        try {
//            broadcast = broadcastStateTransitionInternal(signedStateTransition, statusCheck, retryCallback)
//            logger.info("broadcastStateTransitionAndWait: watch broadcast complete: $watch")
//        } catch (e: StatusRuntimeException) {
//            // should we retry
//            logger.info("broadcastStateTransitionInternal: failure: $e")
//            // cancel all waiting futures
//            logger.info("broadcastStateTransitionAndWait: cancel all waiting threads")
//            futuresList.forEach { it.cancel(true) }
//            logger.info("broadcastStateTransitionAndWait: determine if we should retry")
//            if (!retryCallback.shouldRetry(BroadcastStateTransitionMethod(signedStateTransition), e)) {
//                // what should we do
//                logger.info("Will not retry for $e")
//                throw e
//            }
//            broadcastStateTransitionAndWait(signedStateTransition, retryAttemptsLeft - 1, statusCheck, retryCallback)
//            return
//        }
//
//        var hasProof: Boolean = false
//        val finished = hashSetOf<Future<WaitForStateTransitionResult>>()
//        val waitForTimeout = DEFAULT_BROADCAST_TIMEOUT
//        var lastWaitTime = System.currentTimeMillis()
//        val startWaitTime = System.currentTimeMillis()
//
//        while (finished.size < waitForNodes && !(hasProof && (finished.size >= (waitForNodes/2 + 1))) &&
//            ((startWaitTime + waitForTimeout) >= lastWaitTime)
//        ) {
//            for (future in futuresList) {
//                if (future.isDone && !finished.contains(future)) {
//                    finished.add(future)
//                    hasProof = hasProof || (future.get().proof != null && future.get().proof!!.isValid())
//                    logger.info(
//                        "broadcastStateTransitionAndWait: ${finished.size} of $waitForNodes complete " +
//                            "(hasProof = $hasProof); proof = ${future.get().proof}"
//                    )
//                    logger.info("broadcastStateTransitionAndWait: watch wait for node: $watch")
//                }
//            }
//            lastWaitTime = System.currentTimeMillis()
//            Thread.sleep(TimeUnit.MILLISECONDS.toMillis(250))
//        }
//        var timedout = false
//        if ((startWaitTime + waitForTimeout) < System.currentTimeMillis()) {
//            logger.info("broadcastStateTransitionAndWait: timeout with ${finished.size} of $waitForNodes complete")
//            timedout = finished.size == 0
//        } else {
//            logger.info(
//                "broadcastStateTransitionAndWait: finished waiting in " +
//                    "${(lastWaitTime - startWaitTime) / TimeUnit.SECONDS.toMillis(1)}s"
//            )
//        }
//        // cancel any futures that are not finished
//        futuresList.forEach {
//            if (!it.isDone) {
//                it.cancel(true)
//                if (it == futureWithProof) {
//                    logger.warn("canceling the future that was waiting for the proof")
//                }
//            }
//        }
//
//        val waitForResult = if (!futureWithProof.isCancelled) {
//            futureWithProof.get()
//        } else {
//            null
//        }
//
//        val successRate = if (timedout) {
//            finished.size.toDouble() / futuresList.size
//        } else {
//            futuresList.count {
//                try {
//                    it.get().isSuccess()
//                } catch (e: CancellationException) {
//                    false
//                }
//            }.toDouble() / futuresList.size
//        }
//        logger.info("broadcastStateTransitionAndWait: watch wait complete: $watch")
//
//        when {
//            waitForResult == null -> {
//                logger.info("broadcastStateTransitionAndWait: failure: Timeout or no proof returned")
//                throw StateTransitionBroadcastException(2, "Timeout", ByteArray(0))
//
//                // TODO: remove this line when proofs are enabled
//                logger.info("broadcastStateTransitionAndWait: success ($successRate)")
//            }
//            // count the proof as success
//            waitForResult.isSuccess() -> {
//                logger.info("broadcastStateTransitionAndWait: success ($successRate): ${waitForResult.proof}")
//                // TODO: these are commented out because proofs are disabled
//                // logger.info("root_tree_proof    : ${waitForResult.proof!!.rootTreeProof.toHex()}")
//                // logger.info("store_tree_proof   : ${waitForResult.proof.storeTreeProofs}")
//                // logger.info("signature_llmq_hash: ${waitForResult.proof.signatureLlmqHash.toHex()}")
//                // logger.info("signature          : ${waitForResult.proof.signature.toHex()}")
//                // logger.info("state transition   : ${signedStateTransition.toBuffer().toHex()}")
//                logger.info("ST Hash            : ${Sha256Hash.of(signedStateTransition.toBuffer())}")
//                // logger.info("proof verification : ${verifyProof.verify(waitForResult.proof)}")
//                logger.info("success rate       : $successRate")
//                // logger.info(
//                //    "signature proof    : ${verifyProof(
//                //        waitForResult.proof, waitForResult.metadata,
//                //        broadcast
//                //    ) }"
//                // )
//            }
//            waitForResult.isError() -> {
//                logger.info("broadcastStateTransitionAndWait: failure: ${waitForResult.error}")
//                if (!retryCallback.shouldRetry(broadcast, waitForResult.error!!)) {
//                    throw waitForResult.error
//                }
//                broadcastStateTransitionAndWait(
//                    signedStateTransition,
//                    retryAttemptsLeft - 1,
//                    statusCheck,
//                    retryCallback
//                )
//                return
//            }
//            // success is more than 50% and there is no proof
//            successRate > REQUIRED_SUCCESS_RATE -> {
//                // we need to request the proof from a node
//                logger.info("broadcastStateTransitionAndWait: success ($successRate)")
//            }
//            // success is less than 50% and there is no proof
//            successRate <= REQUIRED_SUCCESS_RATE -> {
//                logger.info("broadcastStateTransitionAndWait: failure($successRate): ${waitForResult.error}")
//                Thread.sleep(TimeUnit.SECONDS.toMillis(3))
//                // what do we do here?
//                if (!retryCallback.shouldRetry(broadcast, waitForResult.error!!)) {
//                    throw waitForResult.error
//                }
//                // TODO: call wait functions only, not sure if this will work
//            }
//        }
//    }

    /**
     * Fetch the identity by id
     * @param id String
     * @param prove Whether to return the proof
     * @return Identity?
     */
    fun getIdentity(
        id: ByteArray,
        prove: Boolean = false,
    ): Identity? {
        logger.info("getIdentity(${id.toBase58()}, $prove)")
        val identityId = Identifier.from(id)
        val exceptionList = arrayListOf<Exception>()
        var retriesLeft = retries
        do {
            val result = dashsdk.platformMobileFetchIdentityFetchIdentityWithSdk(
                rustSdk,
                identityId.toNative()
            )
            try {
                return try {
                    Identity(result.unwrap().get())
                } catch (e: Exception) {
                    null
                }
            } catch (e: Exception) {
                if (e.message?.contains("context provider error: invalid quorum: quorum not found") == true) {
                    exceptionList.add(e)
                    retriesLeft--
                } else {
                    throw e
                }
            }
        } while (retriesLeft > 0)
        throw MaxRetriesReachedException(exceptionList)
    }

    /**
     * Fetch the identity by the first public key hash
     * @param pubKeyHash ByteArray
     * @return Identity?
     */
    fun getIdentityByFirstPublicKey(pubKeyHash: ByteArray, prove: Boolean = false): Identity? {
        logger.info("getIdentityByFirstPublicKey(${pubKeyHash.toHex()})")
        val exceptionList = arrayListOf<Exception>()
        var retriesLeft = retries
        do {
            val result = dashsdk.platformMobileFetchIdentityFetchIdentityWithKeyhashSdk(
                rustSdk,
                pubKeyHash
            )
            try {
                return try {
                    Identity(result.unwrap())
                } catch (e: Exception) {
                    null
                }
            } catch (e: Exception) {
                if (e.message?.contains("context provider error: invalid quorum: quorum not found") == true) {
                    exceptionList.add(e)
                    retriesLeft--
                } else {
                    throw e
                }
            }
        } while (retriesLeft > 0)
        throw MaxRetriesReachedException(exceptionList)
    }

    /**
     * Fetch Data Contract by id
     * @param contractId String
     * @param prove Whether to return the proof
     * @return GetDataContractResponse
     * @throws NotFoundException if the contract is not found
     */
    fun getDataContract(
        contractIdByteArray: ByteArray,
        prove: Boolean = false,
        //retryCallback: GrpcMethodShouldRetryCallback = defaultShouldRetryCallback
    ): DataContract? {
        logger.info("getDataContract(${contractIdByteArray.toBase58()})")
        val contractId = Identifier.from(contractIdByteArray)

        val exceptionList = arrayListOf<Exception>()
        var retriesLeft = retries
        do {
            val dataContractResult = dashsdk.platformMobileDataContractsFetchDataContract(
                rustSdk,
                contractId.toNative()
            )
            try {
                return try {
                    DataContract.from(dataContractResult.unwrap().get())
                } catch (e: Exception) {
                    null
                }
            } catch (e: Exception) {
                if (e.message?.contains("context provider error: invalid quorum: quorum not found") == true) {
                    exceptionList.add(e)
                    retriesLeft--
                } else {
                    logger.error("get data contract error", e)
                    throw NotFoundException("DataContract ${contractIdByteArray.toBase58()} not found")
                }
                logger.error("get data contract error", e)
            }
        } while (retriesLeft > 0)
        throw MaxRetriesReachedException(exceptionList)
    }

    /**
     *
     * @param contractId String The contract id associated with the documents
     * @param type String The type of document
     * @param documentQuery DocumentQuery DocumentQuery that specify which documents to find, sort order
     * @param prove Whether to return the proof
     * @param retryCallback should this call be retried upon failure
     * and pagination
     * @return List<ByteArray>? a list of documents matching the provided parameters
     */
    fun getDocuments(
        contractId: ByteArray,
        type: String,
        documentQuery: DocumentQuery,
        prove: Boolean = false,
        //retryCallback: GrpcMethodShouldRetryCallback = DefaultGetDocumentsRetryCallback()
    ): List<Document> {
        logger.info("getDocuments(contractId={}, type={}, {})", contractId.toBase58(), type, documentQuery)
        val contractIdentifier = Identifier(contractId)
        val rustContractIdentifier = contractIdentifier.toNative()
        val start = when {
            documentQuery.startAt != null -> Start(documentQuery.startAt!!.toBuffer(), true)
            documentQuery.startAfter != null -> Start(documentQuery.startAfter!!.toBuffer(), false)
            else -> null
        }
        var retriesLeft = DEFAULT_RETRY_COUNT
        val exceptionList = arrayListOf<Exception>()
        do {
            val result = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                rustSdk,
                rustContractIdentifier,
                type,
                documentQuery.encodeWhere(),
                documentQuery.encodeOrderBy(),
                if (documentQuery.limit == -1) 100 else documentQuery.limit.toLong(),
                start
            )
            try {
                return result.unwrap().map {
                    Document(it, contractIdentifier)
                }
            } catch (e: Exception) {
                if (e.message?.contains("context provider error: invalid quorum: quorum not found") == true) {
                    exceptionList.add(e)
                    retriesLeft--
                } else {
                    throw e
                }
            }
        } while (retriesLeft > 0)
        throw MaxRetriesReachedException(exceptionList)
    }

    /* Core */
    /** get status of platform node  */
    fun getMasternodeStatus(address: DAPIAddress? = null, retries: Int = USE_DEFAULT_RETRY_COUNT): MasternodeStatus? {
        val method = GetMasternodeStatusMethod()
        val watch = Stopwatch.createStarted()
        val response = grpcRequest(method, retries, address) as CoreOuterClass.GetMasternodeStatusResponse?
        watch.stop()

        return response?.let {
            val result = MasternodeStatus(
                MasternodeStatus.Status.getByCode(it.status.number),
                Sha256Hash.wrap(it.proTxHash.toByteArray()),
                it.posePenalty,
                it.isSynced,
                it.syncProgress
            )

            logger.info("$result")
            result
        }
    }

    private fun logException(e: StatusRuntimeException, masternode: DAPIGrpcMasternode, method: GrpcMethod) {
        if (e.status.code == Status.CANCELLED.code) {
            logger.warn("RPC failed with ${masternode.address.host}: CANCELLED: ${e.trailers}")
        } else {
            logger.warn("RPC failed with ${masternode.address.host}: ${e.status}: ${e.trailers}")
            logger.warn("  method error: ${method.getErrorInfo(e)}")
        }
        logger.warn("  for $method")
    }

    fun getBlockByHeight(height: Int): ByteArray? {
        logger.info("getBlockByHeight($height)")
        Preconditions.checkArgument(height > 0)
        val request = CoreOuterClass.GetBlockRequest.newBuilder()
            .setHeight(height)
            .build()
        return getBlock(request)
    }

    /**
     *
     * @param hash String
     * @return ByteArray?
     */

    fun getBlockByHash(hash: String): ByteArray? {
        return ByteArray(80)
    }

    private fun getBlock(request: CoreOuterClass.GetBlockRequest?): ByteArray? {
        val getBlock = GetBlockMethod(request!!)
        val response = grpcRequest(getBlock) as CoreOuterClass.GetBlockResponse?
        return response?.block!!.toByteArray()
    }

    fun getEstimatedTransactionFee(blocks: Int): Double {
        logger.info("getEstimatedTransactionFee($blocks)")
        val method = GetEstimatedTransactionFeeMethod(blocks)
        val response = grpcRequest(method) as CoreOuterClass.GetEstimatedTransactionFeeResponse?
        return response?.fee!!
    }

    fun subscribeToTransactionsWithProofs(
        bloomFilter: BloomFilter,
        fromBlockHash: Sha256Hash,
        count: Int,
        sendTransactionHashes: Boolean,
        subscribeToTransactionsWithProofs: SubscribeToTransactionsWithProofs
    ) {
        subscribeToTransactionsWithProofs(
            bloomFilter,
            fromBlockHash,
            -1,
            count,
            sendTransactionHashes,
            subscribeToTransactionsWithProofs
        )
    }

    fun subscribeToTransactionsWithProofs(
        bloomFilter: BloomFilter,
        fromBlockHeight: Int,
        count: Int,
        sendTransactionHashes: Boolean,
        subscribeToTransactionsWithProofs: SubscribeToTransactionsWithProofs
    ) {
        subscribeToTransactionsWithProofs(
            bloomFilter,
            Sha256Hash.ZERO_HASH,
            fromBlockHeight,
            count,
            sendTransactionHashes,
            subscribeToTransactionsWithProofs
        )
    }

    private fun subscribeToTransactionsWithProofs(
        bloomFilter: BloomFilter,
        fromBlockHash: Sha256Hash,
        fromBlockHeight: Int,
        count: Int,
        sendTransactionHashes: Boolean,
        subscribeToTransactionsWithProofs: SubscribeToTransactionsWithProofs
    ) {
        val subscribe = SubscribeToTransactionsWithProofsMethod(
            bloomFilter,
            fromBlockHash,
            fromBlockHeight,
            count,
            sendTransactionHashes,
            subscribeToTransactionsWithProofs
        )
        grpcRequest(subscribe)
    }

    /**
     * Make a DAPI call with retry support
     *
     * @param grpcMethod GrpcMethod
     * @param retriesLeft Int The number of times to retry the DAPI call.  (Default = -1, use this.retries)
     * @param dapiAddress DAPIAddress? The node that should used (default = null, choose randomly)
     * @param statusCheck Boolean Should call getStatus on a node before making the DAPI call (default = false)
     * @param retryCallback GrpcMethodShouldRetryCallback Is used upon failure to determine if the DAPI should
     *                      be attempted again after failure
     * @return Any? The result of the call, which must be cast to the correct type by the caller
     */
    private fun grpcRequest(
        grpcMethod: GrpcMethod,
        retriesLeft: Int = USE_DEFAULT_RETRY_COUNT,
        dapiAddress: DAPIAddress? = null,
        statusCheck: Boolean = false,
        retryCallback: GrpcMethodShouldRetryCallback = defaultShouldRetryCallback
    ): Any? {
        totalCalls++
        val retryAttemptsLeft = if (retriesLeft == USE_DEFAULT_RETRY_COUNT) {
            retries // set in constructor
        } else {
            retriesLeft // if called recursively
        }
        val address = dapiAddress ?: dapiAddressListProvider.getLiveAddress()
        val allowSelfSignedCertificate = dpp.params is DevNetParams
        val grpcMasternode = DAPIGrpcMasternode(address, timeOut, allowSelfSignedCertificate)
        lastUsedAddress = address

        logger.info(
            "grpcRequest(${grpcMethod.javaClass.simpleName}, $retriesLeft, $dapiAddress, $statusCheck) with" +
                    " ${address.host} for $grpcMethod"
        )

        val response: Any = try {
//            if (statusCheck) {
//                try {
//                    val status = getStatus(grpcMasternode.address, 0)
//
//                    if (status == null) {
//                        // throw exception and try another node
//                        throw StatusRuntimeException(Status.UNAVAILABLE)
//                    } else if (status.masternode.status == Masternode.Status.ERROR ||
//                        status.masternode.status == Masternode.Status.POSE_BANNED ||
//                        status.masternode.status == Masternode.Status.REMOVED ||
//                        status.masternode.status == Masternode.Status.WAITING_FOR_PROTX ||
//                        status.masternode.status == Masternode.Status.UNKNOWN
//                    ) {
//                        // see github.com/dashpay/dash/src/warnings.cpp
//                        logger.warn("${grpcMasternode.address} has this error state ${status.masternode.status.name}")
//                        // throw exception and try another node
//                        throw StatusRuntimeException(Status.UNAVAILABLE)
//                    }
//                } catch (e: StatusRuntimeException) {
//                    throwExceptionOnError(e)
//
//                    banMasternode(grpcMasternode.address, retriesLeft, e)
//                    // try another node
//                    retriedCalls++
//                    return grpcRequest(grpcMethod, retriesLeft, null, statusCheck, retryCallback)
//                } catch (e: org.dashj.platform.dapiclient.MaxRetriesReachedException) {
//                    banMasternode(grpcMasternode.address, retriesLeft, e.cause as StatusRuntimeException)
//                    // try another node
//                    retriedCalls++
//                    return grpcRequest(grpcMethod, retriesLeft, null, statusCheck, retryCallback)
//                }
//            }
            logger.debug("grpcMethod: executing method after statuscheck($statusCheck) for $grpcMethod")
            val response = grpcMethod.execute(grpcMasternode)
            successfulCalls++
            response
        } catch (e: StatusRuntimeException) {
            logException(e, grpcMasternode, grpcMethod)
            return when (e.status.code) {
                Status.NOT_FOUND.code -> {
                    if (!retryCallback.shouldRetry(grpcMethod, e)) {
                        return null
                    }

                    // only ban the node if the retry == true, meaning that the node
                    // returned an untrustworthy NOT_FOUND result
                    banMasternode(address, retryAttemptsLeft, e)
                    retriedCalls++
                    grpcRequest(grpcMethod, retryAttemptsLeft - 1, dapiAddress, statusCheck, retryCallback)
                }

                Status.CANCELLED.code -> {
                    return null
                }

                else -> {
                    throwExceptionOnError(e, retryCallback)

                    banMasternode(address, retryAttemptsLeft, e)
                    if (!retryCallback.shouldRetry(grpcMethod, e)) {
                        return null
                    }
                    retriedCalls++
                    grpcRequest(grpcMethod, retryAttemptsLeft - 1, dapiAddress, statusCheck, retryCallback)
                }
            }
        } finally {
            grpcMasternode.shutdown()
        }

        address.markAsLive()
        return response
    }

    private fun banMasternode(
        address: DAPIAddress,
        retryAttemptsLeft: Int,
        e: StatusRuntimeException
    ) {
        if (e.status.code != Status.CANCELLED.code) {
            logger.info("banning masternode $address")
            failedCalls++
            address.markAsBanned()
            address.addException(e.status.code)
            if (retryAttemptsLeft == 0) {
                throw MaxRetriesReachedException(e)
            }
            if (!dapiAddressListProvider.hasLiveAddresses()) {
                throw NoAvailableAddressesForRetryException(e)
            }
        }
    }

    private fun throwExceptionOnError(e: StatusRuntimeException, retryCallback: GrpcMethodShouldRetryCallback? = null) {
        if (retryCallback != null) {
            if (retryCallback.shouldThrowException(e)) {
                throw e
            }
        } else {
            if (e.status.code != Status.DEADLINE_EXCEEDED.code &&
                e.status.code != Status.UNAVAILABLE.code &&
                e.status.code != Status.INTERNAL.code &&
                e.status.code != Status.CANCELLED.code &&
                e.status.code != Status.UNKNOWN.code &&
                e.status.code != Status.ALREADY_EXISTS.code && // ST was already submitted
                e.status.code != Status.UNIMPLEMENTED.code // perhaps we contacted an old node
            ) {
                throw e
            }
        }
    }

    fun broadcastTransaction(txBytes: ByteArray, allowHighFees: Boolean = false, bypassLimits: Boolean = false):
            String {
        val method = BroadcastTransactionMethod(txBytes, allowHighFees, bypassLimits)
        val response = grpcRequest(method) as CoreOuterClass.BroadcastTransactionResponse?
        return response?.transactionId!!
    }

    /**
     *
     * @param txHex String
     * @return ByteString?
     */
    fun getTransactionBytes(txHex: String): ByteArray? {
        logger.info("getTransaction($txHex)")
        try {
            val transactionResult =
                dashsdk.platformMobileCoreGetTransactionSdk(
                    rustSdk,
                    Converters.fromHex(txHex)
                )
            return transactionResult.ok
        } catch (e: NullPointerException) {
            logger.error("transaction $txHex not found:", e)
            return null
        }
    }

    /**
     *
     * @param txIdHex String
     * @return GetTransactionResponse?
     */

    fun getTransaction(txIdHex: String): ByteArray? {
        logger.info("getTransaction($txIdHex)")
        return getTransactionBytes(txIdHex)
    }

    /**
     *
     * @param txHex String
     * @return ByteString?
     */
    fun getTransactionBytesKotlin(txHex: String): ByteArray? {
        logger.info("getTransactionBytesKotlin($txHex)")
        val method = GetTransactionMethod(txHex)
        val response = grpcRequest(method) as CoreOuterClass.GetTransactionResponse?
        return response?.transaction?.toByteArray()
    }

    fun getTransactionKotlin(txHex: String): GetTransactionResponse? {
        logger.info("getTransactionKotlin($txHex)")
        val method = GetTransactionMethod(txHex)
        val response = grpcRequest(method) as CoreOuterClass.GetTransactionResponse?
        return if (response != null) {
            GetTransactionResponse(
                response.transaction.toByteArray(),
                if (response.blockHash.size() != 0) {
                    Sha256Hash.wrap(response.blockHash.toByteArray())
                } else {
                    Sha256Hash.ZERO_HASH
                },
                response.height,
                response.confirmations,
                response.isInstantLocked,
                response.isChainLocked
            )
        } else {
            null
        }
    }

    // jRPC methods
    /**
     * Get the best block hash (tip of blockchain)
     * @return String?
     */
    fun getBestBlockHash(): String? {
        logger.info("getBestBlockHash(): jRPC")
        val service = getJRPCService()
        val response = service.getBestBlockHash(JsonRPCRequest("getBestBlockHash", mapOf())).execute()
        if (response.isSuccessful) {
            return response.body()!!.result
        } else {
            throw ResponseException(response.code(), response.errorBody().toString())
        }
    }

    /**
     * Get the block hash at the specified height
     * @return String?
     */
    fun getBlockHash(height: Int): String? {
        logger.info("getBlockHash(): jRPC")
        val service = getJRPCService()
        val parameters = mapOf("height" to height)
        val response = service.getBlockHash(JsonRPCRequest("getBlockHash", parameters)).execute()
        if (response.isSuccessful) {
            return response.body()!!.result
        } else {
            throw ResponseException(response.code(), response.errorBody().toString())
        }
    }

    /**
     * Get the masternode list difference between two block hashes
     * @return String?
     */
    fun getMnListDiff(baseBlockHash: String, blockHash: String): Map<String, Any>? {
        logger.info("getMnListDiff(): jRPC")
        val service = getJRPCService()
        val parameters = mapOf(
            "baseBlockHash" to baseBlockHash,
            "blockHash" to blockHash
        )
        val response = service.getMnListDiff(JsonRPCRequest("getMnListDiff", parameters)).execute()
        if (response.isSuccessful) {
            return response.body()!!.result
        } else {
            throw ResponseException(response.code(), response.errorBody().toString())
        }
    }

    // Internal Methods

    private fun getJRPCService(): DapiService {
        if (initializedJRPC) {
            return dapiService
        }

        val mnIP = dapiAddressListProvider.getLiveAddress().host

        logger.info("Connecting to GRPC host: $mnIP:${DAPIAddress.DEFAULT_JRPC_PORT}")

        retrofit = Retrofit.Builder()
            .addConverterFactory(GsonConverterFactory.create())
            .baseUrl("https://$mnIP:${DAPIAddress.DEFAULT_JRPC_PORT}/")
            .client(if (debugJrpc) debugOkHttpClient else OkHttpClient())
            .build()
        dapiService = retrofit.create(DapiService::class.java)

        return dapiService
    }

//    fun extractException(exception: StatusRuntimeException): ConcensusException {
//        val trailers = GrpcExceptionInfo(exception)
//        return trailers.exception
//    }

    fun setSimplifiedMasternodeListManager(
        masternodeListManager: SimplifiedMasternodeListManager,
        defaultList: List<String>
    ) {
        this.masternodeListManager = masternodeListManager
        dapiAddressListProvider = SimplifiedMasternodeListDAPIAddressProvider(
            masternodeListManager,
            ListDAPIAddressProvider.fromList(
                defaultList,
                DEFAULT_BASE_BAN_TIME
            )
        )

        // TODO: there is not a good way of handling EvoNodes without Evo
        // This code will get the most recent evonode list, but not all
        // can be used for platform
//        val evoNodeList = arrayListOf<String>()
//        masternodeListManager.masternodeList.forEachMN(true) {
//            if (it.isHPMN) {
//                evoNodeList.add(it.service.socketAddress.address.hostAddress)
//            }
//        }
        // This function does not yet exist in the DashSdk
//        if (evoNodeList.isNotEmpty()) {
//            rustSdk = dashsdk.platformMobileSdkCreateDashSdkWithContext(
//                contextProviderContext,
//                BigInteger.valueOf(contextProviderFunction),
//                BigInteger.ZERO,
//                evoNodeList,
//                isTestnet,
//                timeOut,
//                timeOut,
//                retries.toLong()
//            )
//        }
    }

    fun reportNetworkStatus(): String {
        return "DapiClient Network Status\n" +
                "---DAPI Call Statistics ($stopWatch)\n" +
                "   successful: $successfulCalls\n" +
                "   retried   : $retriedCalls\n" +
                "   failure   : $failedCalls\n" +
                "   total     : $totalCalls (calls per minute: ${
                    totalCalls.toDouble() /
                            stopWatch.elapsed(TimeUnit.MINUTES).toDouble()
                }\n" +
                "   retried % : ${retriedCalls.toDouble() / successfulCalls.toDouble() * 100}%\n" +
                "   success % : ${successfulCalls.toDouble() / totalCalls.toDouble() * 100}%\n" +
                "---Masternode Information\n" + dapiAddressListProvider.getStatistics()
    }

    fun reportErrorStatus(): String {
        return dapiAddressListProvider.getErrorStatistics()
    }

    fun getIdentityBalance(identifier: Identifier): Long {
        logger.info("getIdentityBalance({})", identifier)
        val exceptionList = arrayListOf<Exception>()
        var retriesLeft = retries
        do {
            val result = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(rustSdk, identifier.toNative())
            try {
                return result.unwrap()
            } catch (e: Exception) {
                if (e.message?.contains("context provider error: invalid quorum: quorum not found") == true) {
                    exceptionList.add(e)
                    retriesLeft--
                } else {
                    throw e
                }
            }
        } while (retriesLeft > 0)
        throw MaxRetriesReachedException(exceptionList)
    }

    fun getVoteContenders(
        dataContractId: Identifier,
        documentType: String,
        indexName: String,
        indexes: List<String>
    ): Contenders {
        logger.info("getVoteContenders({}, {}, {}, {})", dataContractId, documentType, indexName, indexes)
        val exceptionList = arrayListOf<Exception>()
        var retriesLeft = retries
        do {
            val result = dashsdk.platformMobileVotingGetVoteContenders(
                rustSdk,
                indexName,
                indexes.map { PlatformValue(it) },
                documentType,
                dataContractId.toNative()
            )
            try {
                return Contenders(result.unwrap())
            } catch (e: Exception) {
                if (e.message?.contains("context provider error: invalid quorum: quorum not found") == true) {
                    exceptionList.add(e)
                    retriesLeft--
                } else {
                    throw e
                }
            }
        } while (retriesLeft > 0)
        throw MaxRetriesReachedException(exceptionList)
    }

    fun getContestedResources(
        dataContractId: Identifier,
        documentType: String,
        limit: Int = 100,
        startAt: PlatformValue? = null,
        startAtInclude: Boolean = false
    ): ContestedResources {
        logger.info("getContestedResources({}, {}, {}, ...)", dataContractId, documentType, limit)
        val exceptionList = arrayListOf<Exception>()
        var retriesLeft = retries
        do {
            val result = dashsdk.platformMobileVotingGetContestedResources(
                rustSdk,
                documentType,
                dataContractId.toNative(),
                limit,
                startAt,
                startAtInclude
            )
            try {
                return ContestedResources(result.unwrap())
            } catch (e: Exception) {
                if (e.message?.contains("context provider error: invalid quorum: quorum not found") == true) {
                    exceptionList.add(e)
                    retriesLeft--
                } else {
                    throw e
                }
            }
        } while (retriesLeft > 0)
        throw MaxRetriesReachedException(exceptionList)
    }

    fun deserializeDocument(serializedDocument: ByteArray, dataContractId: Identifier, documentType: String): Document {
        return Document(
            dashsdk.platformMobileFetchDocumentDeserializeDocumentSdk(
                rustSdk,
                serializedDocument,
                dataContractId.toNative(),
                documentType
            ).unwrap(),
            dataContractId
        )
    }

    fun broadcastVote(
        vote: Vote,
        voterProTxHash: Sha256Hash,
        identityPublicKey: IdentityPublicKey,
        signerCallback: Signer
    ): Vote {
        logger.info("broadcastVote($vote, $voterProTxHash, $identityPublicKey, ...)")
        val result = dashsdk.platformMobileVotingPutVoteToPlatform(
            rustSdk,
            vote.toNative(),
            Identifier.from(voterProTxHash.bytes).toNative(),
            identityPublicKey.toNative(),
            signerCallback.nativeContext,
            BigInteger.valueOf(signerCallback.signerCallback)
        )

        return Vote(result.unwrap())
    }

    /**
     * Get vote polls within a specific time frame
     *
     * @param startTime
     * @param startTimeIncluded
     * @param endTime
     * @param endTimeIncluded
     * @param limit Number of records to return (default = 100)
     * @param orderAscending
     * @return [VotePollsGroupedByTimestamp]
     */
    fun getVotePolls(
        startTime: Long,
        startTimeIncluded: Boolean = true,
        endTime: Long,
        endTimeIncluded: Boolean = true,
        limit: Int = DEFAULT_LIMIT,
        orderAscending: Boolean = true
    ): VotePollsGroupedByTimestamp {
        logger.info("getVotePolls($startTime, $endTime, $limit, $orderAscending)")
        val exceptionList = arrayListOf<Exception>()
        var retriesLeft = retries
        do {
            val result = dashsdk.platformMobileVotingGetVotePolls(
                rustSdk,
                startTime.toTimestampMillis(),
                startTimeIncluded,
                endTime.toTimestampMillis(),
                endTimeIncluded,
                limit,
                0,
                orderAscending
            )
            try {
                return VotePollsGroupedByTimestamp(result.unwrap())
            } catch (e: Exception) {
                if (e.message?.contains("context provider error: invalid quorum: quorum not found") == true) {
                    exceptionList.add(e)
                    retriesLeft--
                } else {
                    throw e
                }
            }
        } while (retriesLeft > 0)
        throw MaxRetriesReachedException(exceptionList)
    }

    fun getLastVoteFromMasternode(
        protxHash: Sha256Hash,
        dataContractId: Identifier,
        documentType: String,
        indexName: String,
        indexes: List<String>
    ): ResourceVotesByIdentity {
        logger.info(
            "getLastVoteFromMasternode({}, {}, {}, {}, {})",
            protxHash,
            dataContractId,
            documentType,
            indexName,
            indexes
        )
        val exceptionList = arrayListOf<Exception>()
        var retriesLeft = retries
        do {
            val result = dashsdk.platformMobileVotingGetLastVoteFromMasternode(
                rustSdk,
                Identifier.from(protxHash).toNative(),
                indexName,
                indexes.map { PlatformValue(it) },
                documentType,
                dataContractId.toNative()
            )
            try {
                return ResourceVotesByIdentity(result.unwrap())
            } catch (e: Exception) {
                if (e.message?.contains("context provider error: invalid quorum: quorum not found") == true) {
                    exceptionList.add(e)
                    retriesLeft--
                } else {
                    throw e
                }
            }
        } while (retriesLeft > 0)
        throw MaxRetriesReachedException(exceptionList)
    }
}
