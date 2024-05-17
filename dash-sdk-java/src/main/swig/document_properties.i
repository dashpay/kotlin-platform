%typemap(javaclassname) std_collections_Map_keys_String_values_platform_value_Value* "java.util.Map<String, PlatformValue>"
%typemap(javatype) std_collections_Map_keys_String_values_platform_value_Value* "java.util.Map<String, PlatformValue>"
%typemap(jtype) std_collections_Map_keys_String_values_platform_value_Value* "java.util.Map<String, PlatformValue>"
%typemap(jstype) std_collections_Map_keys_String_values_platform_value_Value* "java.util.Map<String, PlatformValue>"
%typemap(jni) std_collections_Map_keys_String_values_platform_value_Value* "jobject"

%typemap(out) std_collections_Map_keys_String_values_platform_value_Value* {
    $result = fermented_tree_to_java_map_String_Value(jenv, $1);
}

%typemap(in) std_collections_Map_keys_String_values_platform_value_Value* {
    $1 = java_map_String_Value_to_fermented_ValueMap(jenv, $input);
}

%typemap(freearg) struct std_collections_Map_keys_String_values_platform_value_Value* {
    free($1->_0);
}

%typemap(javain) std_collections_Map_keys_String_values_platform_value_Value * "$javainput"

%typemap(javaout) std_collections_Map_keys_String_values_platform_value_Value * {
    return $jnicall;
}


%typemap(throws) std_collections_Map_keys_String_values_platform_value_Value *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null std_collections_Map_keys_String_values_platform_value_Value");
   return $null; %}

%apply struct std_collections_Map_keys_String_values_platform_value_Value {struct std_collections_Map_keys_String_values_platform_value_Value};
%ignore std_collections_Map_keys_String_values_platform_value_Value;