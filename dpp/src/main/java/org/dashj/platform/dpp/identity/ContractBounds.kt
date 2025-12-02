package org.dashj.platform.dpp.identity

import org.dashj.platform.dpp.identifier.Identifier

typealias RustContractBounds = org.dashj.platform.sdk.ContractBounds

abstract class ContractBounds(val type: String, val identifier: Identifier)
{
    companion object {
        fun from(rustContractBounds: RustContractBounds): ContractBounds {
            return when (rustContractBounds.tag) {
                org.dashj.platform.sdk.ContractBounds.Tag.SingleContract -> SingleContractBounds(Identifier(rustContractBounds.single_contract.id))
                org.dashj.platform.sdk.ContractBounds.Tag.SingleContractDocumentType -> SingleContractDocumentType(
                    Identifier(rustContractBounds.single_contract_document_type.id),
                    rustContractBounds.single_contract_document_type.document_type_name
                )
            }
        }

        fun tryFrom(rustContractBounds: RustContractBounds?): ContractBounds? {
            return if (rustContractBounds == null) {
                null
            } else {
                from(rustContractBounds)
            }
        }
    }

    open fun toJSON(): Map<String, Any> {
        return hashMapOf(
            "type" to type,
            "identifier" to identifier.toString()
        )
    }

    open fun toObject(): Map<String, Any> {
        return hashMapOf(
            "type" to type,
            "identifier" to identifier
        )
    }
}

class SingleContractBounds(identifier: Identifier) : ContractBounds("singleContract", identifier)


class SingleContractDocumentType(identifier: Identifier, val documentType: String) : ContractBounds("documentType", identifier) {
    override fun toJSON(): Map<String, Any> {
        val json = super.toJSON().toMutableMap()
        json["documentType"] = documentType
        return json
    }

    override fun toObject(): Map<String, Any> {
        val map = super.toObject().toMutableMap()
        map["documentType"] = documentType
        return map
    }
}