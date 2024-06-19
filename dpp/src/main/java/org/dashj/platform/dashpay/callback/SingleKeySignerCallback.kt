package org.dashj.platform.dashpay.callback

import org.bitcoinj.core.ECKey
import org.bitcoinj.core.Sha256Hash
import org.dashj.platform.sdk.callbacks.Signer

/**
 * SingleKeySignerCallback expects a non-encrypted EC Key
 */

class SingleKeySignerCallback(private val privateKey: ECKey) : Signer() {
    override fun sign(key: ByteArray, data: ByteArray): ByteArray {
        return privateKey.signHash(Sha256Hash.twiceOf(data), null)
    }
}