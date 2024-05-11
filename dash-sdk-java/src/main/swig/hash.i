
%ignore platform_value_Hash256::_0;

%rename (Hash256) platform_value_Hash256;
%extend platform_value_Hash256 {
    platform_value_Hash256(uint8_t (*byteArray)[32]) {
        return platform_value_Hash256_ctor(Arr_u8_32_ctor(32, (uint8_t*)byteArray));
    }

    ~platform_value_Hash256() {
        platform_value_Hash256_destroy($self);
    }
    bool objectEquals(platform_value_Hash256* other) {
        if ($self == other) return true;
        if ($self == nullptr || other == nullptr) return false;
        return $self->_0->count == other->_0->count && memcmp($self->_0->values, other->_0->values, $self->_0->count) == 0;
    }

    Arr_u8_32 * get_0() {
        return $self->_0;
    }
};