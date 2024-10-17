package org.dashj.platform.sdk;


import org.junit.AfterClass;
import org.junit.BeforeClass;
import org.junit.Test;

import java.math.BigInteger;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertNotNull;


public class CoreTest extends SdkBaseTest {

    public static byte[] hexStringToByteArray(String hex) {
        int len = hex.length();
        byte[] data = new byte[len / 2];
        for (int i = 0; i < len; i += 2) {
            data[i / 2] = (byte) ((Character.digit(hex.charAt(i), 16) << 4)
                    + Character.digit(hex.charAt(i+1), 16));
        }
        return data;
    }

    @Test
    public void getTransactionTest() {
        byte[] txid = hexStringToByteArray("13155120729d7ee473e4eb8c71abd5a70370cade586b0c34120c5e0e2c3f0e48");
        Result_ok_Vec_u8_err_String result = dashsdk.platformMobileCoreGetTransactionSdk(sdk, txid);
        assertNotNull(result);
        assertNotNull(result.getOk());

        byte[] txData = hexStringToByteArray("03000500010000000000000000000000000000000000000000000000000000000000000000ffffffff0603d502100101ffffffff0283706e04000000001976a914c69a0bda7daaae481be8def95e5f347a1d00a4b488ac89514b0d000000001976a914c69a0bda7daaae481be8def95e5f347a1d00a4b488ac00000000af0300d502100056c80d9ab42df805ea31f4c6565b0a2fea2d11067ea2b6b68e53273a6baee70269bb07810359b6c1048abc57282e8e25ef8d384d1ebd4e3edd3f265f93a485e10091a3b30a77304fc30528f01b60701ec827220319fd0628942903d9fa728165e16c8eb558a84721f2533d03fd305bfabf04f200af337746e72f8025f3ec451bcda1372da2dfd568d8101aa1a010892a6e3c33198324f502bf6a96dffc7db75f3f966fd81e58000000");
        assertArrayEquals(txData, result.getOk());
    }
}
