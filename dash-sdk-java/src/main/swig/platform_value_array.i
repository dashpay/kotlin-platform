LIST_STRUCT_TYPEMAP(Vec_platform_value_Value, platform_value_Value, PlatformValue, clone);


// %typemap(javaclassname) Vec_platform_value_Value* "java.util.List<PlatformValue>"
// %typemap(javatype) Vec_platform_value_Value* "java.util.List<PlatformValue>"
// %typemap(jtype) Vec_platform_value_Value* "java.util.List<PlatformValue>"
// %typemap(jstype) Vec_platform_value_Value* "java.util.List<PlatformValue>"
// %typemap(jni) Vec_platform_value_Value* "jobject"
//
// %typemap(out) Vec_platform_value_Value* {
//     $result = fermented_vec_to_java_list_Value(jenv, $1);
// }
//
// %typemap(in) Vec_platform_value_Value* {
//     $1 = java_list_Value_to_fermented_vec_Value(jenv, $input);
// }
//
// %typemap(freearg) struct Vec_platform_value_Value* {
//     //free($1->_0);
// }
//
// %typemap(javain) Vec_platform_value_Value * "$javainput"
//
// %typemap(javaout) Vec_platform_value_Value * {
//     return $jnicall;
//   }
//
//
// %typemap(throws) Vec_platform_value_Value *
// %{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null Vec_platform_value_Value");
//    return $null; %}
//
// %apply struct Vec_platform_value_Value {struct Vec_platform_value_Value};
// %ignore Vec_platform_value_Value;