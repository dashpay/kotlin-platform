package org.dashj.platform.sdk;

import org.junit.jupiter.api.Test;

import java.math.BigInteger;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class IdentityPublicKeyTest extends BaseTest {


    @Test
    public void enumConstructorDestructorTest() {
        // can't use this in java
    }

    @Test
    public void createKeyIDinJavaAndDestroy() {
        KeyID id = new KeyID(1);
        assertEquals(1, id.toInt());
        id.delete();
    }

    @Test
    public void createTimestampMillisinJavaAndDestroy() {
        long timestamp = System.currentTimeMillis();
        TimestampMillis ts = new TimestampMillis(timestamp);
        assertEquals(timestamp, ts.toLong());
        ts.delete();
    }

    @Test
    public void createTimestampMillisinJavaAndDestroy2() {
        long timestamp = System.currentTimeMillis();
        TimestampMillis ts = new TimestampMillis(); // uses time(NULL) to get the current time
        assertEquals(timestamp/1000, ts.toLong()/1000);
        ts.delete();
    }


    @Test
    public void enumTest() {
        Purpose purpose = Purpose.AUTHENTICATION;
        KeyType keyType = KeyType.ECDSA_SECP256K1;
        SecurityLevel securityLevel = SecurityLevel.HIGH;
        assertEquals(KeyType.ECDSA_SECP256K1, keyType);
        assertEquals(Purpose.AUTHENTICATION, purpose);
        assertEquals(SecurityLevel.HIGH, securityLevel);
    }

    @Test
    public void createIdentityPublicKeyInJavaAndDestroyWithNullsTest() {
        Identifier contract = new Identifier(contractIdentifier);
        KeyID keyId = new KeyID(2);

        Purpose purpose = Purpose.AUTHENTICATION;
        KeyType keyType = KeyType.ECDSA_SECP256K1;
        SecurityLevel securityLevel = SecurityLevel.HIGH;

        BinaryData data = new BinaryData(publicKey);
        assertArrayEquals(publicKey, data.get_0());

        IdentityPublicKeyV0 ipkv0 = new IdentityPublicKeyV0(
                keyId,
                purpose,
                securityLevel,
                null,
                keyType,
                false,
                data,
                null
        );
        System.out.printf("identitypublickeyv0 %x\n", IdentityPublicKeyV0.getCPtr(ipkv0));
        System.out.flush();
        assertEquals(2, ipkv0.getId().toInt());
        assertEquals(Purpose.AUTHENTICATION, ipkv0.getPurpose());
        assertEquals(KeyType.ECDSA_SECP256K1, ipkv0.getKeyType());
        assertEquals(SecurityLevel.HIGH, ipkv0.getSecurityLevel());
        assertArrayEquals(publicKey, ipkv0.getData().get_0());
        assertFalse(ipkv0.getRead_only());
        assertNull(ipkv0.getDisabled_at());
        assertNull(ipkv0.getContract_bounds());

        ipkv0.delete(); // this still crashes, why? it was due to contract bounds
        data.delete();
        keyId.delete();
    }


    @Test
    public void createIdentityPublicKeyInJavaAndDestroyTest() {
        KeyID keyId = new KeyID(0);
        Purpose purpose = Purpose.AUTHENTICATION;
        KeyType keyType = KeyType.ECDSA_SECP256K1;
        SecurityLevel securityLevel = SecurityLevel.HIGH;

        BinaryData data = new BinaryData(publicKey);
        Identifier id = new Identifier(contractIdentifier);
        ContractBounds singleContract = new ContractBounds(id);

        TimestampMillis timestampMillis = new TimestampMillis();
        assertArrayEquals(publicKey, data.get_0());

        // this constructor will clone all arguments
        IdentityPublicKeyV0 ipkv0 = new IdentityPublicKeyV0(
                keyId,
                purpose,
                securityLevel,
                singleContract,
                keyType,
                false,
                data,
                timestampMillis
        );

        assertEquals(purpose, ipkv0.getPurpose());
        assertEquals(keyType, ipkv0.getKeyType());
        assertEquals(securityLevel, ipkv0.getSecurityLevel());
        assertEquals(keyId.toInt(), ipkv0.getId().toInt());
        assertArrayEquals(publicKey, ipkv0.getData().get_0());
        assertEquals(timestampMillis, ipkv0.getDisabled_at());
        assertEquals(singleContract.getTag(), ipkv0.getContract_bounds().getTag());
        assertEquals(singleContract.getSingle_contract().getId(), ipkv0.getContract_bounds().getSingle_contract().getId());
        assertFalse(ipkv0.getRead_only());


        System.out.printf("ipkv0 0x%016x\n", IdentityPublicKeyV0.getCPtr(ipkv0));
        System.out.flush();

        ContractBounds singleContractDocumentType = new ContractBounds(id, "type");
        // this constructor will clone all arguments
        IdentityPublicKeyV0 ipkv0a = new IdentityPublicKeyV0(
                keyId,
                purpose,
                securityLevel,
                singleContractDocumentType,
                keyType,
                false,
                data,
                timestampMillis
        );
        assertEquals(singleContractDocumentType.getTag(), ipkv0a.getContract_bounds().getTag());
        assertEquals(singleContractDocumentType.getSingle_contract_document_type().getDocument_type_name(), ipkv0a.getContract_bounds().getSingle_contract_document_type().getDocument_type_name());
        assertEquals(singleContractDocumentType.getSingle_contract_document_type().getId(), ipkv0a.getContract_bounds().getSingle_contract_document_type().getId());

        ipkv0.delete(); // this still crashes, why?
        ipkv0a.delete();
        data.delete();

        singleContractDocumentType.delete(); // does not delete, not owner
        singleContract.delete();
        timestampMillis.delete();
        keyId.delete();
        id.delete();
        ipkv0 = null;
    }

//    @Test
//    public void randomKeyTest() {
//        KeyID id = new KeyID(1);
//        IdentityPublicKeyV0 ipkv0 = example.randomKey(id);
//
//        assertEquals(id, ipkv0.getId());
//
//        ipkv0.delete();
//        id.delete();
//    }
//
//    @Test
//    public void randomKeySecondNullArgsTest() {
//        KeyID id = new KeyID(1);
//        Identifier identifier1 = new Identifier(identifier);
//        IdentityPublicKeyV0 ipkv0 = example.randomKeyArgs(id, identifier1, null);
//
//        assertEquals(id, ipkv0.getId());
//        assertArrayEquals(identifier1.get_0().get_0(), ipkv0.getContract_bounds().getSingle_contract().getId().get_0().get_0());
//        assertNull(ipkv0.getDisabled_at());
//
//        ipkv0.delete();
//        id.delete();
//        identifier1.delete();
//    }
//
//    @Test
//    public void randomKeyNullArgsTest() {
//        KeyID id = new KeyID(1);
//        Identifier identifier1 = new Identifier(identifier);
//        IdentityPublicKeyV0 ipkv0 = example.randomKeyArgs(id, null, null);
//
//        assertEquals(id, ipkv0.getId());
//        assertNull(ipkv0.getContract_bounds());
//        assertNull(ipkv0.getDisabled_at());
//
//        ipkv0.delete();
//        id.delete();
//        identifier1.delete();
//    }
//
//    @Test
//    public void randomKeyFirstNullArgsTest() {
//        KeyID id = new KeyID(1);
//        TimestampMillis timestamp = new TimestampMillis();
//        IdentityPublicKeyV0 ipkv0 = example.randomKeyArgs(id, null, timestamp);
//        assertEquals(id, ipkv0.getId());
//        assertNull(ipkv0.getContract_bounds());
//        assertEquals(timestamp, ipkv0.getDisabled_at());
//
//        ipkv0.delete();
//        id.delete();
//        timestamp.delete();
//    }
//
//
//    @Test
//    public void randomKeyNonNullArgsTest() {
//        KeyID id = new KeyID(1);
//        Identifier contractId = new Identifier(contractIdentifier);
//        TimestampMillis timestampMillis = new TimestampMillis();
//        IdentityPublicKeyV0 ipkv0 = example.randomKeyArgs(id, contractId, timestampMillis);
//        assertEquals(id, ipkv0.getId());
//        assertNotNull(ipkv0.getContract_bounds());
//        assertNotNull(ipkv0.getDisabled_at());
//        assertEquals(ContractBounds_Tag.SingleContract, ipkv0.getContract_bounds().getTag());
//        assertEquals(timestampMillis, ipkv0.getDisabled_at());
//
//        ipkv0.delete();
//        contractId.delete();
//        timestampMillis.delete();
//        id.delete();
//    }
}
