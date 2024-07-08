package org.dashj.platform.sdk;

import org.dashj.platform.sdk.base.Result;
import org.junit.jupiter.api.Test;

import java.math.BigInteger;
import java.util.ArrayList;
import java.util.HashMap;
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

    @Test
    public void createDocumentTest() {
        Identifier docId = new Identifier(identifier);
        Identifier ownerId = new Identifier(identifier);
        Revision revision = new Revision(2);

        HashMap<String, PlatformValue> map = new HashMap<>();
        map.put("text", new PlatformValue("value"));
        DocumentV0 documentV0 = new DocumentV0(
                docId, ownerId, map, revision
        );
        Document document = new Document(documentV0);
        assertEquals(Document.Tag.V0, document.getTag());
        DocumentV0 docv0 = document.getV0().get_0();
        assertEquals(docId, docv0.getId());
        assertEquals(ownerId, docv0.getOwner_id());
        assertEquals(new Revision(2), docv0.getRevision());
        Map<String, PlatformValue> properties = docv0.getProperties();
        assertEquals(1, properties.size());
        assertEquals("text", properties.keySet().stream().findFirst().get());
        assertEquals("value", properties.values().stream().findFirst().get().getText());
        document.delete();
        document.delete();
        docId.delete();
        ownerId.delete();
    }

    @Test
    public void queryTest() {
        try {
            Identifier dpnsId = new Identifier(dpnsContractId);
            ArrayList<WhereClause> where = new ArrayList<>();
            ArrayList<OrderClause> orderBy = new ArrayList<>();
            Result<List<Document>, String> docsResult =
                    dashsdk.platformMobileFetchDocumentFetchDocumentsWithQuery(
                    dpnsId,
                    "domain",
                    where,
                    orderBy,
                    5,
                    null,
                    BigInteger.ZERO,
                    BigInteger.ZERO
            );
            docsResult.unwrap().forEach(document -> {
                Map<String, PlatformValue> props = document.getV0().get_0().getProperties();
                System.out.println(props.get("label").getText());
            });
            where.add(new WhereClause("normalizedLabel", WhereOperator.StartsWith, new PlatformValue("bob")));
            where.add(new WhereClause("normalizedParentDomainName", WhereOperator.Equal, new PlatformValue("dash")));
            orderBy.add(new OrderClause("normalizedLabel"));
            Result<List<Document>, String> docs2 = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQuery(
                    dpnsId,
                    "domain",
                    where,
                    orderBy,
                    100,
                    null,
                    BigInteger.ZERO,
                    BigInteger.ZERO
            );
            System.out.println("------ query ------");
            docs2.unwrap().forEach(document -> {
                Map<String, PlatformValue> props = document.getV0().get_0().getProperties();
                System.out.println(props.get("label").getText());
            });
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    @Test
    public void queryTestFail() {
        try {
            Identifier dpnsId = new Identifier(dpnsContractId);
            ArrayList<WhereClause> where = new ArrayList<>();
            ArrayList<OrderClause> orderBy = new ArrayList<>();
            Result<List<Document>, String> docsResult =
                    dashsdk.platformMobileFetchDocumentFetchDocumentsWithQuery(
                            dpnsId,
                            "domain",
                            where,
                            orderBy,
                            5,
                            null,
                            BigInteger.ZERO,
                            BigInteger.ZERO
                    );
            docsResult.unwrap().forEach(document -> {
                Map<String, PlatformValue> props = document.getV0().get_0().getProperties();
                System.out.println(props.get("label").getText());
            });
            where.add(new WhereClause("normalizedLabel", WhereOperator.StartsWith, new PlatformValue("zys-aaa")));
            where.add(new WhereClause("normalizedParentDomainName", WhereOperator.Equal, new PlatformValue("dash")));
            orderBy.add(new OrderClause("normalizedLabel"));
            Result<List<Document>, String> docs2 = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQuery(
                    dpnsId,
                    "domain",
                    where,
                    orderBy,
                    100,
                    null,
                    BigInteger.ZERO,
                    BigInteger.ZERO
            );
            System.out.println("------ query ------");
            docs2.unwrap().forEach(document -> {
                Map<String, PlatformValue> props = document.getV0().get_0().getProperties();
                System.out.println(props.get("label").getText());
            });
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    @Test
    public void startsWithQueryTest() {
        try {
            Identifier dpnsId = new Identifier(dpnsContractId);
            ArrayList<WhereClause> where = new ArrayList<>();
            ArrayList<OrderClause> orderBy = new ArrayList<>();

            where.add(new WhereClause("normalizedLabel", WhereOperator.StartsWith, new PlatformValue("dq-t")));
            where.add(new WhereClause("normalizedParentDomainName", WhereOperator.Equal, new PlatformValue("dash")));
            orderBy.add(new OrderClause("normalizedLabel"));
            Result<List<Document>, String> docs2 = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQuery(
                    dpnsId,
                    "domain",
                    where,
                    orderBy,
                    100,
                    null,
                    BigInteger.ZERO,
                    BigInteger.ZERO
            );
            List<Document> documents = docs2.unwrap();
            assertFalse(documents.isEmpty());
            documents.forEach(document -> {
                Map<String, PlatformValue> props = document.getV0().get_0().getProperties();
                System.out.println(props.get("label").getText());
            });
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    @Test
    public void createSDKTest() throws Exception {
        SWIGTYPE_p_RustSdk sdk = dashsdk.platformMobileConfigCreateSdk(BigInteger.ZERO, BigInteger.ZERO);
        ArrayList list = new ArrayList();
        Result<List<Document>, String> result = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                sdk, new Identifier(dpnsContractId),
                "domain", list, list, 100, null);
        List<Document> documentList = result.unwrap();
        documentList.forEach(doc -> {
            System.out.println(doc.getV0().get_0().getId().getBytes().length);
        });
        //dashsdk.platformMobileConfigRustSdkDestroy(sdk);
    }
}
