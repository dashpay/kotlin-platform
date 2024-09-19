package org.dashj.platform.dpp.voting

import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identifier.RustIdentifier
import org.dashj.platform.dpp.util.convertPlatformValue
import org.dashj.platform.sdk.TupleContestedDocumentVotePollWinnerInfoBlockInfo
import java.util.*

typealias RustContenders = org.dashj.platform.sdk.Contenders
typealias RustContenderWithSerializedDocument = org.dashj.platform.sdk.ContenderWithSerializedDocument
typealias RustContestedResources = org.dashj.platform.sdk.ContestedResources
typealias RustContestedResource = org.dashj.platform.sdk.ContestedResource


data class ContenderWithSerializedDocument(val identityId: Identifier, val seralizedDocument: ByteArray?, val votes: Int) {
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
typealias RustContestedDocumentVotePollWinnerInfo = org.dashj.platform.sdk.ContestedDocumentVotePollWinnerInfo
typealias RustBlockInfo = org.dashj.platform.sdk.BlockInfo
typealias RustEpoch = org.dashj.platform.sdk.Epoch

class Epoch(val index: Long, key: ByteArray) {
    constructor(rustEpoch: RustEpoch) : this (
        rustEpoch.index.toLong(),
        rustEpoch.key
    )
}

class BlockInfo(val epoch: Epoch, val height: Int, val time: Long, val coreHeight: Long) {
    constructor(blockInfo: RustBlockInfo) : this(
        Epoch(blockInfo.epoch),
        blockInfo.height.toLong(),
        blockInfo.time_ms._0.toLong(),
        blockInfo.core_height.toLong()
    )
}

class ContestedDocumentVotePollWinnerInfo(
    tag: org.dashj.platform.sdk.ContestedDocumentVotePollWinnerInfo.Tag,
    val identifier: Identifier?
) {
    constructor(contestedDocumentVotePollWinnerInfo: RustContestedDocumentVotePollWinnerInfo) : this(
        contestedDocumentVotePollWinnerInfo.tag,
        if (contestedDocumentVotePollWinnerInfo.tag == org.dashj.platform.sdk.ContestedDocumentVotePollWinnerInfo.Tag.WonByIdentity) {
            Identifier.from(contestedDocumentVotePollWinnerInfo.won_by_identity._0)
        } else {
            null
        }
    )
    val isLocked = tag == org.dashj.platform.sdk.ContestedDocumentVotePollWinnerInfo.Tag.Locked
    val noWinner = tag == org.dashj.platform.sdk.ContestedDocumentVotePollWinnerInfo.Tag.NoWinner
    fun isWinner(identifier: Identifier) = this.identifier == identifier
}

private fun convertTupleContestedDocumentVotePollWinnerInfoBlockInfo(winner: TupleContestedDocumentVotePollWinnerInfoBlockInfo?): Pair<ContestedDocumentVotePollWinnerInfo, BlockInfo>? {
    return if (winner != null) {
        Pair(ContestedDocumentVotePollWinnerInfo(winner.o_0), BlockInfo(winner.o_1))
    } else {
        null
    }
}

class Contenders(val winner: Optional<Pair<ContestedDocumentVotePollWinnerInfo, BlockInfo>>, val map: Map<Identifier, ContenderWithSerializedDocument>, val abstainVoteTally: Int, val lockVoteTally: Int) {
    constructor(contenders: RustContenders) : this(
        Optional.ofNullable(convertTupleContestedDocumentVotePollWinnerInfoBlockInfo(contenders.winner)),
        convertContenders(contenders.contenders),
        contenders.abstainVoteTally,
        contenders.lockVoteTally
    )

    fun isEmpty() = map.isEmpty()
    fun size() = map.size
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