package org.dashj.platform.sdk;

import org.junit.jupiter.api.Test;

import java.util.HashMap;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;

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
        PlatformValue value = new PlatformValue("value");
        HashMap<PlatformValue, PlatformValue> kvMap = new HashMap<>();
        kvMap.put(key, value);

        HashMap<String, PlatformValue> javaValueMap = new HashMap<>();
        javaValueMap.put("map", new PlatformValue(new PlatformValueMap(kvMap)));
        assertEquals(PlatformValue.Tag.Map, javaValueMap.get("map").getTag());
        PlatformValue value1 = javaValueMap.get("map");
        PlatformValueMap valueMap1 = value1.getMap().get_0();
        Map<PlatformValue, PlatformValue> map1 = valueMap1.get_0();
        map1.forEach((k, v) -> {
            System.out.printf("%s:%s\n", k.getTag(), v.getTag());
            System.out.printf("%s:%s\n", k.getText().get_0(), v.getText().get_0());
        });
        assertNotNull(map1.get(key));
        assertEquals(value, map1.get(key));
        assertNotEquals(value.getCPointer(), map1.get(key).getCPointer(), "objects should not have same c pointer");
    }
}
