/**
 * Copyright (c) 2018-present, Dash Core Team
 *
 * This source code is licensed under the MIT license found in the
 * COPYING file in the root directory of this source tree.
 */

package org.dashj.platform.dpp.statetransition

import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.sdk.dashsdk

/**
 * A lightweight summary of a state transition, produced by natively deserializing its
 * bincode-serialized bytes. Unlike [StateTransitionFactory] (which understands only the legacy
 * CBOR envelope and a subset of transition types), this works for any modern transition — at the
 * cost of exposing only the common fields. Extend as callers need more.
 */
data class StateTransitionInfo(
    /** The transition type discriminant (e.g. 5 = IdentityUpdate). */
    val type: Int,
    /** Human-readable transition name. */
    val name: String,
    /** The owner/identity id, or null when the transition has no owner. */
    val ownerId: Identifier?
)

/** Native (bincode) deserialization of platform state transitions. */
object NativeStateTransition {

    /**
     * Deserializes a bincode-serialized state transition and returns a [StateTransitionInfo].
     *
     * @throws Exception if the bytes are not a valid state transition (surfaced from the SDK).
     */
    @JvmStatic
    fun deserialize(bytes: ByteArray): StateTransitionInfo {
        val result = dashsdk.platformMobileStateTransitionDeserializeStateTransition(bytes)
        val info = result.ok
            ?: throw IllegalArgumentException("could not deserialize state transition: ${result.error}")
        try {
            val ownerId = dashsdk.platformMobileStateTransitionStateTransitionInfoGetOwnerId(info)
            return StateTransitionInfo(
                type = dashsdk.platformMobileStateTransitionStateTransitionInfoGetTransitionType(info).toInt(),
                name = dashsdk.platformMobileStateTransitionStateTransitionInfoGetName(info),
                ownerId = if (ownerId != null && ownerId.isNotEmpty()) Identifier.from(ownerId) else null
            )
        } finally {
            dashsdk.platformMobileStateTransitionStateTransitionInfoDestroy(info)
        }
    }
}
