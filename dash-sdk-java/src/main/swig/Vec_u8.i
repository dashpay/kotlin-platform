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
     $1 = Vec_u8_ctor(size_$1, byteArray_$1);
%}

%typemap(freearg) Vec_u8 *
%{
    // Vec_u8 typemap(freearg)
    Vec_u8_destroy($1);
%}

%typemap(argout) Vec_u8 *
%{
      jenv->ReleaseByteArrayElements($input, (jbyte *) _buffer_$1, 0);
%}

%typemap(out) Vec_u8 * {
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

%typemap(javain) Vec_u8 * "$javainput"

%typemap(javaout) Vec_u8 * {
    return $jnicall;
  }

%typemap(typecheck) Vec_u8 * = char *;

%typemap(throws) Vec_u8 *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null Vec_u8");
   return $null; %}