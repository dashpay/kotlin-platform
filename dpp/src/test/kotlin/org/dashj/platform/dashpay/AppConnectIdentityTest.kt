package org.dashj.platform.dashpay

import org.bitcoinj.params.TestNet3Params
import org.bitcoinj.wallet.*
import org.bitcoinj.wallet.authentication.AuthenticationGroupExtension
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identity.Identity
import org.dashj.platform.dpp.identity.IdentityPublicKey
import org.dashj.platform.sdk.Purpose
import org.dashj.platform.sdk.SecurityLevel
import org.dashj.platform.sdk.KeyType
import org.dashj.platform.sdk.platform.Platform
import org.dashj.platform.dashpay.callback.WalletSignerCallback
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import org.bouncycastle.crypto.params.KeyParameter
import java.util.EnumSet

class AppConnectIdentityTest {
    companion object { init { System.loadLibrary("sdklib") } }
    private val params = TestNet3Params.get()
    private val platform = Platform(params)
    private val seed = DeterministicSeed(ByteArray(32) { 7 }, "", 1_700_000_000L)
    private val wallet = Wallet(params, KeyChainGroup.builder(params).addChain(
        DeterministicKeyChain.builder().seed(seed).build()).build())
    private val extension = AuthenticationGroupExtension(params).also {
        it.addKeyChains(params, seed, EnumSet.of(AuthenticationKeyChain.KeyChainType.BLOCKCHAIN_IDENTITY))
        wallet.addExtension(it)
    }
    private val identity = BlockchainIdentity(platform, 0, wallet, extension)

    @Suppress("UNCHECKED_CAST")
    private fun newKeys(): List<IdentityPublicKey> {
        val method = BlockchainIdentity::class.java.getDeclaredMethod("createIdentityPublicKeys", KeyParameter::class.java)
        method.isAccessible = true
        return (method.invoke(identity, null) as Pair<*, List<IdentityPublicKey>>).second
    }

    @Test fun defaultIdentityIncludesWalletControlledCriticalAuthentication() {
        val keys = newKeys()
        val critical = keys.single { it.purpose == Purpose.AUTHENTICATION && it.securityLevel == SecurityLevel.CRITICAL }
        assertNull(critical.contractBounds)
        assertEquals(5, keys.size)
        assertEquals(5, keys.map { it.id }.distinct().size)
        assertEquals(5, keys.map { it.data.toList() }.distinct().size)
        assertNotNull(extension.identityKeyChain.findKeyFromPubKey(critical.data))
        WalletSignerCallback(wallet, null).use { signer ->
            assertTrue(signer.sign(critical.data, byteArrayOf(1, 2, 3)).isNotEmpty())
        }
        identity.identity = Identity(Identifier.from(ByteArray(32) { 1 }), keys, 1000000, 0, 14)
        assertTrue(identity.hasCriticalAuthenticationKey())
        identity.identity!!.publicKeys.remove(critical)
        assertFalse(identity.hasCriticalAuthenticationKey())
        val method = BlockchainIdentity::class.java.getDeclaredMethod("determineMissingKeys", Boolean::class.javaPrimitiveType)
            .also { it.isAccessible = true }
        @Suppress("UNCHECKED_CAST") val ordinary = method.invoke(identity, false) as List<IdentityPublicKey>
        assertTrue(ordinary.isEmpty())
        @Suppress("UNCHECKED_CAST") val missing = method.invoke(identity, true) as List<IdentityPublicKey>
        assertEquals(listOf(SecurityLevel.CRITICAL), missing.map { it.securityLevel })
        identity.identity!!.publicKeys.removeAll { it.purpose == Purpose.TRANSFER || it.purpose == Purpose.ENCRYPTION }
        @Suppress("UNCHECKED_CAST") val background = method.invoke(identity, false) as List<IdentityPublicKey>
        assertEquals(setOf(Purpose.TRANSFER, Purpose.ENCRYPTION), background.map { it.purpose }.toSet())
    }

    @Test fun fenceUsesMissingIdButNeverAppMaterialOrAnExistingWalletKey() {
        val keys = newKeys()
        val foreign = IdentityPublicKey(10, KeyType.ECDSA_HASH160, Purpose.AUTHENTICATION,
            SecurityLevel.HIGH, null, ByteArray(20) { 99 }, false)
        identity.identity = Identity(Identifier.from(ByteArray(32) { 1 }), keys.filter { it.id != 4 } + foreign, 1000000, 0, 14)
        val first = identity.createRevocationFenceKey(4, null)
        val second = identity.createRevocationFenceKey(9, null, listOf(first))
        assertEquals(4, first.id)
        assertNull(first.contractBounds)
        assertEquals(SecurityLevel.HIGH, first.securityLevel)
        assertFalse(first.data.contentEquals(second.data))
        assertTrue(identity.identity!!.publicKeys.none { it.data.contentEquals(first.data) })
        assertNotNull(extension.identityKeyChain.findKeyFromPubKey(first.data))
        WalletSignerCallback(wallet, null).use { signer ->
            assertTrue(signer.sign(first.data, byteArrayOf(1, 2, 3)).isNotEmpty())
        }
        assertThrows(IllegalArgumentException::class.java) { identity.createRevocationFenceKey(0, null) }
    }
}
