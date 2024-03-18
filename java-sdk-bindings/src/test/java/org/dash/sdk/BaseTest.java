package org.dash.sdk;

import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;

public class BaseTest {
    static {
        System.loadLibrary("sdklib");
    }

    static byte [] publicKey = new byte[] { 0x02, 0x9, 0xA, 0xB, 0, 0, 0, 0, 0, 0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0 };

    static byte [] identifier = new byte[] { 0xA, 0xB, 0xC, 0, 0, 0, 0, 0, 0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0 };
    static byte [] contractIdentifier = new byte[] { 48, 18, (byte) 193, (byte) 155, (byte) 152, (byte) 236, 0, 51,
            (byte) 173, (byte) 219, 54, (byte) 205, 100, (byte) 183, (byte) 245, 16, 103, 15, 42, 53, 26,
            67, 4, (byte) 181, (byte) 246, (byte) 153, 65, 68, 40, 110, (byte) 253, (byte) 172 };

    static MemoryFactory memoryFactory = MemoryFactory.getInstance();

    @BeforeAll
    public static void start() {

    }

    @AfterAll
    public static void end() {
        System.out.printf("objects remaining: %d\n", memoryFactory.size());
    }
}
