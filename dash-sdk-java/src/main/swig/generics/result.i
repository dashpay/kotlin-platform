/**
  * These macros will take a Result object such as Result_ok_dpp_identity_identity_Identity_err_String
  * and convert it to a Java Object such as Result<Identity, String>.
  *
  * It will clone the Ok object and destroy the original Result object.
  */

/**
  * Result<T, String>
  */
%define DEFINE_RESULT(RETURN_TYPE, ERROR_TYPE, CTYPE, CLONE_FN)
%typemap(javaclassname) CTYPE* "org.dashj.platform.sdk.base.Result<RETURN_TYPE, String>"
%typemap(javatype) CTYPE* "org.dashj.platform.sdk.base.Result<RETURN_TYPE,String>"
%typemap(jtype) CTYPE* "org.dashj.platform.sdk.base.Result<RETURN_TYPE,String>"
%typemap(jstype) CTYPE* "org.dashj.platform.sdk.base.Result<RETURN_TYPE,String>"
%typemap(jni) CTYPE* "jobject"

%typemap(out) CTYPE* {
    if (!$1) {
        $result = NULL;
    } else {
        jclass resultClass = jenv->FindClass("org/dashj/platform/sdk/base/Result");

        if ($1->ok != NULL) {
            jclass myClass = jenv->FindClass("org/dashj/platform/sdk/" #RETURN_TYPE);
            jmethodID constructor = jenv->GetMethodID(myClass, "<init>", "(JZ)V");
            void * clonedObject = CLONE_FN($1->ok);
            jobject okObject = jenv->NewObject(myClass, constructor, (jlong) clonedObject, true);

            jmethodID midSuccess = jenv->GetStaticMethodID(resultClass, "Ok", "(Ljava/lang/Object;)Lorg/dashj/platform/sdk/base/Result;");
            $result = jenv->CallStaticObjectMethod(resultClass, midSuccess, okObject);
        } else {
            jstring errorString = jenv->NewStringUTF($1->error);
            jmethodID midFailure = jenv->GetStaticMethodID(resultClass, "Err", "(Ljava/lang/Object;)Lorg/dashj/platform/sdk/base/Result;");
            $result = jenv->CallStaticObjectMethod(resultClass, midFailure, errorString);
        }
        // destroy the Result<T, E>
        CTYPE##_destroy($1);
    }
}

%typemap(in) CTYPE* {
    SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "This operation is not supported: convert from Result<T, E> to " #CTYPE);
    return $null;
}

%typemap(freearg) struct CTYPE* {
    // not used, not created
}

%typemap(javain) CTYPE * "$javainput"

%typemap(javaout) CTYPE * {
    return $jnicall;
  }


%typemap(throws) CTYPE *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null CTYPE");
   return $null; %}

%apply struct CTYPE {struct CTYPE};
%ignore CTYPE;
%enddef


/**
  * Result<Option<T>, String>
  */
%define DEFINE_OPTIONAL_RESULT(RETURN_TYPE, ERROR_TYPE, CTYPE, CLONE_FN)
%typemap(javaclassname) CTYPE* "org.dashj.platform.sdk.base.Result<java.util.Optional<RETURN_TYPE>, String>"
%typemap(javatype) CTYPE* "org.dashj.platform.sdk.base.Result<java.util.Optional<RETURN_TYPE, String>"
%typemap(jtype) CTYPE* "org.dashj.platform.sdk.base.Result<java.util.Optional<RETURN_TYPE>, String>"
%typemap(jstype) CTYPE* "org.dashj.platform.sdk.base.Result<java.util.Optional<RETURN_TYPE>, String>"
%typemap(jni) CTYPE* "jobject"


%typemap(out) CTYPE* {
    if (!$1) {
        $result = NULL; // should we throw an exception instead, or it will be thrown in Java
    } else {
        jclass optionalClass = jenv->FindClass("java/util/Optional");
        jclass resultClass = jenv->FindClass("org/dashj/platform/sdk/base/Result");
        jmethodID emptyMethod = jenv->GetStaticMethodID(optionalClass, "empty", "()Ljava/util/Optional;");
        jmethodID ofMethod = jenv->GetStaticMethodID(optionalClass, "of", "(Ljava/lang/Object;)Ljava/util/Optional;");

        if ($1->error == NULL) {
            if ($1->ok == NULL) {
                jobject okObject = jenv->CallStaticObjectMethod(optionalClass, emptyMethod);
                jmethodID midSuccess = jenv->GetStaticMethodID(resultClass, "Ok", "(Ljava/lang/Object;)Lorg/dashj/platform/sdk/base/Result;");
                $result = jenv->CallStaticObjectMethod(resultClass, midSuccess, okObject);
            } else {
                jclass myClass = jenv->FindClass("org/dashj/platform/sdk/" #RETURN_TYPE);
                jmethodID constructor = jenv->GetMethodID(myClass, "<init>", "(JZ)V");
                void * clonedObject = CLONE_FN($1->ok);
                jobject okObject = jenv->NewObject(myClass, constructor, (jlong) clonedObject, true);
                jobject optionalObject = jenv->CallStaticObjectMethod(optionalClass, ofMethod, okObject);
                jmethodID midSuccess = jenv->GetStaticMethodID(resultClass, "Ok", "(Ljava/lang/Object;)Lorg/dashj/platform/sdk/base/Result;");
                $result = jenv->CallStaticObjectMethod(resultClass, midSuccess, optionalObject);
            }
        } else {
            jstring errorString = jenv->NewStringUTF($1->error);
            jmethodID midFailure = jenv->GetStaticMethodID(resultClass, "Err", "(Ljava/lang/Object;)Lorg/dashj/platform/sdk/base/Result;");
            $result = jenv->CallStaticObjectMethod(resultClass, midFailure, errorString);
        }
        // destroy the Result<T, E>
        CTYPE##_destroy($1);
    }
}

%typemap(in) CTYPE* {
    SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "This operation is not supported: convert from Result<T, E> to " #CTYPE);
    return $null;
}

%typemap(freearg) struct CTYPE* {
    // not used, not created
}

%typemap(javain) CTYPE * "$javainput"

%typemap(javaout) CTYPE * {
    return $jnicall;
  }


%typemap(throws) CTYPE *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null CTYPE");
   return $null; %}

%apply struct CTYPE {struct CTYPE};
%ignore CTYPE;
%enddef

/**
  * Result<Vec<T>, String>
  */
%define DEFINE_LIST_RESULT(RETURN_TYPE, ERROR_TYPE, CTYPE, CTYPE_ITEM)
%typemap(javaclassname) CTYPE* "org.dashj.platform.sdk.base.Result<java.util.List<RETURN_TYPE>, String>"
%typemap(javatype) CTYPE* "org.dashj.platform.sdk.base.Result<java.util.List<RETURN_TYPE>,String>"
%typemap(jtype) CTYPE* "org.dashj.platform.sdk.base.Result<java.util.List<RETURN_TYPE>,String>"
%typemap(jstype) CTYPE* "org.dashj.platform.sdk.base.Result<java.util.List<RETURN_TYPE>,String>"
%typemap(jni) CTYPE* "jobject"


%typemap(out) CTYPE* {
   //printf("processing Result with list: 0x%lx", $1);
    if (!$1) {
        $result = NULL;
    } else {
        jclass resultClass = jenv->FindClass("org/dashj/platform/sdk/base/Result");
        // printf("processing Result with list: 0x%lx", $1->ok);
        if ($1->ok != NULL) {
            jclass listClass = jenv->FindClass("java/util/ArrayList");
            jmethodID ctor = jenv->GetMethodID(listClass, "<init>", "()V");
            jmethodID addMethod = jenv->GetMethodID(listClass, "add", "(Ljava/lang/Object;)Z");
            jobject listObj = jenv->NewObject(listClass, ctor);
            jclass valueClass = jenv->FindClass("org/dashj/platform/sdk/" #RETURN_TYPE);
            // printf("created list\n");
            if (valueClass == nullptr) {
                SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "class not found: org/dashj/platform/sdk/" #RETURN_TYPE);
                return $null;
            }
            jmethodID valueConstructor = jenv->GetMethodID(valueClass, "<init>", "(JZ)V");
            if (valueConstructor == nullptr) {
                SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "constructor not found: org/dashj/platform/sdk/" #RETURN_TYPE);
                return $null;
            }
            for (uintptr_t i = 0; i < $1->ok->count; ++i) {
                auto * valueClone = clone((CTYPE_ITEM *)$1->ok->values[i]);
                jobject elementObj = jenv->NewObject(valueClass, valueConstructor, valueClone, true);
                jenv->CallBooleanMethod(listObj, addMethod, elementObj);
                jenv->DeleteLocalRef(elementObj);
            }

            jobject okObject = listObj;

            jmethodID midSuccess = jenv->GetStaticMethodID(resultClass, "Ok", "(Ljava/lang/Object;)Lorg/dashj/platform/sdk/base/Result;");
            $result = jenv->CallStaticObjectMethod(resultClass, midSuccess, okObject);

            //$result = listObj;
        } else {
            jstring errorString = jenv->NewStringUTF($1->error);
            jmethodID midFailure = jenv->GetStaticMethodID(resultClass, "Err", "(Ljava/lang/Object;)Lorg/dashj/platform/sdk/base/Result;");
            $result = jenv->CallStaticObjectMethod(resultClass, midFailure, errorString);
        }
        // destroy the Result<T, E>
        CTYPE##_destroy($1);
    }
}

%typemap(in) CTYPE* {
    SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "This operation is not supported: convert from Result<T, E> to " #CTYPE);
    return $null;
}

%typemap(freearg) struct CTYPE* {
    // not used, not created
}

%typemap(javain) CTYPE * "$javainput"

%typemap(javaout) CTYPE * {
    return $jnicall;
  }


%typemap(throws) CTYPE *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null CTYPE");
   return $null; %}

%apply struct CTYPE {struct CTYPE};
%ignore CTYPE;
%enddef

/**
  * Result<T, String>
  *   T is a primitive type
  */
%define DEFINE_PRIMITIVE_RESULT(RETURN_TYPE, ERROR_TYPE, CTYPE)
%typemap(javaclassname) CTYPE* "org.dashj.platform.sdk.base.Result<RETURN_TYPE, String>"
%typemap(javatype) CTYPE* "org.dashj.platform.sdk.base.Result<RETURN_TYPE,String>"
%typemap(jtype) CTYPE* "org.dashj.platform.sdk.base.Result<RETURN_TYPE,String>"
%typemap(jstype) CTYPE* "org.dashj.platform.sdk.base.Result<RETURN_TYPE,String>"
%typemap(jni) CTYPE* "jobject"

%typemap(out) CTYPE* {
    if (!$1) {
        $result = NULL;
    } else {
        jclass resultClass = jenv->FindClass("org/dashj/platform/sdk/base/Result");

        if ($1->ok != NULL) {
            printf("ok is non-null: %lx\n", $1->ok);
            jobject elementObj = nullptr;
            if (strcmp(#RETURN_TYPE, "String") == 0) {
                // printf("string item\n");
                // printf("string item: %s\n", $1->ok);
                elementObj = jenv->NewStringUTF((const char *)$1->ok);
            } else if (strcmp(#RETURN_TYPE, "Integer") == 0) {
                //printf("int item\n");
                //printf("int item: %d\n", *$1->ok);
                jclass integerClass = (jenv)->FindClass("java/lang/Integer");
                jmethodID constructor = (jenv)->GetMethodID(integerClass, "<init>", "(I)V");
                elementObj = (jenv)->NewObject(integerClass, constructor, (int)(long)*$1->ok); // ok is a pointer, but acts as a value
            } else if (strcmp(#RETURN_TYPE, "Long") == 0) {
                //printf("long item\n");
                //printf("long item: %lld\n", $1->ok);
                jclass integerClass = (jenv)->FindClass("java/lang/Long");
                jmethodID constructor = (jenv)->GetMethodID(integerClass, "<init>", "(J)V");
                elementObj = (jenv)->NewObject(integerClass, constructor, (long)*$1->ok);  // ok is a pointer, but acts as a value
            } else {
                printf("invalid? item\n");
            }
            printf("ok value is assigned, now create result\n");
            jmethodID midSuccess = jenv->GetStaticMethodID(resultClass, "Ok", "(Ljava/lang/Object;)Lorg/dashj/platform/sdk/base/Result;");
            $result = jenv->CallStaticObjectMethod(resultClass, midSuccess, elementObj);
        } else {
            jstring errorString = jenv->NewStringUTF($1->error);
            jmethodID midFailure = jenv->GetStaticMethodID(resultClass, "Err", "(Ljava/lang/Object;)Lorg/dashj/platform/sdk/base/Result;");
            $result = jenv->CallStaticObjectMethod(resultClass, midFailure, errorString);
        }
        // destroy the Result<T, E>
        CTYPE##_destroy($1);
    }
}

%typemap(in) CTYPE* {
    SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "This operation is not supported: convert from Result<T, E> to " #CTYPE);
    return $null;
}

%typemap(freearg) struct CTYPE* {
    // not used, not created
}

%typemap(javain) CTYPE * "$javainput"

%typemap(javaout) CTYPE * {
    return $jnicall;
  }


%typemap(throws) CTYPE *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null CTYPE");
   return $null; %}

%apply struct CTYPE {struct CTYPE};
%ignore CTYPE;
%enddef

DEFINE_RESULT(Identity, String, Result_ok_dpp_identity_identity_Identity_err_String, platform_mobile_identity_Identity_clone);
DEFINE_OPTIONAL_RESULT(Identity, String, Result_ok_Option_dpp_identity_identity_Identity_err_String, platform_mobile_identity_Identity_clone);
DEFINE_RESULT(Document, String, Result_ok_dpp_document_Document_err_String, clone);
DEFINE_RESULT(DataContract, String, Result_ok_platform_mobile_data_contracts_DataContractFFI_err_String, clone);
DEFINE_OPTIONAL_RESULT(DataContract, String, Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String, clone);

DEFINE_RESULT(ContestedResources, String, Result_ok_drive_proof_verifier_types_ContestedResources_err_String, clone);
DEFINE_RESULT(Contenders, String, Result_ok_drive_proof_verifier_types_Contenders_err_String, clone);
DEFINE_RESULT(Vote, String, Result_ok_dpp_voting_votes_Vote_err_String, clone)
DEFINE_LIST_RESULT(Document, String, Result_ok_Vec_dpp_document_Document_err_String, dpp_document_Document);

// primitive types
DEFINE_PRIMITIVE_RESULT(Long, String, Result_ok_u64_err_String);