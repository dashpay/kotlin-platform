/**
 * Copyright (c) 2020-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

package org.dashj.platform.contracts.wallet

import org.bitcoinj.core.Sha256Hash
import org.bitcoinj.crypto.ChildNumber
import org.bitcoinj.crypto.KeyCrypterAESCBC
import org.bitcoinj.crypto.KeyCrypterException
import org.dashj.platform.assertListEquals
import org.dashj.platform.assertMapEquals
import org.dashj.platform.dashpay.BlockchainIdentity
import org.dashj.platform.dashpay.PlatformNetwork
import org.dashj.platform.dpp.toHex
import org.dashj.platform.dpp.util.Cbor
import org.dashj.platform.dpp.util.Converters
import org.dashj.platform.dpp.util.Entropy
import org.dashj.platform.sdk.KeyType
import org.dashj.platform.wallet.TxMetadataItem
import org.dashj.platform.wallet.WalletUtils
import org.dashj.platform.wallet.WalletUtils.TxMetadataBatch
import org.junit.jupiter.api.Assertions.assertArrayEquals
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class TxMetaDataTests : PlatformNetwork() {
    lateinit var txId: ByteArray
    lateinit var txMetadataItem: TxMetadataItem
    lateinit var txMetadataItemTwo: TxMetadataItem
    lateinit var txMetadataItemThree: TxMetadataItem
    lateinit var txMetadataItemFour: TxMetadataItem
    lateinit var txMetadataItems: List<TxMetadataItem>

    @BeforeEach
    fun beforeEach() {
        txId = Entropy.generateRandomBytes(32)

        // protobuf bytes
        // 0a2028f18c0366f4b3cbf5aca02653d038764b786b5ec1eddc601e82179464f3173f109df3e9bc061a13416c69636527732050697a7a612050617274792100000000008049402a035553443207657870656e7365
        txMetadataItem = TxMetadataItem(
            txId,
            System.currentTimeMillis() / 1000,
            "Alice's Pizza Party",
            51.00,
            "USD",
            "expense",
            null
        )

        txMetadataItemTwo = TxMetadataItem(
            Converters.fromHex("c44d1077cd4628d0ac06e22032a4e8458f9d01be6342453de3eef88657b193ce"),
            System.currentTimeMillis() / 1000,
            "Bob's Burger Joint",
            52.23,
            "USD",
            "expense",
            "DashDirect"
        )

        txMetadataItemThree = TxMetadataItem(
            Entropy.generateRandomBytes(32),
            System.currentTimeMillis() / 1000,
            null,
            52.23,
            "USD",
            "expense"
        )

        txMetadataItemFour = TxMetadataItem(
            Converters.fromHex("c44d1077cd4628d0ac06e22032a4e8458f9d01be6342453de3eef88657b193ce"),
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

        txMetadataItems = listOf(txMetadataItem, txMetadataItemTwo, txMetadataItemThree, txMetadataItemFour)
    }

    @Test
    fun toObjectTest() {
        assertMapEquals(
            mapOf(
                "txId" to txId,
                "memo" to "Alice's Pizza Party",
                "exchangeRate" to 51.00,
                "currencyCode" to "USD",
                "taxCategory" to "expense",
            ),
            txMetadataItem.toObject()
        )
    }

    @Test
    fun toProtobufTest() {
        txMetadataItem.toProtobuf().toByteArray()
    }

    @Test
    fun toCborTestForOneItemTest() {
        assertArrayEquals(
            Converters.fromHex("A6646D656D6F72426F62277320427572676572204A6F696E7464747849645820C44D1077CD4628D0AC06E22032A4E8458F9D01BE6342453DE3EEF88657B193CE67736572766963656A446173684469726563746B74617843617465676F727967657870656E73656C63757272656E6379436F6465635553446C65786368616E676552617465FB404A1D70A3D70A3D".lowercase()),
            Cbor.encode(txMetadataItemTwo.toObject())
        )
    }

    //
    // 0a
    // 20 - 32 bytes
    // 28f18c0366f4b3cbf5aca02653d038764b786b5ec1eddc601e82179464f3173f - txId
    // 10
    // b7f9e9bc061a13416c69636527732050697a7a612050617274792100000000008049402a035553443207657870656e7365

    @Test
    fun toProtobufForOneItemtest() {
        txMetadataItem = TxMetadataItem(
            Sha256Hash.wrap("28f18c0366f4b3cbf5aca02653d038764b786b5ec1eddc601e82179464f3173f").bytes,
            1738177719933 / 1000,
            "Alice's Pizza Party",
            51.00,
            "USD",
            "expense",
            null
        )
        println(System.currentTimeMillis())
        println(txMetadataItem.toProtobuf().toByteArray().toHex())
        assertArrayEquals(
            Converters.fromHex("0a2028f18c0366f4b3cbf5aca02653d038764b786b5ec1eddc601e82179464f3173f10b7f9e9bc061a13416c69636527732050697a7a612050617274792100000000008049402a035553443207657870656e7365"),
            txMetadataItem.toProtobuf().toByteArray()
        )
    }

    @Test
    fun toCborTestForAllItemsTest() {
        val cborData = Cbor.encode(txMetadataItems.map { it.toObject() })
        val map = Cbor.decodeList(cborData)
    }

    @Test
    fun serializedSizeTest() {
        txMetadataItems.forEachIndexed { i, item ->
            val cborData = Cbor.encode(item.toObject())
            val protobufData = item.toProtobuf().toByteArray()
            assertTrue(cborData.size > protobufData.size)
            println("$i: cbor = ${cborData.size} bytes; protobuf = ${protobufData.size}; %less = ${(cborData.size-protobufData.size).toDouble()/cborData.size}")
        }
        val cborData = Cbor.encode(txMetadataItems.map { it.toObject() })
        val protobufData = TxMetadataBatch.newBuilder()
            .addAllItems(txMetadataItems.map { it.toProtobuf() })
            .build().toByteArray()
        assertTrue(cborData.size > protobufData.size)
        println("all: cbor = ${cborData.size} bytes; protobuf = ${protobufData.size}; %less = ${(cborData.size-protobufData.size).toDouble()/cborData.size}")
    }

    @Test
    fun deserializedTest() {
        val serializedBytes = txMetadataItem.toProtobuf().toByteArray()
        assertEquals(84, serializedBytes.size)
        val deserializedTxMetadata = TxMetadataItem(WalletUtils.TxMetadataItem.parseFrom(serializedBytes))
        assertEquals(txMetadataItem, deserializedTxMetadata)
    }

    @Test
    fun roundTripTest() {
        val blockchainIdentity = BlockchainIdentity(platform, 0, wallet, authenticationGroupExtension)

        val privateKey = blockchainIdentity.privateKeyAtPath(1, TxMetadataDocument.childNumber, 0, KeyType.ECDSA_SECP256K1, null)

        val metadataBytes = Cbor.encode(txMetadataItems.map { it.toObject() })

        // encrypt data
        val cipher = KeyCrypterAESCBC()
        val keyParameter = cipher.deriveKey(privateKey)
        val encryptedData = cipher.encrypt(metadataBytes, keyParameter)

        // now decrypt
        val decryptedData = cipher.decrypt(encryptedData, keyParameter)
        assertArrayEquals(metadataBytes, decryptedData)

        val decryptedList = Cbor.decodeList(decryptedData)

        assertListEquals(txMetadataItems, decryptedList.map { TxMetadataItem(it as Map<String, Any?>) })

        // use the wrong key to decrypt
        val incorrectKey = blockchainIdentity.privateKeyAtPath(1, ChildNumber.ONE_HARDENED, 0, KeyType.ECDSA_SECP256K1, null)
        val incorrectKeyParameter = cipher.deriveKey(incorrectKey)
        assertThrows<KeyCrypterException.InvalidCipherText> {
            cipher.decrypt(encryptedData, incorrectKeyParameter)
        }
    }

    @Test
    fun emptyListTest() {
        val blockchainIdentity = BlockchainIdentity(platform, 0, wallet, authenticationGroupExtension)

        val privateKey = blockchainIdentity.privateKeyAtPath(1, TxMetadataDocument.childNumber, 0, KeyType.ECDSA_SECP256K1, null)

        val metadataBytes = Cbor.encode(listOf<TxMetadataItem>())

        // encrypt data
        val cipher = KeyCrypterAESCBC()
        val keyParameter = cipher.deriveKey(privateKey)
        val encryptedData = cipher.encrypt(metadataBytes, keyParameter)

        // 32 bytes = IV (16 bytes) + 1 block (16 bytes)
        assertEquals(32, encryptedData.initialisationVector.size + encryptedData.encryptedBytes.size)
    }

    @Test
    fun publishToPlatform() {

    }
}
