/**
 * Copyright (c) 2022-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

package org.dashj.platform.wallet

import java.util.Date
import kotlin.collections.HashMap
import org.bitcoinj.core.ECKey
import org.dashj.platform.contracts.wallet.TxMetadataDocument
import org.dashj.platform.dapiclient.model.DocumentQuery
import org.dashj.platform.dashpay.callback.SingleKeySignerCallback
import org.dashj.platform.dpp.document.Document
import org.dashj.platform.dpp.document.DocumentCreateTransition
import org.dashj.platform.dpp.document.DocumentsBatchTransition
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identity.Identity
import org.dashj.platform.dpp.util.Cbor
import org.dashj.platform.sdk.BlockHeight
import org.dashj.platform.sdk.CoreBlockHeight
import org.dashj.platform.sdk.Purpose
import org.dashj.platform.sdk.SecurityLevel
import org.dashj.platform.sdk.callbacks.Signer
import org.dashj.platform.sdk.dashsdk
import org.dashj.platform.sdk.platform.Platform
import org.dashj.platform.wallet.WalletUtils.TxMetadataBatch
import java.math.BigInteger

class TxMetadata(
    val platform: Platform
) {

    companion object {
        const val DOCUMENT: String = "wallet-utils.txMetadata"
    }

    fun create(
        keyIndex: Int,
        encryptionKeyIndex: Int,
        encryptedMetadata: ByteArray,
        identity: Identity,
        id: Int,
        signer: Signer
    ): Document {
        val profileDocument = createDocument(keyIndex, encryptionKeyIndex, encryptedMetadata, identity)
        profileDocument.createdAt = Date().time
        val highIdentityPublicKey = identity.getFirstPublicKey(Purpose.AUTHENTICATION, SecurityLevel.HIGH)
            ?: error("can't find a public key with HIGH security level")

        val documentResult = dashsdk.platformMobilePutPutDocumentSdk(
            platform.rustSdk,
            profileDocument.toNative(),
            profileDocument.dataContractId!!.toNative(),
            profileDocument.type,
            highIdentityPublicKey.toNative(),
            BlockHeight(10000),
            CoreBlockHeight(platform.coreBlockHeight),
            signer.nativeContext,
            BigInteger.valueOf(signer.signerCallback),
        )
        val domain = documentResult.unwrap()

        return Document(domain, profileDocument.dataContractId!!)
    }

    fun publish(
        txMetadataDocument: Document,
        identity: Identity,
        signer: Signer
    ): Document {
        val highIdentityPublicKey = identity.getFirstPublicKey(Purpose.AUTHENTICATION, SecurityLevel.HIGH)
            ?: error("can't find a public key with HIGH security level")

        val documentResult = dashsdk.platformMobilePutPutDocumentSdk(
            platform.rustSdk,
            txMetadataDocument.toNative(),
            txMetadataDocument.dataContractId!!.toNative(),
            txMetadataDocument.type,
            highIdentityPublicKey.toNative(),
            BlockHeight(10000),
            CoreBlockHeight(platform.coreBlockHeight),
            signer.nativeContext,
            BigInteger.valueOf(signer.signerCallback),
        )
        val domain = documentResult.unwrap()

        return Document(domain, txMetadataDocument.dataContractId!!)
    }

    fun createDocument(
        keyIndex: Int,
        encryptionKeyIndex: Int,
        encryptedMetadata: ByteArray,
        identity: Identity
    ): Document {
        val document = platform.documents.create(
            DOCUMENT,
            identity.id,
            mutableMapOf(
                "keyIndex" to keyIndex.toLong(),
                "encryptionKeyIndex" to encryptionKeyIndex.toLong(),
                "encryptedMetadata" to encryptedMetadata
            )
        )
        document.revision = DocumentCreateTransition.INITIAL_REVISION
        document.createdAt = Date().time
        return document
    }

    fun getBuffer(version: Int, metadataItems: List<TxMetadataItem>): ByteArray {
        return when (version) {
            TxMetadataDocument.VERSION_CBOR -> Cbor.encode(metadataItems.map { it.toObject() })
            TxMetadataDocument.VERSION_PROTOBUF -> {
                TxMetadataBatch.newBuilder().addAllItems(metadataItems.map { it.toProtobuf() }).build().toByteArray()
            }
            else -> error("Invalid version txmetadata $version")
        }
    }

    fun get(userId: String): List<Document> {
        return get(Identifier.from(userId))
    }

    fun get(userId: Identifier, createdAfter: Long = -1): List<Document> {
        val queryBuilder = DocumentQuery.Builder()
            .where("\$ownerId", "==", userId)
            .orderBy("\$updatedAt")

        if (createdAfter != -1L) {
            queryBuilder.where(listOf("\$updatedAt", ">=", createdAfter))
        }

        val query = queryBuilder.build()

        return platform.documents.get(DOCUMENT, query)
    }
}
