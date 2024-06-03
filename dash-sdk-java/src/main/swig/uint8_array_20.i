// keep this for now.  make a macro of it later where 32 is any type
// uint8_t [] to byte []
%typemap(jni) (uint8_t (*)[20]) "jbyteArray"
%typemap(jtype) (uint8_t (*)[20]) "byte[]"
%typemap(jstype) (uint8_t (*)[20]) "byte[]"
%typemap(in) (uint8_t (*)[20]) (uint8_t * byteArray) {
    if (!$input) {
      SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "null array");
      return $null;
    }
    const jsize sz = JCALL1(GetArrayLength, jenv, $input);
    jbyte* const jarr = JCALL2(GetByteArrayElements, jenv, $input, 0);
    if (!jarr) return $null;
    byteArray = (uint8_t *)memoryFactory.alloc(20); // this is a memory leak?
    memcpy(byteArray, jarr, sz);
    JCALL3(ReleaseByteArrayElements, jenv, $input, jarr, JNI_ABORT);
    $1 = (uint8_t (*) [20])byteArray;
}
%typemap(out) (uint8_t (*)[20]) {
  $result = JCALL1(NewByteArray, jenv, 20);
  JCALL4(SetByteArrayRegion, jenv, $result, 0, 20, (jbyte *)(*$1));
}
%typemap(argout) (uint8_t (*)[20]) {
  //JCALL3(ReleaseByteArrayElements, jenv, $input, (jbyte *) *$1, 0);
}

%typemap(javain) (uint8_t (*)[20]) "$javainput"
%typemap(javaout) (uint8_t (*)[20]) {
    return $jnicall;
  }
%typemap(freearg) (uint8_t (*)[20]) ""