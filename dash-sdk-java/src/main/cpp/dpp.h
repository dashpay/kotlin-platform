#ifndef DPP_H
#define DPP_H

#include <list>
#include <map>
#include <algorithm>
using namespace std;

//class IHaveChainSettings {
//private:
//    const IHaveChainSettings_TraitObject * m_traitObject;
//public:
//    IHaveChainSettings(const IHaveChainSettings_TraitObject * traitObject)
//        : m_traitObject(traitObject) {}
//
//   char *name() const {
//        return m_traitObject->vtable->name(m_traitObject->object);
//    }
//
//    HashID *genesis_hash() const {
//        return m_traitObject->vtable->genesis_hash(m_traitObject->object);
//    }
//
//    uint32_t genesis_height() const {
//        return m_traitObject->vtable->genesis_height(m_traitObject->object);
//    }
//
//    bool has_genesis_hash(HashID *hash) const {
//        return m_traitObject->vtable->has_genesis_hash(m_traitObject->object, hash);
//    }
//
//    HashID *get_hash_by_hash(HashID *hash) const {
//        return m_traitObject->vtable->get_hash_by_hash(m_traitObject->object, hash);
//    }
//
//    bool should_process_llmq_of_type(uint16_t llmq_type) const {
//        return m_traitObject->vtable->should_process_llmq_of_type(m_traitObject->object, llmq_type);
//    }
//};

//class IdentityFactory {
//    IdentityFactory_TraitObject * myFactory;
//public:
//    IdentityFactory(IdentityFactory_TraitObject * myFactory) {
//        this->myFactory = myFactory;
//    }
//
//    Identity * getIdentity(Identifier * id) {
//        return myFactory->vtable->get_identity(myFactory, id);
//    }
//};

class MemoryFactory {
    map<long, list<uint8_t*>> memoryMap;
    list<uint8_t*> memoryList;
    static MemoryFactory * instance;
public:
    static MemoryFactory * getInstance() { return instance; }
    MemoryFactory() {

    }

    ~MemoryFactory() {
        for (pair<long, list<uint8_t*>> pointerList: memoryMap) {
            for (uint8_t * item : pointerList.second) {
                delete [] item;
            }
        }
        memoryMap.clear();
        for (uint8_t * item : memoryList) {
            delete [] item;
        }
        memoryList.clear();
    }

    size_t size() {
        return memoryList.size();
    }

    void * alloc(size_t size) {
        uint8_t * memory = new uint8_t[size];
        memoryList.push_back(memory);
        return reinterpret_cast<void*>(memory);
    }

    char * clone(char * str) {
        if (str != nullptr) {
            int len = strlen(str) + 1;
            char * strClone = reinterpret_cast<char*>(alloc(len));
            strcpy(strClone, str);
            return strClone;
        }
        return nullptr;
    }

    uint8_t * cloneString(const char * str, uint32_t len) {
        if (str != nullptr) {
            uint8_t * strClone = reinterpret_cast<uint8_t*>(alloc(len));
            memcpy(strClone, str, len);
            strClone[len - 1] = 0;
            return strClone;
        }
        return nullptr;
    }

    uint8_t * clone(const uint8_t * str, uint32_t len) {
        if (str != nullptr) {
            uint8_t * strClone = reinterpret_cast<uint8_t*>(alloc(len));
            memcpy(strClone, str, len);
            return strClone;
        }
        return nullptr;
    }

    void * alloc(void * root, size_t size) {
        uint8_t * memory = new uint8_t[size];
        if (memoryMap.count((long)root)) {
            memoryMap[(long)root].push_back(memory);
        } else {
            list<uint8_t*> list;
            list.push_back(memory);
            memoryMap.insert(pair<long, ::list<uint8_t*>>((long)root, list));
        }
        return reinterpret_cast<void*>(memory);
    }

    void destroy(void * root, void * memory) {
        if (memoryMap.count((long)root)) {
            for (uint8_t * item: memoryMap[(long)root]) {
                if (item == memory)
                    delete [] item;
            }
        }
    }

    void destroy(void * root) {
        if (memoryMap.count((long)root)) {
            for (uint8_t * item: memoryMap[(long)root]) {
                delete [] item;
            }
            memoryMap.erase((long)root);
        }
    }

    void destroyItem(void * item) {
        list<uint8_t*>::iterator it = find(memoryList.begin(), memoryList.end(), item);
        if (it != memoryList.end()) {
            delete [] (uint8_t*)item;
            memoryList.erase(it);
        } else {
            printf("not destroying item %lx\n", (unsigned long)item);
        }
    }
};

extern MemoryFactory & memoryFactory;

//dpp_identity_identity_public_key_key_type_KeyType * intToKeyType2(int value) {
//    switch(value) {
//        case dpp_identity_identity_public_key_key_type_KeyType::dpp_identity_identity_public_key_key_type_KeyType_ECDSA_SECP256K1: return dpp_identity_identity_public_key_key_type_KeyType_ECDSA_SECP256K1_ctor();
//        case dpp_identity_identity_public_key_key_type_KeyType::dpp_identity_identity_public_key_key_type_KeyType_BLS12_381: return dpp_identity_identity_public_key_key_type_KeyType_BLS12_381_ctor();
//        case dpp_identity_identity_public_key_key_type_KeyType::dpp_identity_identity_public_key_key_type_KeyType_ECDSA_HASH160: return dpp_identity_identity_public_key_key_type_KeyType_ECDSA_HASH160_ctor();
//        case dpp_identity_identity_public_key_key_type_KeyType::dpp_identity_identity_public_key_key_type_KeyType_BIP13_SCRIPT_HASH: return dpp_identity_identity_public_key_key_type_KeyType_BIP13_SCRIPT_HASH_ctor();
//        case dpp_identity_identity_public_key_key_type_KeyType::dpp_identity_identity_public_key_key_type_KeyType_EDDSA_25519_HASH160: return dpp_identity_identity_public_key_key_type_KeyType_EDDSA_25519_HASH160_ctor();
//    }
//}

#define ENUM_CASE(crate, enum_class, value) \
    case crate##_##enum_class::value: \
        return crate##_##enum_class##_##value##_ctor();

dpp_identity_identity_public_key_key_type_KeyType * intToKeyType(dpp_identity_identity_public_key_key_type_KeyType value) {
    switch(value) {
        ENUM_CASE(dpp_identity_identity_public_key_key_type, KeyType, ECDSA_SECP256K1)
        ENUM_CASE(dpp_identity_identity_public_key_key_type, KeyType, BLS12_381)
        ENUM_CASE(dpp_identity_identity_public_key_key_type, KeyType, ECDSA_HASH160)
        ENUM_CASE(dpp_identity_identity_public_key_key_type, KeyType, BIP13_SCRIPT_HASH)
        ENUM_CASE(dpp_identity_identity_public_key_key_type, KeyType, EDDSA_25519_HASH160)
        default:
            return nullptr;
    }
}

dpp_identity_identity_public_key_security_level_SecurityLevel * intToSecurityLevel(dpp_identity_identity_public_key_security_level_SecurityLevel value) {
    switch(value) {
        ENUM_CASE(dpp_identity_identity_public_key_security_level, SecurityLevel, MASTER)
        ENUM_CASE(dpp_identity_identity_public_key_security_level, SecurityLevel, CRITICAL)
        ENUM_CASE(dpp_identity_identity_public_key_security_level, SecurityLevel, HIGH)
        ENUM_CASE(dpp_identity_identity_public_key_security_level, SecurityLevel, MEDIUM)
        default:
            return nullptr;
    }
}
dpp_identity_identity_public_key_purpose_Purpose * intToPurpose(dpp_identity_identity_public_key_purpose_Purpose value) {
     switch(value) {
        ENUM_CASE(dpp_identity_identity_public_key_purpose, Purpose, AUTHENTICATION)
        ENUM_CASE(dpp_identity_identity_public_key_purpose, Purpose, DECRYPTION)
        ENUM_CASE(dpp_identity_identity_public_key_purpose, Purpose, ENCRYPTION)
        ENUM_CASE(dpp_identity_identity_public_key_purpose, Purpose, TRANSFER)
        ENUM_CASE(dpp_identity_identity_public_key_purpose, Purpose, SYSTEM)
        ENUM_CASE(dpp_identity_identity_public_key_purpose, Purpose, VOTING)
        default:
            return nullptr;
     }
}

dpp_identity_identity_public_key_contract_bounds_ContractBounds * singleContract(platform_value_types_identifier_Identifier * id) {
    platform_value_types_identifier_Identifier * idCopy = platform_mobile_identity_Identifier_clone(id);
    dpp_identity_identity_public_key_contract_bounds_ContractBounds * cb = dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContract_ctor(idCopy);
    return cb;
}

static dpp_identity_identity_public_key_contract_bounds_ContractBounds * singleContractDocument(platform_value_types_identifier_Identifier * id, char * type) {
    platform_value_types_identifier_Identifier * idCopy = platform_mobile_identity_Identifier_clone(id);
    char * typeCopy = memoryFactory.clone(type);
    return dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContractDocumentType_ctor(idCopy, typeCopy);
}

#endif // this file