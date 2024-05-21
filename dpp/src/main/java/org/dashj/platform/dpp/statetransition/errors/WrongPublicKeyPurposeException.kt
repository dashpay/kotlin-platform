/**
 * Copyright (c) 2021-present, Dash Core Team
 *
 * This source code is licensed under the MIT license found in the
 * COPYING file in the root directory of this source tree.
 */

package org.dashj.platform.dpp.statetransition.errors

import org.dashj.platform.dpp.errors.DPPException
import org.dashj.platform.dpp.identity.IdentityPublicKey
import org.dashj.platform.sdk.Purpose

class WrongPublicKeyPurposeException(
    val publicKeyPurpose: Purpose,
    val keyPurposeRequirement: Purpose
) : DPPException(
    "State transition must be signed with a key that has purpose $keyPurposeRequirement," +
        " but was $publicKeyPurpose"
)
