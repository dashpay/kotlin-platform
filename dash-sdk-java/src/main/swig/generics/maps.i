%define MAP_STRUCT_TYPEMAP(STRUCT_TYPE, KEY_TYPE, KEY_TYPE_JAVA, VALUE_TYPE, VALUE_TYPE_JAVA)

%typemap(javaclassname) STRUCT_TYPE* "java.util.Map<KEY_TYPE_JAVA, VALUE_TYPE_JAVA>"
%typemap(javatype) STRUCT_TYPE* "java.util.Map<KEY_TYPE_JAVA, VALUE_TYPE_JAVA>"
%typemap(jtype) STRUCT_TYPE* "java.util.Map<KEY_TYPE_JAVA, VALUE_TYPE_JAVA>"
%typemap(jstype) STRUCT_TYPE* "java.util.Map<KEY_TYPE_JAVA, VALUE_TYPE_JAVA>"
%typemap(jni) STRUCT_TYPE* "jobject"

%typemap(out) STRUCT_TYPE* {
    jclass hashMapClass = jenv->FindClass("java/util/HashMap");
    printf("hashMapClass = 0x%ld\n", (long) hashMapClass);
    jmethodID hashMapInit = jenv->GetMethodID(hashMapClass, "<init>", "()V");
    printf("hashMapInit = 0x%ld\n", (long) hashMapInit);
    jobject hashMapInstance = jenv->NewObject(hashMapClass, hashMapInit);
    printf("hashMapInstance = 0x%ld", (long) hashMapInstance);
    jmethodID putMethod = jenv->GetMethodID(hashMapClass, "put",
                                            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");

    jclass keyClass = jenv->FindClass("org/dashj/platform/sdk/" #KEY_TYPE_JAVA);
    jmethodID keyConstructor = jenv->GetMethodID(keyClass, "<init>", "(JZ)V");
    jclass valueClass = jenv->FindClass("org/dashj/platform/sdk/" #VALUE_TYPE_JAVA);
    jmethodID valueConstructor = jenv->GetMethodID(valueClass, "<init>", "(JZ)V");

    for (uintptr_t i = 0; i < $1->count; ++i) {
        jobject key = jenv->NewObject(keyClass, keyConstructor, (jlong) $1->keys[i], false);
        jobject value = jenv->NewObject(valueClass, valueConstructor,
                                        (jlong) $1->values[i], false);
        jenv->CallObjectMethod(hashMapInstance, putMethod, key, value);
        jenv->DeleteLocalRef(key);
        jenv->DeleteLocalRef(value);
    }

    $result = hashMapInstance;
}

%typemap(in) STRUCT_TYPE* {

    jclass mapClass = jenv->FindClass("java/util/Map");
    jmethodID entrySetMethod = jenv->GetMethodID(mapClass, "entrySet", "()Ljava/util/Set;");
    jobject setOfEntries = jenv->CallObjectMethod($input, entrySetMethod);

// Get iterator for the set
    jclass setClass = jenv->FindClass("java/util/Set");
    jmethodID iteratorMethod = jenv->GetMethodID(setClass, "iterator", "()Ljava/util/Iterator;");
    jobject iterator = jenv->CallObjectMethod(setOfEntries, iteratorMethod);

// Get Iterator class and methods
    jclass iteratorClass = jenv->FindClass("java/util/Iterator");
    jmethodID hasNextMethod = jenv->GetMethodID(iteratorClass, "hasNext", "()Z");
    jmethodID nextMethod = jenv->GetMethodID(iteratorClass, "next", "()Ljava/lang/Object;");

// Get Map.Entry class and methods
    jclass entryClass = jenv->FindClass("java/util/Map$Entry");
    jmethodID getKeyMethod = jenv->GetMethodID(entryClass, "getKey", "()Ljava/lang/Object;");
    jmethodID getValueMethod = jenv->GetMethodID(entryClass, "getValue", "()Ljava/lang/Object;");

// Count the number of entries and allocate memory
    jint entryCount = jenv->CallIntMethod(setOfEntries, jenv->GetMethodID(setClass, "size", "()I"));
    int count = (uintptr_t) entryCount;
    auto **keys = (KEY_TYPE **) malloc(
            entryCount * sizeof(KEY_TYPE * ));
    auto **values = (VALUE_TYPE **) malloc(
            entryCount * sizeof(VALUE_TYPE * ));
    FermentMapKeyIDIdentityPublicKey * result = STRUCT_TYPE##_ctor(count, keys, values);

    jclass keyClass = jenv->FindClass("org/dashj/platform/sdk/" #KEY_TYPE_JAVA);
    jmethodID keyPtrMethod = jenv->GetMethodID(keyClass, "getCPointer", "()J");
    jclass valueClass = jenv->FindClass("org/dashj/platform/sdk/" #VALUE_TYPE_JAVA);
    jmethodID valuePtrMethod = jenv->GetMethodID(valueClass, "getCPointer", "()J");

    jint i = 0;
    while (jenv->CallBooleanMethod(iterator, hasNextMethod)) {
        jobject entry = jenv->CallObjectMethod(iterator, nextMethod);

        jobject keyObject = jenv->CallObjectMethod(entry, getKeyMethod);
        jobject valueObject = jenv->CallObjectMethod(entry, getValueMethod);

        auto *keyID = (KEY_TYPE *) jenv->CallLongMethod(
                keyObject, keyPtrMethod);
        result->keys[i] = clone(keyID);


        jlong nativePtr = jenv->CallLongMethod(valueObject, valuePtrMethod);
        auto *ipk = reinterpret_cast<VALUE_TYPE *>(nativePtr);
        result->values[i] = clone(ipk);

        i++;
    }
    $1 = result;
}

%typemap(freearg) STRUCT_TYPE *
%{
    STRUCT_TYPE##_destroy($1);
%}

%typemap(javain) STRUCT_TYPE * "$javainput"

%typemap(javaout) STRUCT_TYPE * {
    return $jnicall;
}


%typemap(throws) STRUCT_TYPE *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null " #STRUCT_TYPE);
   return $null; %}

%apply struct STRUCT_TYPE {struct STRUCT_TYPE};
%ignore STRUCT_TYPE;

%enddef