package org.dashj.platform.sdk;

import org.bitcoinj.core.Base58;
import org.dashj.platform.sdk.base.Result;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;


import java.math.BigInteger;
import java.util.HashMap;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class IdentityTest extends BaseTest {

    private static final String testIdentifier = "7Yowk46VwwHqmD5yZyyygggh937aP6h2UW7aQWBdWpM5";
    @Test
    public void basicIdentityInRustAndDestroy() {
        Identifier identifier1 = new Identifier(identifier);
        Identity identity = dashsdk.createBasicIdentity(identifier);
        Assertions.assertEquals(Identity.Tag.V0, identity.getTag());
        IdentityV0 identityV0 = identity.getV0().get_0();
        assertNotNull(identityV0);
        assertArrayEquals(identifier, identityV0.getId().get_0().get_0());
        assertEquals(0L, identityV0.getRevision().toLong());
        assertEquals(0L, identityV0.getBalance());
        assertNull(identityV0.getPublicKey(0));
        identity.delete();
        identifier1.delete();
    }

//    @Test
//    public void fetchIdentityAndDestroy() {
//        Identifier identifier1 = new Identifier(contractIdentifier);
//        Identity identity = dashsdk.fetchIdentity2(identifier1);
//        assertEquals(Identity.Tag.V0, identity.getTag());
//        IdentityV0 identityV0 = identity.getV0().get_0();
//        assertNotNull(identityV0);
//        assertArrayEquals(contractIdentifier, identityV0.getId().get_0().get_0());
//        assertEquals(0L, identityV0.getRevision().toLong());
//        assertEquals(0L, identityV0.getBalance());
//        assertNotNull(identityV0.getPublicKey(0));
//        identity.delete();
//        identifier1.delete();
//    }

    @Test
    public void fetchIdentity3AndDestroy() throws Exception {
        Identifier identifier1 = new Identifier(Base58.decode(testIdentifier));
        Result<Identity, String> result = dashsdk.platformMobileFetchIdentityFetchIdentityWithCore(identifier1);
        Identity identity = result.unwrap();
        assertEquals(Identity.Tag.V0, identity.getTag());
        IdentityV0 identityV0 = identity.getV0().get_0();
        assertNotNull(identityV0);
        assertEquals(identifier1, identityV0.getId());
        assertEquals(0L, identityV0.getRevision().toLong());
        assertTrue(identityV0.getBalance() > 0);
        assertNotNull(identityV0.getPublicKey(0));
        identity.delete();
        identifier1.delete();
    }

    @Test
    public void fetchIdentity3FailAndDestroy() throws Exception {
        Identifier identifier1 = new Identifier(identifier);
        Result<Identity, String> result = dashsdk.platformMobileFetchIdentityFetchIdentityWithCore(identifier1);
        assertNotNull(result.unwrapError());
        //String error = result.getError();
        //assertNotNull(error);
        //result.delete();
        //result.unwrap().delete();
        identifier1.delete();
    }

//    @Test
//    public void fetchIdentityByKeyFailAndDestroy() throws Exception {
//        Identifier identifier1 = new Identifier(identifier);
//        Result<Identity, String> result = dashsdk.getIdentityByPublicKeyHash(new byte[20], );
//        assertNotNull(result.unwrapError());
//        //String error = result.getError();
//        //assertNotNull(error);
//        //result.delete();
//        //result.unwrap().delete();
//        identifier1.delete();
//    }

    @Test
    public void getIdentityTest() {
        Identifier identifier = new Identifier(contractIdentifier);
        Identity identity = dashsdk.getIdentity2(identifier);
        assertNotNull(identity);
        assertTrue(identity.swigCMemOwn);
        assertEquals(Identity.Tag.V0, identity.getTag());
        IdentityV0 identityV0 = identity.getV0().get_0();

        assertEquals(2, identityV0.getBalance());
        assertEquals(1, identityV0.getRevision().toLong());
        assertEquals(2, identityV0.getPublicKeyCount());

        assertNotNull(identityV0.getId());
        assertEquals(identifier, identityV0.getId());

        Map<KeyID, IdentityPublicKey> map = identityV0.getPublicKeys();
        map.values().forEach(ipk -> {
            IdentityPublicKeyV0 ipkv0 = ipk.getV0().get_0();
            assertEquals(1, ipkv0.getId().toInt());
            assertFalse(ipkv0.getRead_only());
            Assertions.assertEquals(Purpose.AUTHENTICATION, ipkv0.getPurpose());
            Assertions.assertEquals(SecurityLevel.MASTER, ipkv0.getSecurityLevel());
            Assertions.assertEquals(KeyType.ECDSA_SECP256K1, ipkv0.getKeyType());
            assertNull(ipkv0.getDisabled_at());
            assertEquals(33, ipkv0.getData().get_0().length);
            assertNull(ipkv0.getContract_bounds());
        });

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
        Identity identityWithBounds = dashsdk.getIdentityContractBounds(id, idContract);
        assertNotNull(identityWithBounds);
        assertTrue(identityWithBounds.swigCMemOwn);
        assertEquals(Identity.Tag.V0, identityWithBounds.getTag());
        IdentityV0 identityV0 = identityWithBounds.getV0().get_0();

        assertEquals(2, identityV0.getBalance());
        assertEquals(1, identityV0.getRevision().toLong());
        assertEquals(2, identityV0.getPublicKeyCount());

        assertNotNull(identityWithBounds.getV0().get_0().getId());
        Identifier myId = new Identifier(identifier);
        assertEquals(myId, identityWithBounds.getV0().get_0().getId());
        myId.delete();

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
            assertEquals(idContract, ipkv0.getContract_bounds().getSingle_contract_document_type().getId());
        }

        IdentityPublicKeyV0 ipkv0 = identityV0.getPublicKey(0);
        IdentityPublicKeyV0 identityPublicKeyV0ById = identityWithBounds.getV0().get_0().getPublicKeyById(1);
        assertEquals(ipkv0.getData().get_0().length, identityPublicKeyV0ById.getData().get_0().length);
        assertArrayEquals(ipkv0.getData().get_0(), identityPublicKeyV0ById.getData().get_0());
        identityWithBounds.delete();
        id.delete();
        idContract.delete();
    }

    @Test
    public void createBasicIdentityInRustAndDestroyTest() {
        Identifier identifier = new Identifier(contractIdentifier);
        Identity identity = dashsdk.createBasicIdentity(identifier.get_0().get_0());
        assertNotNull(identity);
        assertTrue(identity.swigCMemOwn);
        assertEquals(Identity.Tag.V0, identity.getTag());
        IdentityV0 identityV0 = identity.getV0().get_0();
        assertArrayEquals(contractIdentifier, identityV0.getId().get_0().get_0());
        assertEquals(0, identityV0.getRevision().toLong());
        assertEquals(0, identityV0.getBalance());
        assertEquals(0, identityV0.getPublicKeyCount());
        identity.delete();
        identifier.delete();
    }

    @Test
    void createIdentityTest() {
        HashMap<KeyID, IdentityPublicKey> keyMap = new HashMap<>();
        for(int i = 0; i < 2; ++i) {
            IdentityPublicKeyV0 ipkv0 = new IdentityPublicKeyV0(
                    new KeyID(i + 1),
                    Purpose.AUTHENTICATION,
                    SecurityLevel.MASTER,
                    null,
                    KeyType.BIP13_SCRIPT_HASH,
                    false,
                    new BinaryData(new byte[20]),
                    new TimestampMillis()
            );
            IdentityPublicKey ipk = new IdentityPublicKey(ipkv0);
            keyMap.put(new KeyID(i + 1), ipk);
        }

        Identifier identifier1 = new Identifier(identifier);
        IdentityV0 identityV0 = new IdentityV0(identifier1, keyMap, BigInteger.TEN, new Revision(1));
        Identity identity = new Identity(identityV0);
        identity.getV0().get_0().getPublicKeys().values().forEach(ipk -> {
            IdentityPublicKeyV0 ipkv0 = ipk.getV0().get_0();
            assertEquals(Purpose.AUTHENTICATION, ipkv0.getPurpose());
        });
        identifier1.delete();
        identityV0.delete();
        identity.delete();
    }

    @Test
    public void assetLockTest() {
        OutPoint outPoint = new OutPoint(identifier, 0);
        assertEquals(0, outPoint.getIndex());
        assertArrayEquals(identifier, outPoint.getTxid());

        InstantAssetLockProof instantAssetLockProof = new InstantAssetLockProof(identifier, identifier, 0);
        assertArrayEquals(identifier, instantAssetLockProof.getInstant_lock());
        assertArrayEquals(identifier, instantAssetLockProof.getTransaction());
        assertEquals(0, instantAssetLockProof.getOutput_index());

        ChainAssetLockProof chainAssetLockProof = new ChainAssetLockProof(2_000_000, outPoint);
        assertEquals(2_000_000, chainAssetLockProof.getCore_chain_locked_height());
        // assertEquals(outPoint, chainAssetLockProof.getOut_point());

        AssetLockProof instantProof = new AssetLockProof(instantAssetLockProof);
        assertEquals(AssetLockProof.Tag.Instant, instantProof.getTag());
        assertArrayEquals(identifier, instantProof.getInstant().get_0().getInstant_lock());
        assertArrayEquals(identifier, instantProof.getInstant().get_0().getTransaction());
        assertEquals(0, instantProof.getInstant().get_0().getOutput_index());

        AssetLockProof chainProof = new AssetLockProof(chainAssetLockProof);
        assertEquals(AssetLockProof.Tag.Chain, chainProof.getTag());
        assertEquals(2_000_000, chainProof.getChain().get_0().getCore_chain_locked_height());
        // assertEquals(outPoint, chainProof.getChain().get_0().getOut_point());

        outPoint.delete();
    }
//
//    @Test
//    public void traitTest() {
//
//        ChainType type = dashsdk.ffiGetChainSettings();
//        IHaveChainSettings_TraitObject trait = dashsdk.chainTypeAsIHaveChainSettingsTraitObject(type);
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
//        MyIdentityFactory myFactory = dashsdk.ffiGetIdentityFactory();
//        IdentityFactory_TraitObject traitObject = dashsdk.myIdentityFactoryAsIdentityFactoryTraitObject(myFactory);
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
//        ChainType mainNet = dashsdk.chainTypeMainNetCtor();
//        assertEquals("ChainType_MainNet", mainNet.getTag().toString());
//        //ChainType chainType = new ChainType();
//        //dashsdk.
//        SWIGTYPE_p_void p_void = new SWIGTYPE_p_void();
//        //String result = dashsdk.ffiGetChainTypeStringAsync(p_void, mainNet);
//    }

    @Test
    public void identityBalanceTest() throws Exception {
        SWIGTYPE_p_DashSdk sdk = dashsdk.platformMobileSdkCreateDashSdk(BigInteger.ZERO, BigInteger.ZERO);
        Result<Long, String> result = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(sdk, new Identifier(identifier));
        result.unwrapError();

        Identifier id = new Identifier(Base58.decode(testIdentifier));
        Result<Long, String> result2 = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(sdk, id);
        result2.unwrap();
    }
}
