package org.dashj.platform.sdk;

import org.junit.Test;
import java.math.BigInteger;
import java.util.Collections;
import org.dashj.platform.sdk.callbacks.ContextProvider;
import static org.junit.Assert.*;

public class NetworkConfigurationTest extends BaseTest {
    @Test
    public void createsAndDestroysExplicitNetworkClientsAcrossJni() throws Exception {
        try (ContextProvider provider = new ContextProvider() {
            @Override
            public byte[] getQuorumPublicKey(int type, byte[] hash, int height) {
                throw new AssertionError("Construction must not fetch quorum keys");
            }
            @Override
            public byte[] getDataContract(Identifier identifier) {
                throw new AssertionError("Construction must not fetch contracts");
            }
        }) {
            for (String network : new String[]{"mainnet", "testnet", "devnet-sakura"}) {
                // A truncated one-digit port becomes invalid, exposing JNI string-copy regressions.
                for (String endpoint : new String[]{"https://127.0.0.1:1", "https://127.0.0.1:443", "https://127.0.0.1:1443"}) {
                    SWIGTYPE_p_DashSdk sdk = dashsdk.platformMobileSdkCreateDashSdkWithNetwork(
                            provider.getNativeContext(), BigInteger.valueOf(provider.getQuorumPublicKeyCallback()),
                            BigInteger.ZERO, network, Collections.singletonList(endpoint), 5, 5, 1);
                    assertNotNull(sdk);
                    try {
                        assertEquals(network.equals("devnet-sakura") ? 14L : 13L,
                                dashsdk.platformMobileSdkGetProtocolVersionWithSdk(sdk).unwrap().longValue());
                    } finally {
                        dashsdk.platformMobileSdkDestroyDashSdk(sdk);
                    }
                }
            }
        }
    }

    @Test(expected = IllegalArgumentException.class)
    public void rejectsEmptyEndpointsAcrossJni() {
        dashsdk.platformMobileSdkCreateDashSdkWithNetwork(0, BigInteger.ZERO, BigInteger.ZERO,
                "devnet-sakura", Collections.emptyList(), 5, 5, 1);
    }

    @Test(expected = IllegalArgumentException.class)
    public void rejectsInvalidNetworkAcrossJni() {
        dashsdk.platformMobileSdkCreateDashSdkWithNetwork(0, BigInteger.ZERO, BigInteger.ZERO,
                "sakura", Collections.singletonList("https://127.0.0.1:1443"), 5, 5, 1);
    }

    @Test(expected = IllegalArgumentException.class)
    public void rejectsMissingQuorumCallbackAcrossJni() {
        dashsdk.platformMobileSdkCreateDashSdkWithNetwork(0, BigInteger.ZERO, BigInteger.ZERO,
                "devnet-sakura", Collections.singletonList("https://127.0.0.1:1443"), 5, 5, 1);
    }
}
