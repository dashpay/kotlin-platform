package org.dashj.platform.sdk;

import org.bitcoinj.core.Base58;
import org.dashj.platform.sdk.base.Result;
import org.junit.AfterClass;
import org.junit.BeforeClass;
import org.junit.Test;

import java.math.BigInteger;
import java.util.HashMap;
import java.util.Map;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertNull;
import static org.junit.Assert.assertTrue;

public class IdentityObjectTest extends BaseTest {

    private static final String testIdentifier = "7Yowk46VwwHqmD5yZyyygggh937aP6h2UW7aQWBdWpM5";
    @Test
    public void basicIdentityInRustAndDestroy() {
        Identifier identifier1 = new Identifier(identifier);
        Identity identity = dashsdk.createBasicIdentity(identifier);
        assertEquals(Identity.Tag.V0, identity.getTag());
        IdentityV0 identityV0 = identity.getV0().get_0();
        assertNotNull(identityV0);
        assertArrayEquals(identifier, identityV0.getId().get_0().get_0());
        assertEquals(0L, identityV0.getRevision().toLong());
        assertEquals(0L, identityV0.getBalance());
        assertNull(identityV0.getPublicKey(0));
        identity.delete();
        identifier1.delete();
    }

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
            assertEquals(Purpose.AUTHENTICATION, ipkv0.getPurpose());
            assertEquals(SecurityLevel.MASTER, ipkv0.getSecurityLevel());
            assertEquals(KeyType.ECDSA_SECP256K1, ipkv0.getKeyType());
            assertNull(ipkv0.getDisabled_at());
            assertEquals(33, ipkv0.getData().get_0().length);
            assertNull(ipkv0.getContract_bounds());
        });

        for (int i = 0; i < identityV0.getPublicKeyCount(); ++i) {
            IdentityPublicKeyV0 ipkv0 = identityV0.getPublicKey(i);
            assertEquals(1, ipkv0.getId().toInt());
            assertFalse(ipkv0.getRead_only());
            assertEquals(Purpose.AUTHENTICATION, ipkv0.getPurpose());
            assertEquals(SecurityLevel.MASTER, ipkv0.getSecurityLevel());
            assertEquals(KeyType.ECDSA_SECP256K1, ipkv0.getKeyType());
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
            assertEquals(ContractBounds.Tag.SingleContract, ipkv0.getContract_bounds().getTag());
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
    public void createIdentityTest() {
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
}
