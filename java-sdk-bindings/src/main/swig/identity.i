%ignore dpp_identity_v0_IdentityV0::public_keys;
%ignore dpp_identity_v0_IdentityV0::balance;
%rename(IdentityV0) dpp_identity_v0_IdentityV0;

%extend dpp_identity_v0_IdentityV0 {
    dpp_identity_v0_IdentityV0(
        platform_value_types_identifier_Identifier * identifier,
        std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey * public_keys,
        uint64_t balance,
        dpp_prelude_Revision *revision
    ) {
        return dpp_identity_v0_IdentityV0_ctor(
            Identifier_clone(identifier),
            platform_mobile_identity_std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_clone(public_keys),
            balance,
            dpp_prelude_Revision_ctor(revision->_0)
        );
    }

    ~dpp_identity_v0_IdentityV0() {
        printf("~IdentityV0(%lx)\n", (uint64_t)$self);
        dpp_identity_v0_IdentityV0_destroy($self);
    }

    int getPublicKeyCount() {
        return $self->public_keys->count;
    }

    struct dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 * getPublicKey(uint32_t index) {
        if (index < $self->public_keys->count) {
            return $self->public_keys->values[index]->v0._0;
        } else {
            return NULL;
        }
    }

    struct dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 * getPublicKeyById(uint32_t id) {
        for (int i = 0; i < $self->public_keys->count; ++i) {
            if ($self->public_keys->keys[i]->_0 == id)
                return $self->public_keys->values[i]->v0._0;
        }
        return NULL;
    }

    long long getBalance() {
        return (long)$self->balance;
    }

    std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey * getPublicKeys() {
        return $self->public_keys;
    }
}

%extend dpp_identity_identity_Identity {
    dpp_identity_identity_Identity(dpp_identity_v0_IdentityV0 * identityV0) {
        return dpp_identity_identity_Identity_V0_ctor(platform_mobile_identity_IdentityV0_clone(identityV0));
    }
    ~dpp_identity_identity_Identity() {
        printf("~Identity(%lx)\n", (uint64_t)$self);
        dpp_identity_identity_Identity_destroy($self);
    }
}
%rename(Identity) dpp_identity_identity_Identity;
//%rename(Identity_Tag) dpp_identity_identity_Identity_Tag;
//%rename(IdentityV0Type) dpp_identity_identity_Identity_V0;

%newobject platform_mobile_identity_get_identity2(platform_value_types_identifier_Identifier *);
%newobject platform_mobile_identity_get_an_identity(void);
%newobject platform_mobile_identity_create_basic_identity(uint8_t (*)[32]);
%newobject platform_mobile_identity_get_identity_contract_bounds(platform_value_types_identifier_Identifier *identifier, platform_value_types_identifier_Identifier *contract_identifier);
%newobject platform_mobile_fetch_identity_fetch_identity3(platform_value_types_identifier_Identifier *identifier);
%newobject platform_mobile_fetch_identity_fetch_identity(platform_value_types_identifier_Identifier *identifier);
%newobject platform_mobile_fetch_identity_get_document(void);
%newobject platform_mobile_fetch_identity_fetch_identity2(platform_value_types_identifier_Identifier *identifier);
%rename (fetchIdentity) platform_mobile_fetch_identity_fetch_identity(platform_value_types_identifier_Identifier *identifier);
%rename (fetchIdentity2) platform_mobile_fetch_identity_fetch_identity2(platform_value_types_identifier_Identifier *identifier);
%rename (fetchIdentity3) platform_mobile_fetch_identity_fetch_identity3(platform_value_types_identifier_Identifier *identifier);
%rename (getDocument) platform_mobile_fetch_identity_get_document();
%rename (createBasicIdentity) platform_mobile_identity_create_basic_identity(uint8_t (*id)[32]);
%rename (getIdentityContractBounds) platform_mobile_identity_get_identity_contract_bounds(platform_value_types_identifier_Identifier *identifier, platform_value_types_identifier_Identifier *contract_identifier);
%rename (getIdentity2) platform_mobile_identity_get_identity2(platform_value_types_identifier_Identifier *);

%rename (IdentityResult) Result_ok_dpp_identity_identity_Identity_err_String;
%extend Result_ok_dpp_identity_identity_Identity_err_String {
    ~Result_ok_dpp_identity_identity_Identity_err_String() {
        Result_ok_dpp_identity_identity_Identity_err_String_destroy($self);
    }
}
%ignore platform_mobile_identity_std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_clone(std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *public_keys);
%ignore dpp_identity_v0_IdentityV0_ctor(platform_value_types_identifier_Identifier *id,
                                                            std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *public_keys,
                                                            uint64_t balance,
                                                            dpp_prelude_Revision *revision);
struct std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey;

%include "stdint.i"

%typemap(javaclassname) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* "java.util.Map<KeyID, IdentityPublicKey>"
%typemap(javatype) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* "java.util.Map<KeyID, IdentityPublicKey>"
%typemap(jtype) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* "java.util.Map<KeyID, IdentityPublicKey>"
%typemap(jstype) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* "java.util.Map<KeyID, IdentityPublicKey>"
%typemap(jni) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* "jobject"

%typemap(out) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* {
    // Create a new HashMap in Java to store the results
    jclass hashMapClass = jenv->FindClass("java/util/HashMap");
    printf("hashMapClass = 0x%ld\n", (long) hashMapClass);
    jmethodID hashMapInit = jenv->GetMethodID(hashMapClass, "<init>", "()V");
    printf("hashMapInit = 0x%ld\n", (long) hashMapInit);
    jobject hashMapInstance = jenv->NewObject(hashMapClass, hashMapInit);
    printf("hashMapInstance = 0x%ld", (long) hashMapInstance);
    jmethodID putMethod = jenv->GetMethodID(hashMapClass, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");

    if (!putMethod){
        SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "cannot find HashMap.put method id");
        return $null;
    };


    jclass keyIDClass = jenv->FindClass("org/dashj/platform/sdk/KeyID");
    jmethodID keyIDConstructor = jenv->GetMethodID(keyIDClass, "<init>", "(JZ)V");
    jclass jclass_IdentityPublicKey = jenv->FindClass("org/dashj/platform/sdk/IdentityPublicKey");
    jmethodID jmethodID_IdentityPublicKey_constructor = jenv->GetMethodID(keyIDClass, "<init>", "(JZ)V");
    if (!keyIDConstructor){
        SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "cannot find keyIDConstructor method id");
        return $null;
    };
    if (!jmethodID_IdentityPublicKey_constructor){
            SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "cannot find jmethodID_IdentityPublicKey_constructor method id");
            return $null;
    };
    for (uintptr_t i = 0; i < $1->count; ++i) {
    printf(" processing key %d", i);
        //jclass keyIDClass = jenv->FindClass("org/dashj/platform/sdk/KeyID");
        //jmethodID keyIDConstructor = jenv->GetMethodID(keyIDClass, "<init>", "(LB)V");

        // Convert C pointer to KeyID to its Java counterpart, assuming it's a simple integer or has its own typemap
        jobject key = jenv->NewObject(keyIDClass, keyIDConstructor, (jlong)$1->keys[i], false);

        // Assuming IdentityPublicKey has a constructor that takes the C pointer directly

        jobject value = jenv->NewObject(jclass_IdentityPublicKey, jmethodID_IdentityPublicKey_constructor, (jlong)$1->values[i], false);

        // Put the key-value pair into the map
        // Step 1: Find the HashMap class

        jenv->CallObjectMethod(hashMapInstance, putMethod, key, value);
    }

    $result = hashMapInstance;
}

%typemap(in) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* {
    // Prepare to convert the incoming Java map to the C structure
//     $1.count = 0;
//     $1.keys = 0;
//     $1.values = 0;
//
//     // Check if the input is null
//     if (!$input) {
//         return; // Handle null input appropriately
//     }


    // Get the Set of entries from the Map
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
    int count = (uintptr_t)entryCount;
    dpp_identity_identity_public_key_KeyID**keys = (dpp_identity_identity_public_key_KeyID**)malloc(entryCount * sizeof(dpp_identity_identity_public_key_KeyID*));
    dpp_identity_identity_public_key_IdentityPublicKey**values = (dpp_identity_identity_public_key_IdentityPublicKey**)malloc(entryCount * sizeof(dpp_identity_identity_public_key_IdentityPublicKey*));
    $1 = std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_ctor(count, keys, values);

    jclass keyIDClass = jenv->FindClass("org/dashj/platform/sdk/KeyID");
    jmethodID keyIDPtrMethod = jenv->GetMethodID(keyIDClass, "getCPointer", "()J");
    jclass identityPublicKeyClass = jenv->FindClass("org/dashj/platform/sdk/IdentityPublicKey");
    jmethodID getNativePtrMethod = jenv->GetMethodID(identityPublicKeyClass, "getCPointer", "()J");
    // Iterate over the entries
    jint i = 0;
    while (jenv->CallBooleanMethod(iterator, hasNextMethod)) {
        jobject entry = jenv->CallObjectMethod(iterator, nextMethod);

        // Convert the key and value from Java to C
        jobject keyObject = jenv->CallObjectMethod(entry, getKeyMethod);
        jobject valueObject = jenv->CallObjectMethod(entry, getValueMethod);

        dpp_identity_identity_public_key_KeyID * keyID = (dpp_identity_identity_public_key_KeyID*)jenv->CallLongMethod(keyObject, keyIDPtrMethod);
        $1->keys[i] = dpp_identity_identity_public_key_KeyID_ctor(keyID->_0);

        // For IdentityPublicKey, you need to extract the native pointer/address from the Java object
        // This requires IdentityPublicKey class to have a method to return the native pointer

        jlong nativePtr = jenv->CallLongMethod(valueObject, getNativePtrMethod);
        dpp_identity_identity_public_key_IdentityPublicKey * ipk = reinterpret_cast<dpp_identity_identity_public_key_IdentityPublicKey*>(nativePtr);
        // TODO -- finish constructor call, lol
        $1->values[i] = platform_mobile_identity_IdentityPublicKey_clone(ipk);

        i++;
    }
}

%typemap(freearg) struct std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* {
    free($1.keys);
    free($1.values);
}

%typemap(javain) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey * "$javainput"

%typemap(javaout) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey * {
    return $jnicall;
  }


%typemap(throws) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey");
   return $null; %}

%apply struct std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey {struct std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey};
%ignore std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey;
%ignore dpp_identity_v0_IdentityV0_ctor(platform_value_types_identifier_Identifier *id,
                                                                    std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *public_keys,
                                                                    uint64_t balance,
                                                                    dpp_prelude_Revision *revision);
%ignore dpp_identity_v0_IdentityV0_get_public_keys(const dpp_identity_v0_IdentityV0 *obj);
%ignore dpp_identity_v0_IdentityV0_set_public_keys(dpp_identity_v0_IdentityV0 *obj,
                                                std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *value);
%ignore std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_destroy(std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *ffi);