package org.dashj.platform.sdk;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;


import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class IdentityTest extends BaseTest {
    @Test
    public void basicIdentityInRustAndDestroy() {
        Identifier identifier1 = new Identifier(identifier);
        Identity identity = example.fetchIdentity(identifier1);
        Assertions.assertEquals(Identity.Tag.V0, identity.getTag());
        IdentityV0 identityV0 = identity.getV0().get_0();
        assertNotNull(identityV0);
        assertArrayEquals(identifier, identityV0.getId().get_0().get_0());
        assertEquals(0L, identityV0.getRevision().toLong());
        assertEquals(0L, identityV0.getBalance());
        assertNull(identityV0.getPublicKey(0));
        example.dppIdentityIdentityIdentityDestroy(identity);
        identifier1.delete();
    }

    @Test
    public void fetchIdentityAndDestroy() {
        Identifier identifier1 = new Identifier(contractIdentifier);
        Identity identity = example.fetchIdentity2(identifier1);
        assertEquals(Identity.Tag.V0, identity.getTag());
        IdentityV0 identityV0 = identity.getV0().get_0();
        assertNotNull(identityV0);
        assertArrayEquals(contractIdentifier, identityV0.getId().get_0().get_0());
        assertEquals(0L, identityV0.getRevision().toLong());
        assertEquals(0L, identityV0.getBalance());
        assertNotNull(identityV0.getPublicKey(0));
        example.dppIdentityIdentityIdentityDestroy(identity);
        identifier1.delete();
    }

    @Test
    public void fetchIdentity3AndDestroy() {
        Identifier identifier1 = new Identifier(contractIdentifier);
        IdentityResult result = example.fetchIdentity3(identifier1);
        Identity identity = result.getOk();
        assertEquals(Identity.Tag.V0, identity.getTag());
        IdentityV0 identityV0 = identity.getV0().get_0();
        assertNotNull(identityV0);
        assertArrayEquals(contractIdentifier, identityV0.getId().get_0().get_0());
        assertEquals(0L, identityV0.getRevision().toLong());
        assertEquals(0L, identityV0.getBalance());
        assertNotNull(identityV0.getPublicKey(0));
        example.dppIdentityIdentityIdentityDestroy(identity);
        identifier1.delete();
    }

    @Test
    public void fetchIdentity3FailAndDestroy() {
        Identifier identifier1 = new Identifier(identifier);
        IdentityResult result = example.fetchIdentity3(identifier1);
        assertNull(result.getOk());
        String error = result.getError();
        assertNotNull(error);
        result.delete();
        identifier1.delete();
    }

    @Test
    public void getDocument() {
        Identifier identifier1 = example.getDocument();

        identifier1.delete();
    }

    @Test
    public void getIdentityTest() {
        Identifier identifier = new Identifier(contractIdentifier);
        Identity identity = example.getIdentity2(identifier);
        assertNotNull(identity);
        assertTrue(identity.swigCMemOwn);
        assertEquals(Identity.Tag.V0, identity.getTag());
        IdentityV0 identityV0 = identity.getV0().get_0();

        assertEquals(2, identityV0.getBalance());
        assertEquals(1, identityV0.getRevision().toLong());
        assertEquals(2, identityV0.getPublicKeyCount());

        assertNotNull(identityV0.getId().get_0().get_0());
        assertArrayEquals(identifier.get_0().get_0(), identityV0.getId().get_0().get_0());

        for (int i = 0; i < identityV0.getPublicKeyCount(); ++i) {
            IdentityPublicKeyV0 ipkv0 = identityV0.getPublicKey(i);
            assertEquals(1, ipkv0.getId().toInt());
            assertFalse(ipkv0.getRead_only());
            Assertions.assertEquals(Purpose.AUTHENTICATION, ipkv0.getPurpose());
            Assertions.assertEquals(SecurityLevel.MASTER, ipkv0.getSecurityLevel());
            Assertions.assertEquals(KeyType.ECDSA_SECP256K1, ipkv0.getKeyType());
            assertNull(ipkv0.getDisabled_at());
            assertEquals(33, ipkv0.getData().get_0().length);
            assertNull(ipkv0.getContract_bounds());
        }

        IdentityPublicKeyV0 ipkv0 = identityV0.getPublicKey(0);
        IdentityPublicKeyV0 identityPublicKeyV0ById = identityV0.getPublicKeyById(1);
        assertEquals(ipkv0.getData().get_0().length, identityPublicKeyV0ById.getData().get_0().length);
        assertArrayEquals(ipkv0.getData().get_0(), identityPublicKeyV0ById.getData().get_0());
        identity.delete();
    }

    @Test
    public void getIdentityWithBoundsFromRustAndDestroyTest() {
        Identifier id = new Identifier(identifier);
        Identifier idContract = new Identifier(contractIdentifier);
        Identity identityWithBounds = example.getIdentityContractBounds(id, idContract);
        assertNotNull(identityWithBounds);
        assertTrue(identityWithBounds.swigCMemOwn);
        assertEquals(Identity.Tag.V0, identityWithBounds.getTag());
        IdentityV0 identityV0 = identityWithBounds.getV0().get_0();

        assertEquals(2, identityV0.getBalance());
        assertEquals(1, identityV0.getRevision().toLong());
        assertEquals(2, identityV0.getPublicKeyCount());

        assertNotNull(identityWithBounds.getV0().get_0().getId().get_0().get_0());
        assertArrayEquals(identifier, identityWithBounds.getV0().get_0().getId().get_0().get_0());

        for (int i = 0; i < identityV0.getPublicKeyCount(); ++i) {
            IdentityPublicKeyV0 ipkv0 = identityV0.getPublicKey(i);
            assertEquals(1, ipkv0.getId().toInt());
            assertFalse(ipkv0.getRead_only());
            assertEquals(Purpose.AUTHENTICATION, ipkv0.getPurpose());
            assertEquals(SecurityLevel.MASTER, ipkv0.getSecurityLevel());
            assertEquals(KeyType.ECDSA_SECP256K1, ipkv0.getKeyType());
            assertNotNull(ipkv0.getDisabled_at());
            assertEquals(33, ipkv0.getData().get_0().length);
            assertNotNull(ipkv0.getContract_bounds());
            Assertions.assertEquals(ContractBounds.Tag.SingleContract, ipkv0.getContract_bounds().getTag());
            assertArrayEquals(contractIdentifier, ipkv0.getContract_bounds().getSingle_contract_document_type().getId().get_0().get_0());
        }

        IdentityPublicKeyV0 ipkv0 = identityV0.getPublicKey(0);
        IdentityPublicKeyV0 identityPublicKeyV0ById = identityWithBounds.getV0().get_0().getPublicKeyById(1);
        assertEquals(ipkv0.getData().get_0().length, identityPublicKeyV0ById.getData().get_0().length);
        assertArrayEquals(ipkv0.getData().get_0(), identityPublicKeyV0ById.getData().get_0());
        // this crashes the system, it was created in Rust
        // this crashes with ContractBounds::drop
        identityWithBounds.delete();
        id.delete();
        idContract.delete();
    }

    @Test
    public void createBasicIdentityInRustAndDestroyTest() {
        Identifier identifier = new Identifier(contractIdentifier);
        Identity identity = example.createBasicIdentity(identifier.get_0().get_0());
        assertNotNull(identity);
        assertTrue(identity.swigCMemOwn);
        assertEquals(Identity.Tag.V0, identity.getTag());
        IdentityV0 identityV0 = identity.getV0().get_0();
        assertArrayEquals(contractIdentifier, identityV0.getId().get_0().get_0());
        assertEquals(0, identityV0.getRevision().toLong());
        assertEquals(0, identityV0.getBalance());
        assertEquals(0, identityV0.getPublicKeyCount());
        example.dppIdentityIdentityIdentityDestroy(identity);
        identifier.delete();
    }
//
//    @Test
//    public void traitTest() {
//
//        ChainType type = example.ffiGetChainSettings();
//        IHaveChainSettings_TraitObject trait = example.chainTypeAsIHaveChainSettingsTraitObject(type);
//        IHaveChainSettings ihcs = new IHaveChainSettings(trait);
//        assertEquals(0, ihcs.genesisHeight());
//        assertEquals(0, ihcs.genesisHash().get_0()[1]);
//
//        byte[] hash = new byte[32];
//        for (int i = 0; i < 32; ++i) {
//            hash[i] = (byte) i;
//        }
//        HashID hashID = new HashID(hash);
//        assertArrayEquals(hashID.get_0(), hash);
//
//        MyIdentityFactory myFactory = example.ffiGetIdentityFactory();
//        IdentityFactory_TraitObject traitObject = example.myIdentityFactoryAsIdentityFactoryTraitObject(myFactory);
//        IdentityFactory factory = new IdentityFactory(traitObject);
//        Identity identity = factory.getIdentity(new Identifier(new byte[32]));
//        assertArrayEquals(new byte[32], identity.getV0().getId().get_0().get_0());
//
//        IdentityPublicKeyV0 ipkv0 = identity.getV0().getPublicKey(0);
//        KeyID keyId = ipkv0.getId();
//        assertEquals(1, keyId.get_0());
//        assertFalse(ipkv0.getRead_only());
//        assertEquals(Purpose.AUTHENTICATION, ipkv0.getPurpose());
//
//        IdentityPublicKeyV0 identityPublicKeyV0ById = identity.getV0().getPublicKeyById(1);
//        assertEquals(ipkv0.getData().get_0().length, identityPublicKeyV0ById.getData().get_0().length);
//        assertArrayEquals(ipkv0.getData().get_0(), identityPublicKeyV0ById.getData().get_0());
//    }


//    @Test
//    public void asyncFunctionTest() {
//        ChainType mainNet = example.chainTypeMainNetCtor();
//        assertEquals("ChainType_MainNet", mainNet.getTag().toString());
//        //ChainType chainType = new ChainType();
//        //example.
//        SWIGTYPE_p_void p_void = new SWIGTYPE_p_void();
//        //String result = example.ffiGetChainTypeStringAsync(p_void, mainNet);
//    }

//    @Test
//    public void createAnIdentityInJavaAndDestroy() {
//        Identity identity = new Identity();
//        identity.delete();
//    }
}
