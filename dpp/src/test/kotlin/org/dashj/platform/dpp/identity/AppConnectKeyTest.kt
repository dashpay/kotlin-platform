package org.dashj.platform.dpp.identity

import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.util.Cbor
import org.dashj.platform.sdk.KeyType
import org.dashj.platform.sdk.Purpose
import org.dashj.platform.sdk.SecurityLevel
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import java.math.BigInteger

class AppConnectKeyTest {
    companion object {
        init { System.loadLibrary("sdklib") }
    }

    private val id = Identifier(ByteArray(32) { 7 })
    private val maxBudget = BigInteger.ONE.shiftLeft(64).subtract(BigInteger.ONE)

    private fun key(bounds: ContractBounds?, version: Int = 1, budget: BigInteger? = maxBudget) =
        IdentityPublicKey(7, KeyType.ECDSA_HASH160, Purpose.AUTHENTICATION, SecurityLevel.HIGH,
            bounds, ByteArray(20) { 3 }, false, disabledAt = 123L,
            totalBudget = budget, expiresAt = if (budget == null) null else 1_900_000_000_000L,
            version = version)

    @Test
    fun nativeRoundTripPreservesAllV1BoundsAndUnsignedLimits() {
        for (bounds in listOf(SingleContractBounds(id), SingleContractDocumentType(id, "profile"), ContractGroupBounds(id))) {
            val original = key(bounds)
            val restored = IdentityPublicKey.from(original.toNative())
            assertEquals(original.id, restored.id)
            assertEquals(original.type, restored.type)
            assertEquals(original.purpose, restored.purpose)
            assertEquals(original.securityLevel, restored.securityLevel)
            assertEquals(original.contractBounds?.toJSON(), restored.contractBounds?.toJSON())
            assertEquals(maxBudget, restored.totalBudget)
            assertEquals(original.expiresAt, restored.expiresAt)
            assertEquals(123L, restored.disabledAt)
            assertArrayEquals(original.data, restored.data)
        }
    }

    @Test
    fun nativeRoundTripKeepsVersionEvenWithoutLimits() {
        for (version in 0..1) {
            val restored = IdentityPublicKey.from(key(null, version, null).toNative())
            assertEquals(version, restored.version)
            assertNull(restored.totalBudget)
            assertNull(restored.expiresAt)
        }
    }

    @Test
    fun persistenceAndCopyKeepLimitsAndGroupBounds() {
        val original = key(ContractGroupBounds(id))
        val fromObject = IdentityPublicKey(original.toObject())
        val fromJson = IdentityPublicKey(original.toJSON())
        val fromCbor = IdentityPublicKey(Cbor.decode(Cbor.encode(original.toObject())))
        for (restored in listOf(fromObject, fromJson, fromCbor, original.copy(true))) {
            assertEquals(maxBudget, restored.totalBudget)
            assertEquals(original.expiresAt, restored.expiresAt)
            assertEquals(original.version, restored.version)
            assertEquals(original.contractBounds?.toJSON(), restored.contractBounds?.toJSON())
        }
        assertEquals(listOf(maxBudget), Cbor.decodeList(Cbor.encode(listOf(maxBudget))))
    }

    @Test
    fun refusesBudgetsOutsideTheWireRange() {
        assertThrows(IllegalArgumentException::class.java) { key(null, budget = BigInteger.valueOf(-1)) }
        assertThrows(IllegalArgumentException::class.java) { key(null, budget = BigInteger.ONE.shiftLeft(64)) }
        assertThrows(IllegalArgumentException::class.java) { key(null, version = 0) }
    }

    @Test
    fun derivationSeparatesDomainsAndDoesNotModifyTheSeed() {
        val seed = ByteArray(32) { 42 }
        val originalSeed = seed.copyOf()
        val identityId = ByteArray(32) { 1 }
        val contractId = ByteArray(32) { 2 }
        val encryption = ApplicationKeyDerivation.encryption(
            seed, identityId, contractId, Purpose.ENCRYPTION, false
        )
        assertEquals(32, encryption.size)
        assertArrayEquals(encryption, ApplicationKeyDerivation.encryption(
            seed, identityId, contractId, Purpose.ENCRYPTION, false
        ))
        assertFalse(encryption.contentEquals(ApplicationKeyDerivation.encryption(
            seed, identityId, contractId, Purpose.DECRYPTION, false
        )))
        assertFalse(encryption.contentEquals(ApplicationKeyDerivation.encryption(
            seed, identityId, contractId, Purpose.ENCRYPTION, true
        )))
        assertFalse(encryption.contentEquals(ApplicationKeyDerivation.sessionAuthentication(
            seed, identityId, contractId, false
        )))
        assertArrayEquals(originalSeed, seed)
    }
}
