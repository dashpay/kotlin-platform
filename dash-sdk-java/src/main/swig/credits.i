%ignore dpp_balances_credits_Credits::_0;
%rename(Credits) dpp_balances_credits_Credits;
%newobject dpp_balances_credits_Credits::fromBigInteger;
%extend dpp_balances_credits_Credits {
    dpp_balances_credits_Credits(long long value) {
        return dpp_balances_credits_Credits_ctor(value);
    }
    ~dpp_balances_credits_Credits() {
        dpp_balances_credits_Credits_destroy($self);
    }
    long long toLong() {
        return $self->_0;
    }
    static dpp_balances_credits_Credits * fromBigInteger(uint64_t value) {
        return dpp_balances_credits_Credits_ctor(value);
    }
    uint64_t toBigInteger() {
        return $self->_0;
    }
    bool objectEquals(dpp_balances_credits_Credits* other) {
        if ($self == other) return true;
        if ($self == nullptr || other == nullptr) return false;
        return $self->_0 == other->_0;
    }
}
