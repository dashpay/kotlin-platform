%ignore dpp_prelude_Revision::_0;
%rename(Revision) dpp_prelude_Revision;
%extend dpp_prelude_Revision {
    //
    dpp_prelude_Revision() {
        return dpp_prelude_Revision_ctor(0);
    }
    dpp_prelude_Revision(long long timestamp) {
        return dpp_prelude_Revision_ctor(timestamp);
    }
    ~dpp_prelude_Revision() {
        dpp_prelude_Revision_destroy($self);
    }

    long long toLong() {
        return $self->_0;
    }

    bool objectEquals(dpp_prelude_Revision* other) {
        if ($self == other) return true;
        if ($self == nullptr || other == nullptr) return false;
        return $self->_0 == other->_0;
    }
}