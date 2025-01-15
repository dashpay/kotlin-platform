/**
 * Copyright (c) 2020-present, Dash Core Team
 *
 * This source code is licensed under the MIT license found in the
 * COPYING file in the root directory of this source tree.
 */

package org.dashj.platform.dpp.identity

import com.google.common.base.Preconditions
import org.dashj.platform.dpp.BaseObject
import org.dashj.platform.dpp.Metadata
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.sdk.Identity
import org.dashj.platform.sdk.IdentityV0
import org.dashj.platform.sdk.KeyID
import org.dashj.platform.sdk.Purpose
import org.dashj.platform.sdk.Revision
import org.dashj.platform.sdk.SecurityLevel
import java.math.BigInteger
import kotlin.math.max

typealias RustIdentity = org.dashj.platform.sdk.Identity

class Identity : BaseObject {

    var id: Identifier
    var balance: Long = 0
    val publicKeys: MutableList<IdentityPublicKey>
    var revision: Int = 0

    var assetLockProof: AssetLockProof? = null
    var metadata: Metadata? = null

    constructor(rawIdentity: Map<String, Any?>) : super(rawIdentity["protocolVersion"] as Int) {
        id = Identifier.from(rawIdentity["id"])
        balance = rawIdentity["balance"].toString().toLong()
        publicKeys  = (rawIdentity["publicKeys"] as List<Any>).map { IdentityPublicKey(it as Map<String, Any?>) }.toMutableList()
    }

    constructor(id: Identifier, publicKeys: List<IdentityPublicKey>, balance: Long, revision: Int, protocolVersion: Int) : super(protocolVersion) {
        this.id = id
        this.balance = balance
        this.publicKeys = publicKeys.toMutableList()
        this.revision = revision
    }

    constructor(identity: RustIdentity) : super() {
        when (identity.tag) {
            Identity.Tag.V0 -> {
                val identity = identity.v0._0
                id = Identifier(identity.id)
                publicKeys = identity.publicKeys.values.map { IdentityPublicKey.from(it) }.toMutableList()
            }
        }
    }


    fun getPublicKeyById(keyId: Int): IdentityPublicKey? {
        Preconditions.checkArgument(keyId >= 0, "keyId ($keyId) must be >= 0")
        return publicKeys.find { it.id == keyId }
    }


    override fun toObject(): Map<String, Any> {
        return mapOf(
            "protocolVersion" to protocolVersion,
            "id" to id,
            "publicKeys" to publicKeys.map { it.toObject() },
            "balance" to balance,
            "revision" to revision
        )
    }

    override fun toJSON(): Map<String, Any> {
        return mapOf(
            "protocolVersion" to protocolVersion,
            "id" to id,
            "publicKeys" to publicKeys.map { it.toJSON() },
            "balance" to balance,
            "revision" to revision
        )
    }

    fun increaseBalance(amount: Long): Long {
        balance += amount
        return balance
    }

    fun reduceBalance(amount: Long): Long {
        balance -= amount
        return balance
    }

    /**
     * Get the biggest public key ID
     * @returns {number}
     */
    fun getPublicKeyMaxId(): Int {
        return publicKeys.fold(-1) { result, publicKey ->
            max(publicKey.id, result)
        }
    }

    fun getFirstPublicKey(securityLevel: SecurityLevel): IdentityPublicKey? {
        return try {
            publicKeys.first { it.securityLevel == securityLevel }
        } catch (e: NoSuchElementException) {
            null
        }
    }

    fun getFirstPublicKey(purpose: Purpose): IdentityPublicKey? {
        return try {
            publicKeys.first { it.purpose == purpose && it.disabledAt == null}
        } catch (e: NoSuchElementException) {
            null
        }
    }

    fun getFirstPublicKey(purpose: Purpose, securityLevel: SecurityLevel): IdentityPublicKey? {
        return try {
            publicKeys.first { it.purpose == purpose && it.securityLevel == securityLevel && it.disabledAt == null}
        } catch (e: NoSuchElementException) {
            null
        }
    }

    fun toNative(): RustIdentity {
        val identityV0 = IdentityV0(
            id.toNative(),
            publicKeys.associateBy({ KeyID(it.id) }, { it.toNative() }),
            BigInteger.ZERO,
            Revision(1)
        )
        return RustIdentity(identityV0)
    }
}
