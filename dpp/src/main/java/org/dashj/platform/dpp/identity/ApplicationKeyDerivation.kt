package org.dashj.platform.dpp.identity

import org.dashj.platform.sdk.Purpose
import org.dashj.platform.sdk.Result_ok_Vec_u8_err_String
import org.dashj.platform.sdk.dashsdk

/** DIP-13 application keys. Testnet and all devnets use the non-mainnet coin type. */
object ApplicationKeyDerivation {
    /** The caller supplies the request leaf (v2: hash256(appEphPub || bindingIndex)). */
    fun sessionAuthentication(
        seed: ByteArray, identityId: ByteArray, requestLeaf: ByteArray, isMainnet: Boolean
    ): ByteArray = privateKey(dashsdk.applicationSessionAuthenticationPrivateKeyFromSeed(
        !isMainnet, seed, identityId, requestLeaf
    ))

    /** Rotation-zero key for ENCRYPTION or DECRYPTION, bound to the app contract. */
    fun encryption(
        seed: ByteArray, identityId: ByteArray, contractId: ByteArray, purpose: Purpose, isMainnet: Boolean
    ): ByteArray {
        require(purpose == Purpose.ENCRYPTION || purpose == Purpose.DECRYPTION)
        return privateKey(dashsdk.applicationEncryptionPrivateKeyFromSeed(
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
