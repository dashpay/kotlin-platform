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
import org.dashj.platform.sdk.DataContract
import org.dashj.platform.sdk.Document
import org.dashj.platform.sdk.Identifier
import org.dashj.platform.sdk.Identity
import org.dashj.platform.sdk.PlatformValue
import org.dashj.platform.sdk.callbacks.ContextProvider
import org.dashj.platform.sdk.callbacks.Signer
import org.dashj.platform.sdk.dashsdk
import java.io.File
import java.io.FileInputStream
import java.io.FileNotFoundException
import java.math.BigInteger
import java.util.*


object PlatformExplorer {
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
                println("searching for quorum: $quorumType, $quorumHash, $coreChainLockedHeight")
                Context.get().masternodeListManager.getQuorumListAtTip(
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
        val sdk = dashsdk.platformMobileConfigCreateSdk(BigInteger.valueOf(contextProvider.quorumPublicKeyCallback), BigInteger.ZERO)

        println("4EfA9Jrvv3nnCFdSf7fad59851iiTRZ6Wcu6YVJ4iSeF")
        val scanner = Scanner(System.`in`)
        var quit = false
        while (!quit) {
            println()
            println("Platform Explorer")
            println("-----------------")
            println("Main Menu")
            println("1. Query Identity from id")
            println("2. Query Identity from public key hash")
            println("3. Query Identity from public key")
            println("4. Query a random DPNS document")
            println("5. Query all DPNS DOMAIN documents")
            println("6. Query DOMAIN documents for id")
            println("7. Query DPNS DOMAIN documents starting with")
            println("8. Query Data contract by id")

            println("w. Wallet info")
            println("q. Quit")
            val menuItem = scanner.nextLine()
            when (menuItem) {
                "1" -> {
                    println("Enter an identity id:")
                    val idString = scanner.nextLine()

                    println(" > $idString")

                    val value = dashsdk.fetchIdentity(
                        Identifier(Base58.decode(idString)),
                        BigInteger.valueOf(contextProvider.quorumPublicKeyCallback),
                        BigInteger.ZERO
                    )
                    try {

                        val identity = value.unwrap();
                        print(identity)
                    } catch (e: Exception) {
                        println("fetch identity error: ${value.unwrapError()}")
                    }
                }

                "2" -> {
                    println("Enter a public key hash:")
                    val publicKeyHashString = scanner.nextLine()

                    println(" > $publicKeyHashString")

                    val value = dashsdk.getIdentityByPublicKeyHash(
                        Converters.fromHex(publicKeyHashString),
                        BigInteger.valueOf(contextProvider.quorumPublicKeyCallback),
                        BigInteger.ZERO
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

                    val value = dashsdk.getIdentityByPublicKeyHash(
                        publicKey.pubKeyHash,
                        BigInteger.valueOf(contextProvider.quorumPublicKeyCallback),
                        BigInteger.ZERO
                    )
                    try {

                        val identity = value.unwrap();
                        print(identity)
                    } catch (e: Exception) {
                        println("fetch identity error: ${value.unwrapError()}")
                    }
                }
                "4" -> {
                    val doc = dashsdk.platformMobileFetchDocumentGetDocument()
                    printDocument(doc)
                }
                "5" -> {
                    val docs = dashsdk.platformMobileFetchDocumentGetDocuments(
                        Identifier(dpnsContractId),
                        "domain",
                        BigInteger.valueOf(contextProvider.quorumPublicKeyCallback),
                        BigInteger.ZERO)
                    docs.forEach { doc ->
                        printDomainDocument(doc)
                    }
                }
                "6" -> {
                    println("Enter an id:")
                    val id = scanner.nextLine()
                    val identifier = Identifier(Base58.decode(id))
                    println(" > $id")
                    val docs = dashsdk.platformMobileFetchDocumentGetDomainDocument(
                        identifier,
                        BigInteger.valueOf(contextProvider.quorumPublicKeyCallback),
                        BigInteger.ZERO
                    )
                    if (docs.isNotEmpty()) {
                        docs.forEach { doc ->
                            printDomainDocument(doc)
                            printDocument(doc)
                        }
                    } else {
                        println("no document found for $id")
                    }
                }
                "7" -> {
                    println("Enter the start of a username:")
                    val startsWith = scanner.nextLine()

                    println(" > $startsWith")

                    val docs = dashsdk.platformMobileFetchDocumentGetDomainDocumentStartsWith(
                        startsWith,
                        BigInteger.valueOf(contextProvider.quorumPublicKeyCallback),
                        BigInteger.ZERO)
                    docs.forEach { doc ->
                        printDomainDocument(doc)
                    }
                }
                "8" -> {
                    println("Enter an data contract id:")
                    val idString = scanner.nextLine()

                    println(" > $idString")

                    val value = dashsdk.platformMobileDataContractsFetchDataContract(
                        Identifier(Base58.decode(idString)),
                        BigInteger.valueOf(contextProvider.quorumPublicKeyCallback),
                        BigInteger.ZERO
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
                    val identityResult = dashsdk.getIdentityByPublicKeyHash(
                        currentKey.pubKeyHash,
                        BigInteger.valueOf(contextProvider.quorumPublicKeyCallback),
                        BigInteger.ZERO
                    )

                    try {
                        identityResult.unwrap()
                        currentKey = authenticationGroupExtension.freshKey(AuthenticationKeyChain.KeyChainType.BLOCKCHAIN_IDENTITY)
                    } catch (e: Exception) {
                        // do nothing
                    }

                    val identity = dashsdk.getIdentity2(Identifier(ByteArray(32)))
                    val result = dashsdk.platformMobilePutPutIdentityCreate(identity, BigInteger.valueOf(signer.signerCallback))
                    print(result)
                }
                "9" -> {
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
            } else if (record == "dashUniqueIdentityId") {
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