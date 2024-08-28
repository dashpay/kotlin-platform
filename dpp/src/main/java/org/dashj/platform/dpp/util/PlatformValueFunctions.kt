package org.dashj.platform.dpp.util

import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identifier.RustIdentifier
import org.dashj.platform.dpp.toBase58
import org.dashj.platform.dpp.toBase64
import org.dashj.platform.dpp.toBase64Padded
import org.dashj.platform.dpp.toHex
import org.dashj.platform.sdk.Hash256
import org.dashj.platform.sdk.PlatformValue
import org.dashj.platform.sdk.PlatformValueMap
import java.math.BigInteger

open class ByteArrayFixedLength(val bytes: ByteArray, maxLength: Int) {
    init {
        require(bytes.size == maxLength)
    }
    val size = maxLength
    fun toBase64(): String = bytes.toBase64()
    fun toBase64Padded(): String = bytes.toBase64Padded()
    fun ToBase58(): String = bytes.toBase58()
    fun toHex(): String = bytes.toHex()
}

class ByteArray20(bytes: ByteArray): ByteArrayFixedLength(bytes, 20)
class ByteArray32(bytes: ByteArray): ByteArrayFixedLength(bytes, 32)
class ByteArray36(bytes: ByteArray): ByteArrayFixedLength(bytes, 36)

class EnumBytes(val bytes: ByteArray) {
    val size = bytes.size
}

class EnumString(strings: List<String>) : List<String> {
    private val strings = arrayListOf<String>()
    init {
        this.strings.addAll(strings)
    }

    override val size: Int
        get() = strings.size

    override fun get(index: Int): String {
        return strings[index]
    }

    override fun isEmpty(): Boolean {
        return strings.isEmpty()
    }

    override fun iterator(): Iterator<String> {
        return strings.iterator()
    }

    override fun listIterator(): ListIterator<String> {
        return strings.listIterator()
    }

    override fun listIterator(index: Int): ListIterator<String> {
        return strings.listIterator(index)
    }

    override fun subList(fromIndex: Int, toIndex: Int): List<String> {
        return strings.subList(fromIndex, toIndex)
    }

    override fun lastIndexOf(element: String): Int {
        return strings.lastIndexOf(element)
    }

    override fun indexOf(element: String): Int {
        return strings.indexOf(element)
    }

    override fun containsAll(elements: Collection<String>): Boolean {
        TODO("Not yet implemented")
    }

    override fun contains(element: String): Boolean {
        TODO("Not yet implemented")
    }
}

fun convertPlatformValue(value: PlatformValue): Any? {
    return when (value.tag) {
        PlatformValue.Tag.U128 -> value.u128
        PlatformValue.Tag.I128 -> value.i128
        PlatformValue.Tag.U64 -> value.u64
        PlatformValue.Tag.I64 -> value.i64
        PlatformValue.Tag.U32 -> value.u32
        PlatformValue.Tag.I32 -> value.i32
        PlatformValue.Tag.U16 -> value.u16
        PlatformValue.Tag.I16 -> value.i16
        PlatformValue.Tag.U8 -> value.u8
        PlatformValue.Tag.I8 -> value.i8
        PlatformValue.Tag.Bytes -> value.bytes
        PlatformValue.Tag.Bytes20 -> ByteArray20(value.bytes20)
        PlatformValue.Tag.Bytes32 -> ByteArray32(value.bytes32)
        PlatformValue.Tag.Bytes36 -> ByteArray36(value.bytes36)
        PlatformValue.Tag.EnumU8 -> EnumBytes(value.enumU8)
        PlatformValue.Tag.EnumString -> EnumString(value.enumString)
        PlatformValue.Tag.Identifier -> Identifier(value.identifier.bytes)
        PlatformValue.Tag.Float -> value.float
        PlatformValue.Tag.Text -> value.text
        PlatformValue.Tag.Bool -> value.bool
        PlatformValue.Tag.Null -> null
        PlatformValue.Tag.Array -> convertPlatformValueArray(value)
        PlatformValue.Tag.Map -> convertPlatformValueMap(value)
        else -> error("Unsupported Value ${value.tag}")
    }
}

fun convertPlatformValueArray(array: PlatformValue): List<Any?> {
    require(array.tag == PlatformValue.Tag.Array)
    return array.array.map { convertPlatformValue(it) }
}

fun convertPlatformValueMap(map: PlatformValue): Map<String, Any?> {
    require(map.tag == PlatformValue.Tag.Map)
    val hashMap = hashMapOf<String, Any?>()
    map.map._0.forEach { (key, value) ->
        require(key.tag == PlatformValue.Tag.Text)
        hashMap[key.text] = convertPlatformValue(value)
    }
    return hashMap
}

fun convertListToPlatformValue(list: List<Any>): PlatformValue {
    return PlatformValue(
        list.map { item ->
            convertToPlatformValue(item)
        }
    )
}

fun convertMapToPlatformValue(map: Map<String, Any?>): PlatformValue {
    val result = hashMapOf<PlatformValue, PlatformValue>()
    map.forEach { (key, value) ->
        result[PlatformValue(key)] = convertToPlatformValue(value)
    }
    return PlatformValue(PlatformValueMap(result))
}

fun convertToPlatformValue(value: Any?) : PlatformValue {
    return when(value) {
        is Boolean -> PlatformValue(value)
        is String -> PlatformValue(value)
        is Byte -> PlatformValue(value)
        is Short -> PlatformValue(value)
        is Int -> PlatformValue(value)
        is Long -> PlatformValue(value)
        is BigInteger -> PlatformValue(value)
        is ByteArray -> PlatformValue(value, true)
        is ByteArray20 -> PlatformValue(value.bytes, true)
        is ByteArray32 -> PlatformValue(value.bytes, true)
        is ByteArray36 -> PlatformValue(value.bytes, true)
        is EnumBytes -> PlatformValue(value.bytes, PlatformValue.Tag.EnumU8)
        is Identifier -> PlatformValue(Hash256(value.toBuffer()))
        is RustIdentifier -> PlatformValue(Hash256(value._0._0))
        is EnumString -> convertEnumStringToPlatformValue(value)
        is List<*> -> convertListToPlatformValue(value as List<Any>)
        is Map<*, *> -> convertMapToPlatformValue(value as Map<String, Any?>)
        else -> if (value == null) PlatformValue() else error("no conversion for $value")
    }
}

fun convertEnumStringToPlatformValue(value: EnumString): PlatformValue {
    return PlatformValue(value, true)
}

fun convertToPlatformProperties(map: Map<String, Any?>) : Map<String, PlatformValue> {
    val result = hashMapOf<String, PlatformValue>()
    map.forEach { (key, value) ->
        result[key] = convertToPlatformValue(value)
    }
    return result
}

fun convertProperties(properties: Map<String, PlatformValue>): Map<String, Any?> {
    val result = hashMapOf<String, Any?>()

    properties.forEach { (key, value) ->
        result[key] = convertPlatformValue(value)
    }
    return result
}