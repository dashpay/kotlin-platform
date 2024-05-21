%rename(Identifier) platform_value_types_identifier_Identifier;
//%ignore platform_value_types_identifier_Identifier::_0;
//%ignore platform_value_types_identifier_IdentifierBytes::_0;

%extend platform_value_types_identifier_Identifier {
    platform_value_types_identifier_Identifier(uint8_t (*byteArray)[32]) {
        platform_value_types_identifier_IdentifierBytes32 * identifierBytes32 = platform_value_types_identifier_IdentifierBytes32_ctor(Arr_u8_32_ctor(32, (uint8_t*)byteArray));
        return platform_value_types_identifier_Identifier_ctor(identifierBytes32);
    }

    ~platform_value_types_identifier_Identifier() {
        printf("~Identifier(%lx)\n", $self);
        platform_value_types_identifier_Identifier_destroy($self);
    }

    uint8_t (*get_0())[32] {
        return (uint8_t (*)[32])$self->_0->_0->values; // Return the modified array
    }

    uint8_t (*getBytes())[32] {
        if ($self == nullptr || $self->_0 || $self->_0->_0) {
            throw std::invalid_argument("Identifier or IdentifierBytes32 is null");
        }
        return (uint8_t (*)[32])$self->_0->_0->values; // Return the modified array
    }

    bool objectEquals(platform_value_types_identifier_Identifier* other) {
        if ($self == other) return true;
        if ($self == nullptr || other == nullptr) return false;
        return $self->_0->_0->count == other->_0->_0->count && memcmp($self->_0->_0->values, other->_0->_0->values, $self->_0->_0->count) == 0;
    }
}

%rename (IdentifierBytes32) platform_value_types_identifier_IdentifierBytes32;
%extend platform_value_types_identifier_IdentifierBytes32 {
    platform_value_types_identifier_IdentifierBytes32(uint8_t (*byteArray)[32]) {
        return platform_value_types_identifier_IdentifierBytes32_ctor(Arr_u8_32_ctor(32, (uint8_t*)byteArray));
    }

    ~platform_value_types_identifier_IdentifierBytes32() {
        printf("~IdentityBytes32(%lx)\n", (uint64_t)$self);
        platform_value_types_identifier_IdentifierBytes32_destroy($self);
    }
    bool objectEquals(platform_value_types_identifier_IdentifierBytes32* other) {
        if ($self == other) return true;
        if ($self == nullptr || other == nullptr) return false;
        return $self->_0->count == other->_0->count && memcmp($self->_0->values, other->_0->values, $self->_0->count) == 0;
    }
};