package org.dashj.platform.dpp.identity

import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.sdk.KeyType
import org.dashj.platform.sdk.Purpose
import org.dashj.platform.sdk.SecurityLevel
import org.junit.jupiter.api.Assertions.assertArrayEquals
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import java.math.BigInteger
import java.util.concurrent.atomic.AtomicBoolean

class IdentityNativeReadbackTest {
    companion object {
        init { System.loadLibrary("sdklib") }
    }

    private fun keys(): List<IdentityPublicKey> = listOf(
        IdentityPublicKey(0, KeyType.ECDSA_SECP256K1, Purpose.AUTHENTICATION,
            SecurityLevel.MASTER, null, ByteArray(33) { 2 }, false),
        IdentityPublicKey(4, KeyType.ECDSA_HASH160, Purpose.AUTHENTICATION,
            SecurityLevel.HIGH, ContractGroupBounds(Identifier(ByteArray(32) { 3 })),
            ByteArray(20) { 4 }, false, totalBudget = BigInteger("5000000000"),
            expiresAt = 1_800_000_000_000L),
        IdentityPublicKey(5, KeyType.ECDSA_HASH160, Purpose.AUTHENTICATION,
            SecurityLevel.HIGH, SingleContractBounds(Identifier(ByteArray(32) { 5 })),
            ByteArray(20) { 6 }, false, totalBudget = BigInteger("18446744073709551615"),
            expiresAt = 1_800_000_000_001L)
    )

    private fun assertKeys(expected: List<IdentityPublicKey>, actual: List<IdentityPublicKey>) {
        assertEquals(expected.size, actual.size)
        expected.forEach { key ->
            val restored = actual.single { it.id == key.id }
            assertEquals(key.version, restored.version)
            assertEquals(key.purpose, restored.purpose)
            assertEquals(key.type, restored.type)
            assertEquals(key.securityLevel, restored.securityLevel)
            assertEquals(key.contractBounds?.toObject(), restored.contractBounds?.toObject())
            assertEquals(key.totalBudget, restored.totalBudget)
            assertEquals(key.expiresAt, restored.expiresAt)
            assertArrayEquals(key.data, restored.data)
        }
    }

    @Test
    fun readsMixedVersionIdentityUnderGarbageCollection() {
        val expected = keys()
        val original = Identity(Identifier(ByteArray(32) { 7 }), expected, 0, 2, 1)
        val running = AtomicBoolean(true)
        val collector = Thread {
            while (running.get()) {
                System.gc()
                Thread.sleep(2)
            }
        }
        collector.start()
        try {
            repeat(200) {
                val restored = Identity(original.toNative())
                assertEquals(original.id, restored.id)
                assertEquals(original.revision, restored.revision)
                assertKeys(expected, restored.publicKeys)
            }
        } finally {
            running.set(false)
            collector.join()
        }
    }

    @Test
    fun readsStandaloneV1KeyUnderGarbageCollection() {
        val key = keys()[1]
        repeat(100) {
            val native = key.toNative()
            System.gc()
            assertKeys(listOf(key), listOf(IdentityPublicKey.from(native)))
        }
    }
}
