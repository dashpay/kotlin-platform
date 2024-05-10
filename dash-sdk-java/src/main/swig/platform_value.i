#define CTOR(RTYPE, CTYPE) \
    platform_value_Value(CTYPE value) { \
        return platform_value_Value_##RTYPE##_ctor(value); \
    }
#define CTOR_CLONE(RTYPE, CTYPE, CLONE_FN) \
    platform_value_Value(CTYPE value) { \
        CTYPE clone = CLONE_FN(value); \
        return platform_value_Value_##RTYPE##_ctor(clone); \
    }
//DEFINE_CLASS(PlatformValue, platform_value_Value)
%rename (PlatformValue) platform_value_Value;
%extend platform_value_Value {
//     platform_value_Value(char * text) {
//         return platform_value_Value_Text_ctor(text);
//     }
    CTOR_CLONE(Text, char *, memoryFactory.clone)
    CTOR(U128, u128)
    CTOR(I128, i128)
    CTOR(U64, uint64_t)
    CTOR(I64, int64_t)
    CTOR(U32, uint32_t) // ignored by SWIG
    CTOR(I32, int32_t)
    CTOR(U16, uint16_t)
    CTOR(I16, int16_t)
    CTOR(U8, uint8_t)
    CTOR(I8, int8_t)
    CTOR(Bytes, Vec_u8*)
    CTOR(Bytes20, Arr_u8_20*)
    CTOR(Bytes32, Arr_u8_32*)
    CTOR(Bytes36, Arr_u8_36*)
    // CTOR(EnumU8, Vec_u8) // the problem here that Bytes uses Vec_u8
    CTOR(EnumString, Vec_String*)
    CTOR(Identifier, platform_value_Hash256*)
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
}
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