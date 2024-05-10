package org.dashj.platform.sdk;

import org.junit.jupiter.api.Test;

import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;

public class DocumentTest extends BaseTest {
    @Test
    public void getDocumentTest() {
        dpp_document_Document value = dashsdk.getDocument();
        assertEquals(dpp_document_Document.Tag.V0, value.getTag());

        dpp_document_v0_DocumentV0 doc = value.getV0().get_0();
        assertEquals(new Revision(1), doc.getRevision());
        //assertArrayEquals(identifier, doc.getOwner_id().get_0().get_0());
        Map<String, PlatformValue> map = doc.getProperties();
        assertEquals(PlatformValue.Tag.Text, map.get("label").getTag());
        map.forEach((key, v) -> {
            System.out.printf("%s -> %s\n", key, v);
            System.out.printf("  %s - ", v.getTag());
            String str = null;
            PlatformValue.Tag t = v.getTag();
            if (t == PlatformValue.Tag.Bool) {
                str = String.valueOf(v.getBool_().get_0());
            } else if (t == PlatformValue.Tag.Text) {
                str = v.getText().get_0();
            } else if (t == PlatformValue.Tag.Map) {
                str = v.getMap().get_0().get_0().toString();
            } else {
                str = "unknown";
            }
            System.out.printf("%s\n", str);
        });
        System.out.printf("to_string: %s\n", dashsdk.platformMobileFetchIdentityDocumentToString(value));
        value.delete();
    }
}
