package org.dashj.platform.sdk;

import org.junit.Test;
import static org.junit.Assert.*;

public class TokenPurchaseBridgeTest extends BaseTest {
    @Test
    public void rejectsMalformedOperationBytesAcrossJniWithoutNetworkAccess() {
        Identifier owner = new Identifier(identifier);
        try {
            for (int i = 0; i < 100; i++) {
                for (byte[] bytes : new byte[][]{new byte[0], new byte[]{(byte) 255}, new byte[4097]}) {
                    // Invalid payloads are rejected before an SDK handle or network is needed.
                    Result_ok_Vec_u8_err_String result = dashsdk.prepareTokenPurchaseSdk(null, bytes, owner);
                    try {
                        assertNotNull(result.getError());
                        assertNull(result.getOk());
                    } finally {
                        result.delete();
                    }
                }
            }
        } finally {
            owner.delete();
        }
    }
}
