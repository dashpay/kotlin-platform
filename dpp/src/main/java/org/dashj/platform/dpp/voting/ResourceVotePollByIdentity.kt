package org.dashj.platform.dpp.voting

import org.dashj.platform.dpp.identifier.Identifier

typealias RustResourceVotesByIdentity = org.dashj.platform.sdk.ResourceVotesByIdentity


class ResourceVotesByIdentity(val list: Map<Identifier, ResourceVote>) {
    constructor(rustObject: RustResourceVotesByIdentity) : this(
        rustObject._0.map { entry ->
            Pair(Identifier.from(entry.key.bytes), ResourceVote(entry.value))
        }.associateBy({it.first},{it.second})
    )
}