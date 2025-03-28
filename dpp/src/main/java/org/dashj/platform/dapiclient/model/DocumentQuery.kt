/**
 * Copyright (c) 2020-present, Dash Core Team
 *
 * This source code is licensed under the MIT license found in the
 * COPYING file in the root directory of this source tree.
 */
package org.dashj.platform.dapiclient.model

import com.google.common.base.Preconditions
import org.dashj.platform.dpp.BaseObject
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.util.convertToPlatformValue
import org.dashj.platform.sdk.OrderClause
import org.dashj.platform.sdk.WhereClause
import org.dashj.platform.sdk.WhereOperator
import org.dashj.platform.sdk.platform.Documents
import org.json.JSONArray

/**
 * These options are used by getDocument to filter results
 * @property where MutableList<MutableList<String>>?
 * @property orderBy MutableList<MutableList<String>>?
 * @property limit Int
 * @property startAt Int
 * @property startAfter Int
 */
class DocumentQuery private constructor(
    var where: List<Any>? = null,
    var orderBy: List<List<String>>? = null,
    var limit: Int = Documents.DOCUMENT_LIMIT,
    var startAt: Identifier? = null,
    var startAfter: Identifier? = null
) : BaseObject() {

    companion object {
        val emptyByteArray = ByteArray(0)
        const val ascending = "asc"
        const val descending = "desc"
        val operators = mapOf(
            "<" to WhereOperator.LessThan,
            "<=" to WhereOperator.LessThanOrEquals,
            "==" to WhereOperator.Equal,
            ">=" to WhereOperator.GreaterThanOrEquals,
            ">" to WhereOperator.GreaterThan,
            "in" to WhereOperator.In,
            "startsWith" to WhereOperator.StartsWith
        )
        const val WHERE_CLAUSE_SIZE = 3

        fun builder(): Builder { return Builder() }
    }

    data class Builder(
        private var where: MutableList<List<Any>>? = mutableListOf(),
        private var orderBy: MutableList<List<String>>? = null,
        private var limit: Int = -1,
        private var startAt: Identifier? = null,
        private var startAfter: Identifier? = null
    ) {

        fun where(where: List<Any>) = apply {
            Preconditions.checkArgument(
                where.size == WHERE_CLAUSE_SIZE,
                "A where clause must have three items"
            )
            Preconditions.checkArgument(
                operators.contains(where[1]),
                "Where clause operator must be one of : $operators"
            )
            if (this.where == null) {
                this.where = ArrayList()
            }
            this.where!!.add(where)
        }

        fun where(where: String): Builder {
            return where(JSONArray(where).toList())
        }

        fun where(left: String, operator: String, right: String): Builder {
            return where(listOf(left, operator, right))
        }

        fun where(left: String, operator: String, right: ByteArray): Builder {
            return where(listOf(left, operator, right))
        }

        fun where(left: String, operator: String, right: Identifier): Builder {
            return where(listOf(left, operator, right))
        }

        fun where(left: String, operator: String, right: Int): Builder {
            return where(listOf(left, operator, right))
        }

        fun where(left: String, operator: String, right: Long): Builder {
            return where(listOf(left, operator, right))
        }

        fun whereIn(left: String, right: List<Any>): Builder {
            return where(listOf(left, "in", right))
        }

        fun orderBy(orderBy: List<String>) = apply {
            if (this.orderBy == null) {
                this.orderBy = ArrayList()
            }
            this.orderBy!!.add(orderBy)
        }

        fun orderByJson(orderByJson: String): Builder {
            return orderBy(JSONArray(orderByJson).toList() as List<String>)
        }

        fun orderBy(index: String, orderByAscending: Boolean = true): Builder {
            return orderBy(listOf(index, if (orderByAscending) ascending else descending))
        }

        fun limit(limit: Int) = apply { this.limit = limit }
        fun startAt(startAt: Identifier?) = apply { this.startAt = startAt }
        fun startAfter(startAfter: Identifier?) = apply { this.startAfter = startAfter }

        fun build() = DocumentQuery(where, orderBy, limit, startAt, startAfter)
    }

    fun hasLimit(): Boolean {
        return limit != -1
    }

    fun hasStartAt(): Boolean {
        return startAt != null
    }

    fun hasStartAfter(): Boolean {
        return startAfter != null
    }

    fun encodeWhere(): List<WhereClause> {
        val whereClauses = where
        return whereClauses?.map { clause ->
            when (clause) {
                is List<*> -> {
                    val field = clause[0] as String
                    val operator = operators[clause[1]]
                    val value = clause[2]
                    val valueObject = convertToPlatformValue(value)
                    WhereClause(field, operator, valueObject)
                }

                else -> throw IllegalStateException()
            }
        }
            ?: listOf()
    }



    fun encodeOrderBy(): List<OrderClause> {
        val orderClauses = orderBy;
        return orderClauses?.map {
            val field = it[0]
            val sortOrder = when (it[1]) {
                "asc" -> true
                else -> false
            }
            OrderClause(field, sortOrder)
        }
            ?: listOf()
    }

    override fun toObject(): Map<String, Any?> {
        val json = hashMapOf<String, Any?>()
        if (where != null) {
            json["where"] = where
        }
        if (orderBy != null) {
            json["orderBy"] = orderBy
        }
        if (limit != -1) {
            json["limit"] = limit
        }
        if (startAt != null) {
            json["startAt"] = startAt!!.toBuffer()
        }
        if (startAfter != null) {
            json["startAfter"] = startAfter!!.toBuffer()
        }
        return json
    }

    override fun toJSON(): Map<String, Any?> {
        val json = hashMapOf<String, Any?>()
        if (where != null) {
            json["where"] = where
        }
        if (orderBy != null) {
            json["orderBy"] = orderBy
        }
        if (limit != -1) {
            json["limit"] = limit
        }
        if (startAt != null) {
            json["startAt"] = startAt!!.toString()
        }
        if (startAfter != null) {
            json["startAfter"] = startAfter!!.toString()
        }
        return json
    }

    fun clone(): DocumentQuery {
        return DocumentQuery(where, orderBy, limit, startAt, startAfter)
    }

    override fun toString(): String {
        return toJSON().toString()
    }
}
