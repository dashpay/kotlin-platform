// %naturalvar Vec_u8;

struct Vec_u8;

//%feature("valuewrapper") struct Vec_u8 *;

// Vec_u8
%typemap(javaclassname) Vec_u8* "byte[]"
%typemap(jni) Vec_u8 * "jbyteArray"
%typemap(jtype) Vec_u8 * "byte[]"
%typemap(jstype) Vec_u8 * "byte[]"

%typemap(in) Vec_u8 *
%{
     uint8_t * _buffer_$1 = (uint8_t*)(jenv)->GetByteArrayElements($input, 0);
     int size_$1 = (jenv)->GetArrayLength($input);
     uint8_t * byteArray_$1 = (uint8_t *)memoryFactory.alloc(size_$1);
     memcpy(byteArray_$1, _buffer_$1, size_$1);
     printf("typemap(in) Vec_u8 *: %ld, [%lx]%d\n", size_$1, (long)_buffer_$1, _buffer_$1[0]);
     $1 = Vec_u8_ctor(size_$1, byteArray_$1);
     //$1 = Vec_u8_ctor((uintptr_t)byteArray_$1, (uint8_t*)size_$1); // problem with order of parameters
     printf("typemap(in) Vec_u8 *: %lx\n", $1);
     printf("typemap(in) Vec_u8 *: count: %ld\n", $1->count);
     printf("typemap(in) Vec_u8 *: count: %ld, values: [%lx]\n", $1->count, (long)$1->values);
     printf("typemap(in) Vec_u8 *: count: %ld, values: [%lx]%d\n", $1->count, (long)$1->values, $1->values[0]);
%}

%typemap(argout) Vec_u8 *
%{
     printf("typemap(argout) Vec_u8 *: %ld, [%lx]%d\n", $1->count, (long)$1->values, $1->values[0]);
     //JCALL3(ReleaseByteArrayElements, jenv, $input, (jbyte *) _buffer_$1, 0);
     jenv->ReleaseByteArrayElements($input, (jbyte *) _buffer_$1, 0);
     printf("typemap(argout) Vec_u8 *: %ld, [%lx]%d\n", $1->count, (long)$1->values, $1->values[0]);
%}

%typemap(out) Vec_u8 * {
    printf("typemap(out) Vec_u8* %lx\n", (long)$1);
    if (!$1) {
      SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Vec_u8* null array ");
      return $null;
    }
    if (!$1->values) {
      SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Vec_u8.values null array");
      return $null;
    }
    printf("  (count: %ld, values: [%lx], %d)\n", $1->count, (long)$1->values, $1->values[0]);
    $result = JCALL1(NewByteArray, jenv, $1->count);
    JCALL4(SetByteArrayRegion, jenv, $result, 0, $1->count, (jbyte *) $1->values);
}

%typemap(javain) Vec_u8 * "$javainput"

%typemap(javaout) Vec_u8 * {
    return $jnicall;
  }

%typemap(typecheck) Vec_u8 * = char *;

%typemap(throws) Vec_u8 *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null Vec_u8");
   return $null; %}