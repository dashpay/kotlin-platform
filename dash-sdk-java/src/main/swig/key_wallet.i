%rename(applicationSessionAuthenticationPath) platform_mobile_key_wallet_application_session_authentication_path(bool is_testnet, Arr_u8_32 *identity_id, Arr_u8_32 *request_id);
%rename(applicationSessionAuthenticationPrivateKeyFromSeed) platform_mobile_key_wallet_application_session_authentication_private_key_from_seed(bool is_testnet, Vec_u8 *seed, Arr_u8_32 *identity_id, Arr_u8_32 *request_id);
%rename(applicationEncryptionPrivateKeyFromSeed) platform_mobile_key_wallet_application_encryption_private_key_from_seed;
%newobject platform_mobile_key_wallet_application_session_authentication_private_key_from_seed;
%newobject platform_mobile_key_wallet_application_encryption_private_key_from_seed;

%extend Result_ok_Vec_u8_err_String {
    ~Result_ok_Vec_u8_err_String() {
        if ($self->ok != nullptr) {
            volatile uint8_t * bytes = $self->ok->values;
            for (uintptr_t i = 0; i < $self->ok->count; ++i) bytes[i] = 0;
        }
        Result_ok_Vec_u8_err_String_destroy($self);
    }
}
