%ignore dpp_identity_identity_public_key_TimestampMillis::_0;
%rename(TimestampMillis) dpp_identity_identity_public_key_TimestampMillis;
// TODO: apply this to all types or specific types
// %typemap(javacode) SWIGTYPE %{
//   public boolean equals(Object obj) {
//     boolean equal = false;
//     if (obj instanceof $javaclassname) {
//       equal = ((($javaclassname)obj).swigCPtr == this.swigCPtr) || objectEquals(($javaclassname)obj);
//     }
//     return equal;
//   }
//   public int hashCode() {
//     return (int)swigCPtr;
//   }
// %}

%extend dpp_identity_identity_public_key_TimestampMillis {
    dpp_identity_identity_public_key_TimestampMillis() {
        return dpp_identity_identity_public_key_TimestampMillis_ctor(time(NULL) * 1000);
    }
    dpp_identity_identity_public_key_TimestampMillis(long long timestamp) {
        return dpp_identity_identity_public_key_TimestampMillis_ctor(timestamp);
    }
    ~dpp_identity_identity_public_key_TimestampMillis() {
        dpp_identity_identity_public_key_TimestampMillis_destroy($self);
    }

    long long toLong() {
        return $self ? $self->_0 : -1;
    }

//     bool isNull() {
//         return (uint64_t)$self < 10;
//     }

    bool objectEquals(dpp_identity_identity_public_key_TimestampMillis* other) {
        if ($self == other) return true;
        if ($self == nullptr || other == nullptr) return false;
        return $self->_0 == other->_0;
    }
}