package org.dashj.platform.sdk;

import org.junit.AfterClass;
import org.junit.BeforeClass;

import java.math.BigInteger;

import static org.junit.Assert.assertNotEquals;
import static org.junit.Assert.assertNotNull;

public class SdkBaseTest extends BaseTest {

    static SWIGTYPE_p_DashSdk sdk;
    static SWIGTYPE_p_DashSdk mainNetSdk;

    @BeforeClass
    public static void startUp() {
        sdk = dashsdk.platformMobileSdkCreateDashSdk(BigInteger.ZERO, BigInteger.ZERO, true);
        assertNotNull(sdk);
        assertNotEquals(sdk.getCPointer(), 0L);
        mainNetSdk = dashsdk.platformMobileSdkCreateDashSdk(BigInteger.ZERO, BigInteger.ZERO, false);
        assertNotNull(mainNetSdk);
        assertNotEquals(mainNetSdk.getCPointer(), 0L);
    }

    @AfterClass
    public static void tearDown() {
        dashsdk.platformMobileSdkDestroyDashSdk(sdk);
        if (mainNetSdk != null) {
            dashsdk.platformMobileSdkDestroyDashSdk(mainNetSdk);
        }
        // SWIGTYPE_p_DashSdk does not own the pointer
        // assertEquals(sdk.getCPointer(), 0);
    }
}
