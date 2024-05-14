package org.dashj.platform.sdk;

import org.junit.jupiter.api.Test;

import java.math.BigInteger;
import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class DocumentTest extends BaseTest {
    @Test
    public void getDocumentTest() {
        Document value = dashsdk.platformMobileFetchDocumentGetDocument();
        assertEquals(Document.Tag.V0, value.getTag());

        DocumentV0 doc = value.getV0().get_0();
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
                str = String.valueOf(v.getBool());
            } else if (t == PlatformValue.Tag.Text) {
                str = v.getText();
            } else if (t == PlatformValue.Tag.Map) {
                str = v.getMap().get_0().toString();
            } else {
                str = "unknown";
            }
            System.out.printf("%s\n", str);
        });
        System.out.printf("to_string: %s\n", dashsdk.platformMobileFetchDocumentDocumentToString(value));
        value.delete();
    }

    @Test
    public void getDocumentsTest() {
        Identifier domainId = new Identifier(dpnsContractId);
        List<Document> docs = dashsdk.platformMobileFetchDocumentGetDocuments(domainId, "domain", BigInteger.ZERO, BigInteger.ZERO);
        assertTrue(!docs.isEmpty());
    }

    @Test
    public void documentListTest() {
        // don't have a way to create a document yet
    }
}
