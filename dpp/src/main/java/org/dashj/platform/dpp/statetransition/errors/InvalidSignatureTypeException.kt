/**
 * Copyright (c) 2020-present, Dash Core Team
 *
 * This source code is licensed under the MIT license found in the
 * COPYING file in the root directory of this source tree.
 */
package org.dashj.platform.dpp.statetransition.errors

import org.dashj.platform.dpp.errors.DPPException
import org.dashj.platform.dpp.identity.IdentityPublicKey
import org.dashj.platform.sdk.KeyType

class InvalidSignatureTypeException(val signatureType: KeyType) :
    DPPException("Invalid Signature Type: $signatureType")
