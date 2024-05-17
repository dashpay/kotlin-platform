// %naturalvar Arr_u8_32;

struct Arr_u8_32;
//%feature("valuewrapper") struct Arr_u8_32 *;

// Arr_u8_32
%typemap(javaclassname) Arr_u8_32 * "byte[]"
%typemap(jni) Arr_u8_32 * "jbyteArray"
%typemap(jtype) Arr_u8_32 * "byte[]"
%typemap(jstype) Arr_u8_32 * "byte[]"

%typemap(in) Arr_u8_32 *
%{
     uint8_t * _buffer_$1 = (uint8_t*)(jenv)->GetByteArrayElements($input, 0);
     int size_$1 = (jenv)->GetArrayLength($input);
     uint8_t * byteArray_$1 = (uint8_t *)memoryFactory.alloc(size_$1);
     memcpy(byteArray_$1, _buffer_$1, size_$1);
     printf("typemap(in) Arr_u8_32 *: %ld, [%lx]%d\n", size_$1, (long)_buffer_$1, _buffer_$1[0]);
     $1 = Arr_u8_32_ctor(size_$1, byteArray_$1);
     printf("typemap(in) Arr_u8_32 *: %lx\n", $1);
     printf("typemap(in) Arr_u8_32 *: count: %ld\n", $1->count);
     printf("typemap(in) Arr_u8_32 *: count: %ld, values: [%lx]\n", $1->count, (long)$1->values);
     printf("typemap(in) Arr_u8_32 *: count: %ld, values: [%lx]%d\n", $1->count, (long)$1->values, $1->values[0]);
%}

%typemap(argout) Arr_u8_32 *
%{
     printf("typemap(argout) Arr_u8_32 *: %ld, [%lx]%d\n", $1->count, (long)$1->values, $1->values[0]);
     //JCALL3(ReleaseByteArrayElements, jenv, $input, (jbyte *) _buffer_$1, 0);
     jenv->ReleaseByteArrayElements($input, (jbyte *) _buffer_$1, 0);
     printf("typemap(argout) Arr_u8_32 *: %ld, [%lx]%d\n", $1->count, (long)$1->values, $1->values[0]);
%}

%typemap(out) Arr_u8_32 * {
    printf("typemap(out) Arr_u8_32 * %lx\n", (long)$1);
    if (!$1) {
      SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Arr_u8_32 * null array");
      return $null;
    }
    if (!$1->values) {
      SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "Arr_u8_32.values null array");
      return $null;
    }
    printf("  (count: %ld, values: [%lx], %d)\n", $1->count, (long)$1->values, $1->values[0]);
    $result = JCALL1(NewByteArray, jenv, $1->count);
    JCALL4(SetByteArrayRegion, jenv, $result, 0, $1->count, (jbyte *) $1->values);
}

%typemap(javain) Arr_u8_32 * "$javainput"

%typemap(javaout) Arr_u8_32 * {
    return $jnicall;
  }

%typemap(typecheck) Arr_u8_32 * = char *;

%typemap(throws) Arr_u8_32 *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null Arr_u8_32");
   return $null; %}