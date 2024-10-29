package org.dashj.platform.dpp.voting

typealias RustVotePollsGroupedByTimestamp = org.dashj.platform.sdk.VotePollsGroupedByTimeStamp

class VotePollsGroupedByTimestamp(val list: List<Pair<Long, List<VotePoll>>>) {
    constructor(rustObject: RustVotePollsGroupedByTimestamp) : this(
        rustObject._0.map { item -> Pair(item.o_0.toLong(), item.o_1.map { VotePoll.from(it) }) }
    )
}