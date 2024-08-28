/**
 * Copyright (c) 2018-present, Dash Core Team
 *
 * This source code is licensed under the MIT license found in the
 * COPYING file in the root directory of this source tree.
 */

package org.dashj.platform.dpp.document

import org.dashj.platform.dpp.BaseObject
import org.dashj.platform.dpp.Metadata
import org.dashj.platform.dpp.ProtocolVersion
import org.dashj.platform.dpp.contract.DataContract
import org.dashj.platform.dpp.deepCopy
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.util.Converters
import org.dashj.platform.dpp.util.convertProperties
import org.dashj.platform.dpp.util.convertToPlatformProperties
import org.dashj.platform.sdk.*
import org.dashj.platform.sdk.Document.Tag.V0
import kotlin.collections.HashMap

typealias RustDocument = org.dashj.platform.sdk.Document

class Document : BaseObject {

    var dataContract: DataContract? = null
    var id: Identifier
    var type: String? = null
    var dataContractId: Identifier?
    var ownerId: Identifier
    lateinit var entropy: ByteArray
    var revision: Long = 0
    var data: Map<String, Any?>
    var createdAt: Long?
    var updatedAt: Long?
    var metadata: Metadata? = null

    constructor(rawDocument: Map<String, Any?>, dataContract: DataContract) {
        this.dataContract = dataContract
        val data = HashMap(rawDocument)

        this.id = Identifier.from(data.remove("\$id")!!)
        this.type = data.remove("\$type") as String
        this.dataContractId = Identifier.from(data.remove("\$dataContractId")!!)
        this.ownerId = Identifier.from(data.remove("\$ownerId")!!)
        this.revision = if (data.containsKey("\$revision")) {
            data.remove("\$revision") as Long
        } else {
            DocumentCreateTransition.INITIAL_REVISION
        }.toLong()
        this.createdAt = data.remove("\$createdAt")?.let { it as Long }
        this.updatedAt = data.remove("\$updatedAt")?.let { it as Long }
        this.protocolVersion = if (data.containsKey("\$protocolVersion")) {
            data.remove("\$protocolVersion") as Int
        } else {
            ProtocolVersion.latestVersion
        }

        this.data = data
    }

    constructor(document: RustDocument, dataContractId: Identifier) {
        this.dataContractId = dataContractId
        when (document.tag) {
            V0 -> {
                val doc = document.v0._0
                val data = convertProperties(doc.properties)

                this.id = Identifier.from(doc.id)
                this.ownerId = Identifier.from(doc.owner_id)
                this.revision = doc.revision?.toLong() ?: 1L // can revision be null?
                this.createdAt = doc.created_at?.toLong()
                this.updatedAt = doc.updated_at?.toLong()
                this.data = data
            }
            else -> error("Document version not supported: ${document.tag}")
        }

    }

    override fun toObject(): Map<String, Any?> {
        return toObject(false)
    }

    fun toObject(skipIdentifierConversion: Boolean): MutableMap<String, Any?> {
        val map = hashMapOf(
            "\$protocolVersion" to protocolVersion,
            "\$id" to id,
            "\$type" to type,
            "\$dataContractId" to dataContractId,
            "\$ownerId" to ownerId,
            "\$revision" to revision
        )

        val deepCopy = data.deepCopy()
        map.putAll(deepCopy)

        createdAt?.let { map["\$createdAt"] = it }
        updatedAt?.let { map["\$updatedAt"] = it }

        if (!skipIdentifierConversion) {
            map["\$id"] = id.toBuffer()
            map["\$dataContractId"] = dataContractId?.toBuffer()
            map["\$ownerId"] = ownerId.toBuffer()

            // change binary items items in data to ByteArray
            Converters.convertIdentifierToByteArray(map)
        }

        return map
    }

    override fun toJSON(): Map<String, Any?> {
        val json = toObject(true)
        // change binary items in data to base64
        Converters.convertDataToString(json)
        return json
    }

    fun get(path: String): Any? {
        val keys = path.split("/")
        var value: Any? = data
        for (key in keys) {
            if ((value as Map<String, Any?>).containsKey(key)) {
                value = (value as Map<*, *>?)!![key]
            } else {
                return null
            }
        }
        return value
    }

    fun set(path: String, value: Any) {
        TODO("set field specified by path to the value")
    }

    fun setRevision(revision: Long): Document {
        this.revision = revision
        return this
    }

    fun toNative(): RustDocument {
        return RustDocument(
            DocumentV0(
                id.toNative(),
                ownerId.toNative(),
                convertToPlatformProperties(data),
                Revision(revision.toInt()),
                createdAt?.let { TimestampMillis(it) },
                updatedAt?.let { TimestampMillis(it) },
                null,
                null,
                null,
                null,
                null,
                null,
                null
            )
        )
    }
}
