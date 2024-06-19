package org.dashj.platform.dashpay.callback

import org.bitcoinj.core.ECKey
import org.bitcoinj.core.Sha256Hash
import org.bitcoinj.core.Utils
import org.bouncycastle.crypto.params.KeyParameter
import org.dashj.platform.dpp.identity.IdentityPublicKey
import org.dashj.platform.sdk.callbacks.Signer

class SimpleSignerCallback(
    private val keyMap: Map<IdentityPublicKey, ECKey>,
    private val keyParameter: KeyParameter?
) : Signer() {

    override fun sign(publicKey: ByteArray, data: ByteArray): ByteArray {
        val identityPublicKey = keyMap.keys.find { it.data.contentEquals(publicKey) }
        identityPublicKey?.let {
            val key = keyMap[identityPublicKey]
            if (key != null) {
                return key.signHash(Sha256Hash.twiceOf(data), keyParameter)
            }
        }
        error("cannot find public key ${Utils.HEX.encode(publicKey)}")
    }
}