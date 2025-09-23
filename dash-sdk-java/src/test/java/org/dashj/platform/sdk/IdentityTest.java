package org.dashj.platform.sdk;

import org.bitcoinj.core.Base58;
import org.bitcoinj.core.Utils;
import org.dashj.platform.sdk.base.Result;
import org.junit.Test;


import java.util.Optional;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

public class IdentityTest extends SdkBaseTest {
//    static SWIGTYPE_p_DashSdk sdk;
//
//    @BeforeClass
//    public static void startUp() {
//        sdk = dashsdk.platformMobileSdkCreateDashSdk(BigInteger.ZERO, BigInteger.ZERO, true);
//    }
//
//    @AfterClass
//    public static void tearDown() {
//        dashsdk.platformMobileSdkDestroyDashSdk(sdk);
//    }

    private static final String testIdentifier = "7Yowk46VwwHqmD5yZyyygggh937aP6h2UW7aQWBdWpM5";
    private static final String testIdentifierNotExist = "7Yowk46VwwHqmD5yZyyyhijh937aP6h2UW7aQWBdWpM5";

    @Test
    public void fetchIdentity3AndDestroy() throws Exception {
        Identifier identifier1 = new Identifier(Base58.decode(testIdentifier));
        Result<Optional<Identity>, String> result = dashsdk.platformMobileFetchIdentityFetchIdentityWithSdk(sdk, identifier1);
        Identity identity = result.unwrap().get();
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
        Result<Optional<Identity>, String> result = dashsdk.platformMobileFetchIdentityFetchIdentityWithSdk(sdk, new Identifier(Base58.decode(testIdentifierNotExist)));
        assertTrue(result.unwrap().isEmpty());
        identifier1.delete();
    }

    @Test
    public void identityBalanceTest() throws Exception {
        Identifier id = new Identifier(Base58.decode(testIdentifier));
        Result<Long, String> result2 = dashsdk.platformMobileFetchIdentityFetchIdentityBalanceWithSdk(sdk, id);
        result2.unwrap();
    }


    @Test
    public void identityFromHash160Test() throws Exception {
        byte [] id = Utils.HEX.decode("a9579df520c44c8d8773887bc5c9fbec579b962a");
        Result<Identity, String> result2 = dashsdk.platformMobileFetchIdentityFetchIdentityWithKeyhashSdk(mainNetSdk, id);
        result2.unwrap();
    }
}
