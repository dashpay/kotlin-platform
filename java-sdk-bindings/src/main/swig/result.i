// CTYPE
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
            jclass myClass = jenv->FindClass("org/dashj/platform/sdk/RETURN_TYPE");
            jmethodID constructor = jenv->GetMethodID(myClass, "<init>", "(JZ)V");
            void * clonedObject = CLONE_FN($1->ok);
            jobject okObject = jenv->NewObject(myClass, constructor, (jlong) clonedObject, false);

            jmethodID midSuccess = jenv->GetStaticMethodID(resultClass, "Ok", "(Ljava/lang/Object;)Lorg/dashj/platform/sdk/base/Result;");
            $result = jenv->CallStaticObjectMethod(resultClass, midSuccess, okObject);
        } else {
            jstring errorString = jenv->NewStringUTF($1->error);
            jmethodID midFailure = jenv->GetStaticMethodID(resultClass, "Err", "(Ljava/lang/Object;)Lorg/dashj/platform/sdk/base/Result");
            $result = jenv->CallStaticObjectMethod(resultClass, midFailure, errorString);
        }
        // destroy the Result<T, E>
        CTYPE##_destroy($1);
    }
}

%typemap(in) CTYPE* {
    
}

%typemap(freearg) struct CTYPE* {
    free($1.keys);
    free($1.values);
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
// %rename (RETURN_TYPE##Result) CTYPE;
// %extend CTYPE {
//      ~CTYPE() {
//          CTYPE##_destroy($self);
//      }
//  }
%enddef

DEFINE_RESULT(Identity, String, Result_ok_dpp_identity_identity_Identity_err_String, platform_mobile_identity_Identity_clone);