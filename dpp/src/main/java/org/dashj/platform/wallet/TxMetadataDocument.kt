/**
 * Copyright (c) 2022-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

package org.dashj.platform.contracts.wallet

import org.bitcoinj.crypto.ChildNumber
import org.bitcoinj.crypto.EncryptedData
import org.bitcoinj.crypto.KeyCrypterAESCBC
import org.bouncycastle.crypto.params.KeyParameter
import org.dashj.platform.dpp.document.Document
import org.dashj.platform.dpp.util.Cbor
import org.dashj.platform.sdk.platform.AbstractDocument
import org.dashj.platform.wallet.TxMetadataItem
import org.dashj.platform.wallet.WalletUtils.TxMetadataBatch

class TxMetadataDocument(document: Document) : AbstractDocument(document) {

    companion object {
        // 2^15 + 1
        val childNumber = ChildNumber((2 shl 14) + 1, true)
        const val MAX_ENCRYPTED_SIZE = 4096 - 32 // leave room for a partially filled block and the IV

        const val VERSION_UNKNOWN = -1
        const val VERSION_CBOR = 0
        const val VERSION_PROTOBUF = 1
    }

    val keyIndex: Int
        get() = (document.data["keyIndex"] as Long).toInt()
    val encryptionKeyIndex: Int
        get() = (document.data["encryptionKeyIndex"] as Long).toInt()
    val encryptedMetadata: ByteArray
        get() = getFieldByteArray("encryptedMetadata")!!
    var txMetadataVersion = VERSION_UNKNOWN
        private set

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as TxMetadataDocument

        return document.equals(other.document)
    }

    override fun hashCode(): Int {
        return document.id.hashCode()
    }

    override fun toString(): String {
        return "TxMetadata(${document.toJSON()})"
    }

    fun decrypt(keyParameter: KeyParameter): List<TxMetadataItem> {
        val cipher = KeyCrypterAESCBC()
        // use AES-CBC-256 to obtain byte data
        val iv = encryptedMetadata.copyOfRange(0, 16)
        val encryptedData = encryptedMetadata.copyOfRange(16, encryptedMetadata.size)
        val decryptedData = cipher.decrypt(EncryptedData(iv, encryptedData), keyParameter)
        val version = decryptedData.copyOfRange(0, 1)[0].toInt() and 0xFF
        return when (version) {
            VERSION_CBOR -> {
                val list = Cbor.decodeList(decryptedData.copyOfRange(1, decryptedData.size))
                this.txMetadataVersion = VERSION_CBOR
                // use .map to convert to List<TxMetadataItem>
                list.map { TxMetadataItem(it as Map<String, Any?>) }
            }
            VERSION_PROTOBUF -> {
                val batch = TxMetadataBatch.parser().parseFrom(decryptedData, 1, decryptedData.size - 1)
                txMetadataVersion = VERSION_PROTOBUF
                batch.itemsList.map { TxMetadataItem(it) }
            }
            else -> error("unknown txmetadata version $version")
        }
    }
}
