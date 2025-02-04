package org.dashj.platform.contracts.wallet

import com.google.common.collect.ImmutableList
import com.google.protobuf.InvalidProtocolBufferException
import org.bitcoinj.core.Sha256Hash
import org.bitcoinj.crypto.ChildNumber
import org.bitcoinj.crypto.DeterministicKey
import org.bitcoinj.crypto.KeyCrypterException
import org.bitcoinj.params.TestNet3Params
import org.bitcoinj.wallet.AuthenticationKeyChain
import org.bitcoinj.wallet.DerivationPathFactory
import org.bitcoinj.wallet.DeterministicKeyChain
import org.bitcoinj.wallet.DeterministicSeed
import org.bitcoinj.wallet.KeyChainGroup
import org.bitcoinj.wallet.Wallet
import org.bitcoinj.wallet.authentication.AuthenticationGroupExtension
import org.bouncycastle.crypto.params.KeyParameter
import org.dashj.platform.dashpay.BlockchainIdentity
import org.dashj.platform.dpp.document.Document
import org.dashj.platform.dpp.util.Converters
import org.dashj.platform.dpp.util.Entropy
import org.dashj.platform.sdk.KeyType
import org.dashj.platform.sdk.platform.Platform
import org.dashj.platform.wallet.TxMetadata
import org.dashj.platform.wallet.TxMetadataItem
import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import java.util.*

class TxMetadataWalletTest {
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

    private val txMetadataItem = TxMetadataItem(
        Entropy.generateRandomBytes(32),
        System.currentTimeMillis() / 1000,
        "Alice's Pizza Party",
        51.00,
        "USD",
        "expense",
        null
    )

    private val txMetadataItemTwo = TxMetadataItem(
        Converters.fromHex("c44d1077cd4628d0ac06e22032a4e8458f9d01be6342453de3eef88657b193ce"),
        System.currentTimeMillis() / 1000,
        "Bob's Burger Joint",
        52.23,
        "USD",
        "expense",
        "DashDirect"
    )

    private val txMetadataItemThree = TxMetadataItem(
        Entropy.generateRandomBytes(32),
        System.currentTimeMillis() / 1000,
        null,
        52.23,
        "USD",
        "expense"
    )

    private val txMetadataItemFour = TxMetadataItem(
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

    private val txMetadataItems = listOf(txMetadataItem, txMetadataItemTwo, txMetadataItemThree, txMetadataItemFour)
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

    @AfterEach
    fun afterEachTest() {
        println(platform.client.reportNetworkStatus())
    }

    @Test
    fun queryTest() {
        val documents = txMetadata.get(blockchainIdentity.uniqueIdString)
        documents.forEach {
            val txMetadataDocument = TxMetadataDocument(it)
            println(txMetadataDocument)
            println(blockchainIdentity.decryptTxMetadata(txMetadataDocument, null))
        }
    }

//    @Test
//    fun publishTest() {
//        val balanceBefore = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
//        blockchainIdentity.publishTxMetaData(txMetadataItems, null, 1, TxMetadataDocument.VERSION_PROTOBUF)
//        val balanceAfter = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
//        println("before: $balanceBefore")
//        println("after: $balanceAfter")
//    }

    @Test
    fun roundTripTest() {
        val documents = blockchainIdentity.createTxMetadata(txMetadataItems, null, 100, TxMetadataDocument.VERSION_PROTOBUF)
        assertEquals(1, documents.size)
        val firstDocument = documents.first()
        assertEquals("txMetadata", firstDocument.type)
        assertEquals("6xEV5s8FiJUReCuEBCdF3TfMwKggbCNb9RbuxGMEykY8", firstDocument.ownerId.toString())
        assertEquals("7CSFGeF4WNzgDmx94zwvHkYaG3Dx4XEe5LFsFgJswLbm", firstDocument.dataContractId.toString())
        assertEquals(3, firstDocument.data.size)
        val txMetadataDocument = TxMetadataDocument(firstDocument)
        assertEquals(2, txMetadataDocument.keyIndex)
        val ipk = blockchainIdentity.identity!!.getPublicKeyById(txMetadataDocument.keyIndex)!!
        assertEquals(KeyType.ECDSA_SECP256K1, ipk.type)
        assertEquals(100, txMetadataDocument.encryptionKeyIndex)
        assertEquals(609, txMetadataDocument.encryptedMetadata.size)
        val decryptedItems = blockchainIdentity.decryptTxMetadata(txMetadataDocument, null)
        assertEquals(4, decryptedItems.size)
        assertEquals(txMetadataItemTwo, decryptedItems[1])
    }

    @Test
    fun errorTest() {
        val documents =
            blockchainIdentity.createTxMetadata(txMetadataItems, null, 100, TxMetadataDocument.VERSION_PROTOBUF)
        val firstDocument = documents.first()
        val modifiedData = firstDocument.toObject().toMutableMap()
        modifiedData["keyIndex"] = 1L
        val modifiedDocument = Document(modifiedData, firstDocument.dataContract!!)
        val modifiedMetadataDocument = TxMetadataDocument(modifiedDocument)
        assertThrows<Exception> {
            blockchainIdentity.decryptTxMetadata(modifiedMetadataDocument, null)
        }
        val modifiedData2 = firstDocument.toObject().toMutableMap()
        modifiedData2["encryptionKeyIndex"] = 1L
        val modifiedDocument2 = Document(modifiedData2, firstDocument.dataContract!!)
        val modifiedMetadataDocument2 = TxMetadataDocument(modifiedDocument2)
        assertThrows<Exception> {
            blockchainIdentity.decryptTxMetadata(modifiedMetadataDocument2, null)
        }
    }

    @Test
    fun invalidKeyParameterTest() {
        val documents = blockchainIdentity.createTxMetadata(txMetadataItems, null, 100, TxMetadataDocument.VERSION_PROTOBUF)
        val firstDocument = documents.first()
        val txMetadataDocument = TxMetadataDocument(firstDocument)
        val keyParameter = KeyParameter(Sha256Hash.hash(ByteArray(32)))
        assertThrows<IllegalArgumentException> {
            blockchainIdentity.decryptTxMetadata(txMetadataDocument, keyParameter)
        }
    }

    @Test
    fun derivationPathTest() {
        val documents = blockchainIdentity.createTxMetadata(txMetadataItems, null, 100, TxMetadataDocument.VERSION_PROTOBUF)
        val firstDocument = documents.first()
        val txMetadataDocument = TxMetadataDocument(firstDocument)
        val actualEncryptionKey = blockchainIdentity.privateKeyAtPath(
            txMetadataDocument.keyIndex,
            TxMetadataDocument.childNumber,
            txMetadataDocument.encryptionKeyIndex,
            KeyType.ECDSA_SECP256K1,
            null
        )

        val parentEncryptionKeyPath = DerivationPathFactory.get(blockchainIdentity.params).blockchainIdentityECDSADerivationPath()
        val parentEncryptionKey = authenticationGroupExtension.identityKeyChain.getKeyByPath(parentEncryptionKeyPath)
            .deriveChildKey(ChildNumber(2, true))
        assertNotNull(parentEncryptionKey)
        val expectedEncryptionKey = parentEncryptionKey
            .deriveChildKey(TxMetadataDocument.childNumber)
            .derive(100)
        assertEquals(expectedEncryptionKey, actualEncryptionKey)
        println(expectedEncryptionKey)
    }

//    private fun publishTransactions(txMetadata: TxMetadataItem, count: Int) {
//        val txMetadataList = arrayListOf<TxMetadataItem>()
//        for (i in IntRange(0, count)) {
//            txMetadataList.add(txMetadata)
//        }
//        val balanceBefore = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
//        blockchainIdentity.publishTxMetaData(txMetadataList, null, 1, TxMetadataDocument.VERSION_PROTOBUF)
//        val balanceAfter = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
//        println("$count tx before: $balanceBefore")
//        println("$count tx after: $balanceAfter")
//    }
//    @Test
//    fun publishTxsTest() {
//
//        var balanceBefore = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
//        blockchainIdentity.publishTxMetaData(listOf(txMetadataItemThree), null, 1, TxMetadataDocument.VERSION_PROTOBUF)
//        var balanceAfter = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
//        println("1 tx before: $balanceBefore")
//        println("1 tx after: $balanceAfter")
//        balanceBefore = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
//        blockchainIdentity.publishTxMetaData(listOf(txMetadataItemThree, txMetadataItemThree), null, 1, TxMetadataDocument.VERSION_PROTOBUF)
//        balanceAfter = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
//        println("2 tx before: $balanceBefore")
//        println("2 tx after: $balanceAfter")
//        balanceBefore = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
//        blockchainIdentity.publishTxMetaData(listOf(txMetadataItemThree, txMetadataItemThree, txMetadataItemThree), null, 1, TxMetadataDocument.VERSION_PROTOBUF)
//        balanceAfter = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
//        println("3 tx before: $balanceBefore")
//        println("3 tx after: $balanceAfter")
////        balanceBefore = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
////        blockchainIdentity.publishTxMetaData(listOf(txMetadataItemFour), null, 1, TxMetadataDocument.VERSION_PROTOBUF)
////        balanceAfter = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, blockchainIdentity.uniqueIdentifier.toNative())
////        println("1 big tx before: $balanceBefore")
////        println("1 big tx after: $balanceAfter")
//
//        publishTransactions(txMetadataItemThree, 10)
//        publishTransactions(txMetadataItemThree, 20)
//        publishTransactions(txMetadataItemThree, 39)
//    }
}