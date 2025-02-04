/**
 * Copyright (c) 2024-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

package org.dashj.platform.wallet

import org.dashj.platform.dapiclient.model.DocumentQuery
import org.dashj.platform.dpp.document.Document
import org.dashj.platform.dpp.document.DocumentCreateTransition
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identity.Identity
import org.dashj.platform.sdk.BlockHeight
import org.dashj.platform.sdk.CoreBlockHeight
import org.dashj.platform.sdk.SecurityLevel
import org.dashj.platform.sdk.callbacks.Signer
import org.dashj.platform.sdk.dashsdk
import org.dashj.platform.sdk.platform.Names
import org.dashj.platform.sdk.platform.Platform
import org.slf4j.LoggerFactory
import java.math.BigInteger

class IdentityVerify(
    val platform: Platform
) {

    companion object {
        const val DOCUMENT: String = "identity-verify.identityVerify"
        private val log = LoggerFactory.getLogger(IdentityVerify::class.java)
    }

    fun createForDashDomain(username: String,
                            url: String,
                            identity: Identity,
                            signer: Signer): IdentityVerifyDocument {
        return create(
            Names.normalizeString(username),
            Names.DEFAULT_PARENT_DOMAIN,
            url,
            identity,
            signer
        )
    }

    /**
     * this will publish an identity verify document unless it already exists
     */
    fun create(
        normalizedLabel: String,
        normalizedParentDomainName: String,
        url: String,
        identity: Identity,
        signer: Signer
    ): IdentityVerifyDocument {
        val existingIdentityVerifyDocument = get(identity.id, normalizedLabel, normalizedParentDomainName)
        if (existingIdentityVerifyDocument != null) {
            log.info("identity verify document already exists")
            return existingIdentityVerifyDocument
        }
        val identityVerifyDocument = createDocument(normalizedLabel, normalizedParentDomainName, url, identity)
        val highIdentityPublicKey = identity.getFirstPublicKey(SecurityLevel.HIGH)
            ?: error("can't find a public key with HIGH security level")

        val documentResult = dashsdk.platformMobilePutPutDocumentSdk(
            platform.rustSdk,
            identityVerifyDocument.toNative(),
            identityVerifyDocument.dataContractId!!.toNative(),
            identityVerifyDocument.type,
            highIdentityPublicKey.toNative(),
            BlockHeight(10000),
            CoreBlockHeight(platform.coreBlockHeight),
            signer.nativeContext,
            BigInteger.valueOf(signer.signerCallback),
        )
        val domain = documentResult.unwrap()

        return IdentityVerifyDocument(Document(domain, identityVerifyDocument.dataContractId!!))
    }

    fun createDocument(
        normalizedLabel: String,
        normalizedParentDomainName: String,
        url: String,
        identity: Identity
    ): IdentityVerifyDocument {
        val document = platform.documents.create(
            DOCUMENT,
            identity.id,
            mutableMapOf(
                "normalizedLabel" to normalizedLabel,
                "normalizedParentDomainName" to normalizedParentDomainName,
                "url" to url,
            )
        )
        document.revision = DocumentCreateTransition.INITIAL_REVISION
        return IdentityVerifyDocument(document)
    }

    fun get(userId: String): List<IdentityVerifyDocument> {
        return get(Identifier.from(userId))
    }

    fun get(userId: Identifier, createdAfter: Long = -1): List<IdentityVerifyDocument> {
        val queryBuilder = DocumentQuery.Builder()
            .where("\$ownerId", "==", userId)

        if (createdAfter != -1L) {
            queryBuilder.where(listOf("\$createdAt", ">=", createdAfter))
        }

        val query = queryBuilder.build()

        return platform.documents.get(DOCUMENT, query).map { IdentityVerifyDocument(it) }
    }

    fun get(userId: Identifier, username: String): IdentityVerifyDocument? {
        return get(userId, Names.normalizeString(username), Names.DEFAULT_PARENT_DOMAIN)
    }

    fun get(userId: Identifier, normalizedLabel: String, normalizedParentDomainName: String): IdentityVerifyDocument? {
        val queryBuilder = DocumentQuery.Builder()
            .where("\$ownerId", "==", userId)
            .where("normalizedParentDomainName", "==", normalizedParentDomainName)
            .where("normalizedLabel", "==", normalizedLabel)
            .orderBy("normalizedLabel")

        val query = queryBuilder.build()

        val result = platform.documents.get(DOCUMENT, query)
        return if (result.isNotEmpty()) {
            IdentityVerifyDocument(result.first())
        } else {
            null
        }
    }

    fun get(query: DocumentQuery): List<IdentityVerifyDocument> {
        val result = platform.documents.get(DOCUMENT, query)
        return result.map { IdentityVerifyDocument(it) }
    }
}
