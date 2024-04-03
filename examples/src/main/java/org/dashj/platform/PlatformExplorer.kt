package org.dashj.platform

import com.google.common.util.concurrent.SettableFuture
import org.bitcoinj.core.*
import org.bitcoinj.kits.WalletAppKit
import org.bitcoinj.net.discovery.ThreeMethodPeerDiscovery
import org.bitcoinj.params.MainNetParams
import org.bitcoinj.params.OuzoDevNetParams
import org.bitcoinj.params.RegTestParams
import org.bitcoinj.params.TestNet3Params
import org.bitcoinj.quorums.LLMQParameters
import org.bitcoinj.utils.BriefLogFormatter
import org.bitcoinj.utils.Threading
import org.dashj.platform.dpp.toHex
import org.dashj.platform.dpp.util.Converters
import org.dashj.platform.sdk.Identifier
import org.dashj.platform.sdk.Identity
import org.dashj.platform.sdk.callbacks.ContextProvider;
import org.dashj.platform.sdk.example

import java.io.File
import java.io.FileInputStream
import java.io.FileNotFoundException
import java.math.BigInteger
import java.util.*

object PlatformExplorer {

    val quitFuture = SettableFuture.create<Boolean>()
    init {
        System.loadLibrary("sdklib")
    };


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
        val kit = object : WalletAppKit(params, File("."), filePrefix) {
            override fun onSetupCompleted() {
                //TODO: init auth keychains using AuthenticationGroupExtension
                peerGroup().maxConnections = 6 // for small devnets
                peerGroup().useLocalhostPeerWhenPossible = false
                peerGroup()
                    .setDropPeersAfterBroadcast(params.dropPeersAfterBroadcast)
                wallet().context.isDebugMode = false
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

        val identifier = example.platformMobileFetchIdentityGetDocumentWithCallbacks(BigInteger.valueOf(contextProvider.quorumPublicKeyCallback), BigInteger.ZERO)
        println(Base58.encode(identifier._0._0))

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
            println("q. Quit")
            val menuItem = scanner.nextLine()
            when (menuItem) {
                "1" -> {
                    println("Enter an identity id:")
                    val idString = scanner.nextLine()

                    println(" > $idString")

                    val value = example.fetchIdentity(
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

                    val value = example.getIdentityByPublicKeyHash(
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

                    val value = example.getIdentityByPublicKeyHash(
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
                "q", "Q" -> {
                    quit = true
                }
            }
        }
        quitFuture.set(true)
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
}