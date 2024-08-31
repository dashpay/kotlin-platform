// STRUCT_TYPE is the structure name that follows this pattern:
// struct Vec_u8 {
//     uintptr_t count;
//     uint8_t *values;
// };
// There must be a count field and a values field
//
// SIZE is the required size of the byte array.  -1 is any size
%define DEFINE_BYTE_ARRAY(STRUCT_TYPE, SIZE)

struct STRUCT_TYPE;
%ignore STRUCT_TYPE;

%typemap(javaclassname) STRUCT_TYPE* "byte[]"
%typemap(jni) STRUCT_TYPE * "jbyteArray"
%typemap(jtype) STRUCT_TYPE * "byte[]"
%typemap(jstype) STRUCT_TYPE * "byte[]"

%typemap(in) STRUCT_TYPE *
%{
    int size_$1 = (jenv)->GetArrayLength($input);
    if (SIZE != -1 && size_$1 != SIZE) {
        SWIG_JavaThrowException(jenv, SWIG_JavaIllegalArgumentException, "Invalid number of bytes for " #STRUCT_TYPE);
        return $null;
    }
    uint8_t * _buffer_$1 = (uint8_t*)(jenv)->GetByteArrayElements($input, 0);
    uint8_t * byteArray_$1 = (uint8_t *)memoryFactory.alloc(size_$1);
    memcpy(byteArray_$1, _buffer_$1, size_$1);
    $1 = STRUCT_TYPE##_ctor(size_$1, byteArray_$1);
%}

%typemap(freearg) STRUCT_TYPE *
%{
    STRUCT_TYPE##_destroy($1);
%}

%typemap(argout) STRUCT_TYPE *
%{
     jenv->ReleaseByteArrayElements($input, (jbyte *) _buffer_$1, 0);
%}

%typemap(out) STRUCT_TYPE * {
    if (!$1) {
      SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Vec_u8* null array ");
      return $null;
    }
    if (!$1->values) {
      SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Vec_u8.values null array");
      return $null;
    }
    $result = JCALL1(NewByteArray, jenv, $1->count);
    JCALL4(SetByteArrayRegion, jenv, $result, 0, $1->count, (jbyte *) $1->values);
}

%typemap(javain) STRUCT_TYPE * "$javainput"

%typemap(javaout) STRUCT_TYPE * {
    return $jnicall;
  }

%typemap(typecheck) STRUCT_TYPE * = char *;

%typemap(throws) STRUCT_TYPE *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "null " #STRUCT_TYPE);
   return $null; %}

%enddef