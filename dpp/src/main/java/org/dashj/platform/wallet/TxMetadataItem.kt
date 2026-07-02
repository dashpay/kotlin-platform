/**
 * Copyright (c) 2022-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

package org.dashj.platform.wallet

import com.google.common.primitives.Ints
import com.google.protobuf.ByteString
import org.bitcoinj.core.Sha256Hash
import org.dashj.platform.contracts.wallet.TxMetadataDocument
import org.dashj.platform.dpp.util.Cbor
import java.text.DecimalFormat

/**
 * Transaction metadata item
 *
 * @property txId The transaction id (hash) in big endian format
 * @property memo
 * @property exchangeRate
 * @property currencyCode
 * @property taxCategory
 * @property service
 * @property customIconUrl
 * @property giftCardNumber
 * @property giftCardPin
 * @property merchantName
 * @property originalPrice
 * @property barcodeValue
 * @property barcodeFormat
 * @property merchantUrl
 * @property otherData Additional key-value string data
 * @property order order number related to this tx
 * @property giftCardChallenge
 * @property index
 * @constructor Create empty Tx metadata item
 */

class TxMetadataItem(
    val txId: ByteArray,
    val timestamp: Long? = 0,
    val memo: String? = null,
    val exchangeRate: Double? = null,
    val currencyCode: String? = null,
    val taxCategory: String? = null,
    val service: String? = null,
    val customIconUrl: String? = null,
    val giftCardNumber: String? = null,
    val giftCardPin: String? = null,
    val merchantName: String? = null,
    val originalPrice: Double? = null,
    val barcodeValue: String? = null,
    val barcodeFormat: String? = null,
    val merchantUrl: String? = null,
    val otherData: Map<String, String>? = null,
    val order: String? = null,
    val giftCardChallenge: String? = null,
    val index: Int? = null
) {
    val data = hashMapOf<String, Any?>()

    constructor(rawObject: Map<String, Any?>) : this(
        rawObject["txId"] as ByteArray,
        rawObject["timestamp"] as? Long,
        rawObject["memo"] as? String,
        rawObject["exchangeRate"] as? Double,
        rawObject["currencyCode"] as? String,
        rawObject["taxCategory"] as? String,
        rawObject["service"] as? String,
        rawObject["customIconUrl"] as? String,

        // Gift Cards
        rawObject["giftCardNumber"] as? String,
        rawObject["giftCardPin"] as? String,
        rawObject["merchantName"] as? String,
        rawObject["originalPrice"] as? Double,
        rawObject["barcodeValue"] as? String,
        rawObject["barcodeFormat"] as? String,
        rawObject["merchantUrl"] as? String,
        rawObject["otherData"] as? Map<String, String>,
        rawObject["order"] as? String,
        rawObject["giftCardChallenge"] as? String,
        index = rawObject["index"] as? Int
    ) {
        data.putAll(rawObject)
    }

    constructor(protoTxMetadata: WalletUtils.TxMetadataItem) : this(
        protoTxMetadata.txId.toByteArray(),
        if (protoTxMetadata.hasTimestamp()) protoTxMetadata.timestamp else null,
        if (protoTxMetadata.hasMemo()) protoTxMetadata.memo else null,
        if (protoTxMetadata.hasExchangeRate()) protoTxMetadata.exchangeRate else null,
        if (protoTxMetadata.hasCurrencyCode()) protoTxMetadata.currencyCode else null,
        if (protoTxMetadata.hasTaxCategory()) protoTxMetadata.taxCategory else null,
        if (protoTxMetadata.hasService()) protoTxMetadata.service else null,
        if (protoTxMetadata.hasCustomIconUrl()) protoTxMetadata.customIconUrl else null,
        if (protoTxMetadata.hasGiftCardNumber()) protoTxMetadata.giftCardNumber else null,
        if (protoTxMetadata.hasGiftCardPin()) protoTxMetadata.giftCardPin else null,
        if (protoTxMetadata.hasMerchantName()) protoTxMetadata.merchantName else null,
        if (protoTxMetadata.hasOriginalPrice()) protoTxMetadata.originalPrice else null,
        if (protoTxMetadata.hasBarcodeValue()) protoTxMetadata.barcodeValue else null,
        if (protoTxMetadata.hasBarcodeFormat()) protoTxMetadata.barcodeFormat else null,
        if (protoTxMetadata.hasMerchantUrl()) protoTxMetadata.merchantUrl else null,
        protoTxMetadata.otherDataMap.ifEmpty { null },
        if (protoTxMetadata.hasOrder()) protoTxMetadata.order else null,
        if (protoTxMetadata.hasGiftCardChallenge()) protoTxMetadata.giftCardChallenge else null,
        if (protoTxMetadata.hasIndex()) protoTxMetadata.index else null
    )

    fun toObject(): Map<String, Any?> {
        val map = hashMapOf<String, Any?>(
            "txId" to txId,
        )
        timestamp?.let {
            map["timestamp"] = it
        }

        memo?.let {
            map["memo"] = it
        }

        exchangeRate?.let {
            map["exchangeRate"] = it
        }

        currencyCode?.let {
            map["currencyCode"] = it
        }

        taxCategory?.let {
            map["taxCategory"] = it
        }

        service?.let {
            map["service"] = it
        }

        customIconUrl?.let {
            map["customIconUrl"] = it
        }

        giftCardNumber?.let {
            map["giftCardNumber"] = it
        }

        giftCardPin?.let {
            map["giftCardPin"] = it
        }

        merchantName?.let {
            map["merchantName"] = it
        }

        originalPrice?.let {
            map["originalPrice"] = it
        }

        barcodeValue?.let {
            map["barcodeValue"] = it
        }

        barcodeFormat?.let {
            map["barcodeFormat"] = it
        }

        merchantUrl?.let {
            map["merchantUrl"] = it
        }

        otherData?.let {
            map["otherData"] = it
        }

        order?.let {
            map["order"] = it
        }

        giftCardChallenge?.let {
            map["giftCardChallenge"] = it
        }

        index?.let {
            map["index"] = it
        }

        return map
    }

    fun toJson(): Map<String, Any?> {
        val map = hashMapOf<String, Any?>(
            "txId" to Sha256Hash.wrap(txId).toString()
        )
        timestamp?.let {
            map["timestamp"] = it.toString()
        }

        memo?.let {
            map["memo"] = it
        }

        exchangeRate?.let {
            val format = DecimalFormat.getCurrencyInstance()
            map["exchangeRate"] = format.format(it)
        }

        currencyCode?.let {
            map["currencyCode"] = it
        }

        taxCategory?.let {
            map["taxCategory"] = it
        }

        service?.let {
            map["service"] = it
        }

        customIconUrl?.let {
            map["customIconUrl"] = it
        }

        giftCardNumber?.let {
            map["giftCardNumber"] = it
        }

        giftCardPin?.let {
            map["giftCardPin"] = it
        }

        merchantName?.let {
            map["merchantName"] = it
        }

        originalPrice?.let {
            val format = DecimalFormat.getCurrencyInstance()
            map["originalPrice"] = format.format(it)
        }

        barcodeValue?.let {
            map["barcodeValue"] = it
        }

        barcodeFormat?.let {
            map["barcodeFormat"] = it
        }

        merchantUrl?.let {
            map["merchantUrl"] = it
        }

        otherData?.let {
            map.putAll(it)
        }

        order?.let {
            map["order"] = it
        }

        giftCardChallenge?.let {
            map["giftCardChallenge"] = it
        }

        index?.let {
            map["index"] = it
        }

        return map
    }

    fun getSize(version: Int): Int {
        return if (version == TxMetadataDocument.VERSION_CBOR) {
            Cbor.encode(toObject()).size
        } else if (version == TxMetadataDocument.VERSION_PROTOBUF) {
            toProtobuf().serializedSize
        } else {
            error("unknown version $version")
        }
    }

    // does not compare timestamp
    override fun equals(other: Any?): Boolean {
        if (this === other) {
            return true
        } else if (other is TxMetadataItem) {
            return txId.contentEquals(other.txId) &&
                memo == other.memo &&
                exchangeRate == other.exchangeRate &&
                currencyCode == other.currencyCode &&
                taxCategory == other.taxCategory &&
                service == other.service &&
                customIconUrl == other.customIconUrl &&
                giftCardNumber == other.giftCardNumber &&
                giftCardPin == other.giftCardPin &&
                merchantName == other.merchantName &&
                originalPrice == other.originalPrice &&
                barcodeValue == other.barcodeValue &&
                barcodeFormat == other.barcodeFormat &&
                merchantUrl == other.merchantUrl &&
                otherData == other.otherData &&
                order == other.order &&
                giftCardChallenge == other.giftCardChallenge &&
                index == other.index
        }
        return false
    }

    override fun hashCode(): Int {
        return Ints.fromBytes(
            txId[3],
            txId[2],
            txId[1],
            txId[0]
        )
    }

    fun toProtobuf(): WalletUtils.TxMetadataItem {
        val builder = WalletUtils.TxMetadataItem.newBuilder().apply {
            setTxId(ByteString.copyFrom(this@TxMetadataItem.txId))
        }
        timestamp?.let { builder.timestamp = it }
        memo?.let { builder.memo = it }
        exchangeRate?.let { builder.exchangeRate = it }
        currencyCode?.let { builder.currencyCode = it }
        taxCategory?.let { builder.taxCategory = it }
        service?.let { builder.service = it }
        customIconUrl?.let { builder.customIconUrl = it }
        giftCardNumber?.let { builder.giftCardNumber = it }
        giftCardPin?.let { builder.giftCardPin = it }
        merchantName?.let { builder.merchantName = it }
        originalPrice?.let { builder.originalPrice = it }
        barcodeValue?.let { builder.barcodeValue = it }
        barcodeFormat?.let { builder.barcodeFormat = it }
        merchantUrl?.let { builder.merchantUrl = it }
        otherData?.let { builder.putAllOtherData(it) }
        order?.let { builder.order = it }
        giftCardChallenge?.let { builder.giftCardChallenge = it }
        index?.let { builder.index = it }
        return builder.build()
    }

    override fun toString(): String {
        return "TxMetadataItem${toJson()}"
    }

    fun isNotEmpty(): Boolean {
        return (timestamp != null && timestamp != 0L) || taxCategory != null || memo != null ||
            currencyCode != null || exchangeRate != null || service != null || customIconUrl != null ||
            giftCardNumber != null || giftCardPin != null || merchantName != null || originalPrice != null ||
            barcodeValue != null || barcodeFormat != null || merchantUrl != null || order != null ||
            giftCardChallenge != null || index != null || (otherData != null && otherData.isNotEmpty())
    }
}
