/**
 * Copyright (c) 2020-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
package org.dashj.platform.sdk.platform

import org.bitcoinj.core.ECKey
import org.bitcoinj.core.Transaction
import org.bitcoinj.evolution.AssetLockTransaction
import org.bitcoinj.quorums.InstantSendLock
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identity.*
import org.dashj.platform.sdk.AssetLockProof
import org.dashj.platform.sdk.IdentityV0
import org.dashj.platform.sdk.KeyID
import org.dashj.platform.sdk.Revision
import org.dashj.platform.sdk.callbacks.Signer
import org.dashj.platform.sdk.dashsdk
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import java.math.BigInteger

class Identities(val platform: Platform) {

    companion object {
        private val log: Logger = LoggerFactory.getLogger(Identities::class.java)
    }

    fun register(
        signedLockTransaction: AssetLockTransaction,
        instantLock: InstantSendLock,
        identityPublicKeys: List<IdentityPublicKey>,
        signer: Signer
    ): Identity {
        return register(
            0,
            signedLockTransaction,
            instantLock,
            signedLockTransaction.assetLockPublicKey,
            identityPublicKeys,
            signer
        )
    }

    fun register(
        outputIndex: Long,
        transaction: AssetLockTransaction,
        instantLock: InstantSendLock,
        assetLockPrivateKey: ECKey,
        identityPublicKeys: List<IdentityPublicKey>,
        signer: Signer
    ): Identity {
        try {
            val assetLock = InstantAssetLockProof(outputIndex, transaction, instantLock)
            val identityId = assetLock.createIdentifier()

            val identityV0 = IdentityV0(
                identityId.toNative(),
                identityPublicKeys.associateBy({ KeyID(it.id) }, { it.toNative() }),
                BigInteger.ZERO,
                Revision(1)
            )
            val identity = RustIdentity(identityV0)
            val identityResult = dashsdk.platformMobilePutPutIdentitySdk(
                platform.rustSdk,
                identity,
                AssetLockProof(assetLock.toNative()),
                assetLockPrivateKey.privKeyBytes,
                signer.nativeContext,
                BigInteger.valueOf(signer.signerCallback),
                platform.isTestNet()
            )

            return Identity(identityResult.unwrap())
        } catch (e: Exception) {
            log.info("registerIdentity failure: $e")
            throw e
        }
    }

    fun register(
        outputIndex: Long,
        transaction: AssetLockTransaction,
        coreHeight: Long,
        assetLockPrivateKey: ECKey,
        identityPublicKeys: List<IdentityPublicKey>,
        signer: Signer
    ): Identity {
        try {
            log.info("AssetLockTransaction: ${transaction.txId}")
            val assetLock = ChainAssetLockProof(coreHeight, transaction.getOutput(outputIndex).outPointFor)
            val identityId = assetLock.createIdentifier()

            val identityV0 = IdentityV0(
                identityId.toNative(),
                identityPublicKeys.associateBy({ KeyID(it.id) }, { it.toNative() }),
                BigInteger.ZERO,
                Revision(1)
            )
            val identity = RustIdentity(identityV0)
            val identityResult = dashsdk.platformMobilePutPutIdentitySdk(
                platform.rustSdk,
                identity,
                AssetLockProof(assetLock.toNative()),
                assetLockPrivateKey.privKeyBytes,
                signer.nativeContext,
                BigInteger.valueOf(signer.signerCallback),
                platform.isTestNet()
            )
            return Identity(identityResult.unwrap())
        } catch (e: Exception) {
            log.info("registerIdentity failure: ", e)
            throw e
        }
    }

    fun get(id: String): Identity? {
        return get(Identifier.from(id))
    }

    fun get(id: Identifier): Identity? {
        return platform.stateRepository.fetchIdentity(id)
    }

    fun getByPublicKeyHash(pubKeyHash: ByteArray): Identity? {
        return platform.stateRepository.fetchIdentityFromPubKeyHash(pubKeyHash)
    }

    fun topUp(
        identityId: Identifier,
        signedLockTransaction: AssetLockTransaction,
        instantLock: InstantSendLock
    ): Boolean {
        return topUp(
            identityId,
            0,
            signedLockTransaction,
            instantLock,
            signedLockTransaction.assetLockPublicKey
        )
    }

    fun topUp(
        identityId: Identifier,
        outputIndex: Long,
        transaction: Transaction,
        instantLock: InstantSendLock,
        assetLockPrivateKey: ECKey
    ): Boolean {
        try {
            val assetLock = InstantAssetLockProof(outputIndex, transaction, instantLock)

            val identityTopupTransition = platform.dpp.identity.createIdentityTopUpTransition(identityId, assetLock)

            identityTopupTransition.signByPrivateKey(assetLockPrivateKey)

            platform.broadcastStateTransition(identityTopupTransition)

            return true
        } catch (e: Exception) {
            log.info("topup failure: ", e)
            throw e
        }
    }
}
