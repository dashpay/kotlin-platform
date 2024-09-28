/**
 * Copyright (c) 2020-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
package org.dashj.platform.sdk.platform

import java.io.ByteArrayOutputStream
import org.bitcoinj.core.ECKey
import org.bitcoinj.core.Sha256Hash
import org.dashj.platform.dapiclient.SystemIds
import org.dashj.platform.dapiclient.model.DocumentQuery
import org.dashj.platform.dpp.document.DataDocumentTransition
import org.dashj.platform.dpp.document.Document
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identity.Identity
import org.dashj.platform.dpp.identity.IdentityPublicKey
import org.dashj.platform.dpp.util.Entropy
import org.dashj.platform.dpp.voting.Contenders
import org.dashj.platform.dpp.voting.ResourceVoteChoice
import org.dashj.platform.dpp.voting.Vote
import org.dashj.platform.sdk.callbacks.Signer
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class Names(val platform: Platform) {

    companion object {
        private val log: Logger = LoggerFactory.getLogger(Names::class.java)

        const val DEFAULT_PARENT_DOMAIN = "dash"
        const val DPNS_DATA_CONTRACT = "dpns"
        const val DOMAIN_DOCUMENT = "domain"
        const val PREORDER_DOCUMENT = "preorder"
        const val DPNS_DOMAIN_DOCUMENT = "dpns.domain"
        const val DPNS_PREORDER_DOCUMENT = "dpns.preorder"
        const val CONTESTED_INDEX = "parentNameAndLabel"

        fun isUniqueIdentity(domainDocument: Document): Boolean {
            val records = domainDocument.data["records"] as Map<*, *>
            return records.containsKey("identity")
        }

        fun isUniqueIdentity(dataDocumentTransition: DataDocumentTransition): Boolean {
            val records = dataDocumentTransition.data["records"] as Map<*, *>
            return records.containsKey("identity")
        }

        fun normalizedNames(name: String): Pair<String, String> {
            val nameSlice = name.indexOf('.')
            val normalizedParentDomainName =
                if (nameSlice == -1) DEFAULT_PARENT_DOMAIN else name.slice(nameSlice + 1 until name.length)

            val label = if (nameSlice == -1) name else name.slice(0 until nameSlice)

            val normalizedLabel = normalizeString(label)
            return Pair(normalizedParentDomainName, normalizedLabel)
        }

        fun normalizeString(text: String) = text.lowercase()
            .replace('o', '0')
            .replace('i', '1')
            .replace('l', '1')

        private fun getLabel(name: String): String {
            val nameSlice = name.indexOf('.')
            return if (nameSlice == -1) name else name.slice(0..nameSlice)
        }

        fun getSaltedDomainHashBytes(
            preOrderSaltRaw: ByteArray,
            name: String
        ): ByteArray {
            return getSaltedDomainHash(preOrderSaltRaw, name).bytes
        }

        fun getSaltedDomainHash(
            preOrderSaltRaw: ByteArray,
            fullName: String
        ): Sha256Hash {
            val baos = ByteArrayOutputStream(preOrderSaltRaw.size + fullName.length)
            baos.write(preOrderSaltRaw)
            baos.write(normalizeString(fullName).toByteArray())
            return Sha256Hash.twiceOf(baos.toByteArray())
        }

        fun isUsernameContestable(username: String): Boolean {
            val regex = Regex("^[a-zA-Z01-]{3,19}$")
            return regex.matches(username)
        }
    }

    fun register(
        name: String,
        identity: Identity,
        identityHDPrivateKey: ECKey,
        isUniqueIdentity: Boolean = true
    ): Document? {
        val entropy = Entropy.generate()
        val document = preorder(name, identity, identityHDPrivateKey, entropy)
        return if (document != null) {
            registerName(name, identity, identityHDPrivateKey, entropy, isUniqueIdentity)
        } else null
    }

    fun preorder(name: String, identity: Identity, identityHDPrivateKey: ECKey, preOrderSaltRaw: ByteArray): Document? {
        val (normalizedParentDomainName, normalizedLabel) = normalizedNames(name)
        val fullDomainName = "$normalizedLabel.$normalizedParentDomainName"

        val saltedDomainHash = getSaltedDomainHash(preOrderSaltRaw, fullDomainName)

        if (platform.apps["dpns"] == null) {
            throw Error("DPNS is required to register a new name.")
        }
        // 1. Create preorder document

        val preorderDocument = createPreorderDocument(saltedDomainHash, identity)

        val map = hashMapOf(
            "create" to listOf(preorderDocument)
        )

        // TODO: add code for put document
        return try {
            preorderDocument
        } catch (x: Exception) {
            null
        }
    }

    fun createPreorderDocument(
        saltedDomainHash: Sha256Hash,
        identity: Identity
    ): Document {
        val map = HashMap<String, Any?>(1)
        map["saltedDomainHash"] = saltedDomainHash.bytes; // .bytes.toBase64()

        val preorderDocument = platform.documents.create(
            DPNS_PREORDER_DOCUMENT,
            identity.id,
            map
        )
        return preorderDocument
    }

    fun registerName(
        name: String,
        identity: Identity,
        identityHDPrivateKey: ECKey,
        preorderSaltBase: ByteArray,
        isUniqueIdentity: Boolean = true
    ): Document? {
        val domainDocument = createDomainDocument(identity, name, preorderSaltBase, isUniqueIdentity)

        log.info("domainDocument: ${domainDocument.toJSON()}")

        // 4. Create and send domain state transition
        val map = hashMapOf<String, List<Document>?>(
            "create" to listOf(domainDocument)
        )
        // TODO: add code for put document

        return domainDocument
    }

    fun createDomainDocument(
        identity: Identity,
        name: String,
        preorderSaltBase: ByteArray,
        isUniqueIdentity: Boolean = true
    ): Document {
        val recordType = if (isUniqueIdentity) {
            "identity"
        } else {
            "dashAliasIdentityId"
        }
        val records = hashMapOf<String, Any?>(
            recordType to identity.id
        )

        val subdomainRules = hashMapOf<String, Any?>(
            "allowSubdomains" to false // do not allow
        )

        val (normalizedParentDomainName, normalizedLabel) = normalizedNames(name)

        val fields = HashMap<String, Any?>(7)
        fields["label"] = getLabel(name)
        fields["normalizedLabel"] = normalizeString(normalizedLabel)
        fields["normalizedParentDomainName"] = normalizedParentDomainName
        fields["parentDomainName"] = normalizedParentDomainName
        fields["preorderSalt"] = preorderSaltBase
        fields["records"] = records
        fields["subdomainRules"] = subdomainRules

        // 3. Create domain document
        val domainDocument = platform.documents.create(
            DPNS_DOMAIN_DOCUMENT,
            identity.id,
            fields
        )
        return domainDocument
    }

    private fun getDocumentQuery(name: String, parentDomain: String = DEFAULT_PARENT_DOMAIN): DocumentQuery {
        // with DPP 0.22 and the dpns contract, normalizedParentDomainName must be
        // before normalizedLabel in the where clauses
        return DocumentQuery.Builder()
            .where("normalizedParentDomainName", "==", parentDomain)
            .where("normalizedLabel", "==", normalizeString(name))
            .build()
    }

    /**
     * Gets the document for the given name if it exists under the default parent domain
     * @param name String
     * @return Document? The document for the given name or null if the name does not exist
     */
    fun get(name: String): Document? {
        return get(name, DEFAULT_PARENT_DOMAIN)
    }

    /**
     * Gets the document for the given name if it exists under the default parent domain
     * @param name String The name in the form of label.domain
     * @return Document? The document for the given name or null if the name does not exist
     */

    fun resolve(name: String): Document? {
        val (domain, label) = normalizedNames(name)
        return get(label, domain)
    }

    /**
     * Gets the document for the given name if it exists
     * @param name String
     * @param parentDomain String
     * @return Document? The document for the given name or null if the name does not exist
     */

    fun get(name: String, parentDomain: String): Document? {
        val documents = platform.documents.get(DPNS_DOMAIN_DOCUMENT, getDocumentQuery(name, parentDomain))
        return if (documents.isNotEmpty()) documents[0] else null
    }

    /**
     * Searches for and returns a list of all name documents that match the given name based
     * on these criteria: starts with.  Contains is not supported
     * @param text String the text to search the start of normalized labels
     * @param parentDomain String
     * @param retrieveAll Boolean whether or not to obtain all match results, regardless of the number
     * @param limit Int the number of items to return (-1 is the default)
     * @param startAfter the last item in the previous query.  The first query should have a null value.  If retrieveAll
     *                   is true, then this value should be null.
     * @return List<Documents>
     */
    fun search(text: String, parentDomain: String, retrieveAll: Boolean, limit: Int = -1, startAfter: Identifier? = null): List<Document> {
        val documentQuery = DocumentQuery.Builder()
            .where("normalizedParentDomainName", "==", parentDomain)
            .where("normalizedLabel", "startsWith", normalizeString(text))
            .orderBy("normalizedLabel", true)
            .limit(if (retrieveAll) -1 else limit)
            .startAfter(startAfter)

        return platform.documents.getAll(DPNS_DOMAIN_DOCUMENT, documentQuery.build())
    }

    /**
     * Gets all of the usernames associated with userId
     */

    fun getByOwnerId(ownerId: ByteArray): List<Document> {
        return getByOwnerId(Identifier.from(ownerId))
    }

    fun getByOwnerId(ownerId: String): List<Document> {
        return getByOwnerId(Identifier.from(ownerId))
    }

    fun getByOwnerId(ownerId: Identifier): List<Document> {
        return resolveByRecord("identity", ownerId)
    }

    /**
     * Gets all of the alias usernames associated with userId
     */
    fun getByUserIdAlias(ownerId: Identifier): List<Document> {
        return try {
            // this method returns an error
            // Query by not indexed field \"records.dashAliasIdentityId\" is not allowed"
            resolveByRecord("dashAliasIdentityId", ownerId)
        } catch (e: Exception) {
            arrayListOf()
        }
    }

    fun resolveByRecord(record: String, value: ByteArray): List<Document> {
        return resolveByRecord(record, Identifier.from(value))
    }

    fun resolveByRecord(record: String, value: String): List<Document> {
        return resolveByRecord(record, Identifier.from(value))
    }

    fun resolveByRecord(record: String, value: Identifier): List<Document> {
        val documentQuery = DocumentQuery.Builder()
            .where(listOf("records.$record", "==", value))

        val results = platform.documents.get(DPNS_DOMAIN_DOCUMENT, documentQuery.build())

        return results
    }

    // TODO: getList/getListHelper can be refactored into Documents
    /**
     * Gets all of the unique usernames associated with a list of userId's
     */

    fun getList(
        userIds: List<Identifier>
    ): List<Document> {
        var startAt = 0
        val documents = ArrayList<Document>()

        while (startAt < userIds.size) {
            val subsetSize = if (startAt + Documents.DOCUMENT_LIMIT > userIds.size) {
                userIds.size - startAt
            } else {
                Documents.DOCUMENT_LIMIT
            }
            val userIdSubSet = userIds.subList(startAt, startAt + subsetSize)
            val documentSubset = getListHelper(userIdSubSet)
            documents.addAll(documentSubset)
            startAt += subsetSize
        }

        return documents
    }

    /**
     * obtains the domain documents for the associated identities
     *
     * @param userIds A list of identity ids that must be limited to 100 in size
     */
    private fun getListHelper(
        userIds: List<Identifier>
    ): List<Document> {
        val documentQuery = DocumentQuery.Builder()
        documentQuery.whereIn("records.identity", userIds)
            .orderBy("records.identity", true)

        val documents = platform.documents.get(DPNS_DOMAIN_DOCUMENT, documentQuery.build())

        return documents
    }

    fun getVoteContenders(name: String): Contenders {
        return platform.client.getVoteContenders(
            platform.apps[DPNS_DATA_CONTRACT]!!.contractId,
            DOMAIN_DOCUMENT,
            "parentNameAndLabel",
            listOf(DEFAULT_PARENT_DOMAIN, name)
        )
    }

    fun getContestedNames(): List<String> {
        val resources = platform.client.getContestedResources(
            platform.apps[DPNS_DATA_CONTRACT]!!.contractId,
            DOMAIN_DOCUMENT
        )
        return if (resources.list.isNotEmpty()) {
            resources.list.map {
                if (it.value is String) {
                    it.value
                } else {
                    error("${it.value} is not a String")
                }
            }
        } else {
            listOf()
        }
    }

    fun deserialize(bytes: ByteArray): Document {
        return platform.documents.deserialize(
            bytes,
            platform.apps[DPNS_DATA_CONTRACT]!!.contractId,
            DOMAIN_DOCUMENT
        )
    }

    fun broadcastVote(
        resourceVoteChoice: ResourceVoteChoice,
        normalizedLabel: String,
        voterProTxHash: Sha256Hash,
        identityPublicKey: IdentityPublicKey,
        signerCallback: Signer
    ): Vote {
        return platform.documents.broadcastVote(
            resourceVoteChoice,
            SystemIds.dpnsDataContractId,
            DOMAIN_DOCUMENT,
            CONTESTED_INDEX,
            listOf(DEFAULT_PARENT_DOMAIN, normalizeString(normalizedLabel)),
            voterProTxHash,
            identityPublicKey,
            signerCallback
        )
    }
}
