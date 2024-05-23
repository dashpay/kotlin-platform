#define CTOR(RTYPE, CTYPE) \
    platform_value_Value(CTYPE value) { \
        return platform_value_Value_##RTYPE##_ctor(value); \
    }
#define CTOR_CLONE(RTYPE, CTYPE, CLONE_FN) \
    platform_value_Value(CTYPE value) { \
        CTYPE copy = CLONE_FN(value); \
        return platform_value_Value_##RTYPE##_ctor(copy); \
    }

#define CTOR_SIZE_CLONE(RTYPE, CTYPE, CLONE_FN) \
    platform_value_Value(CTYPE value, int size) { \
        CTYPE clone = CLONE_FN(value); \
        return platform_value_Value_##RTYPE##_ctor(clone); \
    }

#define VALUE_GET(RTYPE, CTYPE, cvalue) \
CTYPE get##RTYPE() { \
    if ($self->tag != platform_value_Value::Tag::RTYPE) { \
        throw std::invalid_argument("Value is not " #RTYPE); \
    } \
    return $self->cvalue._0; \
}

//DEFINE_CLASS(PlatformValue, platform_value_Value)
%rename (PlatformValue) platform_value_Value;
%extend platform_value_Value {
//     platform_value_Value(char * text) {
//         return platform_value_Value_Text_ctor(text);
//     }
    CTOR_CLONE(Text, char *, memoryFactory.clone)
    CTOR(U128, u128)
    CTOR(I128, __int128)
    //CTOR(U64, uint64_t)
    CTOR(I64, int64_t)
    //CTOR(U32, uint32_t) // ignored by SWIG
    CTOR(I32, int32_t)
    //CTOR(U16, uint16_t)
    CTOR(I16, int16_t)
    //CTOR(U8, uint8_t)
    CTOR(I8, int8_t)
    platform_value_Value(Vec_u8 * value, bool slice) {
        if (!slice) {
            return platform_value_Value_Bytes_ctor(value);
        }
        switch (value->count) {
            case 20: {
                Arr_u8_20 * bytes20 = Arr_u8_20_ctor(value->count, value->values);
                return platform_value_Value_Bytes20_ctor(bytes20);
            }
            case 32: {
                Arr_u8_32 * bytes32 = Arr_u8_32_ctor(value->count, value->values);
                return platform_value_Value_Bytes32_ctor(bytes32);
            }
            case 36: {
                Arr_u8_36 * bytes36 = Arr_u8_36_ctor(value->count, value->values);
                return platform_value_Value_Bytes36_ctor(bytes36);
            }
            default: {
                //Vec_u8 * bytes = Vec_u8_ctor(size, value);
                return platform_value_Value_Bytes_ctor(value);
            }
        }
    }
    // CTOR(EnumU8, Vec_u8) // the problem here that Bytes uses Vec_u8
    platform_value_Value(Vec_String * value, bool slice) {
        return platform_value_Value_EnumString_ctor(value);
    }
    //CTOR(EnumString, Vec_String*)
    CTOR_CLONE(Identifier, platform_value_Hash256*, clone)
    CTOR(Float, double)
    CTOR(Bool, bool)
    platform_value_Value() {
        return platform_value_Value_Null_ctor();
    }
    CTOR(Array, Vec_platform_value_Value*)
    CTOR(Map, platform_value_value_map_ValueMap*)
    ~platform_value_Value() {
        platform_value_Value_destroy($self);
    }
    bool objectEquals(platform_value_Value * other) {
        if ($self == other) return true;
        if ($self == nullptr || other == nullptr) return false;
        return platform_mobile_operators_Value_eq($self, other);
    }

    int objectHashCode() {
        return platform_mobile_operators_Value_hash($self);
    }

    VALUE_GET(Text, char *, text)

    VALUE_GET(Bool, bool, bool_)
    VALUE_GET(I8, int8_t, i8)
    VALUE_GET(I16, int16_t, i16)
    VALUE_GET(I32, int32_t, i32)
    VALUE_GET(I64, int64_t, i64)
    VALUE_GET(I128, __int128, i128)

    VALUE_GET(U8, uint8_t, u8)
    VALUE_GET(U16, uint16_t, u16)
    VALUE_GET(U32, uint32_t, u32)
    VALUE_GET(U64, int64_t, u64)
    VALUE_GET(U128, uint128_t, u128)

    VALUE_GET(Float, double, float_)

    VALUE_GET(Bytes, Vec_u8*, bytes)
    VALUE_GET(Bytes20, Arr_u8_20*, bytes20)
    VALUE_GET(Bytes32, Arr_u8_32*, bytes32)
    VALUE_GET(Bytes36, Arr_u8_36*, bytes36)

    VALUE_GET(EnumString, Vec_String *, enum_string)
    VALUE_GET(Identifier, platform_value_Hash256*, identifier)

    VALUE_GET(Array, Vec_platform_value_Value*, array)
    VALUE_GET(Map, platform_value_value_map_ValueMap*, map)
}

%define VALUE_IGNORE(FIELD, STRUCT)
%ignore platform_value_Value::FIELD;
%ignore platform_value_Value::STRUCT##_Body;
%enddef



VALUE_IGNORE(text, Text);
VALUE_IGNORE(bool_, Bool);
VALUE_IGNORE(i8, I8);
VALUE_IGNORE(i16, I16);
VALUE_IGNORE(i32, I32);
VALUE_IGNORE(i64, I64);
VALUE_IGNORE(i128, I128);
VALUE_IGNORE(u8, U8);
VALUE_IGNORE(u16, U16);
VALUE_IGNORE(u32, U32);
VALUE_IGNORE(u64, U64);
VALUE_IGNORE(u128, U128);
VALUE_IGNORE(float_, Float);

VALUE_IGNORE(bytes, Bytes);
VALUE_IGNORE(bytes20, Bytes20);
VALUE_IGNORE(bytes32, Bytes32);
VALUE_IGNORE(bytes36, Bytes36);

VALUE_IGNORE(enum_u8, EnumU8);
VALUE_IGNORE(identifier, Identifier);
VALUE_IGNORE(enum_string, EnumString);

VALUE_IGNORE(array, Array);
VALUE_IGNORE(map, Map);


START_CLASS(PlatformValueMap, platform_value_value_map_ValueMap)
    platform_value_value_map_ValueMap(Vec_Tuple_platform_value_Value_platform_value_Value * valueMap) {
        return platform_value_value_map_ValueMap_ctor(valueMap);
    }
    platform_value_value_map_ValueMap(platform_value_value_map_ValueMap * valueMap) {
        return platform_mobile_clone_ValueMap_clone(valueMap);
    }
END_CLASS()

START_CLASS(PlatformError, platform_value_error_Error)
END_CLASS()

// for the ValueMap Vec Tuple
// this doesn't need to be a macro unless there are other Vec<(a, a)>
%typemap(javaclassname) Vec_Tuple_platform_value_Value_platform_value_Value* "java.util.Map<PlatformValue, PlatformValue>"
%typemap(javatype) Vec_Tuple_platform_value_Value_platform_value_Value* "java.util.Map<PlatformValue, PlatformValue>"
%typemap(jtype) Vec_Tuple_platform_value_Value_platform_value_Value* "java.util.Map<PlatformValue, PlatformValue>"
%typemap(jstype) Vec_Tuple_platform_value_Value_platform_value_Value* "java.util.Map<PlatformValue, PlatformValue>"
%typemap(jni) Vec_Tuple_platform_value_Value_platform_value_Value* "jobject"

%typemap(out) Vec_Tuple_platform_value_Value_platform_value_Value* {
    $result = fermented_tree_to_java_map_Value_Value(jenv, $1);
}

%typemap(in) Vec_Tuple_platform_value_Value_platform_value_Value* {
    $1 = java_map_Value_Value_to_fermented_ValueMap(jenv, $input);
}

%typemap(freearg) struct Vec_Tuple_platform_value_Value_platform_value_Value* {
    free($1->_0);
}

%typemap(javain) Vec_Tuple_platform_value_Value_platform_value_Value * "$javainput"

%typemap(javaout) Vec_Tuple_platform_value_Value_platform_value_Value * {
    return $jnicall;
  }


%typemap(throws) Vec_Tuple_platform_value_Value_platform_value_Value *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null Vec_Tuple_platform_value_Value_platform_value_Value");
   return $null; %}

%apply struct Vec_Tuple_platform_value_Value_platform_value_Value {struct Vec_Tuple_platform_value_Value_platform_value_Value};
%ignore Vec_Tuple_platform_value_Value_platform_value_Value;