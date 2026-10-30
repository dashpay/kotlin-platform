// Transfer the successful opaque SDK handle without cloning or freeing it.
%typemap(jni) Result_ok_platform_mobile_sdk_DashSdk_err_String * "jlong"
%typemap(jtype) Result_ok_platform_mobile_sdk_DashSdk_err_String * "long"
%typemap(jstype) Result_ok_platform_mobile_sdk_DashSdk_err_String * "SWIGTYPE_p_DashSdk"
%typemap(javaout) Result_ok_platform_mobile_sdk_DashSdk_err_String * {
    long cPtr = $jnicall;
    return cPtr == 0 ? null : new SWIGTYPE_p_DashSdk(cPtr, true);
}
%typemap(out) Result_ok_platform_mobile_sdk_DashSdk_err_String * {
    if (!$1) {
        SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "Missing SDK construction result");
        $result = 0;
    } else {
        if ($1->error) {
            SWIG_JavaThrowException(jenv, SWIG_JavaIllegalArgumentException, $1->error);
            $result = 0;
        } else {
            $result = (jlong)$1->ok;
            $1->ok = nullptr;
        }
        Result_ok_platform_mobile_sdk_DashSdk_err_String_destroy($1);
    }
}
%ignore Result_ok_platform_mobile_sdk_DashSdk_err_String;

%newobject platform_mobile_sdk_create_dash_sdk_with_context(uintptr_t context_provider_context,
                                                          uint64_t quorum_public_key_callback,
                                                          uint64_t data_contract_callback,
                                                          bool is_testnet,
                                                          uintptr_t connect_timeout,
                                                          uintptr_t timeout,
                                                          uintptr_t retries);

/*
FFI-representation of the [`platform_mobile :: sdk :: create_dash_sdk`]
 */
%newobject platform_mobile_sdk_create_dash_sdk(uint64_t quorum_public_key_callback,
                                             uint64_t data_contract_callback,
                                             bool is_testnet);

/*
FFI-representation of the [`platform_mobile :: sdk :: create_dash_sdk_using_single_evonode`]
 */
%newobject platform_mobile_sdk_create_dash_sdk_using_single_evonode(char *evonode,
                                                                  uint64_t quorum_public_key_callback,
                                                                  uint64_t data_contract_callback,
                                                                  bool is_testnet);
