package org.dashj.platform.dashpay.callback

import org.bitcoinj.core.Sha256Hash
import org.bitcoinj.core.Utils
import org.bitcoinj.wallet.Wallet
import org.bitcoinj.wallet.authentication.AuthenticationGroupExtension
import org.bouncycastle.crypto.params.KeyParameter
import org.dashj.platform.sdk.callbacks.Signer

class WalletSignerCallback @JvmOverloads constructor(
    wallet: Wallet,
    val keyParameter: KeyParameter?,
    private val checkAuthorization: () -> Unit = {}
) : Signer() {

    private val authenticationGroup = wallet.getKeyChainExtension(AuthenticationGroupExtension.EXTENSION_ID) as AuthenticationGroupExtension

    fun requireAuthorized() = checkAuthorization()

    override fun sign(publicKey: ByteArray, data: ByteArray): ByteArray {
        requireAuthorized()
        val key = authenticationGroup.identityKeyChain?.findKeyFromPubKey(publicKey)
        if (key != null) {
            return key.signHash(Sha256Hash.twiceOf(data), keyParameter)
        } else {
            error("cannot find public key ${Utils.HEX.encode(publicKey)}")
        }
    }
}
