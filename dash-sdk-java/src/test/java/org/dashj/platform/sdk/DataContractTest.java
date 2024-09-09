package org.dashj.platform.sdk;

import org.dashj.platform.sdk.base.Result;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import java.math.BigInteger;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;

public class DataContractTest extends BaseTest {
    static SWIGTYPE_p_DashSdk sdk;

    @BeforeAll
    static void startUp() {
        sdk = dashsdk.platformMobileSdkCreateDashSdk(BigInteger.ZERO, BigInteger.ZERO, true);
    }

    @AfterAll
    static void tearDown() {
        dashsdk.platformMobileSdkDestroyDashSdk(sdk);
    }
    @Test
    public void fetchDataContractTest() throws Exception {
        Identifier contractId = new Identifier(dpnsContractId);
        Result<DataContract, String> result = dashsdk.platformMobileDataContractsFetchDataContract(
                sdk, contractId);

        DataContract dataContract = result.unwrap();
        assertArrayEquals(dpnsContractId, dataContract.getId().get_0().get_0());
        assertEquals(2, dataContract.getDoc_types().size());
        dataContract.getDoc_types().forEach(System.out::println);
    }

    @Test
    public void fetchDataContractFailureTest() throws Exception {
        Identifier contractId = new Identifier(identifier);
        Result<DataContract, String> result = dashsdk.platformMobileDataContractsFetchDataContract(
                sdk, contractId);
        System.out.println("missing contract request: " + result.toString());
    }
}
