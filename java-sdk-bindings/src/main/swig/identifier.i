%rename(Identifier) platform_value_types_identifier_Identifier;
%extend platform_value_types_identifier_Identifier {
    platform_value_types_identifier_Identifier(uint8_t (*byteArray)[32]) {
        platform_value_types_identifier_IdentifierBytes32 * identifierBytes32 = platform_value_types_identifier_IdentifierBytes32_ctor(byteArray);
        return platform_value_types_identifier_Identifier_ctor(identifierBytes32);
    }

    ~platform_value_types_identifier_Identifier() {
        printf("~Identifier(%lx)\n", $self);
        //memoryFactory.destroyItem($self->_0->_0); //crash
        platform_value_types_identifier_Identifier_destroy($self);
    }
}

%rename (IdentifierBytes32) platform_value_types_identifier_IdentifierBytes32;
%extend platform_value_types_identifier_IdentifierBytes32 {
    platform_value_types_identifier_IdentifierBytes32(uint8_t (*identifierBytes)[32]) {
        return platform_value_types_identifier_IdentifierBytes32_ctor(identifierBytes);
    }

    ~platform_value_types_identifier_IdentifierBytes32() {
        printf("~IdentityBytes32(%lx)\n", (uint64_t)$self);
        // memoryFactory.destroyItem($self->_0); // crash
        platform_value_types_identifier_IdentifierBytes32_destroy($self);
    }
};