/**
 * Copyright (c) 2024-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

package org.dashj.platform.wallet

import org.dashj.platform.dpp.document.Document
import org.dashj.platform.sdk.platform.AbstractDocument

class IdentityVerifyDocument(document: Document) : AbstractDocument(document) {

//    val label: String
//        get() = getFieldString("label")!!
    val normalizedLabel: String
        get() = getFieldString("normalizedLabel")!!
    val normalizedParentDomainName: String
        get() = getFieldString("normalizedParentDomainName")!!
//    val parentDomainName: String
//        get() = getFieldString("parentDomainName")!!
    val url: String
        get() = getFieldString("url")!!

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as IdentityVerifyDocument

        return document.equals(other.document)
    }

    override fun hashCode(): Int {
        return document.id.hashCode()
    }

    override fun toString(): String {
        return "IdentityVerifyDocument(${document.toJSON()})"
    }
}
