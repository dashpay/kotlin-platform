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