package org.dashj.platform.dashpay

enum class UsernameStatus(val value: Int) {
    NOT_PRESENT(0),
    INITIAL(1),
    PREORDER_REGISTRATION_PENDING(2),
    PREORDERED(3),
    REGISTRATION_PENDING(4),
    CONFIRMED(5),
    TAKEN_ON_NETWORK(6),
    VOTING_PERIOD(7),
    LOCKED(8);

    companion object {
        private val values = values()
        fun getByCode(code: Int): UsernameStatus {
            return values.filter { it.value == code }[0]
        }
    }
}