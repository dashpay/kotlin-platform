package org.dashj.platform.sdk;

import org.dashj.platform.sdk.base.Result;
import org.junit.jupiter.api.Test;

import java.math.BigInteger;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;

public class DataContractTest extends BaseTest {
    @Test
    public void fetchDataContractTest() throws Exception {
        Identifier contractId = new Identifier(dpnsContractId);
        Result<DataContract, String> result = dashsdk.platformMobileDataContractsFetchDataContract(contractId, BigInteger.ZERO, BigInteger.ZERO);

        DataContract dataContract = result.unwrap();
        assertArrayEquals(dpnsContractId, dataContract.getId().get_0().get_0());
        assertEquals(2, dataContract.getDoc_types().size());
        dataContract.getDoc_types().forEach(System.out::println);
    }
}
