%ignore dpp_identity_identity_public_key_KeyCount::_0;
%rename(KeyCount) dpp_identity_identity_public_key_KeyCount;
%extend dpp_identity_identity_public_key_KeyCount {
    dpp_identity_identity_public_key_KeyCount(int id) {
        return dpp_identity_identity_public_key_KeyCount_ctor(id);
    }
    ~dpp_identity_identity_public_key_KeyCount() {
        dpp_identity_identity_public_key_KeyCount_destroy($self);
    }
    int toInt() {
        return $self->_0;
    }
    bool objectEquals(dpp_identity_identity_public_key_KeyCount* other) {
        if ($self == other) return true;
        if ($self == nullptr || other == nullptr) return false;
        return $self->_0 == other->_0;
    }
}