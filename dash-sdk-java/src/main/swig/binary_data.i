%rename (BinaryData) platform_value_types_binary_data_BinaryData;
%extend platform_value_types_binary_data_BinaryData {
    platform_value_types_binary_data_BinaryData(Vec_u8 *o_0) {
        //printf("BinaryData(%lx)\n", (uint64_t)o_0);
        return platform_value_types_binary_data_BinaryData_ctor(clone(o_0));
    }
    ~platform_value_types_binary_data_BinaryData() {
        //printf("~BinaryData(%lx)\n", (uint64_t)$self);
        //printf("~BinaryData->_0(%lx)\n", (uint64_t)$self->_0);

        uint8_t * ptr = $self->_0->values;
        // memoryFactory.destroyItem($self->_0->values); // causes BinaryData_destroy crash
        //printf("~BinaryData->_0->values(%lx), [0] = %d\n", (uint64_t)$self->_0->values, (int)ptr[0]);

        //printf("~BinaryData->_0(%lx)\n", (uint64_t)$self->_0);
        platform_value_types_binary_data_BinaryData_destroy($self);
        //printf("~BinaryData complete(%lx)\n", (uint64_t)$self);
        //printf("~BinaryData->_0->values[0] = %d\n", (int)ptr[0]);
    }
}