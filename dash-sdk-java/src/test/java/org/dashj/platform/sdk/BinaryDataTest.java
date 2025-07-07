package org.dashj.platform.sdk;

import org.junit.Test;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

public class BinaryDataTest extends BaseTest {
    @Test
    public void binaryDataTest() {
        BinaryData data = new BinaryData(identifier);
        assertTrue(data.swigCMemOwn);
        assertEquals(32, data.get_0().length);
        assertArrayEquals(identifier, data.get_0());
        data.delete();
    }

    // if the object is created in Java, then the Java object does own it
    // delete will call the destructor
    @Test
    public void createBinaryDataInJavaAndDestroyTest() {
        System.out.println("Java-------");
        BinaryData data = new BinaryData(identifier);
        assertEquals(32, data.get_0().length);
        assertArrayEquals(identifier, data.get_0());
        System.out.printf("BinaryData %x\n", BinaryData.getCPtr(data));
        data.delete();
    }
}
