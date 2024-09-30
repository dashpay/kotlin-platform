package org.dashj.platform.dpp.voting

import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.util.convertToPlatformValue
import org.dashj.platform.sdk.ResourceVoteV0

typealias RustResourceVote = org.dashj.platform.sdk.ResourceVote

open class VersionedObject(val version: Int = 0)

typealias RustResourceVoteChoice = org.dashj.platform.sdk.ResourceVoteChoice

abstract class ResourceVoteChoice {
    abstract val tag: org.dashj.platform.sdk.ResourceVoteChoice.Tag
    companion object {
        fun from(resourceVoteChoice: RustResourceVoteChoice): ResourceVoteChoice {
            return when (resourceVoteChoice.tag) {
                org.dashj.platform.sdk.ResourceVoteChoice.Tag.TowardsIdentity -> {
                    TowardsIdentity(Identifier.from(resourceVoteChoice.towards_identity._0.bytes))
                }
                org.dashj.platform.sdk.ResourceVoteChoice.Tag.Abstain -> {
                    AbstainVoteChoice()
                }
                org.dashj.platform.sdk.ResourceVoteChoice.Tag.Lock -> {
                    LockVoteChoice()
                }
                else -> error("Unknown resource vote choice: " + resourceVoteChoice.tag)
            }
        }
        fun from (text: String): ResourceVoteChoice {
            val parts = text.split(":")
            return when {
                parts.isEmpty() -> error("invalid ResourceVoteChoice text")
                parts[0] == org.dashj.platform.sdk.ResourceVoteChoice.Tag.TowardsIdentity.name -> {
                    TowardsIdentity(Identifier.from(parts[1]))
                }
                parts[0] == org.dashj.platform.sdk.ResourceVoteChoice.Tag.Abstain.name -> {
                    AbstainVoteChoice()
                }
                parts[0] == org.dashj.platform.sdk.ResourceVoteChoice.Tag.Lock.name -> {
                    LockVoteChoice()
                }
                else -> error("Unknown resource vote choice: " + parts[0])
            }
        }
        fun towardsIdentity(identity: Identifier): ResourceVoteChoice {
            return TowardsIdentity(identity)
        }
        fun abstain(): ResourceVoteChoice {
            return AbstainVoteChoice()
        }
        fun lock(): ResourceVoteChoice {
            return LockVoteChoice()
        }
    }
    open fun toNative(): RustResourceVoteChoice {
        return RustResourceVoteChoice(tag, null)
    }

    override fun toString(): String {
        return tag.name
    }
}

class TowardsIdentity(val identifier: Identifier): ResourceVoteChoice() {
    override val tag: org.dashj.platform.sdk.ResourceVoteChoice.Tag
        get() = org.dashj.platform.sdk.ResourceVoteChoice.Tag.TowardsIdentity

    override fun toNative(): RustResourceVoteChoice {
        return RustResourceVoteChoice(tag, identifier.toNative())
    }

    override fun toString(): String {
        return super.toString() + ":" + identifier.toString()
    }
}
class AbstainVoteChoice : ResourceVoteChoice() {
    override val tag: org.dashj.platform.sdk.ResourceVoteChoice.Tag
        get() = org.dashj.platform.sdk.ResourceVoteChoice.Tag.Abstain
}
class LockVoteChoice : ResourceVoteChoice() {
    override val tag: org.dashj.platform.sdk.ResourceVoteChoice.Tag
        get() = org.dashj.platform.sdk.ResourceVoteChoice.Tag.Lock
}

typealias RustVotePoll = org.dashj.platform.sdk.VotePoll

abstract class VotePoll {
    companion object {
        fun from(votePoll: RustVotePoll): VotePoll {
            return when(votePoll.tag) {
                org.dashj.platform.sdk.VotePoll.Tag.ContestedDocumentResourceVotePoll -> {
                    ContestedDocumentResourceVotePoll(votePoll.contested_document_resource_vote_poll._0)
                }
                else -> error("unknown VotePoll tag: " + votePoll.tag)
            }
        }
    }
    abstract fun toNative(): RustVotePoll
}

typealias RustContestedDocumentResourceVotePoll = org.dashj.platform.sdk.ContestedDocumentResourceVotePoll

class ContestedDocumentResourceVotePoll(
    val dataContractId: Identifier,
    val documentTypeName: String,
    val indexName: String,
    val indexValues: List<Any>
): VotePoll() {
    constructor(votePoll: RustContestedDocumentResourceVotePoll) : this(
        Identifier.from(votePoll.contract_id.bytes),
        votePoll.document_type_name,
        votePoll.index_name,
        votePoll.index_values
    )

    override fun toNative(): RustVotePoll {
        return RustVotePoll(
            RustContestedDocumentResourceVotePoll(
                dataContractId.toNative(),
                documentTypeName,
                indexName,
                indexValues.map { convertToPlatformValue(it) }
            )
        )
    }
}

class ResourceVote : VersionedObject {
    companion object {
        const val CURRENT_VERSION = 0
    }
    val resourceVoteChoice: ResourceVoteChoice
    val votePoll: VotePoll

    constructor(resourceVoteChoice: ResourceVoteChoice, votePoll: VotePoll) : super(CURRENT_VERSION) {
        this.resourceVoteChoice = resourceVoteChoice
        this.votePoll = votePoll
    }

    constructor(resourceVote: RustResourceVote) : super(resourceVote.tag.swigValue()) {
        when (resourceVote.tag) {
            org.dashj.platform.sdk.ResourceVote.Tag.V0 -> {
                resourceVoteChoice = ResourceVoteChoice.from(resourceVote.v0._0.resource_vote_choice)
                votePoll = VotePoll.from(resourceVote.v0._0.vote_poll)
            }
            else -> error("unknown tag for ResourceVote: " + resourceVote.tag)
        }
    }

    fun toNative(): RustResourceVote {
        return when (version) {
                0 -> RustResourceVote(
                    ResourceVoteV0(
                        votePoll.toNative(),
                        resourceVoteChoice.toNative()
                    )
                )
                else -> error("Unsupported version ($version) of ResourceVote")
            }
    }
}

typealias RustVote = org.dashj.platform.sdk.Vote

class Vote(val resourceVote: ResourceVote) {
    constructor(vote: RustVote) : this(ResourceVote(vote.resource_vote._0))

    fun toNative(): RustVote {
        return RustVote(resourceVote.toNative())
    }
}