package org.dashj.platform.dashpay

data class UsernameInfo(
    var salt: ByteArray?,
    var usernameStatus: UsernameStatus,
    var username: String?,
    var requested: Boolean? = null,
    var votingStartedAt: Long? = null
) {
    override fun equals(other: Any?): Boolean {
        return if (other is UsernameInfo) {
            username == other.username
        } else {
            false
        }
    }

    override fun hashCode(): Int {
        return username.hashCode()
    }
}
