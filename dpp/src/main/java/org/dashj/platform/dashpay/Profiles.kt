/**
 * Copyright (c) 2020-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

package org.dashj.platform.dashpay

import java.util.Date
import org.dashj.platform.dapiclient.model.DocumentQuery
import org.dashj.platform.dashpay.callback.WalletSignerCallback
import org.dashj.platform.dpp.document.Document
import org.dashj.platform.dpp.document.DocumentCreateTransition
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identity.Identity
import org.dashj.platform.sdk.BlockHeight
import org.dashj.platform.sdk.CoreBlockHeight
import org.dashj.platform.sdk.SecurityLevel
import org.dashj.platform.sdk.dashsdk
import org.dashj.platform.sdk.platform.Documents
import org.dashj.platform.sdk.platform.Platform
import org.slf4j.LoggerFactory
import java.math.BigInteger

class Profiles(
    val platform: Platform
) {

    companion object {
        const val DOCUMENT: String = "dashpay.profile"
        val log = LoggerFactory.getLogger(Profiles::class.java)
    }

    fun create(
        displayName: String?,
        publicMessage: String?,
        avatarUrl: String?,
        avatarHash: ByteArray?,
        avatarFingerprint: ByteArray?,
        identity: Identity,
        id: Int,
        signer: WalletSignerCallback
    ): Document {
        val profileDocument = createProfileDocument(displayName, publicMessage, avatarUrl, avatarHash, avatarFingerprint, identity)
        profileDocument.createdAt = Date().time

        val highIdentityPublicKey = identity.getFirstPublicKey(SecurityLevel.HIGH)
            ?: error("can't find a public key with HIGH security level")

        val credits = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, identity!!.id.toNative()).unwrap()
        log.info("credit balance: {}", credits)

        val profileResult = dashsdk.platformMobilePutPutDocumentSdk(
            platform.rustSdk,
            profileDocument.toNative(),
            profileDocument.dataContractId!!.toNative(),
            profileDocument.type,
            highIdentityPublicKey.toNative(),
            BlockHeight(10000),
            CoreBlockHeight(platform.coreBlockHeight),
            signer.nativeContext,
            BigInteger.valueOf(signer.signerCallback)
        )
        return Document(profileResult.unwrap(), profileDocument.dataContractId!!)
    }

    fun replace(
        displayName: String?,
        publicMessage: String?,
        avatarUrl: String?,
        avatarHash: ByteArray?,
        avatarFingerprint: ByteArray?,
        identity: Identity,
        id: Int,
        signer: WalletSignerCallback
    ): Document {
        val currentProfile = get(identity.id)

        val profileData = hashMapOf<String, Any?>()
        profileData.putAll(currentProfile!!.toObject())
        if (displayName != null) {
            profileData["displayName"] = displayName
        }
        if (publicMessage != null) {
            profileData["publicMessage"] = publicMessage
        }
        if (avatarUrl != null) {
            profileData["avatarUrl"] = avatarUrl
            if (avatarHash != null) {
                profileData["avatarHash"] = avatarHash
            }
            if (avatarFingerprint != null) {
                profileData["avatarFingerprint"] = avatarFingerprint
            }
        }
        profileData["\$type"] = "profile"

        val profileDocument = platform.dpp.document.createFromObject(profileData)
        profileDocument.updatedAt = Date().time
        profileDocument.revision += 1

        val highIdentityPublicKey = identity.getFirstPublicKey(SecurityLevel.HIGH)
            ?: error("can't find a public key with HIGH security level")

        val credits = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(platform.rustSdk, identity!!.id.toNative()).unwrap()
        log.info("credit balance: {}", credits)

        // under the hood this calls new_document_creation_transition_from_document
        // and not new_document_replacement_transition_from_document
        val profileResult = dashsdk.platformMobilePutReplaceDocumentSdk(
            platform.rustSdk,
            profileDocument.toNative(),
            profileDocument.dataContractId!!.toNative(),
            profileDocument.type,
            highIdentityPublicKey.toNative(),
            BlockHeight(10000),
            CoreBlockHeight(platform.coreBlockHeight),
            signer.nativeContext,
            BigInteger.valueOf(signer.signerCallback)
        )
        return Document(profileResult.unwrap(), profileDocument.dataContractId!!)
    }

    fun createProfileDocument(
        displayName: String?,
        publicMessage: String?,
        avatarUrl: String?,
        avatarHash: ByteArray?,
        avatarFingerprint: ByteArray?,
        identity: Identity,
        revision: Long = DocumentCreateTransition.INITIAL_REVISION
    ): Document {
        val profileData = hashMapOf<String, Any?>()
        if (displayName != null) {
            profileData["displayName"] = displayName
        }
        if (publicMessage != null) {
            profileData["publicMessage"] = publicMessage
        }
        if (avatarUrl != null) {
            profileData["avatarUrl"] = avatarUrl
            if (avatarHash != null) {
                profileData["avatarHash"] = avatarHash
            }
            if (avatarFingerprint != null) {
                profileData["avatarFingerprint"] = avatarFingerprint
            }
        }
        profileData["\$type"] = "profile"
        val document = platform.documents.create(
            DOCUMENT, identity.id,
            profileData
        )
        document.revision = revision.toLong()
        if (revision == DocumentCreateTransition.INITIAL_REVISION) {
            document.createdAt = Date().time
            document.updatedAt = document.createdAt
        } else {
            document.updatedAt = Date().time
        }
        return document
    }

    fun get(userId: String): Document? {
        return get(Identifier.from(userId))
    }

    fun get(userId: Identifier, updatedAt: Long = -1): Document? {
        val queryBuilder = DocumentQuery.Builder()
            .where("\$ownerId", "==", userId)

        if (updatedAt != -1L) {
            queryBuilder.where(listOf("\$updatedAt", "==", updatedAt))
        }

        val query = queryBuilder.build()
        try {
            val documents = platform.documents.get(DOCUMENT, query)
            return if (documents.isNotEmpty()) documents[0] else null
        } catch (e: Exception) {
            throw e
        }
    }

    /**
     * Returns all profiles associated with the given identity ids
     *
     * @param userIds The identities for which to obtain the profiles.  This supports more than 100 identities
     * @param timestamp The timestamp that the profile updatedAt time must be after
     */
    fun getList(
        userIds: List<Identifier>,
        timestamp: Long = 0L
    ): List<Document> {
        var startAt = 0 // this parameter is not used by a getDocuments query, so 0 is good
        val documents = ArrayList<Document>()

        while (startAt < userIds.size) {
            val subsetSize = if (startAt + Documents.DOCUMENT_LIMIT > userIds.size) {
                userIds.size - startAt
            } else {
                Documents.DOCUMENT_LIMIT
            }
            val userIdSubSet = userIds.subList(startAt, startAt + subsetSize)
            val documentSubset = getListHelper(userIdSubSet, timestamp)
            documents.addAll(documentSubset)
            startAt += subsetSize
        }

        return documents
    }

    /**
     * gets a list of profiles using the identities in userIds (max 100)
     *
     * This query never has more than 100 results, since userIds are primary keys
     */
    private fun getListHelper(
        userIds: List<Identifier>,
        timestamp: Long = 0L
    ): List<Document> {
        val documentQuery = DocumentQuery.builder()
            .whereIn("\$ownerId", userIds)
            .orderBy("\$ownerId", true)
            .build()

        return platform.documents.get(DOCUMENT, documentQuery)
    }

    @Deprecated("not required with wait functions")
    suspend fun watchProfile(
        userId: String,
        retryCount: Int,
        delayMillis: Long,
        retryDelayType: RetryDelayType
    ): Document? {
        val documentQuery = DocumentQuery.Builder()
        documentQuery.where("\$ownerId", "==", userId)
        val result = platform.documents.get(DOCUMENT, documentQuery.build())

        if (result.isNotEmpty()) {
            return result[0]
        } else {
            if (retryCount > 0) {
                val nextDelay = delayMillis * when (retryDelayType) {
                    RetryDelayType.SLOW20 -> 5 / 4
                    RetryDelayType.SLOW50 -> 3 / 2
                    else -> 1
                }
                kotlinx.coroutines.delay(nextDelay)
                return watchProfile(userId, retryCount - 1, nextDelay, retryDelayType)
            }
        }
        return null
    }
}
