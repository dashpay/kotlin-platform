package org.dashj.platform.sdk;

import org.junit.jupiter.api.Test;

import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;

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
}
