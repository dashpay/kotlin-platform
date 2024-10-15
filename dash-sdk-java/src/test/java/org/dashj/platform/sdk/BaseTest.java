package org.dashj.platform.sdk;

import org.junit.AfterClass;
import org.junit.BeforeClass;

public class BaseTest {
    static {
        System.loadLibrary("sdklib");
    }

    static byte [] publicKey = new byte[] { 0x02, 0x9, 0xA, 0xB, 0, 0, 0, 0, 0, 0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0 };

    static byte [] identifier = new byte[] { 0xA, 0xB, 0xC, 0, 0, 0, 0, 0, 0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0,0, 0, 0, 0 };
    static byte [] contractIdentifier = new byte[] { 48, 18, (byte) 193, (byte) 155, (byte) 152, (byte) 236, 0, 51,
            (byte) 173, (byte) 219, 54, (byte) 205, 100, (byte) 183, (byte) 245, 16, 103, 15, 42, 53, 26,
            67, 4, (byte) 181, (byte) 246, (byte) 153, 65, 68, 40, 110, (byte) 253, (byte) 172 };

    static byte [] dpnsContractId = new byte [] {
            (byte) 230, 104, (byte) 198, 89, (byte) 175, 102, (byte) 174, (byte) 225, (byte) 231, 44, 24, 109, (byte) 222,
            123, 91, 126, 10, 29, 113, 42, 9, (byte) 196, 13, 87, 33, (byte) 246, 34,
            (byte) 191, 83, (byte) 197, 49, 85
    };

    static byte[] dashPayContractId = new byte[] {
            (byte) 162, (byte)161, (byte)180, (byte)172, 111,(byte) 239, 34, (byte)234, 42, 26, 104, (byte)232, 18, 54, 68, (byte)179, 87, (byte)135, 95, 107, 65,
            44, 24, 16, (byte)146, (byte)129, (byte)193, 70, (byte)231, (byte)178, 113, (byte)188,
    };


    static MemoryFactory memoryFactory = MemoryFactory.getInstance();

    @BeforeClass
    public static void start() {

    }

    @AfterClass
    public static void end() {
        System.out.printf("objects remaining: %d\n", memoryFactory.size());
    }
}
