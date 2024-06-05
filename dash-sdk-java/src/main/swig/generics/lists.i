%define LIST_STRUCT_TYPEMAP(STRUCT_TYPE, ITEM_TYPE, SHORT_TYPE, CLONE_FN)
%typemap(javaclassname) STRUCT_TYPE* "java.util.List<SHORT_TYPE>"
%typemap(javatype) STRUCT_TYPE* "java.util.List<SHORT_TYPE>"
%typemap(jtype) STRUCT_TYPE* "java.util.List<SHORT_TYPE>"
%typemap(jstype) STRUCT_TYPE* "java.util.List<SHORT_TYPE>"
%typemap(jni) STRUCT_TYPE* "jobject"

%typemap(javain) STRUCT_TYPE * "$javainput"

%typemap(javaout) STRUCT_TYPE * {
    return $jnicall;
}

%typemap(throws) STRUCT_TYPE *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null "##STRUCT_TYPE);
   return $null; %}

%typemap(in) STRUCT_TYPE* {
    if (!$input) {
        SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "List is null");
        return $null;
    }

    jclass listClass = jenv->GetObjectClass($input);
    jmethodID sizeMethod = jenv->GetMethodID(listClass, "size", "()I");
    jint size = jenv->CallIntMethod($input, sizeMethod);

    ITEM_TYPE ** values = new ITEM_TYPE*[size];
    $1 = STRUCT_TYPE##_ctor(size, values);

    jmethodID getMethod = jenv->GetMethodID(listClass, "get", "(I)Ljava/lang/Object;");
    for (jint i = 0; i < size; ++i) {
        jobject elementObj = jenv->CallObjectMethod($input, getMethod, i);
        jclass valueClass = jenv->FindClass("org/dashj/platform/sdk/" #SHORT_TYPE);
        if (valueClass == nullptr) {
            SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "class not found: org/dashj/platform/sdk/" #SHORT_TYPE);
            return $null;
        }
        jmethodID getNativePtrMethod = jenv->GetMethodID(valueClass, "getCPointer", "()J");
        if (getNativePtrMethod == nullptr) {
            SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "getCPointer not found: org/dashj/platform/sdk/" #SHORT_TYPE);
            return $null;
        }
        jlong nativePtr = jenv->CallLongMethod(elementObj, getNativePtrMethod);

        auto *ipk = reinterpret_cast<ITEM_TYPE *>(nativePtr);
        $1->values[i] = CLONE_FN(ipk);
    }
}

%typemap(freearg) STRUCT_TYPE *
%{
    STRUCT_TYPE##_destroy($1);
%}

%typemap(out) STRUCT_TYPE* {
    jclass listClass = jenv->FindClass("java/util/ArrayList");
    jmethodID ctor = jenv->GetMethodID(listClass, "<init>", "()V");
    jmethodID addMethod = jenv->GetMethodID(listClass, "add", "(Ljava/lang/Object;)Z");
    jobject listObj = jenv->NewObject(listClass, ctor);
    jclass valueClass = jenv->FindClass("org/dashj/platform/sdk/" #SHORT_TYPE);
    if (valueClass == nullptr) {
        SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "class not found: org/dashj/platform/sdk/" #SHORT_TYPE);
        return $null;
    }
    jmethodID valueConstructor = jenv->GetMethodID(valueClass, "<init>", "(JZ)V");
    if (valueConstructor == nullptr) {
        SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "constructor not found: org/dashj/platform/sdk/" #SHORT_TYPE);
        return $null;
    }
    for (uintptr_t i = 0; i < $1->count; ++i) {
        jobject elementObj = jenv->NewObject(valueClass, valueConstructor, $1->values[i], false);
        jenv->CallBooleanMethod(listObj, addMethod, elementObj);
        jenv->DeleteLocalRef(elementObj);
    }
    $result = listObj;
}

%apply struct STRUCT_TYPE {struct STRUCT_TYPE};
%ignore STRUCT_TYPE;

%enddef