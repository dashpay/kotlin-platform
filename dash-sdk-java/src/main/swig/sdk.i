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