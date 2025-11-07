/**
 * Copyright (c) 2022-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
package dashj.org.platform

import org.bitcoinj.params.TestNet3Params
import org.bitcoinj.wallet.AuthenticationKeyChain
import org.bitcoinj.wallet.DerivationPathFactory
import org.bitcoinj.wallet.DeterministicKeyChain
import org.bitcoinj.wallet.DeterministicSeed
import org.bitcoinj.wallet.KeyChainGroup
import org.bitcoinj.wallet.Wallet
import org.bitcoinj.wallet.authentication.AuthenticationGroupExtension
import org.dashj.platform.contracts.wallet.TxMetadataDocument
import org.dashj.platform.wallet.TxMetadataItem
import org.dashj.platform.dashpay.BlockchainIdentity
import org.dashj.platform.dpp.util.Converters
import org.dashj.platform.dpp.util.Entropy
import org.dashj.platform.sdk.Client
import org.dashj.platform.sdk.client.ClientOptions
import org.dashj.platform.sdk.client.WalletOptions
import org.dashj.platform.sdk.dashsdk
import org.dashj.platform.sdk.platform.Platform
import org.dashj.platform.wallet.TxMetadata
import org.json.JSONObject
import java.util.*

object CreateTxMetadataTest {

    val platform = Platform(TestNet3Params.get())
    //val platform = Platform(MainNetParams.get())
    val txMetadata = TxMetadata(platform)
    val seed = "caution kitchen month seven humble olympic author session dutch direct decrease moment"
    // val seed = "half sample spirit edit dawn humor eight refuse hundred suit critic print"
    val wallet: Wallet = Wallet(
        platform.params,
        KeyChainGroup.builder(platform.params)
            .addChain(
                DeterministicKeyChain.builder()
                    .accountPath(DerivationPathFactory.get(platform.params).bip44DerivationPath(0))
                    .seed(DeterministicSeed(seed, null, "", Date().time))
                    .build()
            )
            .build()
    )
    val authenticationGroupExtension = AuthenticationGroupExtension(platform.params)
    val blockchainIdentity = BlockchainIdentity(platform, 0, wallet, authenticationGroupExtension)

    val txMetadataItem = TxMetadataItem(
        Entropy.generateRandomBytes(32),
        System.currentTimeMillis() / 1000,
        "Alice's Pizza Party",
        51.00,
        "USD",
        "expense",
        null
    )

    val txMetadataItemTwo = TxMetadataItem(
        Converters.fromHex("c44d1077cd4628d0ac06e22032a4e8458f9d01be6342453de3eef88657b193ce"),
        System.currentTimeMillis() / 1000,
        "Bob's Burger Joint",
        52.23,
        "USD",
        "expense",
        "DashDirect"
    )

    val txMetadataItemThree = TxMetadataItem(
        Entropy.generateRandomBytes(32),
        System.currentTimeMillis() / 1000,
        null,
        52.23,
        "USD",
        "expense"
    )

    val txMetadataItemFour = TxMetadataItem(
        Entropy.generateRandomBytes(32),
        System.currentTimeMillis() / 1000,
        "Bob's Burger Joint",
        52.23,
        "USD",
        "expense",
        "DashDirect",
        customIconUrl = "https://storage.googleapis.com/mash-potato-fireplace.appspot.com/alhi323/alhi/alhirafiyun.png",
        giftCardNumber = "123456-7899-abcdef",
        giftCardPin = "1234",
        merchantName = "1-800-HotSoup",
        originalPrice = 54.99,
        barcodeValue = "3249u3ou234238403820820840238082304823047234098234023802384",
        barcodeFormat = "EAN_8",
        merchantUrl = "https://1-800-hotsoup.com"
    )
    val txMetadataItems = listOf(txMetadataItem, txMetadataItemTwo, txMetadataItemThree, txMetadataItemFour)
    init {
        authenticationGroupExtension.addKeyChains(
            platform.params, wallet.keyChainSeed,
            EnumSet.of(
                AuthenticationKeyChain.KeyChainType.BLOCKCHAIN_IDENTITY,
                AuthenticationKeyChain.KeyChainType.BLOCKCHAIN_IDENTITY_FUNDING,
                AuthenticationKeyChain.KeyChainType.BLOCKCHAIN_IDENTITY_TOPUP,
                AuthenticationKeyChain.KeyChainType.INVITATION_FUNDING
            )
        )
        wallet.addExtension(authenticationGroupExtension)

        println("initializing platform")
        platform.useValidNodes()
        if (blockchainIdentity.recoverIdentity(authenticationGroupExtension.identityKeyChain.getKey(0).pubKeyHash))  {
            println("recovered blockchain identity: ${blockchainIdentity.uniqueIdString}")
        } else {
            error("recovery failed to find identity")
        }
    }



    lateinit var client: Client

    @JvmStatic
    fun main(args: Array<String>) {
        println("CreateTxMetadataTest")
        client = Client(ClientOptions(network = "testnet"))
        client.platform.useValidNodes()
        runTests(args.size >= 2 && args[1] == "true")
    }

    fun publishTest() {
        val balanceBefore = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
        blockchainIdentity.publishTxMetaData(txMetadataItems, null, 1, TxMetadataDocument.VERSION_PROTOBUF)
        val balanceAfter = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
        println("before: $balanceBefore")
        println("after: $balanceAfter")
    }

    private fun publishTransactions(txMetadata: TxMetadataItem, count: Int) {
        val txMetadataList = arrayListOf<TxMetadataItem>()
        repeat(count) {
            txMetadataList.add(txMetadata)
        }
        val balanceBefore = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
        blockchainIdentity.publishTxMetaData(txMetadataList, null, 1, TxMetadataDocument.VERSION_PROTOBUF)
        val balanceAfter = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
        println("$count tx before: $balanceBefore")
        println("$count tx after: $balanceAfter")
    }

    fun publishTxsTest() {

        var balanceBefore = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
        blockchainIdentity.publishTxMetaData(listOf(txMetadataItemThree), null, 1, TxMetadataDocument.VERSION_PROTOBUF)
        var balanceAfter = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
        println("1 tx before: $balanceBefore")
        println("1 tx after: $balanceAfter")
        balanceBefore = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
        blockchainIdentity.publishTxMetaData(listOf(txMetadataItemThree, txMetadataItemThree), null, 1, TxMetadataDocument.VERSION_PROTOBUF)
        balanceAfter = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
        println("2 tx before: $balanceBefore")
        println("2 tx after: $balanceAfter")
        balanceBefore = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
        blockchainIdentity.publishTxMetaData(listOf(txMetadataItemThree, txMetadataItemThree, txMetadataItemThree), null, 1, TxMetadataDocument.VERSION_PROTOBUF)
        balanceAfter = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
        println("3 tx before: $balanceBefore")
        println("3 tx after: $balanceAfter")
//        balanceBefore = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
//        blockchainIdentity.publishTxMetaData(listOf(txMetadataItemFour), null, 1, TxMetadataDocument.VERSION_PROTOBUF)
//        balanceAfter = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
//        println("1 big tx before: $balanceBefore")
//        println("1 big tx after: $balanceAfter")

        publishTransactions(txMetadataItemThree, 10)
        publishTransactions(txMetadataItemThree, 20)
        publishTransactions(txMetadataItemThree, 39)
    }

    private fun runTests(showOnly: Boolean) {
        //val blockchainIdentity = BlockchainIdentity(client.platform, 0, client.wallet!!, client.authenticationExtension)
        //blockchainIdentity.recoverIdentity()
        val identity = blockchainIdentity.identity!!

        publishTest()
        publishTxsTest()

        val documents = txMetadata.get(identity.id)

        println("Tx Metadata: -----------------------------------")
        for (doc in documents) {
            val txDoc = TxMetadataDocument(doc)
            if (txDoc.encryptedMetadata.isNotEmpty() && txDoc.encryptedMetadata[0] != 0.toByte()) {
                println(JSONObject(doc.toJSON()).toString(2))
                val txList = blockchainIdentity.decryptTxMetadata(txDoc, null)
                println("  $txList")
            }
        }
    }
}
