%define LIST_PRIMITIVE_TYPEMAP(STRUCT_TYPE, ITEM_TYPE, SHORT_TYPE, CLONE_FN)
%typemap(javaclassname) STRUCT_TYPE* "java.util.List<java.lang.SHORT_TYPE>"
%typemap(javatype) STRUCT_TYPE* "java.util.List<java.lang.SHORT_TYPE>"
%typemap(jtype) STRUCT_TYPE* "java.util.List<java.lang.SHORT_TYPE>"
%typemap(jstype) STRUCT_TYPE* "java.util.List<java.lang.SHORT_TYPE>"
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
        SWIG_JavaThrowException(jenv, SWIG_JavaNullPointerException, "List of primitives is null");
        return $null;
    }

    jclass listClass = jenv->GetObjectClass($input);
    jmethodID sizeMethod = jenv->GetMethodID(listClass, "size", "()I");
    jint size = jenv->CallIntMethod($input, sizeMethod);

    ITEM_TYPE * values = new ITEM_TYPE[size];
    $1 = STRUCT_TYPE##_ctor((uintptr_t)size, values);

    jmethodID getMethod = jenv->GetMethodID(listClass, "get", "(I)Ljava/lang/Object;");
    for (jint i = 0; i < size; ++i) {
        jobject elementObj = jenv->CallObjectMethod($input, getMethod, i);
        jclass valueClass = jenv->FindClass("java/lang/" #SHORT_TYPE);
        if (valueClass == nullptr) {
            SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "class not found: java/lang/" #SHORT_TYPE);
            return $null;
        }

        if (strcmp(#SHORT_TYPE, "String") == 0) {
            jint length = jenv->GetStringUTFLength((jstring)elementObj);
            const char * bytes = jenv->GetStringUTFChars((jstring)elementObj, nullptr);
            LOGI("converting String \"%s\" %d", bytes, length);
            $1->values[i] = reinterpret_cast<char*>(memoryFactory.cloneString(bytes, length));
            jenv->ReleaseStringUTFChars((jstring)elementObj, bytes);
        } else {
            $1->values[i] = 0;
        }
    }
}

%typemap(out) STRUCT_TYPE* {
    jclass listClass = jenv->FindClass("java/util/ArrayList");
    jmethodID ctor = jenv->GetMethodID(listClass, "<init>", "()V");
    jmethodID addMethod = jenv->GetMethodID(listClass, "add", "(Ljava/lang/Object;)Z");
    jobject listObj = jenv->NewObject(listClass, ctor);
    jclass valueClass = jenv->FindClass("java/lang/" #SHORT_TYPE);
    if (valueClass == nullptr) {
        SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "class not found: java/lang/" #SHORT_TYPE);
        return $null;
    }

    for (uintptr_t i = 0; i < $1->count; ++i) {
        jobject elementObj = nullptr;
        if (strcmp(#SHORT_TYPE, "String") == 0) {
            elementObj = jenv->NewStringUTF((const char *)$1->values[i]);
        } else if (strcmp(#SHORT_TYPE, "Integer") == 0) {
            jclass integerClass = (jenv)->FindClass("java/lang/Integer");
            jmethodID constructor = (jenv)->GetMethodID(integerClass, "<init>", "(I)V");
            elementObj = (jenv)->NewObject(integerClass, constructor, *$1->values[i]);
        } else if (strcmp(#SHORT_TYPE, "Long") == 0) {
              jclass integerClass = (jenv)->FindClass("java/lang/Long");
              jmethodID constructor = (jenv)->GetMethodID(integerClass, "<init>", "(J)V");
              elementObj = (jenv)->NewObject(integerClass, constructor, *$1->values[i]);
        }
        jenv->CallBooleanMethod(listObj, addMethod, elementObj);
        jenv->DeleteLocalRef(elementObj);
    }
    $result = listObj;
}

%apply struct STRUCT_TYPE {struct STRUCT_TYPE};
%ignore STRUCT_TYPE;

%enddef

LIST_PRIMITIVE_TYPEMAP(Vec_String, char *, String, memoryFactory.clone);

