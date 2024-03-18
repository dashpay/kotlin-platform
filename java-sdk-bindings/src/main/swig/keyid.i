%ignore dpp_identity_identity_public_key_KeyID::_0;
%rename(KeyID) dpp_identity_identity_public_key_KeyID;
%extend dpp_identity_identity_public_key_KeyID {
    dpp_identity_identity_public_key_KeyID(int id) {
        return dpp_identity_identity_public_key_KeyID_ctor(id);
    }
    ~dpp_identity_identity_public_key_KeyID() {
        dpp_identity_identity_public_key_KeyID_destroy($self);
    }
    int toInt() {
        return $self->_0;
    }
    bool objectEquals(dpp_identity_identity_public_key_KeyID* other) {
        if ($self == other) return true;
        if ($self == nullptr || other == nullptr) return false;
        return $self->_0 == other->_0;
    }
}