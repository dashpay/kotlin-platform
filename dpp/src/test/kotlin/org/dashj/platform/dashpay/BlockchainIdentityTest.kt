package org.dashj.platform.dashpay

import org.bitcoinj.core.Sha256Hash
import org.bitcoinj.crypto.ChildNumber
import org.bitcoinj.crypto.KeyCrypterException
import org.bouncycastle.crypto.params.KeyParameter
import org.dashj.platform.contracts.wallet.TxMetadataDocument
import org.dashj.platform.dpp.document.Document
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identity.Identity
import org.dashj.platform.dpp.util.Entropy
import org.dashj.platform.wallet.TxMetadataItem
import kotlin.random.Random
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Assertions.assertThrows
import org.junit.jupiter.api.Test
import kotlin.math.pow

class BlockchainIdentityTest : PlatformNetwork() {

    val blockchainIdentity = BlockchainIdentity(platform, 0, wallet, authenticationGroupExtension)

    @Test
    fun checkWalletSeedTest() {
        assertEquals(seed.split(' '), blockchainIdentity.wallet!!.keyChainSeed.mnemonicCode)
    }

    @Test
    fun updateProfileTest() {
        val displayName = "Bob Lee " + Random.nextInt()

        blockchainIdentity.recoverIdentity(authenticationGroupExtension.identityKeyChain.getKey(0, true).pubKeyHash)

        val currentProfile = blockchainIdentity.getProfileFromPlatform()!!

        val updatedProfile = blockchainIdentity.updateProfile(displayName, null, null, null, null, null)

        val retrievedProfile = blockchainIdentity.getProfileFromPlatform()!!

        assertNotEquals(currentProfile, retrievedProfile)
        assertEquals(updatedProfile, Profile(retrievedProfile))
    }

//    @Test
//    fun sendContactRequestTest() {
//
//        blockchainIdentity.recoverIdentity(authenticationGroupExtension.identityKeyChain.getKey(0, true).pubKeyHash)
//
//        val cr = ContactRequests(platform)
//
//        // my-test-1
//        val toUserIdentity = platform.identities.get("6ffk46v6m79RPbiomNT1kLXX7unqgw5FkWhsTuPi4Aj7")
//
//        val document = cr.create(blockchainIdentity, toUserIdentity!!, null)
//
//        println(document)
//    }

    @Test
    fun createTxMetadataTest() {
        if (blockchainIdentity.recoverIdentity(authenticationGroupExtension.identityKeyChain.getKey(0, true).pubKeyHash)) {
            val txMetadataItems = listOf(
                TxMetadataItem(
                    Entropy.generateRandomBytes(32),
                    System.currentTimeMillis() / 1000,
                    "Test Transaction 1",
                    25.50,
                    "USD",
                    "expense",
                    "TestService"
                ),
                TxMetadataItem(
                    Entropy.generateRandomBytes(32),
                    System.currentTimeMillis() / 1000,
                    "Test Transaction 2",
                    75.25,
                    "USD",
                    "income"
                )
            )

            val documents = blockchainIdentity.createTxMetadata(
                txMetadataItems,
                null,
                1,
                TxMetadataDocument.VERSION_PROTOBUF
            )

            assertEquals(1, documents.size)
            val document = documents.first()
            assertEquals("txMetadata", document.type)
            assertTrue(document.data.containsKey("keyIndex"))
            assertTrue(document.data.containsKey("encryptionKeyIndex"))
            assertTrue(document.data.containsKey("encryptedMetadata"))

            // Test decryption to verify round-trip functionality
            val txMetadataDocument = TxMetadataDocument(document)
            val decryptedItems = blockchainIdentity.decryptTxMetadata(txMetadataDocument, null)
            
            assertEquals(2, decryptedItems.size)
            assertEquals(txMetadataItems[0], decryptedItems[0])
            assertEquals(txMetadataItems[1], decryptedItems[1])
        } else {
            println("Skipping test - could not recover identity")
        }
    }

    @Test
    fun createTxMetadataEmptyListTest() {
        if (blockchainIdentity.recoverIdentity(authenticationGroupExtension.identityKeyChain.getKey(0, true).pubKeyHash)) {
            val documents = blockchainIdentity.createTxMetadata(
                emptyList(),
                null,
                1,
                TxMetadataDocument.VERSION_PROTOBUF
            )

            assertEquals(0, documents.size)
        }
    }


    @Test
    fun createTxMetadataLargeDataTest() {
        if (blockchainIdentity.recoverIdentity(authenticationGroupExtension.identityKeyChain.getKey(0, true).pubKeyHash)) {
            val largeItems = (1..50).map { i ->
                TxMetadataItem(
                    Entropy.generateRandomBytes(32),
                    System.currentTimeMillis() / 1000,
                    "Transaction $i with very long memo to simulate large data size that might exceed document limits",
                    100.0 * i,
                    "USD",
                    "expense",
                    "TestService",
                    customIconUrl = "https://example.com/very-long-url-$i",
                    merchantName = "Very Long Merchant Name $i",
                    merchantUrl = "https://very-long-merchant-url-$i.com"
                )
            }

            val documents = blockchainIdentity.createTxMetadata(
                largeItems,
                null,
                1,
                TxMetadataDocument.VERSION_PROTOBUF
            )

            assertTrue(documents.size > 1, "Large data should be split into multiple documents")
            
            // Verify each document structure and key derivation
            documents.forEach { document ->
                assertEquals("txMetadata", document.type)
                assertTrue(document.data.containsKey("encryptedMetadata"))
                val encryptedData = document.data["encryptedMetadata"] as ByteArray
                assertTrue(encryptedData.size <= TxMetadataDocument.MAX_ENCRYPTED_SIZE + 100)
                
                // Verify key derivation matches expected path
                val txMetadataDocument = TxMetadataDocument(document)
                val actualEncryptionKey = blockchainIdentity.privateKeyAtPath(
                    txMetadataDocument.keyIndex,
                    TxMetadataDocument.childNumber,
                    txMetadataDocument.encryptionKeyIndex,
                    org.dashj.platform.sdk.KeyType.ECDSA_SECP256K1,
                    null
                )
                
                val parentEncryptionKeyPath = org.bitcoinj.wallet.DerivationPathFactory.get(blockchainIdentity.params).blockchainIdentityECDSADerivationPath()
                val parentEncryptionKey = authenticationGroupExtension.identityKeyChain.getKeyByPath(parentEncryptionKeyPath)
                    .deriveChildKey(ChildNumber(2, true))
                val expectedEncryptionKey = parentEncryptionKey
                    .deriveChildKey(ChildNumber((2 shl 14) + 1, true))
                    .derive(1)
                assertEquals(expectedEncryptionKey, actualEncryptionKey)
                val expectedEncryptionKey2 = parentEncryptionKey
                    .deriveChildKey(ChildNumber(2.toDouble().pow(15).toInt() + 1, true))
                    .derive(1)
                assertEquals(expectedEncryptionKey2, actualEncryptionKey)

            }
            
            // Verify decryption and reconstruction of all items
            val allDecryptedItems = mutableListOf<TxMetadataItem>()
            documents.forEach { document ->
                val txMetadataDocument = TxMetadataDocument(document)
                val decryptedItems = blockchainIdentity.decryptTxMetadata(txMetadataDocument, null)
                allDecryptedItems.addAll(decryptedItems)
            }
            
            assertEquals(largeItems.size, allDecryptedItems.size)
            largeItems.forEachIndexed { index, originalItem ->
                assertEquals(originalItem, allDecryptedItems[index])
            }
            println("success!")
        }
    }
}
