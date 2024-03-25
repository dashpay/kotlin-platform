/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.2
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package org.dashj.platform.sdk;

public class exampleJNI {
  public final static native int KeyType_ECDSA_SECP256K1_get();
  public final static native int KeyType_BLS12_381_get();
  public final static native int KeyType_ECDSA_HASH160_get();
  public final static native int KeyType_BIP13_SCRIPT_HASH_get();
  public final static native int KeyType_EDDSA_25519_HASH160_get();
  public final static native int Purpose_AUTHENTICATION_get();
  public final static native int Purpose_ENCRYPTION_get();
  public final static native int Purpose_DECRYPTION_get();
  public final static native int Purpose_TRANSFER_get();
  public final static native int Purpose_SYSTEM_get();
  public final static native int Purpose_VOTING_get();
  public final static native int SecurityLevel_MASTER_get();
  public final static native int SecurityLevel_CRITICAL_get();
  public final static native int SecurityLevel_HIGH_get();
  public final static native int SecurityLevel_MEDIUM_get();
  public final static native long new_Revision__SWIG_0();
  public final static native long new_Revision__SWIG_1(long jarg1);
  public final static native void delete_Revision(long jarg1);
  public final static native long Revision_toLong(long jarg1, Revision jarg1_);
  public final static native boolean Revision_objectEquals(long jarg1, Revision jarg1_, long jarg2, Revision jarg2_);
  public final static native long new_KeyID(int jarg1);
  public final static native void delete_KeyID(long jarg1);
  public final static native int KeyID_toInt(long jarg1, KeyID jarg1_);
  public final static native boolean KeyID_objectEquals(long jarg1, KeyID jarg1_, long jarg2, KeyID jarg2_);
  public final static native void dpp_identity_identity_public_key_KeyCount__0_set(long jarg1, dpp_identity_identity_public_key_KeyCount jarg1_, long jarg2, KeyID jarg2_);
  public final static native long dpp_identity_identity_public_key_KeyCount__0_get(long jarg1, dpp_identity_identity_public_key_KeyCount jarg1_);
  public final static native void delete_dpp_identity_identity_public_key_KeyCount(long jarg1);
  public final static native long new_TimestampMillis__SWIG_0();
  public final static native long new_TimestampMillis__SWIG_1(long jarg1);
  public final static native void delete_TimestampMillis(long jarg1);
  public final static native long TimestampMillis_toLong(long jarg1, TimestampMillis jarg1_);
  public final static native boolean TimestampMillis_objectEquals(long jarg1, TimestampMillis jarg1_, long jarg2, TimestampMillis jarg2_);
  public final static native void IdentifierBytes32__0_set(long jarg1, IdentifierBytes32 jarg1_, byte[] jarg2);
  public final static native byte[] IdentifierBytes32__0_get(long jarg1, IdentifierBytes32 jarg1_);
  public final static native long new_IdentifierBytes32(byte[] jarg1);
  public final static native void delete_IdentifierBytes32(long jarg1);
  public final static native void Identifier__0_set(long jarg1, Identifier jarg1_, long jarg2, IdentifierBytes32 jarg2_);
  public final static native long Identifier__0_get(long jarg1, Identifier jarg1_);
  public final static native long new_Identifier(byte[] jarg1);
  public final static native void delete_Identifier(long jarg1);
  public final static native void ContractBounds_SingleContract_Body_id_set(long jarg1, ContractBounds.SingleContract_Body jarg1_, long jarg2, Identifier jarg2_);
  public final static native long ContractBounds_SingleContract_Body_id_get(long jarg1, ContractBounds.SingleContract_Body jarg1_);
  public final static native void delete_ContractBounds_SingleContract_Body(long jarg1);
  public final static native void ContractBounds_SingleContractDocumentType_Body_id_set(long jarg1, ContractBounds.SingleContractDocumentType_Body jarg1_, long jarg2, Identifier jarg2_);
  public final static native long ContractBounds_SingleContractDocumentType_Body_id_get(long jarg1, ContractBounds.SingleContractDocumentType_Body jarg1_);
  public final static native void ContractBounds_SingleContractDocumentType_Body_document_type_name_set(long jarg1, ContractBounds.SingleContractDocumentType_Body jarg1_, String jarg2);
  public final static native String ContractBounds_SingleContractDocumentType_Body_document_type_name_get(long jarg1, ContractBounds.SingleContractDocumentType_Body jarg1_);
  public final static native void delete_ContractBounds_SingleContractDocumentType_Body(long jarg1);
  public final static native void ContractBounds_tag_set(long jarg1, ContractBounds jarg1_, int jarg2);
  public final static native int ContractBounds_tag_get(long jarg1, ContractBounds jarg1_);
  public final static native void ContractBounds_single_contract_set(long jarg1, ContractBounds jarg1_, long jarg2, ContractBounds.SingleContract_Body jarg2_);
  public final static native long ContractBounds_single_contract_get(long jarg1, ContractBounds jarg1_);
  public final static native void ContractBounds_single_contract_document_type_set(long jarg1, ContractBounds jarg1_, long jarg2, ContractBounds.SingleContractDocumentType_Body jarg2_);
  public final static native long ContractBounds_single_contract_document_type_get(long jarg1, ContractBounds jarg1_);
  public final static native long new_ContractBounds__SWIG_0(long jarg1, Identifier jarg1_);
  public final static native long new_ContractBounds__SWIG_1(long jarg1, Identifier jarg1_, String jarg2);
  public final static native void delete_ContractBounds(long jarg1);
  public final static native void BinaryData__0_set(long jarg1, BinaryData jarg1_, byte[] jarg2);
  public final static native byte[] BinaryData__0_get(long jarg1, BinaryData jarg1_);
  public final static native long new_BinaryData(byte[] jarg1);
  public final static native void delete_BinaryData(long jarg1);
  public final static native void IdentityPublicKeyV0_id_set(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2, KeyID jarg2_);
  public final static native long IdentityPublicKeyV0_id_get(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void IdentityPublicKeyV0_contract_bounds_set(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2, ContractBounds jarg2_);
  public final static native long IdentityPublicKeyV0_contract_bounds_get(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void IdentityPublicKeyV0_read_only_set(long jarg1, IdentityPublicKeyV0 jarg1_, boolean jarg2);
  public final static native boolean IdentityPublicKeyV0_read_only_get(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void IdentityPublicKeyV0_data_set(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2, BinaryData jarg2_);
  public final static native long IdentityPublicKeyV0_data_get(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void IdentityPublicKeyV0_disabled_at_set(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2, TimestampMillis jarg2_);
  public final static native long IdentityPublicKeyV0_disabled_at_get(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native long new_IdentityPublicKeyV0(long jarg1, KeyID jarg1_, int jarg2, int jarg3, long jarg4, ContractBounds jarg4_, int jarg5, boolean jarg6, long jarg7, BinaryData jarg7_, long jarg8, TimestampMillis jarg8_);
  public final static native void delete_IdentityPublicKeyV0(long jarg1);
  public final static native int IdentityPublicKeyV0_getKeyType(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native int IdentityPublicKeyV0_getPurpose(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native int IdentityPublicKeyV0_getSecurityLevel(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void IdentityPublicKey_V0_Body__0_set(long jarg1, IdentityPublicKey.V0_Body jarg1_, long jarg2, IdentityPublicKeyV0 jarg2_);
  public final static native long IdentityPublicKey_V0_Body__0_get(long jarg1, IdentityPublicKey.V0_Body jarg1_);
  public final static native void delete_IdentityPublicKey_V0_Body(long jarg1);
  public final static native void IdentityPublicKey_tag_set(long jarg1, IdentityPublicKey jarg1_, int jarg2);
  public final static native int IdentityPublicKey_tag_get(long jarg1, IdentityPublicKey jarg1_);
  public final static native void IdentityPublicKey_v0_set(long jarg1, IdentityPublicKey jarg1_, long jarg2, IdentityPublicKey.V0_Body jarg2_);
  public final static native long IdentityPublicKey_v0_get(long jarg1, IdentityPublicKey jarg1_);
  public final static native long new_IdentityPublicKey(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void delete_IdentityPublicKey(long jarg1);
  public final static native void IdentityV0_id_set(long jarg1, IdentityV0 jarg1_, long jarg2, Identifier jarg2_);
  public final static native long IdentityV0_id_get(long jarg1, IdentityV0 jarg1_);
  public final static native void IdentityV0_revision_set(long jarg1, IdentityV0 jarg1_, long jarg2, Revision jarg2_);
  public final static native long IdentityV0_revision_get(long jarg1, IdentityV0 jarg1_);
  public final static native long new_IdentityV0(long jarg1, Identifier jarg1_, java.util.Map<KeyID, IdentityPublicKey> jarg2, java.math.BigInteger jarg3, long jarg4, Revision jarg4_);
  public final static native void delete_IdentityV0(long jarg1);
  public final static native int IdentityV0_getPublicKeyCount(long jarg1, IdentityV0 jarg1_);
  public final static native long IdentityV0_getPublicKey(long jarg1, IdentityV0 jarg1_, long jarg2);
  public final static native long IdentityV0_getPublicKeyById(long jarg1, IdentityV0 jarg1_, long jarg2);
  public final static native long IdentityV0_getBalance(long jarg1, IdentityV0 jarg1_);
  public final static native java.util.Map<KeyID, IdentityPublicKey> IdentityV0_getPublicKeys(long jarg1, IdentityV0 jarg1_);
  public final static native void Identity_V0_Body__0_set(long jarg1, Identity.V0_Body jarg1_, long jarg2, IdentityV0 jarg2_);
  public final static native long Identity_V0_Body__0_get(long jarg1, Identity.V0_Body jarg1_);
  public final static native void delete_Identity_V0_Body(long jarg1);
  public final static native void Identity_tag_set(long jarg1, Identity jarg1_, int jarg2);
  public final static native int Identity_tag_get(long jarg1, Identity jarg1_);
  public final static native void Identity_v0_set(long jarg1, Identity jarg1_, long jarg2, Identity.V0_Body jarg2_);
  public final static native long Identity_v0_get(long jarg1, Identity jarg1_);
  public final static native long new_Identity(long jarg1, IdentityV0 jarg1_);
  public final static native void delete_Identity(long jarg1);
  public final static native void platform_value_Hash256__0_set(long jarg1, platform_value_Hash256 jarg1_, byte[] jarg2);
  public final static native byte[] platform_value_Hash256__0_get(long jarg1, platform_value_Hash256 jarg1_);
  public final static native void delete_platform_value_Hash256(long jarg1);
  public final static native void IdentityResult_ok_set(long jarg1, IdentityResult jarg1_, long jarg2, Identity jarg2_);
  public final static native long IdentityResult_ok_get(long jarg1, IdentityResult jarg1_);
  public final static native void IdentityResult_error_set(long jarg1, IdentityResult jarg1_, String jarg2);
  public final static native String IdentityResult_error_get(long jarg1, IdentityResult jarg1_);
  public final static native void delete_IdentityResult(long jarg1);
  public final static native java.math.BigInteger dppPreludeRevisionGet0(long jarg1, Revision jarg1_);
  public final static native void dppPreludeRevisionSet0(long jarg1, Revision jarg1_, java.math.BigInteger jarg2);
  public final static native long dppIdentityIdentityPublicKeyKeyCountGet0(long jarg1, dpp_identity_identity_public_key_KeyCount jarg1_);
  public final static native void dppIdentityIdentityPublicKeyKeyCountSet0(long jarg1, dpp_identity_identity_public_key_KeyCount jarg1_, long jarg2, KeyID jarg2_);
  public final static native long dppIdentityIdentityPublicKeyKeyIDGet0(long jarg1, KeyID jarg1_);
  public final static native void dppIdentityIdentityPublicKeyKeyIDSet0(long jarg1, KeyID jarg1_, long jarg2);
  public final static native java.math.BigInteger dppIdentityIdentityPublicKeyTimestampMillisGet0(long jarg1, TimestampMillis jarg1_);
  public final static native void dppIdentityIdentityPublicKeyTimestampMillisSet0(long jarg1, TimestampMillis jarg1_, java.math.BigInteger jarg2);
  public final static native long dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0GetId(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native long dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0GetPurpose(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native long dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0GetSecurityLevel(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native long dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0GetContractBounds(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native long dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0GetKeyType(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native boolean dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0GetReadOnly(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native long dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0GetData(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native long dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0GetDisabledAt(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native void dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0SetId(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2, KeyID jarg2_);
  public final static native void dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0SetPurpose(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2);
  public final static native void dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0SetSecurityLevel(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2);
  public final static native void dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0SetContractBounds(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2, ContractBounds jarg2_);
  public final static native void dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0SetKeyType(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2);
  public final static native void dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0SetReadOnly(long jarg1, IdentityPublicKeyV0 jarg1_, boolean jarg2);
  public final static native void dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0SetData(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2, BinaryData jarg2_);
  public final static native void dppIdentityIdentityPublicKeyV0IdentityPublicKeyV0SetDisabledAt(long jarg1, IdentityPublicKeyV0 jarg1_, long jarg2, TimestampMillis jarg2_);
  public final static native long dppIdentityV0IdentityV0GetId(long jarg1, IdentityV0 jarg1_);
  public final static native java.math.BigInteger dppIdentityV0IdentityV0GetBalance(long jarg1, IdentityV0 jarg1_);
  public final static native long dppIdentityV0IdentityV0GetRevision(long jarg1, IdentityV0 jarg1_);
  public final static native void dppIdentityV0IdentityV0SetId(long jarg1, IdentityV0 jarg1_, long jarg2, Identifier jarg2_);
  public final static native void dppIdentityV0IdentityV0SetBalance(long jarg1, IdentityV0 jarg1_, java.math.BigInteger jarg2);
  public final static native void dppIdentityV0IdentityV0SetRevision(long jarg1, IdentityV0 jarg1_, long jarg2, Revision jarg2_);
  public final static native long platformValueHash256Ctor(byte[] jarg1);
  public final static native byte[] platformValueHash256Get0(long jarg1, platform_value_Hash256 jarg1_);
  public final static native void platformValueHash256Set0(long jarg1, platform_value_Hash256 jarg1_, byte[] jarg2);
  public final static native byte[] platformValueTypesBinaryDataBinaryDataGet0(long jarg1, BinaryData jarg1_);
  public final static native void platformValueTypesBinaryDataBinaryDataSet0(long jarg1, BinaryData jarg1_, byte[] jarg2);
  public final static native long platformValueTypesIdentifierIdentifierBytes32Ctor(byte[] jarg1);
  public final static native byte[] platformValueTypesIdentifierIdentifierBytes32Get0(long jarg1, IdentifierBytes32 jarg1_);
  public final static native void platformValueTypesIdentifierIdentifierBytes32Set0(long jarg1, IdentifierBytes32 jarg1_, byte[] jarg2);
  public final static native long platformValueTypesIdentifierIdentifierGet0(long jarg1, Identifier jarg1_);
  public final static native void platformValueTypesIdentifierIdentifierSet0(long jarg1, Identifier jarg1_, long jarg2, IdentifierBytes32 jarg2_);
  public final static native long platformMobileGetBinaryData2();
  public final static native long platformMobileGetBinaryData();
  public final static native long getDocument();
  public final static native long fetchIdentity(long jarg1, Identifier jarg1_);
  public final static native long fetchIdentity2(long jarg1, Identifier jarg1_);
  public final static native long fetchIdentity3(long jarg1, Identifier jarg1_);
  public final static native long createBasicIdentity(byte[] jarg1);
  public final static native long platformMobileIdentityIdentityClone(long jarg1, Identity jarg1_);
  public final static native long platformMobileIdentityIdentityV0Clone(long jarg1, IdentityV0 jarg1_);
  public final static native long getIdentity2(long jarg1, Identifier jarg1_);
  public final static native long platformMobileIdentityIdentityPublicKeyClone(long jarg1, IdentityPublicKey jarg1_);
  public final static native long getIdentityContractBounds(long jarg1, Identifier jarg1_, long jarg2, Identifier jarg2_);
  public final static native long platformMobileIdentityIdentityPublicKeyV0Clone(long jarg1, IdentityPublicKeyV0 jarg1_);
  public final static native long MemoryFactory_getInstance();
  public final static native long new_MemoryFactory();
  public final static native void delete_MemoryFactory(long jarg1);
  public final static native long MemoryFactory_size(long jarg1, MemoryFactory jarg1_);
  public final static native long MemoryFactory_alloc__SWIG_0(long jarg1, MemoryFactory jarg1_, long jarg2);
  public final static native String MemoryFactory_clone(long jarg1, MemoryFactory jarg1_, String jarg2);
  public final static native long MemoryFactory_alloc__SWIG_1(long jarg1, MemoryFactory jarg1_, long jarg2, long jarg3);
  public final static native void MemoryFactory_destroy__SWIG_0(long jarg1, MemoryFactory jarg1_, long jarg2, long jarg3);
  public final static native void MemoryFactory_destroy__SWIG_1(long jarg1, MemoryFactory jarg1_, long jarg2);
  public final static native void MemoryFactory_destroyItem(long jarg1, MemoryFactory jarg1_, long jarg2);
  public final static native void memoryFactory_set(long jarg1, MemoryFactory jarg1_);
  public final static native long memoryFactory_get();
  public final static native long intToKeyType(int jarg1);
  public final static native long intToSecurityLevel(int jarg1);
  public final static native long intToPurpose(int jarg1);
  public final static native long identifierClone(long jarg1, Identifier jarg1_);
  public final static native long singleContract(long jarg1, Identifier jarg1_);
  public final static native long singleContractDocument(long jarg1, Identifier jarg1_, String jarg2);
}
