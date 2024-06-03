package org.dashj.platform.dpp.errors.concensus.signature

import org.dashj.platform.sdk.SecurityLevel

class InvalidSignaturePublicKeySecurityLevelException(
    val publicKeySecurityLevel: SecurityLevel,
    val keySecurityLevelRequirement: SecurityLevel
) : SignatureException(
    "Invalid public key security level $publicKeySecurityLevel. " +
        "This state transition requires $keySecurityLevelRequirement."
) {
    constructor(arguments: List<Any>) : this(
        SecurityLevel.swigToEnum(arguments[0] as Int),
        SecurityLevel.swigToEnum(arguments[1] as Int)
    ) {
        setArguments(arguments)
    }
}
