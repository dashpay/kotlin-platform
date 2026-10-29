package org.dashj.platform.dpp.identity

import org.dashj.platform.sdk.Purpose
import org.dashj.platform.sdk.Result_ok_Vec_u8_err_String
import org.dashj.platform.sdk.dashsdk

/** DIP-13 application keys. Testnet and all devnets use the non-mainnet coin type. */
object ApplicationKeyDerivation {
    private const val ROTATION_LIMIT = 1L shl 31

    /** The caller supplies the request leaf (v2: hash256(appEphPub || bindingIndex)). */
    fun sessionAuthentication(
        seed: ByteArray, identityId: ByteArray, requestLeaf: ByteArray, isMainnet: Boolean
    ): ByteArray = privateKey(dashsdk.applicationSessionAuthenticationPrivateKeyFromSeed(
        !isMainnet, seed, identityId, requestLeaf
    ))

    /**
     * Key for ENCRYPTION or DECRYPTION, bound to the app contract, at
     * `m/9'/coin'/5'/7'/0'/identityId'/contractId'/purpose'/rotation'` (DIP-13).
     * New keys use `rotation = 0`; `rotation` is a hardened 31-bit index (0 until 2^31).
     */
    fun encryption(
        seed: ByteArray,
        identityId: ByteArray,
        contractId: ByteArray,
        purpose: Purpose,
        rotation: Long,
        isMainnet: Boolean
    ): ByteArray {
        require(purpose == Purpose.ENCRYPTION || purpose == Purpose.DECRYPTION)
        require(rotation in 0 until ROTATION_LIMIT) { "rotation must be below 2^31" }
        return privateKey(dashsdk.applicationEncryptionPrivateKeyFromSeed(
            !isMainnet, seed, identityId, contractId, purpose.swigValue().toLong(), rotation
        ))
    }

    /**
     * Key at the pre-rotation path `m/9'/coin'/5'/7'/0'/identityId'/contractId'/purpose'`.
     * Only for recognising keys that wallets registered on testnet/devnet before DIP-13 added
     * the rotation level. Never use it to derive new keys: use [encryption] instead.
     */
    fun legacyEncryptionWithoutRotation(
        seed: ByteArray, identityId: ByteArray, contractId: ByteArray, purpose: Purpose, isMainnet: Boolean
    ): ByteArray {
        require(purpose == Purpose.ENCRYPTION || purpose == Purpose.DECRYPTION)
        return privateKey(dashsdk.applicationEncryptionLegacyPrivateKeyFromSeed(
            !isMainnet, seed, identityId, contractId, purpose.swigValue().toLong()
        ))
    }

    private fun privateKey(result: Result_ok_Vec_u8_err_String): ByteArray {
        try {
            check(result.error == null) { "Application key derivation failed: ${result.error}" }
            return checkNotNull(result.ok).also { check(it.size == 32) }
        } finally {
            result.delete()
        }
    }
}
