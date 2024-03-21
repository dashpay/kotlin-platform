package org.dashj.platform.sdk;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class BinaryDataTest extends BaseTest {
    @Test
    public void binaryDataTest() {
        BinaryData data = new BinaryData(identifier);
        assertTrue(data.swigCMemOwn);
        assertEquals(32, data.get_0().length);
        assertArrayEquals(identifier, data.get_0());
        data.delete(); // does not crash
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
        data.delete(); // does not crash
    }

    // if the object is created in rust, then the Java object does not own it
    // .delete will not call the destructor
    @Test
    public void createBinaryDataInRustAndDestroy() {
        System.out.println("Rust-------");
        BinaryData data = example.platformValueTypesBinaryDataBinaryDataCtor(identifier);
        assertFalse(data.swigCMemOwn);
        assertEquals(32, data.get_0().length);
        assertArrayEquals(identifier, data.get_0());
        System.out.printf("BinaryData %s %x\n", data.swigCMemOwn ? "owns" : "doesn't own", BinaryData.getCPtr(data));
        example.platformValueTypesBinaryDataBinaryDataDestroy(data); //does not crash
    }
}
