package org.dashj.platform.dapiclient

import com.google.common.base.Converter
import com.google.common.base.Preconditions.checkState
import com.google.common.base.Stopwatch
import com.hashengineering.crypto.X11
import io.grpc.Status
import io.grpc.Status.ALREADY_EXISTS
import io.grpc.Status.FAILED_PRECONDITION
import io.grpc.StatusRuntimeException
import org.bitcoinj.core.Base58
import org.bitcoinj.core.Block
import org.bitcoinj.core.Context
import org.bitcoinj.core.ECKey
import org.bitcoinj.core.Sha256Hash
import org.bitcoinj.core.Transaction
import org.bitcoinj.core.Utils
import org.bitcoinj.params.DevNetParams
import org.bitcoinj.params.TestNet3Params
//import org.dashj.dpp.DPP
import org.dashj.platform.dapiclient.errors.NotFoundException
import org.dashj.platform.dapiclient.model.DocumentQuery
import org.dashj.platform.dapiclient.provider.DAPIAddress
import org.dashj.platform.dapiclient.provider.ListDAPIAddressProvider
import org.dashj.platform.dpp.DashPlatformProtocol
import org.dashj.platform.dpp.document.Document
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.toHex
import org.dashj.platform.dpp.util.Cbor
import org.dashj.platform.dpp.util.Converters
import org.json.JSONObject
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import org.junit.jupiter.api.fail
import java.io.File

class DapiGrpcClientTest : BaseTest() {

    val PARAMS = TestNet3Params.get()
    val CONTEXT = Context.getOrCreate(PARAMS)
    // TODO: the default list has many non-EvoNodes
    //val masternodeList = PARAMS.defaultHPMasternodeList.toList()
    // copied from platform-mobile
    val masternodeList = listOf(
        "34.214.48.68",
        // "35.166.18.166",
        // "35.165.50.126",
        "52.42.202.128",
        // "52.12.176.90",
        // "44.233.44.95",
        // "35.167.145.149",
        //"52.34.144.50",
        "44.240.98.102",
        "54.201.32.131",
        "52.10.229.11",
        "52.13.132.146",
        "44.228.242.181",
        "35.82.197.197",
        // "52.40.219.41",
        "44.239.39.153",
        "54.149.33.167",
        "35.164.23.245",
        "52.33.28.47",
        "52.43.86.231",
        "52.43.13.92",
        // "35.163.144.230",
        "52.89.154.48",
        "52.24.124.162",
        "44.227.137.77",
        "35.85.21.179",
        "54.187.14.232",
        // "54.68.235.201",
        "52.13.250.182",
        //"35.82.49.196",
        //"44.232.196.6",
        // "54.189.164.39",
        //"54.213.204.85"
    )
    val dpnsContractId = SystemIds.dpnsDataContractId // DPNS contract
    val dashPayContractId = SystemIds.dashpayDataContractId
    val identityId = SystemIds.dpnsOwnerId
    val badDpnsContractId = Identifier.from("5BrYpaW5s26UWoBk9zEAYWxJANX7LFinmToprWo3VwgS") // DPNS contract
    val badDashPayContractId = Identifier.from("8FmdUoXZJijvARgA3Vcg73ThYp5P4AaLis1WpXp9VGg1")

    val identityIdString = SystemIds.dpnsOwnerId.toString()
    val stateRepository = StateRepositoryMock()
    val dpp = DashPlatformProtocol(stateRepository, PARAMS)

    // default client used in most unit tests
    private val client = DapiClient(masternodeList, dpp, false, true)

//    @Test
//    fun getStatusOfInvalidNodeTest() {
//        val watch = Stopwatch.createStarted()
//        val list = ListDAPIAddressProvider(listOf("211.30.243.83").map { DAPIAddress(it) }, 0)
//        val client = DapiClient(list, dpp, false, 3000, 0, 3)
//        try {
//            client.getStatus(DAPIAddress("211.30.243.82"), 0)
//            fail<Nothing>("The node queried should not exist")
//        } catch (e: Exception) {
//            if (e is NoAvailableAddressesForRetryException || e is MaxRetriesReachedException) {
//                println("timeout after $watch")
//                val cause = e.cause as StatusRuntimeException
//                if (cause.status.code != Status.UNAVAILABLE.code && cause.status.code != Status.DEADLINE_EXCEEDED.code) {
//                    fail<Nothing>("Invalid node test failed with a different error")
//                }
//                println(client.reportNetworkStatus())
//            }
//        }
//    }
//
    @Test
    fun getStatusTest() {
        assertThrows<MaxRetriesReachedException> {
            val status = client.getMasternodeStatus()
            println(status)
        }
    }

    @Test
    fun getBlockTest() {
        assertThrows<MaxRetriesReachedException> {
            val block1 = when (PARAMS) {
                is DevNetParams -> (PARAMS as DevNetParams).devNetGenesisBlock
                is TestNet3Params -> Block(PARAMS, Utils.HEX.decode("020000002cbcf83b62913d56f605c0e581a48872839428c92e5eb76cd7ad94bcaf0b00007f11dcce14075520e8f74cc4ddf092b4e26ebd23b8d8665a1ae5bfc41b58fdb4c3a95e53ffff0f1ef37a00000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0a510101062f503253482fffffffff0100743ba40b0000002321020131f38ae3eb0714531dbfc3f45491b4131d1211e3777177636388bb5a74c3e4ac00000000"))
                else -> fail("Invalid network")
            }
            val block1data = block1.bitcoinSerialize().toHex()
            val block1Hash = block1.hashAsString

            // request the block from the height
            val blockFromHeight = client.getBlockByHeight(1)
            assertEquals(block1data, blockFromHeight!!.toHex())

            // hash the block header and compare to the actual value
            val hash = Sha256Hash.wrapReversed(X11.x11Digest(blockFromHeight!!.take(80).toByteArray()))
            assertEquals(block1Hash, hash.toString())

            // request the block from the hash and compare to the block obtained from the height
            val blockFromHash = client.getBlockByHash(block1Hash)
            assertEquals(blockFromHeight.toHex(), blockFromHash!!.toHex())
        }
    }

    @Test
    fun getEstimatedFeeTest() {
        // request the block from the height
        assertThrows<MaxRetriesReachedException> {
            val fee = client.getEstimatedTransactionFee(1)
            assertEquals(0.0, fee)
        }
    }

    @Test
    fun getMasternodeInfoTest() {
        // request the block from the height
//        val fee = client.get(1)
//        assertEquals(0.0, fee)
    }

//    @Test
//    fun getDPNSContractTest() {
//        try {
//            val contract = client.getDataContract(dpnsContractId.toBuffer())!!
//
//            val jsonDpnsFile = File("src/test/resources/dpns-contract.json").readText()
//            val jsonDpns = JSONObject(jsonDpnsFile)
//            val rawContract = jsonDpns.toMap()
//            val dpnsContract = dpp.dataContract.createFromObject(rawContract)
//
//            assertEquals(dpnsContract.toJSON(), contract.toJSON())
//        } finally {
//        }
//    }

//    @Test
//    fun getDashPayContractTest() {
//        try {
//            val contractResponse = client.getDataContract(dashPayContractId.toBuffer())
//
//            val contract = dpp.dataContract.createFromBuffer(contractResponse.dataContract)
//            val jsonDpnsFile = File("src/test/resources/dashpay-contract.json").readText()
//            val jsonDpns = JSONObject(jsonDpnsFile)
//            val rawContract = jsonDpns.toMap()
//            val dpnsContract = dpp.dataContract.createFromObject(rawContract)
//
//            assertEquals(dpnsContract.toJSON(), contract.toJSON())
//        } finally {
//        }
//    }

    @Test
    fun getNonExistantContract() {
        val client = DapiClient(masternodeList.toList(), dpp, false, true)
        val contractId = Base58.decode("88w8Xqn25HwJhjodrHW133aXhjuTsTv9ozQaYpSHACE3")
        assertNull(client.getDataContract(contractId))
    }

    @Test
    fun getDocumentsTest() {
        val client = DapiClient(masternodeList.toList(), dpp, false, true)
        try {
            val query = DocumentQuery.Builder()
                .where("normalizedParentDomainName", "==", "dash")
                .orderBy("normalizedLabel", true)
                .build()
            val documentsResponse = client.getDocuments(dpnsContractId.toBuffer(), "domain", query)
            assertTrue(documentsResponse.isNotEmpty())
//            val jsonDpnsFile = File("src/test/resources/dpns-contract.json").readText()
//            val jsonDpns = JSONObject(jsonDpnsFile)
//            val rawContract = jsonDpns.toMap()
//            val dpnsContract = dpp.dataContract.createFromObject(rawContract)
//            stateRepository.storeDataContract(dpnsContract)
            val document = documentsResponse[0]

            val docs = ArrayList<Document>(documentsResponse.size)
            for (doc in documentsResponse) {
                docs.add(doc)
            }

            println(document.toJSON())
        } finally {
        }
    }

    @Test
    fun getDocumentsStartAtTest() {
        val client = DapiClient(masternodeList.toList(), dpp, false, true)
        try {
            val query = DocumentQuery.Builder()
                .orderBy("normalizedLabel")
                .build()
            val documentsResponse = client.getDocuments(dpnsContractId.toBuffer(), "domain", query)

//            val jsonDpnsFile = File("src/test/resources/dpns-contract.json").readText()
//            val jsonDpns = JSONObject(jsonDpnsFile)
//            val rawContract = jsonDpns.toMap()
//            val dpnsContract = dpp.dataContract.createFromObject(rawContract)
//            stateRepository.storeDataContract(dpnsContract)
            val document = documentsResponse[0]
            println(document.toJSON())

            val docs = ArrayList<Document>(documentsResponse.size)
            for (doc in documentsResponse) {
                docs.add(doc)
            }

            val queryWithStartAt = DocumentQuery.Builder()
                .orderBy("normalizedLabel")
                .startAt(docs.first().id)
                .build()

            val documentsStartAtResponse = client.getDocuments(dpnsContractId.toBuffer(), "domain", queryWithStartAt)

            val docsStartAt = ArrayList<Document>(documentsResponse.size)
            for (doc in documentsStartAtResponse) {
                docsStartAt.add(doc)
            }
            assertEquals(queryWithStartAt.startAt, docsStartAt.first().id)
        } finally {
        }
    }

    @Test
    fun getIdentityTest() {
        val id = "7633TgdebkBWnBQ7peF56mxLaGSTBxuzCavHYbN6ZW8V";
        val result = client.getIdentity(Identifier.from(id).toBuffer())
        //assertEquals(125, result.size)
        //print(IdentityFactory(dpp, stateRepository).createFromBuffer(result.identity).toJSON())
    }

    @Test
    fun getIdentityFromBadPubKeyBytes() {
        val key = ECKey()

        val identity = client.getIdentityByFirstPublicKey(key.pubKeyHash)
        assertEquals(null, identity)

        val identitiesResponse = client.getIdentityByFirstPublicKey(key.pubKeyHash)
        assertNull(identitiesResponse)
    }

    @Test
    fun getTransationTest() {
        val txid = "e43b58cf1f24d3366d7f6e96586586453b706aa3ef1226d2eff32b9b6c077e24"

        val response = client.getTransactionKotlin(txid)
        val result = response?.transaction

        println(response)
        println(Utils.HEX.encode(result!!))

        val resultTwo: ByteArray? = client.getTransactionBytesKotlin(txid)

        println(Utils.HEX.encode(resultTwo!!))

        for (i in 0..100) {
            val resultOfI: ByteArray? = client.getTransactionBytesKotlin(txid)
            println("$i. ${Utils.HEX.encode(resultOfI!!)}")
        }
    }

    @Test
    fun broadcastTransactionTest() {
        try {
            val txBytes =
                Converters.fromHex("03000500010000000000000000000000000000000000000000000000000000000000000000ffffffff060364b9110101ffffffff037a681d04000000001976a914c69a0bda7daaae481be8def95e5f347a1d00a4b488ac8815a10400000000016ae523b707000000001976a91464f2b2b84f62d68a2cd7f7f5fb2b5aa75ef716d788ac00000000af030064b9110094eebcf1fb7bc4da99704e4ff804737a45318f34727a1ed4fd1df8f29b7afdd2f5de49adf732ab6c32063000daf6e86b23f901c91b3e32e8e48ebae8d6ea1ccd00a1711858b3d289385250a017da892f67ce2c63db4c1734a8ee4b805fe9be3ec225efb5ce292ca459ce2c8efdd41466b811b21919947b4186f3f548bdaa50cb7828a220211a2f70810ca37037641c867d9854526344b786e97db9254ce0d8980265ac343a10070000")
            client.broadcastTransaction(txBytes)
            fail("this transaction is a coinbase tx on testnet")
        } catch (e: StatusRuntimeException) {
            checkState(FAILED_PRECONDITION.code == e.status.code)
        }

        try {

            val txBytes = client.getTransactionBytesKotlin(
                "d881e3bc6c47eea64adec49e31c311f882616a098bbfe1a6f42c644bea68dc85"
            )!!
            client.broadcastTransaction(txBytes)
            fail("this transaction already exists on testnet")
        } catch (e: StatusRuntimeException) {
            checkState(ALREADY_EXISTS.code == e.status.code)
        }
    }

    @Test
    fun getDocumentsFailureTest() {
        val query = DocumentQuery.Builder()
            .where("normalizedParentDomainName", "==", "dash")
            .where("normalizedLabel", "==", "hash")
            .build()
        val response = client.getDocuments(dpnsContractId.toBuffer(), "domain", query, false)
        println("documents: ${response.size}")

        val badContractQuery = DocumentQuery.Builder()
            .where("normalizedParentDomainName", "==", "dash")
            .where("normalizedLabel", "startsWith", "RT-")
            .orderBy("normalizedLabel", true)
            .build()
        val badContractResponse = client.getDocuments(dpnsContractId.toBuffer(), "domain", badContractQuery, false)
        println("documents: ${badContractResponse.size}")
    }

    @Test
    fun getDocumentsByQueryTests() {
        client.getDocuments(
            dashPayContractId.toBuffer(),
            "profile",
            DocumentQuery.builder()
                .where("\$ownerId", "==", Identifier.from("3HSUPuMgR5qpZt1y5NbE2BBheM11yLRXKZoqdsKgxVNt"))
                //.where("\$updatedAt", ">", 0)
                //.orderBy("\$updatedAt", true)
                .build()
        )

        client.getDocuments(
            dpnsContractId.toBuffer(),
            "domain",
            DocumentQuery.builder()
                .where("normalizedParentDomainName", "==", "dash")
                .where("normalizedLabel", "startsWith", "test")
                .orderBy("normalizedLabel", true)
                .build()
        )

        // orderBy with two operators
        client.getDocuments(
            dpnsContractId.toBuffer(),
            "domain",
            DocumentQuery.builder()
                .where("normalizedParentDomainName", "==", "dash")
                .where("normalizedLabel", "startsWith", "tut")
                .orderBy("normalizedLabel", true)
                .build()
        )

        // as of 0.22-dev-7 this query should fail
        // where clauses must be in a particular order
        // as of 0.23 this query will be successful
        client.getDocuments(
            dpnsContractId.toBuffer(),
            "domain",
            DocumentQuery.builder()
                .where("normalizedLabel", "startsWith", "test")
                .where("normalizedParentDomainName", "==", "dash")
                .orderBy("normalizedLabel", true)
                .build()
        )

        // use the reverse order as the previous query
        client.getDocuments(
            dpnsContractId.toBuffer(),
            "domain",
            DocumentQuery.builder()
                .where("normalizedParentDomainName", "==", "dash")
                .where("normalizedLabel", "startsWith", "test")
                .orderBy("normalizedLabel", true)
                .build()
        )

        // as of 0.22-dev-7 this query should fail
        // multiple ranges are not supported
        // in 0.23, this still fails
        // INVALID_ARGUMENT: Invalid query: where clause on non indexed property error: query must be for valid indexes
        assertThrows<Exception> {
            client.getDocuments(
                dashPayContractId.toBuffer(),
                "profile",
                DocumentQuery.builder()
                    .whereIn("\$ownerId", listOf(
                        Identifier.from("JB4ytoMv22noi4D71wFGzEBrwG74ys8ghjg7CEfZL9s7"),
                        Identifier.from("G9PKZFj5MZD6ptazLWA1zDXCh7g6ri4cPfinxpATapD7"),
                        Identifier.from("H8fQpvkxAtwh9F1iZZCRT7Vahfit1ZRXPYWvTb2w3TYX")
                    ))
                    .where("\$updatedAt", ">", 0)
                    .orderBy("\$updatedAt")
                    .orderBy("\$ownerId")
                    .build()
            )
        }

        val profiles = client.getDocuments(
            dashPayContractId.toBuffer(),
            "profile",
            DocumentQuery.builder()
                .whereIn("\$ownerId", listOf(
                    Identifier.from("JB4ytoMv22noi4D71wFGzEBrwG74ys8ghjg7CEfZL9s7"),
                    Identifier.from("G9PKZFj5MZD6ptazLWA1zDXCh7g6ri4cPfinxpATapD7"),
                    Identifier.from("H8fQpvkxAtwh9F1iZZCRT7Vahfit1ZRXPYWvTb2w3TYX")
                ))
                .orderBy("\$ownerId")
                .build()
        )
        println(profiles)

        client.getDocuments(
            dpnsContractId.toBuffer(),
            "preorder",
            DocumentQuery.builder()
                .where("saltedDomainHash", "==", Sha256Hash.ZERO_HASH.bytes)
                .build()
        )

        val domainsAsc = client.getDocuments(
            dpnsContractId.toBuffer(),
            "domain",
            DocumentQuery.builder()
                .where("records.identity", "<", Identifier.from("AYN4srupPWDrp833iG5qtmaAsbapNvaV7svAdncLN5Rh"))
                .orderBy("records.identity")
                //.limit(3)
                .build()
        )
        println("Ascending Order")
        domainsAsc.forEach {
            println(it.data["records"])
        }
        val domainsDesc = client.getDocuments(
            dpnsContractId.toBuffer(),
            "domain",
            DocumentQuery.builder()
                .where("records.identity", "<", Identifier.from("AYN4srupPWDrp833iG5qtmaAsbapNvaV7svAdncLN5Rh"))
                .orderBy("records.identity", false)
                //.limit(3)
                .build()
        )
        println("Descending Order")
        domainsDesc.forEach {
            println(it.data["records"]?.let { ((it as Map<String, Any?>)["identity"] as Identifier).toBuffer().toHex() })
        }
    }
}
