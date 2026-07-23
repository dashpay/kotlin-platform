/**
 * Copyright (c) 2018-present, Dash Core Team
 *
 * This source code is licensed under the MIT license found in the
 * COPYING file in the root directory of this source tree.
 */

package org.dashj.platform.dpp.statetransition

import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identity.IdentityPublicKey
import org.dashj.platform.sdk.dashsdk

/**
 * A summary of a state transition, produced by natively deserializing its bincode-serialized
 * bytes. Unlike [StateTransitionFactory] (CBOR-only, and lacking IdentityUpdate support) this
 * works for any modern transition. The identity-update fields ([revision], [identityNonce],
 * [addPublicKeys]) are populated only when [type] is IdentityUpdate (5).
 */
data class StateTransitionInfo(
    /** The transition type discriminant (e.g. 5 = IdentityUpdate). */
    val type: Int,
    /** Human-readable transition name. */
    val name: String,
    /** The owner/identity id, or null when the transition has no owner. */
    val ownerId: Identifier?,
    /** Identity-update: the new identity revision (0 otherwise). */
    val revision: Long,
    /** Identity-update: the identity nonce (0 otherwise). */
    val identityNonce: Long,
    /** Identity-update: the public keys being added (empty otherwise). */
    val addPublicKeys: List<IdentityPublicKey>
)

/** Native (bincode) deserialization of platform state transitions. */
object NativeStateTransition {

    /**
     * Deserializes a bincode-serialized state transition and returns a [StateTransitionInfo],
     * including the public keys added when the transition is an identity update.
     *
     * @throws Exception if the bytes are not a valid state transition (surfaced from the SDK).
     */
    @JvmStatic
    fun deserialize(bytes: ByteArray): StateTransitionInfo {
        val result = dashsdk.platformMobileStateTransitionDeserializeStateTransition(bytes)
        val info = result.ok
            ?: throw IllegalArgumentException("could not deserialize state transition: ${result.error}")

        val type: Int
        val name: String
        val ownerId: Identifier?
        val revision: Long
        val identityNonce: Long
        try {
            type = dashsdk.platformMobileStateTransitionStateTransitionInfoGetTransitionType(info).toInt()
            name = dashsdk.platformMobileStateTransitionStateTransitionInfoGetName(info)
            val ownerBytes = dashsdk.platformMobileStateTransitionStateTransitionInfoGetOwnerId(info)
            ownerId = if (ownerBytes != null && ownerBytes.isNotEmpty()) Identifier.from(ownerBytes) else null
            revision = dashsdk.platformMobileStateTransitionStateTransitionInfoGetRevision(info).toLong()
            identityNonce = dashsdk.platformMobileStateTransitionStateTransitionInfoGetIdentityNonce(info).toLong()
        } finally {
            dashsdk.platformMobileStateTransitionStateTransitionInfoDestroy(info)
        }

        val addPublicKeys = dashsdk.platformMobileStateTransitionIdentityUpdatePublicKeysToAdd(bytes)
            .unwrap()
            .map { IdentityPublicKey.from(it) }

        return StateTransitionInfo(type, name, ownerId, revision, identityNonce, addPublicKeys)
    }
}
