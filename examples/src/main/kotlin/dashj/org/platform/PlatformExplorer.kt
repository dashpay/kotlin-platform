package dashj.org.platform

import com.google.common.util.concurrent.SettableFuture
import org.bitcoinj.core.Base58
import org.bitcoinj.core.Context
import org.bitcoinj.core.ECKey
import org.bitcoinj.core.NetworkParameters
import org.bitcoinj.core.Sha256Hash
import org.bitcoinj.kits.WalletAppKit
import org.bitcoinj.net.discovery.ThreeMethodPeerDiscovery
import org.bitcoinj.params.MainNetParams
import org.bitcoinj.params.OuzoDevNetParams
import org.bitcoinj.params.RegTestParams
import org.bitcoinj.params.TestNet3Params
import org.bitcoinj.quorums.LLMQParameters
import org.bitcoinj.utils.BriefLogFormatter
import org.bitcoinj.utils.Threading
import org.bitcoinj.wallet.AuthenticationKeyChain
import org.bitcoinj.wallet.Wallet
import org.bitcoinj.wallet.WalletExtension
import org.bitcoinj.wallet.authentication.AuthenticationGroupExtension
import org.dashj.platform.dpp.toHex
import org.dashj.platform.dpp.util.Converters
import org.dashj.platform.sdk.ContestedDocumentVotePollWinnerInfo
import org.dashj.platform.sdk.DataContract
import org.dashj.platform.sdk.Document
import org.dashj.platform.sdk.Hash256
import org.dashj.platform.sdk.Identifier
import org.dashj.platform.sdk.Identity
import org.dashj.platform.sdk.OrderClause
import org.dashj.platform.sdk.PlatformValue
import org.dashj.platform.sdk.WhereClause
import org.dashj.platform.sdk.WhereOperator
import org.dashj.platform.sdk.callbacks.ContextProvider
import org.dashj.platform.sdk.callbacks.Signer
import org.dashj.platform.sdk.dashsdk
import java.io.File
import java.io.FileInputStream
import java.io.FileNotFoundException
import java.io.FileOutputStream
import java.io.FileWriter
import java.math.BigInteger
import java.text.SimpleDateFormat
import java.util.*


object PlatformExplorer {
    var monitorQuorumUsage = false
    val dateFormat: SimpleDateFormat
    var dpnsContractId: ByteArray = byteArrayOf(
        230.toByte(),
        104,
        198.toByte(),
        89,
        175.toByte(),
        102,
        174.toByte(),
        225.toByte(),
        231.toByte(),
        44,
        24,
        109,
        222.toByte(),
        123,
        91,
        126,
        10,
        29,
        113,
        42,
        9,
        196.toByte(),
        13,
        87,
        33,
        246.toByte(),
        34,
        191.toByte(),
        83,
        197.toByte(),
        49,
        85
    )
    val quitFuture = SettableFuture.create<Boolean>()
    init {
        System.loadLibrary("sdklib")
    };

    lateinit var signer: Signer
    lateinit var kit: WalletAppKit
    lateinit var authenticationGroupExtension: AuthenticationGroupExtension
    lateinit var fileOutputWriter: FileWriter

    init {
        val tz = TimeZone.getTimeZone("UTC")
        dateFormat = SimpleDateFormat("yyyy-MM-dd'T'HH:mm'Z'") // Quoted "Z" to indicate UTC, no timezone offset
        dateFormat.timeZone = tz
    }

    @JvmStatic
    fun main(args: Array<String>) {
        // This line makes the log output more compact and easily read, especially when using the JDK log adapter.
        BriefLogFormatter.initWithSilentBitcoinJ()
        if (args.isEmpty()) {
            System.err.println("Usage: [regtest|testnet|333|devnet]")
            return
        }

        // Figure out which network we should connect to. Each one gets its own set of files.
        val params: NetworkParameters
        val filePrefix: String
        var checkpoints: String? = null

        if (args[0] == "testnet") {
            params = TestNet3Params.get()
            filePrefix = "platform-explorer-testnet"
            checkpoints = "checkpoints-testnet.txt"
        } else if (args[0] == "regtest") {
            params = RegTestParams.get()
            filePrefix = "platform-explorer-regtest"
        } else if (args[0] == "ouzo") {
            params = OuzoDevNetParams.get()
            filePrefix = "platform-explorer-ouzo"
        } else {
            params = MainNetParams.get()
            filePrefix = "platform-explorer"
            checkpoints = "checkpoints.txt"
        }

        // Start up a basic app using a class that automates some boilerplate.
        kit = object : WalletAppKit(params, File("."), filePrefix) {
            override fun createWallet(): Wallet {
                val wallet =  super.createWallet()
                // create the wallet
                authenticationGroupExtension = AuthenticationGroupExtension(params)
                wallet.addExtension(authenticationGroupExtension)
                authenticationGroupExtension.addKeyChains(
                    params, wallet.keyChainSeed,
                    EnumSet.of(
                        AuthenticationKeyChain.KeyChainType.BLOCKCHAIN_IDENTITY,
                        AuthenticationKeyChain.KeyChainType.BLOCKCHAIN_IDENTITY_FUNDING,
                        AuthenticationKeyChain.KeyChainType.BLOCKCHAIN_IDENTITY_TOPUP,
                        AuthenticationKeyChain.KeyChainType.INVITATION_FUNDING
                    )
                )
                return wallet
            }

            override fun provideWalletExtensions(): MutableList<WalletExtension> {
                return mutableListOf(AuthenticationGroupExtension(params))
            }

            override fun onSetupCompleted() {
                //TODO: init auth keychains using AuthenticationGroupExtension
                peerGroup().maxConnections = 6 // for small devnets
                peerGroup().useLocalhostPeerWhenPossible = false
                peerGroup()
                    .setDropPeersAfterBroadcast(params.dropPeersAfterBroadcast)
                wallet().context.isDebugMode = false

                signer = object : Signer() {
                    override fun sign(key: ByteArray, data: ByteArray): ByteArray {

                        return ByteArray(65)
                    }
                }
                authenticationGroupExtension = wallet().getKeyChainExtension(AuthenticationGroupExtension.EXTENSION_ID) as AuthenticationGroupExtension
            }
        }
        kit.setDiscovery(
            ThreeMethodPeerDiscovery(
                params,
                Context.get().masternodeListManager
            )
        )

        if (params === RegTestParams.get()) {
            // Regression test mode is designed for testing and development only, so there's no public network for it.
            // If you pick this mode, you're expected to be running a local "bitcoind -regtest" instance.
            kit.connectToLocalHost()
        }

        if (checkpoints != null) {
            try {
                val checkpointStream = FileInputStream("./$checkpoints")
                kit.setCheckpoints(checkpointStream)
            } catch (x: FileNotFoundException) {
                //swallow
            }
        }

        // Download the block chain and wait until it's done.
        kit.startAsync()
        kit.awaitRunning()

        kit.chain().addNewBestBlockListener{ block ->

        }

        kit.peerGroup().addMnListDownloadCompleteListener( {
            println(Context.get().masternodeListManager)
            run()
        }, Threading.USER_THREAD)

        try {
            quitFuture.get()
        } catch (ignored: InterruptedException) {
        }
    }

    private fun run() {
        val contextProvider = object : ContextProvider() {
            override fun getQuorumPublicKey(
                quorumType: Int,
                quorumHashBytes: ByteArray?,
                coreChainLockedHeight: Int
            ): ByteArray? {
                val quorumHash = Sha256Hash.wrap(quorumHashBytes)
                var quorumPublicKey: ByteArray? = null
                if (monitorQuorumUsage) {
                    fileOutputWriter.write(
                        dateFormat.format(Date()) +
                            ", " + Sha256Hash.wrap(quorumHashBytes) +
                            ", " + coreChainLockedHeight +
                            ", " + kit.chain().bestChainHeight +
                            "\n"
                    )
                    fileOutputWriter.flush()
                }
                println("searching for quorum: $quorumType, $quorumHash, $coreChainLockedHeight")
                kit.wallet().context.masternodeListManager.getQuorumListAtTip(
                    LLMQParameters.LLMQType.fromValue(
                        quorumType
                    )
                ).forEachQuorum(true) {
                    if (it.llmqType.value == quorumType && it.quorumHash == quorumHash) {
                        quorumPublicKey = it.quorumPublicKey.serialize(false)
                    }
                }
                println("searching for quorum: result: ${quorumPublicKey?.toHex()}")
                return quorumPublicKey
            }

            override fun getDataContract(identifier: Identifier?): ByteArray {
                return byteArrayOf(0)
            }
        }
        val sdk = dashsdk.platformMobileSdkCreateDashSdkWithContext(
            contextProvider.nativeContext,
            BigInteger.valueOf(contextProvider.quorumPublicKeyCallback),
            BigInteger.ZERO,
            kit.params().id != NetworkParameters.ID_MAINNET,
            5,
            5,
            5
        )

        println("4EfA9Jrvv3nnCFdSf7fad59851iiTRZ6Wcu6YVJ4iSeF")
        val scanner = Scanner(System.`in`)
        var quit = false
        while (!quit) {
            println()
            println("Platform Explorer")
            println("-----------------")
            println("Main Menu")
            println("1.  Query Identity from id")
            println("2.  Query Identity from public key hash")
            println("3.  Query Identity from public key")
            println("4.  Query a random DPNS document")
            println("5.  Query all DPNS DOMAIN documents")
            println("6a. Query DPNS DOMAIN documents for name")
            println("6b. Query DPNS DOMAIN documents for id")
            println("6c. Query DPNS DOMAIN documents for identity")
            println("7.  Query DPNS DOMAIN documents starting with")
            println("8.  Query Data contract by id")
            println("10. Query all contested resources for DOMAIN")
            println("11. Query votes for DOMAIN documents for a name")
            println("12. Monitor Quorum Usage")


            println("w. Wallet info")
            println("q. Quit")
            val menuItem = scanner.nextLine()
            when (menuItem) {
                "1" -> {
                    println("Enter an identity id:")
                    val idString = scanner.nextLine()

                    println(" > $idString")

                    val value = dashsdk.platformMobileFetchIdentityFetchIdentityWithSdk(
                        sdk,
                        Identifier(Base58.decode(idString))
                    )
                    try {

                        val identityOptional = value.unwrap()
                        if (identityOptional.isPresent) {
                            print(identityOptional.get())
                        } else {
                            print("identity not found")
                        }
                    } catch (e: Exception) {
                        println("fetch identity error: ${value.unwrapError()}")
                    }
                }

                "2" -> {
                    println("Enter a public key hash:")
                    val publicKeyHashString = scanner.nextLine()

                    println(" > $publicKeyHashString")

                    val value = dashsdk.platformMobileFetchIdentityFetchIdentityWithKeyhashSdk(
                        sdk,
                        Converters.fromHex(publicKeyHashString)
                    )
                    try {
                        val identity = value.unwrap();
                        print(identity)
                    } catch (e: Exception) {
                        println("fetch identity error: ${value.unwrapError()}")
                    }
                }
                "3" -> {
                    println("Enter a public key:")
                    val publicKeyString = scanner.nextLine()
                    val publicKey = ECKey.fromPublicOnly(Converters.fromHex(publicKeyString))
                    println(" > $publicKeyString")
                    println(" > ${publicKey.pubKeyHash.toHex()}")

                    val value = dashsdk.platformMobileFetchIdentityFetchIdentityWithKeyhashSdk(
                        sdk,
                        publicKey.pubKeyHash
                    )
                    try {

                        val identity = value.unwrap();
                        print(identity)
                    } catch (e: Exception) {
                        println("fetch identity error: ${value.unwrapError()}")
                    }
                }
                "4" -> {
                    println("This function does not exist, replace it.")
                }
                "5" -> {
                    try {
                        val result = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                            sdk, Identifier(dpnsContractId),
                            "domain", listOf(), listOf(), 100, null
                        )
                        val docs = result.unwrap()
                        docs.forEach { doc ->
                            printDomainDocument(doc)
                        }
                    } catch (e: Exception) {
                        println("error!")
                    }
//                    docs.forEach { doc ->
//                        printDocument(doc)
//                    }
                }
                "6a" -> {
                    println("Enter the username:")
                    val startsWith = scanner.nextLine()

                    println(" > $startsWith")

                    val docs = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                        sdk,
                        Identifier(dpnsContractId),
                        "domain",
                        listOf(
                            WhereClause("normalizedLabel", WhereOperator.Equal, PlatformValue(startsWith)),
                            WhereClause("normalizedParentDomainName", WhereOperator.Equal, PlatformValue("dash"))
                        ),
                        listOf(OrderClause("normalizedLabel", true)),
                        100,
                        null
                    )

                    docs.unwrap().forEach { doc ->
                        printDomainDocument(doc)
                    }
                }
                "6b" -> {
                    println("Enter an id:")
                    val id = scanner.nextLine()
                    val identifier = Identifier(Base58.decode(id))
                    println(" > $id")
                    val docs_result = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                        sdk,
                        Identifier(dpnsContractId),
                        "domain",
                        listOf(WhereClause("\$id", WhereOperator.Equal, PlatformValue(Hash256(identifier.bytes)))),
                        listOf(OrderClause("\$id", true)),
                        100,
                        null
                    )
                    try {
                        val docs = docs_result.unwrap()
                        if (docs.isNotEmpty()) {
                            docs.forEach { doc ->
                                printDomainDocument(doc)
                                printDocument(doc)
                            }
                        } else {
                            println("no document found for $id")
                        }
                    } catch (e: Exception) {
                        println("error retrieving document for $id")
                    }
                }
                "6c" -> {
                    println("Enter an identity id:")
                    val id = scanner.nextLine()
                    val identifier = Identifier(Base58.decode(id))
                    println(" > $id")
                    val docs_result = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                        sdk,
                        Identifier(dpnsContractId),
                        "domain",
                        listOf(WhereClause("records.identity", WhereOperator.Equal, PlatformValue(Hash256(identifier.bytes)))),
                        listOf(OrderClause("records.identity", true)),
                        100,
                        null
                    )
                    try {
                        val docs = docs_result.unwrap()
                        if (docs.isNotEmpty()) {
                            docs.forEach { doc ->
                                printDomainDocument(doc)
                                printDocument(doc)
                            }
                        } else {
                            println("no document found for $id")
                        }
                    } catch (e: Exception) {
                        println("error retrieving document for $id")
                    }
                }
                "7" -> {
                    println("Enter the start of a username:")
                    val startsWith = scanner.nextLine()

                    println(" > $startsWith")

                    val docs = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                        sdk,
                        Identifier(dpnsContractId),
                        "domain",
                        listOf(
                            WhereClause("normalizedLabel", WhereOperator.StartsWith, PlatformValue(startsWith)),
                            WhereClause("normalizedParentDomainName", WhereOperator.Equal, PlatformValue("dash"))
                        ),
                        listOf(OrderClause("normalizedLabel", true)),
                        100,
                        null
                    )

                    docs.unwrap().forEach { doc ->
                        printDomainDocument(doc)
                    }
                }
                "8" -> {
                    println("Enter an data contract id:")
                    val idString = scanner.nextLine()

                    println(" > $idString")

                    val value = dashsdk.platformMobileDataContractsFetchDataContract(
                        sdk,
                        Identifier(Base58.decode(idString)),
                    )
                    try {

                        val dataContract = value.unwrap();
                        print(dataContract)
                    } catch (e: Exception) {
                        println("fetch identity error: ${value.unwrapError()}")
                    }
                }
                "9" -> {
                    var currentKey = authenticationGroupExtension.currentKey(AuthenticationKeyChain.KeyChainType.BLOCKCHAIN_IDENTITY)
                    val identityResult = dashsdk.platformMobileFetchIdentityFetchIdentityWithKeyhashSdk(
                        sdk,
                        currentKey.pubKeyHash
                    )

                    try {
                        identityResult.unwrap()
                        currentKey = authenticationGroupExtension.freshKey(AuthenticationKeyChain.KeyChainType.BLOCKCHAIN_IDENTITY)
                    } catch (e: Exception) {
                        // do nothing
                    }

                    val identity = dashsdk.getIdentity2(Identifier(ByteArray(32)))
                    //val result = dashsdk.platformMobilePutPutIdentityCreate(identity, BigInteger.valueOf(signer.signerCallback))
                    //print(result)
                }
                "9a" -> {
                    val dpnsContractId = byteArrayOf(
                        230.toByte(),
                        104,
                        198.toByte(),
                        89,
                        175.toByte(),
                        102,
                        174.toByte(),
                        225.toByte(),
                        231.toByte(),
                        44,
                        24,
                        109,
                        222.toByte(),
                        123,
                        91,
                        126,
                        10,
                        29,
                        113,
                        42,
                        9,
                        196.toByte(),
                        13,
                        87,
                        33,
                        246.toByte(),
                        34,
                        191.toByte(),
                        83,
                        197.toByte(),
                        49,
                        85
                    )
                    println(Base58.encode(dpnsContractId))
                }
                "10" -> {
                    val result = dashsdk.platformMobileVotingGetContestedResources(
                        sdk,
                        "domain",
                        Identifier(dpnsContractId),
                        100,
                        null,
                        false
                    )
                    val contestedResources = result.unwrap()
                    val list = contestedResources._0
                    println("${list.size} contested resources: ")
                    for (item in list) {
                        println(item.value.text)
                    }

                    val result2 = dashsdk.platformMobileVotingGetContestedResources(
                        sdk,
                        "domain",
                        Identifier(dpnsContractId),
                        100,
                        contestedResources._0.lastOrNull()?._0,
                        false
                    )
                    val contestedResources2 = result2.unwrap()
                    val list2 = contestedResources2._0
                    println("${list2.size} contested resources: ")
                    for (item in list2) {
                        println(item.value.text)
                    }
                }
                "11" -> {
                    println("Enter an name:")
                    val name = scanner.nextLine()

                    println(" > $name")

                    val indexes = ArrayList<PlatformValue>()
                    indexes.add(PlatformValue("dash"))
                    indexes.add(PlatformValue(name))
                    val result = dashsdk.platformMobileVotingGetVoteContenders(
                        sdk,
                        "parentNameAndLabel",
                        indexes,
                        "domain",
                        Identifier(dpnsContractId)
                    )
                    val contenders = result.unwrap()

                    println("Contenders: " + contenders.contenders.size)
                    if (contenders.winner != null) {
                        print("  Winner: " + contenders.winner.o_0.tag + " ")
                        if (contenders.winner.o_0.tag == ContestedDocumentVotePollWinnerInfo.Tag.WonByIdentity) {
                            print(Base58.encode(contenders.winner.o_0.won_by_identity._0.bytes))
                        }
                        println()
                    }
                    println("  Abstain: " + contenders.abstainVoteTally)
                    println("  Lock: " + contenders.lockVoteTally)
                    for ((key, value) in contenders.contenders) {
                        println("  Identifier: " + Base58.encode(key._0._0))
                        val serializedDocument = value.v0._0.serialized_document
                        println("  Serialized:" + if (serializedDocument != null) {
                            Base64.getEncoder().encodeToString(serializedDocument)
                        } else {
                            "null"
                        })
                        println("  Votes: " + value.v0._0.voteTally)
                    }
                }
                "12" -> {
                    println("Monitor Quorum Usage")
                    //fileOutputStream = FileOutputStream("platform-quorum-usage-" + dateFormat.format(Date()))
                    fileOutputWriter = FileWriter("platform-quorum-usage-" + dateFormat.format(Date()) + ".txt")
                    fileOutputWriter.write("Time, Quorum Hash, Block Index\n")
                    var quitMonitor = false
                    monitorQuorumUsage = true
                    while(!quitMonitor) {
                        try {
                            dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                                sdk,
                                Identifier(dpnsContractId),
                                "domain",
                                listOf(),
                                listOf(),
                                5,
                                null
                            )
                        } catch(e: Exception) {
                            e.printStackTrace()
                        }
                        Thread.sleep(60*1000);
                    }
                }
                "w" -> {
                    println("Wallet")
                    println("--------")
                    println("balance:         ${kit.wallet().balance.toFriendlyString()}")
                    println("current address: ${kit.wallet().currentReceiveAddress()}")
                }
                "q", "Q" -> {
                    quit = true
                }
            }
        }
        dashsdk.platformMobileSdkDestroyDashSdk(sdk)
        contextProvider.close()
        quitFuture.set(true)
    }

    private fun printDocument(doc: Document) {
        if (doc.tag == Document.Tag.V0) {
            val docv0 = doc.v0._0
            println("Document")
            println("---------")
            println("id:         ${Base58.encode(docv0.id._0._0)}")
            println("ownerId:    ${Base58.encode(docv0.owner_id._0._0)}")
            println("rev:        ${docv0.revision.toLong()}")
            docv0.created_at?.let {
                println("created:    ${Date(docv0.created_at.toLong())}")
            }
            docv0.updated_at?.let {
                println("updated:    ${Date(docv0.updated_at.toLong())}")
            }
            println("properties: {")
            docv0.properties.forEach { (key, value) ->
                val strValue = printValue(value)
                println("  $key:$strValue")
            }
            println("}")
        } else {
            println("returned document is of an unknown version");
        }
    }

    private fun printDomainDocument(doc: Document) {
        if (doc.tag == Document.Tag.V0) {
            val docv0 = doc.v0._0
            print(Base58.encode(docv0.owner_id._0._0))
            print(" ")
            val properties = docv0.properties
            val label = properties["label"]?.text
            print("$label ")
            val records = properties["records"]
            val recordsMap = records?.map?._0
            val record = recordsMap?.keys?.first()?.text
            if (record == "dashAliasIdentityId") {
                print("alias ->")
                print(Base58.encode(recordsMap.values.first().identifier.bytes))
            } else if (record == "identity") {
                print("unique ")
                print(Base58.encode(recordsMap.values.first().identifier.bytes))

            }
            println()
        } else {
            println("returned document is of an unknown version");
        }
    }

    private fun print(identity: Identity) {
        println()
        println("Identity Results:")
        println("version: ${identity.tag}")
        when (identity.tag) {
            Identity.Tag.V0 -> {
                println("id: ${org.dashj.platform.dpp.identifier.Identifier(identity.v0._0.id._0._0)}")
                println("balance: ${identity.v0._0.balance}")
                println("keys: ${identity.v0._0.publicKeyCount}")
                identity.v0._0.publicKeys.forEach {
                    val ipkv0 = it.value.v0._0
                    println(" ${ipkv0.id.toInt()} ${ipkv0.keyType} ${ipkv0.purpose} ${ipkv0.securityLevel} ${ipkv0.data._0.toHex()}")
                }
            }
            else -> {
                println("This version is not support")
            }
        }
    }

    private fun print(dataContract: DataContract) {
        println()
        println("Data Contract Results:")

        println("id: ${org.dashj.platform.dpp.identifier.Identifier(dataContract.id)}")
        println("doc types:")
        dataContract.doc_types.forEach { type ->
            println(" $type")
        }
    }

    private fun printValue(value: PlatformValue, indent: Int = 2): String {
        val indentStr = " ".repeat(indent)
        val strValue = when(value.tag) {
            PlatformValue.Tag.Bool -> value.bool.toString()
            PlatformValue.Tag.Text -> value.text
            PlatformValue.Tag.Identifier -> Base58.encode(value.identifier.bytes)
            PlatformValue.Tag.Map -> {
                " {\n$indentStr" +
                value.map._0.map { (k, v) ->
                    printValue(k, indent + 2) + ":" + printValue(v, indent + 2)
                }.joinToString("\n") +
                "\n$indentStr}"
            }
            PlatformValue.Tag.Array -> {
                " [\n$indentStr" +
                        value.array.joinToString(",\n") { v ->
                            printValue(v, indent + 2)
                        } +
                        "\n$indentStr]"
            }
            else -> value.toString()
        }
        return "$indentStr$strValue"
    }
}