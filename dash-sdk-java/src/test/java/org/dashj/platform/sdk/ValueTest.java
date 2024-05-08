package org.dashj.platform.sdk;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class ValueTest extends BaseTest {
    @Test
    public void getValueTest() {
        platform_mobile_MyValue value = dashsdk.platformMobileGetValue();
        assertEquals(value.getTag(), platform_mobile_MyValue.Tag.Bool);

        assertEquals(false, value.getBool_().get_0());
        value.delete();
    }

    @Test
    public void getValueMapTest() {
        platform_mobile_MyValueMap valueMap = dashsdk.platformMobileGetValueMap();
        Vec_Tuple_platform_mobile_MyValue_platform_mobile_MyValue tuple = valueMap.get_0();

        valueMap.delete();
    }

    @Test
    public void getValueWithMapTest() {
        platform_mobile_MyValue value = dashsdk.platformMobileGetValueWithMap();
        assertEquals(value.getTag(), platform_mobile_MyValue.Tag.Map);
        platform_mobile_MyValueMap valueMap = value.getMap().get_0();
        Vec_Tuple_platform_mobile_MyValue_platform_mobile_MyValue tuple = valueMap.get_0();

        value.delete();
    }
}
