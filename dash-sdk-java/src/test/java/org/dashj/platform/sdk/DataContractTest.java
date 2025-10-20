package org.dashj.platform.sdk;

import org.dashj.platform.sdk.base.Result;
import org.junit.Test;

import java.util.Optional;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

public class DataContractTest extends SdkBaseTest {

    @Test
    public void fetchDataContractTest() throws Exception {
        Identifier contractId = new Identifier(dpnsContractId);
        Result<Optional<DataContract>, String> result = dashsdk.platformMobileDataContractsFetchDataContract(
                sdk, contractId);

        Optional<DataContract> dataContractResult = result.unwrap();
        assertTrue(dataContractResult.isPresent());
        DataContract dataContract = dataContractResult.get();
        assertArrayEquals(dpnsContractId, dataContract.getId().get_0().get_0());
        assertEquals(2, dataContract.getDoc_types().size());
        dataContract.getDoc_types().forEach(System.out::println);
    }

    @Test
    public void fetchDashPayDataContractTest() throws Exception {
        Identifier contractId = new Identifier(dashPayContractId);
        Result<Optional<DataContract>, String> result = dashsdk.platformMobileDataContractsFetchDataContract(
                sdk, contractId);

        Optional<DataContract> dataContractResult = result.unwrap();
        assertTrue(dataContractResult.isPresent());
        DataContract dataContract = dataContractResult.get();
        assertArrayEquals(dashPayContractId, dataContract.getId().get_0().get_0());
        assertEquals(3, dataContract.getDoc_types().size());
        dataContract.getDoc_types().forEach(System.out::println);
    }

    @Test
    public void fetchWalletUtilsDataContractTest() throws Exception {
        Identifier contractId = new Identifier(walletUtilsContracId);
        Result<Optional<DataContract>, String> result = dashsdk.platformMobileDataContractsFetchDataContract(
                sdk, contractId);

        Optional<DataContract> dataContractResult = result.unwrap();
        assertTrue(dataContractResult.isPresent());
        DataContract dataContract = dataContractResult.get();
        assertArrayEquals(walletUtilsContracId, dataContract.getId().get_0().get_0());
        assertEquals(1, dataContract.getDoc_types().size());
        assertTrue("didn't find txMetadata", dataContract.getDoc_types().stream().allMatch(type -> type.equals("txMetadata")));
        dataContract.getDoc_types().forEach(System.out::println);
    }

    @Test
    public void fetchDataContractFailureTest() throws Exception {
        Identifier contractId = new Identifier(identifier);
        Result<Optional<DataContract>, String> result = dashsdk.platformMobileDataContractsFetchDataContract(
                sdk, contractId);
        System.out.println("missing contract request: " + result.toString());
    }
}
