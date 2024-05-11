package org.dashj.platform.sdk;


import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class ValueTest extends BaseTest {
    @Test
    public void getMyValueTest() {
        platform_mobile_MyValue value = dashsdk.platformMobileGetValue();
        assertEquals(value.getTag(), platform_mobile_MyValue.Tag.Bool);

        assertEquals(false, value.getBool_().get_0());
        value.delete();
    }

    @Test
    public void getMyValueMapTest() {
        platform_mobile_MyValueMap valueMap = dashsdk.platformMobileGetValueMap();
        Vec_Tuple_platform_mobile_MyValue_platform_mobile_MyValue tuple = valueMap.get_0();

        valueMap.delete();
    }

    @Test
    public void getMyValueWithMapTest() {
        platform_mobile_MyValue value = dashsdk.platformMobileGetValueWithMap();
        assertEquals(value.getTag(), platform_mobile_MyValue.Tag.Map);
        platform_mobile_MyValueMap valueMap = value.getMap().get_0();
        Vec_Tuple_platform_mobile_MyValue_platform_mobile_MyValue tuple = valueMap.get_0();

        value.delete();
    }

    public void getValueTest() {
        platform_mobile_MyValue value = dashsdk.platformMobileGetValueWithMap();
        assertEquals(value.getTag(), platform_mobile_MyValue.Tag.Bool);

        assertEquals(false, value.getBool_().get_0());
        value.delete();
    }

    @Test
    public void getPlatformValueTest() {
        PlatformValue value = dashsdk.platformMobileGetPlatformValue();
        assertEquals(value.getTag(), PlatformValue.Tag.Bool);

        value.delete();
    }

    @Test
    public void getPlatformValueMapTest() {
        PlatformValue value = dashsdk.platformMobileGetPlatformValueWithMap();
        assertEquals(value.getTag(), PlatformValue.Tag.Map);
        PlatformValueMap valueMap = value.getMap().get_0();
        Map<PlatformValue, PlatformValue> map = valueMap.get_0();
        assertNotNull(map);
        assertEquals(1, map.size());
        map.forEach((k, v) -> {
            assertEquals(PlatformValue.Tag.Text, k.getTag());
            assertEquals(PlatformValue.Tag.I32, v.getTag());
        });

        value.delete();
    }

    @Test
    public void createPlatformValueTest() {
        PlatformValue v1 = new PlatformValue((byte)1);
        assertEquals(PlatformValue.Tag.I8, v1.getTag());
        assertEquals((byte) 1, v1.getI8().get_0());
        //PlatformValue v2 = new PlatformValue((short)1);
        //assertEquals(PlatformValue.Tag.I16, v2.getTag());
        PlatformValue v3 = new PlatformValue(1);
        assertEquals(PlatformValue.Tag.I32, v3.getTag());
        assertEquals(1, v3.getI32().get_0());

        PlatformValue v4 = new PlatformValue(1L);
        assertEquals(PlatformValue.Tag.I64, v4.getTag());
        assertEquals(1L, v4.getI64().get_0());

        PlatformValue v5 = new PlatformValue("text");
        assertEquals(PlatformValue.Tag.Text, v5.getTag());
        assertEquals("text", v5.getText().get_0());

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
        PlatformValueMap valueMap1 = value1.getMap().get_0();
        Map<PlatformValue, PlatformValue> map1 = valueMap1.get_0();
        map1.forEach((k, v) -> {
            System.out.printf("%s:%s\n", k.getTag(), v.getTag());
            assertEquals(PlatformValue.Tag.Text, k.getTag());
            System.out.printf("%s:", k.getText().get_0());
            if (v.getTag() == PlatformValue.Tag.Text) {
                System.out.printf("%s", v.getText().get_0());
            } else if (v.getTag() == PlatformValue.Tag.Array) {
                System.out.print("[");
                v.getArray().get_0().forEach(item -> {
                    System.out.printf("%s,", item.getText().get_0());
                });
                System.out.print("]");
            }
            System.out.println();
        });
        assertNotNull(map1.get(key));
        assertEquals(value, map1.get(key));
        assertNotEquals(value.getCPointer(), map1.get(key).getCPointer(), "objects should not have same c pointer");
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

        listValues.getArray().get_0().forEach(v -> {
            System.out.printf("tag: %s\n", v.getTag());
            if (v.getTag() == PlatformValue.Tag.Text) {
                System.out.printf("%s", v.getText().get_0());
            } else if (v.getTag() == PlatformValue.Tag.Array) {
                System.out.print("[");
                value.getArray().get_0().forEach(item -> {
                    System.out.printf("%s,", item.getText().get_0());
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
        testValue(true, PlatformValue.Tag.Bool, () -> new PlatformValue(true), v -> v.getBool_().get_0());
//        testValue(128, PlatformValue.Tag.U8, () -> new PlatformValue((byte)128), v -> v.getU8().get_0());
        testValue((byte)1, PlatformValue.Tag.I8, () -> new PlatformValue((byte)1), v -> v.getI8().get_0());
//        testValue(Short.MAX_VALUE + 1, PlatformValue.Tag.U16, () -> new PlatformValue(true), v -> v.getU16().get_0());
//        testValue(Integer.MAX_VALUE + 1, PlatformValue.Tag.U32, () -> new PlatformValue(true), v -> v.getU32().get_0());
//        testValue(Long.MAX_VALUE + 1, PlatformValue.Tag.U64, () -> new PlatformValue(true), v -> v.getU64().get_0());
//        testValue(Long.MAX_VALUE + 1, PlatformValue.Tag.U128, () -> new PlatformValue(true), v -> v.getU128().get_0());
        testValue(Short.MAX_VALUE, PlatformValue.Tag.I16, () -> new PlatformValue(Short.MAX_VALUE), v -> v.getI16().get_0());
        testValue(Integer.MAX_VALUE, PlatformValue.Tag.I32, () -> new PlatformValue(Integer.MAX_VALUE), v -> v.getI32().get_0());
        testValue(Long.MAX_VALUE, PlatformValue.Tag.I64, () -> new PlatformValue(Long.MAX_VALUE), v -> v.getI64().get_0());
//        testValue(Long.MAX_VALUE, PlatformValue.Tag.I128, () -> new PlatformValue(Long.MAX_VALUE), v -> v.getI128().get_0());
        testValue(Double.MAX_VALUE, PlatformValue.Tag.Float, () -> new PlatformValue(Double.MAX_VALUE), v -> v.getFloat_().get_0());
        testValue("text", PlatformValue.Tag.Text, () -> new PlatformValue("text"), v -> v.getText().get_0());
    }

    @Test
    public void createPlatformValueVectorsTest() {
        testArrayValue(identifier, PlatformValue.Tag.Bytes, () -> new PlatformValue(identifier, false), v -> v.getBytes().get_0());
        byte [] hash160 = Arrays.copyOfRange(identifier, 0, 20);
        testArrayValue(hash160, PlatformValue.Tag.Bytes20, () -> new PlatformValue(hash160, true), v -> v.getBytes().get_0());
        testArrayValue(identifier, PlatformValue.Tag.Bytes32, () -> new PlatformValue(identifier, true), v -> v.getBytes().get_0());
        byte [] outpointBytes = Arrays.copyOf(identifier, 36);
        testArrayValue(outpointBytes, PlatformValue.Tag.Bytes36, () -> new PlatformValue(outpointBytes, true), v -> v.getBytes().get_0());
        byte [] bytes = Arrays.copyOfRange(identifier, 0, 16);
        testArrayValue(bytes, PlatformValue.Tag.Bytes, () -> new PlatformValue(bytes, true), v -> v.getBytes().get_0());
    }
}
