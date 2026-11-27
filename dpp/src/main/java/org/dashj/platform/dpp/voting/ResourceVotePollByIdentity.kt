package org.dashj.platform.dpp.voting

import org.dashj.platform.dpp.identifier.Identifier

typealias RustResourceVotesByIdentity = org.dashj.platform.sdk.ResourceVotesByIdentity


class ResourceVotesByIdentity(
    val list: Map<Identifier, ResourceVote>,
    val voteCounts: Map<Identifier, Int>
) {
    constructor(rustObject: RustResourceVotesByIdentity) : this(
        rustObject._0.mapKeys { Identifier.from(it.key.bytes) }
            .mapValues { ResourceVote(it.value.resource_vote) },
        rustObject._0.mapKeys { Identifier.from(it.key.bytes) }
            .mapValues { it.value.vote_count }
    )
}
