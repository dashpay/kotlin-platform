package org.dashj.platform.sdk;


import org.junit.Test;

import java.math.BigInteger;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertNotEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertThrows;


public class ValueTest extends BaseTest {

    @Test
    public void createPlatformValueTest() {
        PlatformValue v1 = new PlatformValue((byte)1);
        assertEquals(PlatformValue.Tag.I8, v1.getTag());
        assertEquals((byte) 1, v1.getI8());
        //PlatformValue v2 = new PlatformValue((short)1);
        //assertEquals(PlatformValue.Tag.I16, v2.getTag());
        PlatformValue v3 = new PlatformValue(1);
        assertEquals(PlatformValue.Tag.I32, v3.getTag());
        assertEquals(1, v3.getI32());

        PlatformValue v4 = new PlatformValue(1L);
        assertEquals(PlatformValue.Tag.I64, v4.getTag());
        assertEquals(1L, v4.getI64());

        PlatformValue v5 = new PlatformValue("text");
        assertEquals(PlatformValue.Tag.Text, v5.getTag());
        assertEquals("text", v5.getText());

        PlatformValue v6 = new PlatformValue();
        assertEquals(PlatformValue.Tag.Null, v6.getTag());
        v1.delete();
        //v2.delete();
        v3.delete();
        v4.delete();
        v5.delete();
        v6.delete();
    }

    @Test
    public void createPlatformValueMapTest() {
        PlatformValue key = new PlatformValue("key");
        PlatformValue listKey = new PlatformValue("listKey");
        PlatformValue value = new PlatformValue("value");
        HashMap<PlatformValue, PlatformValue> kvMap = new HashMap<>();
        kvMap.put(key, value);
        ArrayList<PlatformValue> list = new ArrayList<>();
        list.add(key);
        list.add(value);
        PlatformValue listValue = new PlatformValue(list);
        kvMap.put(listKey, listValue);

        HashMap<String, PlatformValue> javaValueMap = new HashMap<>();
        javaValueMap.put("map", new PlatformValue(new PlatformValueMap(kvMap)));
        assertEquals(PlatformValue.Tag.Map, javaValueMap.get("map").getTag());
        PlatformValue value1 = javaValueMap.get("map");
        PlatformValueMap valueMap1 = value1.getMap();
        Map<PlatformValue, PlatformValue> map1 = valueMap1.get_0();
        map1.forEach((k, v) -> {
            System.out.printf("%s:%s\n", k.getTag(), v.getTag());
            assertEquals(PlatformValue.Tag.Text, k.getTag());
            System.out.printf("%s:", k.getText());
            if (v.getTag() == PlatformValue.Tag.Text) {
                System.out.printf("%s", v.getText());
            } else if (v.getTag() == PlatformValue.Tag.Array) {
                System.out.print("[");
                v.getArray().forEach(item -> {
                    System.out.printf("%s,", item.getText());
                });
                System.out.print("]");
            }
            System.out.println();
        });
        assertNotNull(map1.get(key));
        assertEquals(value, map1.get(key));
        assertNotEquals(value.getCPointer(), map1.get(key).getCPointer());
    }

    @Test
    public void createPlatformValueArrayTest() {
        PlatformValue key = new PlatformValue("key");
        PlatformValue listKey = new PlatformValue("listKey");
        PlatformValue value = new PlatformValue("value");
        ArrayList<PlatformValue> list = new ArrayList<>();
        list.add(key);
        list.add(value);
        PlatformValue listValues = new PlatformValue(list);

        listValues.getArray().forEach(v -> {
            System.out.printf("tag: %s\n", v.getTag());
            if (v.getTag() == PlatformValue.Tag.Text) {
                System.out.printf("%s", v.getText());
            } else if (v.getTag() == PlatformValue.Tag.Array) {
                System.out.print("[");
                value.getArray().forEach(item -> {
                    System.out.printf("%s,", item.getText());
                });
                System.out.print("[");
            }
            System.out.println();
        });
    }

    interface MyTest<T> {
        T get(PlatformValue value);
    }
    interface MyArrayTest<T> {
        byte[] get(PlatformValue value);
    }
    interface Construct {
        PlatformValue construct();
    }

    public static <T> void testValue(T value, PlatformValue.Tag tag, Construct constructor, MyTest<T> tester) {
        PlatformValue platformValue = constructor.construct();
        assertEquals(tag, platformValue.getTag());
        assertEquals(value, tester.get(platformValue));
        platformValue.delete();
    }

    public static <T> void testArrayValue(byte[] value, PlatformValue.Tag tag, Construct constructor, MyArrayTest<T> tester) {
        PlatformValue platformValue = constructor.construct();
        assertEquals(tag, platformValue.getTag());
        assertArrayEquals(value, tester.get(platformValue));
        platformValue.delete();
    }

    @Test
    public void createPlatformValuePrimitiveTest() {
        testValue(true, PlatformValue.Tag.Bool, () -> new PlatformValue(true), PlatformValue::getBool);
        BigInteger i128Value = BigInteger.valueOf(Long.MAX_VALUE).multiply(BigInteger.ONE);
        // testValue(i128Value, PlatformValue.Tag.I128, () -> new PlatformValue(i128Value), PlatformValue::getI128);
        testValue((byte)1, PlatformValue.Tag.I8, () -> new PlatformValue((byte)1), PlatformValue::getI8);
//        testValue(Short.MAX_VALUE + 1, PlatformValue.Tag.U16, () -> new PlatformValue(true), v -> v.getU16().get_0());
//        testValue(Integer.MAX_VALUE + 1, PlatformValue.Tag.U32, () -> new PlatformValue(true), v -> v.getU32().get_0());
//        testValue(Long.MAX_VALUE + 1, PlatformValue.Tag.U64, () -> new PlatformValue(true), v -> v.getU64().get_0());
//        testValue(Long.MAX_VALUE + 1, PlatformValue.Tag.U128, () -> new PlatformValue(true), v -> v.getU128().get_0());
        testValue(Short.MAX_VALUE, PlatformValue.Tag.I16, () -> new PlatformValue(Short.MAX_VALUE), PlatformValue::getI16);
        testValue(Integer.MAX_VALUE, PlatformValue.Tag.I32, () -> new PlatformValue(Integer.MAX_VALUE), PlatformValue::getI32);
        testValue(Long.MAX_VALUE, PlatformValue.Tag.I64, () -> new PlatformValue(Long.MAX_VALUE), PlatformValue::getI64);
//        testValue(Long.MAX_VALUE, PlatformValue.Tag.I128, () -> new PlatformValue(Long.MAX_VALUE), v -> v.getI128().get_0());
        testValue(Double.MAX_VALUE, PlatformValue.Tag.Float, () -> new PlatformValue(Double.MAX_VALUE), PlatformValue::getFloat);
        testValue("text", PlatformValue.Tag.Text, () -> new PlatformValue("text"), PlatformValue::getText);
    }

    @Test
    public void createPlatformValueVectorsTest() {
        testArrayValue(identifier, PlatformValue.Tag.Bytes, () -> new PlatformValue(identifier, false), PlatformValue::getBytes);
        byte [] hash160 = Arrays.copyOfRange(identifier, 0, 20);
        testArrayValue(hash160, PlatformValue.Tag.Bytes20, () -> new PlatformValue(hash160, true), PlatformValue::getBytes20);
        testArrayValue(identifier, PlatformValue.Tag.Bytes32, () -> new PlatformValue(identifier, true), PlatformValue::getBytes32);
        byte [] outpointBytes = Arrays.copyOf(identifier, 36);
        testArrayValue(outpointBytes, PlatformValue.Tag.Bytes36, () -> new PlatformValue(outpointBytes, true), PlatformValue::getBytes36);
        byte [] bytes = Arrays.copyOfRange(identifier, 0, 16);
        testArrayValue(bytes, PlatformValue.Tag.Bytes, () -> new PlatformValue(bytes, true), PlatformValue::getBytes);
        testArrayValue(identifier, PlatformValue.Tag.Identifier, () -> new PlatformValue(new Hash256(identifier)), v -> v.getIdentifier().getBytes());
    }

    @Test
    public void createPlatformValueVectorsTestTwo() {
        PlatformValue bytes20 = new PlatformValue(identifier, true);
        assertEquals(PlatformValue.Tag.Bytes32, bytes20.getTag());
        bytes20.delete();
    }

    @Test
    public void accessInvalidTypeTest() {
        PlatformValue platformValue = new PlatformValue("text");
        platformValue.getText(); // should not throw an exception
        // assertThrows(IllegalArgumentException.class, platformValue::getI128);
        assertThrows(IllegalArgumentException.class, platformValue::getI64);
        assertThrows(IllegalArgumentException.class, platformValue::getI32);
        assertThrows(IllegalArgumentException.class, platformValue::getI8);
        assertThrows(IllegalArgumentException.class, platformValue::getFloat);
        assertThrows(IllegalArgumentException.class, platformValue::getArray);
        assertThrows(IllegalArgumentException.class, platformValue::getMap);
        assertThrows(IllegalArgumentException.class, platformValue::getBytes);
        assertThrows(IllegalArgumentException.class, platformValue::getBytes20);
        assertThrows(IllegalArgumentException.class, platformValue::getBytes32);
        assertThrows(IllegalArgumentException.class, platformValue::getBytes36);
        assertThrows(IllegalArgumentException.class, platformValue::getIdentifier);
        assertThrows(IllegalArgumentException.class, platformValue::getEnumString);
        assertThrows(IllegalArgumentException.class, platformValue::getArray);
        assertThrows(IllegalArgumentException.class, platformValue::getMap);
    }
}
