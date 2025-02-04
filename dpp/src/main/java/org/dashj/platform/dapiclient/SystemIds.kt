/**
 * Copyright (c) 2022-present, Dash Core Team
 *
 * This source code is licensed under the MIT license found in the
 * COPYING file in the root directory of this source tree.
 */

package org.dashj.platform.dapiclient

import org.bitcoinj.core.Sha256Hash
import org.dashj.platform.dpp.identifier.Identifier

object SystemIds {
    val dpnsOwnerId = Identifier.from(Sha256Hash.ZERO_HASH)
    val dpnsDataContractId = Identifier.from("GWRSAVFMjXx8HpQFaNJMqBV7MBgMK4br5UESsB4S31Ec")
    val dashpayOwnerId = Identifier.from(Sha256Hash.ZERO_HASH)
    val dashpayDataContractId = Identifier.from("Bwr4WHCPz5rFVAD87RqTs3izo4zpzwsEdKPWUT1NS1C7")
    val walletUtilsOwnerId = Identifier.from(Sha256Hash.ZERO_HASH)
    val walletUtilsDataContractId = Identifier.from("7CSFGeF4WNzgDmx94zwvHkYaG3Dx4XEe5LFsFgJswLbm")
}
