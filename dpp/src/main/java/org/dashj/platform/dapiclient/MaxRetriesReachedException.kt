/**
 * Copyright (c) 2020-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
package org.dashj.platform.dapiclient

class MaxRetriesReachedException(exceptions: List<Exception>) : Exception(exceptions.first()) {
    constructor(e: Exception) : this(listOf(e))
}


