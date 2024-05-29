%rename (BinaryData) platform_value_types_binary_data_BinaryData;
%extend platform_value_types_binary_data_BinaryData {
    platform_value_types_binary_data_BinaryData(Vec_u8 *o_0) {
        return platform_value_types_binary_data_BinaryData_ctor(clone(o_0));
    }
    ~platform_value_types_binary_data_BinaryData() {
        platform_value_types_binary_data_BinaryData_destroy($self);
    }
}