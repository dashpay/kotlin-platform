/**
 * Copyright (c) 2020-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

package org.dashj.platform.dpp.identity

import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.util.Cbor
import org.dashj.platform.sdk.KeyType
import org.dashj.platform.sdk.Purpose
import org.dashj.platform.sdk.SecurityLevel
import org.junit.jupiter.api.Assertions.assertArrayEquals
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertInstanceOf
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Assertions.assertDoesNotThrow
import org.junit.jupiter.api.Test

/**
 * Tests that [IdentityPublicKey.toObject] produces a map that the [Cbor] encoder can serialize.
 *
 * Regression coverage for the case where a key carries [ContractBounds] (in particular a
 * [SingleContractDocumentType]): previously `toObject()` stored the raw `ContractBounds` object
 * in the map, so `Cbor.encode(toObject())` threw `IllegalArgumentException("No converter for ...")`
 * because the encoder has no converter for that type. `toObject()` must instead store the nested
 * `Map` produced by `ContractBounds.toObject()`.
 */
class IdentityPublicKeyTest {

    companion object {
        init {
            // KeyType/Purpose/SecurityLevel are SWIG enums whose constants call into JNI,
            // so the native library must be loaded before they are referenced.
            System.loadLibrary("sdklib")
        }
    }

    // 33-byte compressed ECDSA public key
    private val keyData = ByteArray(33) { (it + 1).toByte() }

    private val documentTypeBounds = SingleContractDocumentType(
        Identifier(ByteArray(32) { it.toByte() }),
        "note"
    )

    private val singleContractBounds = SingleContractBounds(
        Identifier(ByteArray(32) { (it + 100).toByte() })
    )

    private fun key(contractBounds: ContractBounds?) = IdentityPublicKey(
        0,
        KeyType.ECDSA_SECP256K1,
        Purpose.AUTHENTICATION,
        SecurityLevel.MASTER,
        contractBounds,
        keyData,
        true
    )

    @Test
    fun toObjectStoresContractBoundsAsMap() {
        // The regression: contractBounds must be a Map (from ContractBounds.toObject()),
        // not the raw ContractBounds object which the Cbor encoder cannot handle.
        val obj = key(documentTypeBounds).toObject()

        val contractBounds = obj["contractBounds"]
        assertNotNull(contractBounds, "contractBounds should be present in toObject()")
        assertInstanceOf(
            Map::class.java,
            contractBounds,
            "contractBounds must be serialized as a Map, not a ${contractBounds!!::class.java.name}"
        )

        @Suppress("UNCHECKED_CAST")
        val boundsMap = contractBounds as Map<String, Any>
        assertEquals("documentType", boundsMap["type"])
        assertEquals("note", boundsMap["documentType"])
        assertInstanceOf(Identifier::class.java, boundsMap["identifier"])
    }

    @Test
    fun toCborWithSingleContractDocumentTypeDoesNotThrow() {
        // Before the fix this threw IllegalArgumentException("No converter for ...").
        val obj = key(documentTypeBounds).toObject()
        val encoded = assertDoesNotThrow<ByteArray> { Cbor.encode(obj) }
        assertTrue(encoded.isNotEmpty())

        val decoded = Cbor.decode(encoded)

        @Suppress("UNCHECKED_CAST")
        val boundsMap = decoded["contractBounds"] as Map<String, Any?>
        assertEquals("documentType", boundsMap["type"])
        assertEquals("note", boundsMap["documentType"])
        // Identifier is encoded as its raw 32-byte buffer.
        assertArrayEquals(ByteArray(32) { it.toByte() }, boundsMap["identifier"] as ByteArray)
    }

    @Test
    fun toCborWithSingleContractBoundsDoesNotThrow() {
        val obj = key(singleContractBounds).toObject()
        val encoded = assertDoesNotThrow<ByteArray> { Cbor.encode(obj) }

        val decoded = Cbor.decode(encoded)

        @Suppress("UNCHECKED_CAST")
        val boundsMap = decoded["contractBounds"] as Map<String, Any?>
        assertEquals("singleContract", boundsMap["type"])
        assertFalse(boundsMap.containsKey("documentType"))
        assertArrayEquals(ByteArray(32) { (it + 100).toByte() }, boundsMap["identifier"] as ByteArray)
    }

    @Test
    fun toCborWithoutContractBoundsRoundTrips() {
        val obj = key(null).toObject()
        assertFalse(obj.containsKey("contractBounds"))

        val encoded = assertDoesNotThrow<ByteArray> { Cbor.encode(obj) }
        val decoded = Cbor.decode(encoded)

        assertEquals(0, decoded["id"])
        assertEquals(KeyType.ECDSA_SECP256K1.swigValue(), decoded["type"])
        assertEquals(Purpose.AUTHENTICATION.swigValue(), decoded["purpose"])
        assertEquals(SecurityLevel.MASTER.swigValue(), decoded["securityLevel"])
        assertEquals(true, decoded["readOnly"])
        assertArrayEquals(keyData, decoded["data"] as ByteArray)
        assertFalse(decoded.containsKey("contractBounds"))
    }

    @Test
    fun fromObjectReconstructsContractBoundsFromMap() {
        // Full round-trip: a CBOR-decoded key map carries contractBounds as a Map,
        // and the Map constructor must rebuild the typed ContractBounds from it.
        val decoded = Cbor.decode(Cbor.encode(key(documentTypeBounds).toObject()))
        val roundTripped = IdentityPublicKey(decoded)

        val bounds = roundTripped.contractBounds
        assertInstanceOf(SingleContractDocumentType::class.java, bounds)
        bounds as SingleContractDocumentType
        assertEquals("note", bounds.documentType)
        assertEquals(documentTypeBounds.identifier, bounds.identifier)

        val decodedSingle = Cbor.decode(Cbor.encode(key(singleContractBounds).toObject()))
        val singleBounds = IdentityPublicKey(decodedSingle).contractBounds
        assertInstanceOf(SingleContractBounds::class.java, singleBounds)
        assertEquals(singleContractBounds.identifier, singleBounds!!.identifier)
    }

    @Test
    fun toNativePreservesSingleContractDocumentType() {
        // Round-trip through the Rust SDK: toNative() must carry the contract bounds so that
        // signing paths that reconstruct the key don't silently drop them.
        val roundTripped = IdentityPublicKey.from(key(documentTypeBounds).toNative())

        val bounds = roundTripped.contractBounds
        assertInstanceOf(SingleContractDocumentType::class.java, bounds)
        bounds as SingleContractDocumentType
        assertEquals("note", bounds.documentType)
        assertEquals(documentTypeBounds.identifier, bounds.identifier)
    }

    @Test
    fun toNativePreservesSingleContractBounds() {
        val roundTripped = IdentityPublicKey.from(key(singleContractBounds).toNative())

        val bounds = roundTripped.contractBounds
        assertInstanceOf(SingleContractBounds::class.java, bounds)
        assertEquals(singleContractBounds.identifier, bounds!!.identifier)
    }

    @Test
    fun toNativeWithoutContractBoundsHasNoBounds() {
        val roundTripped = IdentityPublicKey.from(key(null).toNative())
        assertNull(roundTripped.contractBounds)
    }

    @Test
    fun toBufferWithContractBoundsDoesNotThrow() {
        // toBuffer() runs toObject() through the Cbor encoder with a protocol-version prefix;
        // it is the path that actually failed in production for keys with contract bounds.
        val buffer = assertDoesNotThrow<ByteArray> { key(documentTypeBounds).toBuffer() }
        assertTrue(buffer.isNotEmpty())
    }
}