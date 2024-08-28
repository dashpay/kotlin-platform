package org.dashj.platform.dpp.voting

import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identifier.RustIdentifier
import org.dashj.platform.dpp.util.convertPlatformValue

typealias RustContenders = org.dashj.platform.sdk.Contenders
typealias RustContenderWithSerializedDocument = org.dashj.platform.sdk.ContenderWithSerializedDocument
typealias RustContestedResources = org.dashj.platform.sdk.ContestedResources
typealias RustContestedResource = org.dashj.platform.sdk.ContestedResource


data class ContenderWithSerializedDocument(val identityId: Identifier, val seralizedDocument: ByteArray, val votes: Int) {
    constructor(contenderWithSerializedDocument: RustContenderWithSerializedDocument) : this(
        Identifier.from(contenderWithSerializedDocument.v0._0.identity_id.bytes),
        contenderWithSerializedDocument.v0._0.serialized_document,
        contenderWithSerializedDocument.v0._0.voteTally
    )
}

private fun convertContenders(map: Map<RustIdentifier, RustContenderWithSerializedDocument>): Map<Identifier, ContenderWithSerializedDocument> {
    val result = hashMapOf<Identifier, ContenderWithSerializedDocument>()
    map.forEach { (t, u) ->  result[Identifier.from(t)] = ContenderWithSerializedDocument(u) }
    return result
}

class Contenders(val map: Map<Identifier, ContenderWithSerializedDocument>, val abstainVoteTally: Int, val lockVoteTally: Int) {
    constructor(contenders: RustContenders) : this(
        convertContenders(contenders.contenders),
        contenders.abstainVoteTally,
        contenders.lockVoteTally
    )

    fun isEmpty() = map.isEmpty()
}

private fun convertContestedResource(contestedResource: RustContestedResource): Any? {
    return when (contestedResource.tag) {
        org.dashj.platform.sdk.ContestedResource.Tag.Value -> {
            convertPlatformValue(contestedResource.value)
        }
        else -> error("${contestedResource.tag} not supported")
    }
}

class ContestedResource(val value: Any?) {
    constructor(contestedResource: RustContestedResource) : this(
        convertContestedResource(contestedResource)
    )
}

class ContestedResources(val list: List<ContestedResource>) {
    constructor(contestedResources: RustContestedResources): this (
        contestedResources._0.map { ContestedResource(it) }
    )
}