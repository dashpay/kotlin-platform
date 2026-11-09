package org.dashj.platform.dpp.contract

import org.dashj.platform.dpp.identifier.Identifier
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

class VerifiedContractGroupTest {
    private val id = Identifier.from(ByteArray(32) { 1 }).toString()
    private val admin = Identifier.from(ByteArray(32) { 2 }).toString()
    private fun json(admins: String = "\"$admin\"", tokens: String = "{\"contractId\":\"$id\",\"tokenPosition\":3}") = """
        {"id":"$id","ownerId":"$id","adminIds":[$admins],"name":null,"description":null,
        "contracts":["$id"],"documentTypes":[{"contractId":"$id","documentType":"profile"}],"tokens":[$tokens]}
    """.trimIndent().toByteArray()

    @Test fun `all member types and admins survive decoding`() {
        val group = VerifiedContractGroup.decode(json())!!
        assertEquals(setOf(Identifier.from(admin)), group.adminIds)
        assertEquals(setOf(Identifier.from(id)), group.contracts)
        assertEquals(setOf(ContractGroupDocumentMember(Identifier.from(id), "profile")), group.documentTypes)
        assertEquals(setOf(ContractGroupTokenMember(Identifier.from(id), 3)), group.tokens)
        assertNull(VerifiedContractGroup.decode("null".toByteArray()))
    }

    @Test fun `semantic equality ignores order but not changed scope`() {
        val a = VerifiedContractGroup.decode(json("\"$id\",\"$admin\""))!!
        val b = VerifiedContractGroup.decode(json("\"$admin\",\"$id\""))!!
        assertEquals(a, b)
        assertNotEquals(a, a.copy(adminIds = emptySet()))
        assertNotEquals(a, a.copy(tokens = emptySet()))
        assertNotEquals(a, a.copy(documentTypes = emptySet()))
        assertNotEquals(a, a.copy(contracts = emptySet()))
    }

    @Test fun `incomplete duplicate and malformed scope fails closed`() {
        for (bytes in listOf("{}".toByteArray(), json("\"$admin\",\"$admin\""),
            json(tokens = "{\"contractId\":\"$id\",\"tokenPosition\":65536}"))) {
            assertThrows(Exception::class.java) { VerifiedContractGroup.decode(bytes) }
        }
    }
}
