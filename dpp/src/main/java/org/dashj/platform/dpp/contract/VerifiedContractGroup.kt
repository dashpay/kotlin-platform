package org.dashj.platform.dpp.contract

import com.google.gson.JsonParser
import org.dashj.platform.dpp.identifier.Identifier

data class ContractGroupDocumentMember(val contractId: Identifier, val documentType: String)
data class ContractGroupTokenMember(val contractId: Identifier, val tokenPosition: Int)

/** A complete proof-verified group scope. Sets deliberately ignore transport ordering. */
data class VerifiedContractGroup(
    val id: Identifier,
    val ownerId: Identifier,
    val adminIds: Set<Identifier>,
    val name: String?,
    val description: String?,
    val contracts: Set<Identifier>,
    val documentTypes: Set<ContractGroupDocumentMember>,
    val tokens: Set<ContractGroupTokenMember>
) {
    companion object {
        internal fun decode(bytes: ByteArray): VerifiedContractGroup? {
            val value = JsonParser().parse(String(bytes, Charsets.UTF_8))
            if (value.isJsonNull) return null
            val json = value.asJsonObject
            fun id(field: String) = Identifier.from(json.get(field).asString)
            fun text(field: String) = json.get(field).let { if (it.isJsonNull) null else it.asString }
            val admins = json.getAsJsonArray("adminIds").map { Identifier.from(it.asString) }
            val contracts = json.getAsJsonArray("contracts").map { Identifier.from(it.asString) }
            val documents = json.getAsJsonArray("documentTypes").map { entry -> entry.asJsonObject.let {
                ContractGroupDocumentMember(Identifier.from(it.get("contractId").asString), it.get("documentType").asString)
            } }
            val tokens = json.getAsJsonArray("tokens").map { entry -> entry.asJsonObject.let {
                val position = it.get("tokenPosition").asInt
                require(position in 0..65535) { "Invalid token position" }
                ContractGroupTokenMember(Identifier.from(it.get("contractId").asString), position)
            } }
            require(admins.size == admins.toSet().size && contracts.size == contracts.toSet().size &&
                documents.size == documents.toSet().size && tokens.size == tokens.toSet().size) { "Duplicate group scope entries" }
            return VerifiedContractGroup(id("id"), id("ownerId"), admins.toSet(), text("name"), text("description"),
                contracts.toSet(), documents.toSet(), tokens.toSet())
        }
    }
}
