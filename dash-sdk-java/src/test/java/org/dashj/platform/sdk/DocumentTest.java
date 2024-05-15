package org.dashj.platform.sdk;

import org.junit.jupiter.api.Test;

import java.math.BigInteger;
import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class DocumentTest extends BaseTest {
    @Test
    public void getDocumentTest() {
        Document document = dashsdk.platformMobileFetchDocumentGetDocument();
        assertEquals(Document.Tag.V0, document.getTag());

        DocumentV0 doc = document.getV0().get_0();
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
        System.out.printf("to_string: %s\n", dashsdk.platformMobileFetchDocumentDocumentToString(document));
        document.delete();
    }

    @Test
    public void getDocumentsTest() {
        Identifier domainId = new Identifier(dpnsContractId);
        List<Document> docs = dashsdk.platformMobileFetchDocumentGetDocuments(domainId, "domain", BigInteger.ZERO, BigInteger.ZERO);
        assertTrue(!docs.isEmpty());
    }

    @Test
    public void getDocumentMapTest() {
        Identifier domainId = new Identifier(dpnsContractId);
        List<Document> docs = dashsdk.platformMobileFetchDocumentGetDocuments(domainId, "domain", BigInteger.ZERO, BigInteger.ZERO);
        assertTrue(!docs.isEmpty());
    }


    @Test
    public void documentListTest() {
        // don't have a way to create a document yet
    }

    @Test
    public void getDocumentWithIDTest() {
        Identifier domainId = new Identifier(dpnsContractId);
        List<Document> all = dashsdk.platformMobileFetchDocumentGetDocuments(domainId, "domain", BigInteger.ZERO, BigInteger.ZERO);
        Identifier id = null;
        for (Document d : all) {
            PlatformValue value = d.getV0().get_0().getProperties().get("records").getMap().get_0().get(new PlatformValue("dashUniqueIdentityId"));
            if (value != null)
                id = new Identifier(value.getIdentifier().getBytes());
        }
        assertNotNull(id);
        List<Document> docs = dashsdk.platformMobileFetchDocumentGetDomainDocument(id, BigInteger.ZERO, BigInteger.ZERO);
        assertFalse(docs.isEmpty());
    }

    @Test
    public void getDocumentStartsWithTest() {
        List<Document> docs = dashsdk.platformMobileFetchDocumentGetDomainDocumentStartsWith("dq-", BigInteger.ZERO, BigInteger.ZERO);
        assertFalse(docs.isEmpty());
        docs.forEach(document -> {
            Map<String, PlatformValue> props = document.getV0().get_0().getProperties();
            System.out.println(props.get("label").getText());
        });
    }
}
