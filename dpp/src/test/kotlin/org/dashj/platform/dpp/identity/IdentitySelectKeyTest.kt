/**
 * Copyright (c) 2020-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

package org.dashj.platform.dpp.identity

import org.dashj.platform.dapiclient.SystemIds
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.sdk.KeyType
import org.dashj.platform.sdk.Purpose
import org.dashj.platform.sdk.SecurityLevel
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Test

/**
 * Verifies contract-bounds-aware key selection used when picking the recipient's key for a
 * DashPay ContactRequest: a key whose [IdentityPublicKey.contractBounds] is scoped to the
 * requested contract must be preferred over an otherwise-identical key with no/other bounds.
 */
class IdentitySelectKeyTest {

    companion object {
        init {
            // KeyType/Purpose/SecurityLevel are SWIG enums whose constants call into JNI.
            System.loadLibrary("sdklib")
        }
    }

    private val dashpayContractId = SystemIds.dashpayDataContractId
    private val otherContractId = Identifier(ByteArray(32) { (it + 7).toByte() })

    private fun encryptionKey(id: Int, contractBounds: ContractBounds?, disabledAt: Long? = null) =
        IdentityPublicKey(
            id,
            KeyType.ECDSA_SECP256K1,
            Purpose.ENCRYPTION,
            SecurityLevel.MEDIUM,
            contractBounds,
            ByteArray(33) { (id + it).toByte() },
            false,
            disabledAt
        )

    private fun identityWith(vararg keys: IdentityPublicKey) =
        Identity(Identifier(ByteArray(32) { it.toByte() }), keys.toList(), 0L, 1, 1)

    @Test
    fun prefersKeyBoundToRequestedContract() {
        val unbound = encryptionKey(0, null)
        val dashpayBound = encryptionKey(1, SingleContractDocumentType(dashpayContractId, "contactRequest"))
        val otherBound = encryptionKey(2, SingleContractBounds(otherContractId))
        val identity = identityWith(unbound, dashpayBound, otherBound)

        val selected = identity.getFirstPublicKey(
            Purpose.ENCRYPTION,
            SecurityLevel.MEDIUM,
            KeyType.ECDSA_SECP256K1,
            dashpayContractId
        )

        assertEquals(1, selected?.id, "should pick the key bound to the DashPay contract")
    }

    @Test
    fun matchesSingleContractBoundsByIdentifier() {
        val identity = identityWith(
            encryptionKey(0, null),
            encryptionKey(1, SingleContractBounds(otherContractId))
        )

        val selected = identity.getFirstPublicKey(
            Purpose.ENCRYPTION,
            SecurityLevel.MEDIUM,
            KeyType.ECDSA_SECP256K1,
            otherContractId
        )

        assertEquals(1, selected?.id)
    }

    @Test
    fun returnsNullWhenNoKeyIsBoundToTheContract() {
        // A key with no bounds (or bounds for a different contract) must not match a contract query.
        val identity = identityWith(
            encryptionKey(0, null),
            encryptionKey(1, SingleContractBounds(otherContractId))
        )

        val selected = identity.getFirstPublicKey(
            Purpose.ENCRYPTION,
            SecurityLevel.MEDIUM,
            KeyType.ECDSA_SECP256K1,
            dashpayContractId
        )

        assertNull(selected)
        // ...while the bounds-agnostic overload still returns the first matching encryption key.
        assertEquals(
            0,
            identity.getFirstPublicKey(Purpose.ENCRYPTION, SecurityLevel.MEDIUM, KeyType.ECDSA_SECP256K1)?.id
        )
    }

    @Test
    fun skipsDisabledKeysBoundToTheContract() {
        val identity = identityWith(
            encryptionKey(0, SingleContractDocumentType(dashpayContractId, "contactRequest"), disabledAt = 1L),
            encryptionKey(1, SingleContractDocumentType(dashpayContractId, "contactRequest"))
        )

        val selected = identity.getFirstPublicKey(
            Purpose.ENCRYPTION,
            SecurityLevel.MEDIUM,
            KeyType.ECDSA_SECP256K1,
            dashpayContractId
        )

        assertEquals(1, selected?.id, "disabled keys must be skipped")
    }
}