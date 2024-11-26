package org.dashj.platform.sdk;

import org.bitcoinj.core.Base58;
import org.dashj.platform.sdk.base.Result;
import org.junit.Test;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.fail;

public class DocumentTest extends SdkBaseTest {

//    static SWIGTYPE_p_DashSdk sdk;
//
//    @BeforeClass
//    public static void startUp() {
//        sdk = dashsdk.platformMobileSdkCreateDashSdk(BigInteger.ZERO, BigInteger.ZERO, true);
//    }
//
//    @AfterClass
//    public static void tearDown() {
//        dashsdk.platformMobileSdkDestroyDashSdk(sdk);
//    }


//    @Test
//    public void getDocumentTest() {
//        SWIGTYPE_p_DashSdk sdk = dashsdk.platformMobileSdkCreateDashSdk(BigInteger.ZERO, BigInteger.ZERO, true);
//        Document document = dashsdk.platformMobileFetchDocumentGetDocument();
//        assertEquals(Document.Tag.V0, document.getTag());
//
//        DocumentV0 doc = document.getV0().get_0();
//        assertEquals(new Revision(1), doc.getRevision());
//        //assertArrayEquals(identifier, doc.getOwner_id().get_0().get_0());
//        Map<String, PlatformValue> map = doc.getProperties();
//        assertEquals(PlatformValue.Tag.Text, map.get("label").getTag());
//        map.forEach((key, v) -> {
//            System.out.printf("%s -> %s\n", key, v);
//            System.out.printf("  %s - ", v.getTag());
//            String str = null;
//            PlatformValue.Tag t = v.getTag();
//            if (t == PlatformValue.Tag.Bool) {
//                str = String.valueOf(v.getBool());
//            } else if (t == PlatformValue.Tag.Text) {
//                str = v.getText();
//            } else if (t == PlatformValue.Tag.Map) {
//                str = v.getMap().get_0().toString();
//            } else {
//                str = "unknown";
//            }
//            System.out.printf("%s\n", str);
//        });
//        System.out.printf("to_string: %s\n", dashsdk.platformMobileFetchDocumentDocumentToString(document));
//        document.delete();
//        dashsdk.platformMobileSdkDestroyDashSdk(sdk);
//    }

    @Test
    public void getDocumentsTest() throws Exception {
        Identifier domainId = new Identifier(dpnsContractId);
        Result<List<Document>, String> docs = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                sdk,
                new Identifier(dpnsContractId),
                "domain",
                new ArrayList<>(),
                new ArrayList<>(),
                100,
                null
        );
        assertFalse(docs.unwrap().isEmpty());
    }

//    @Test
//    public void getDocumentMapTest() {
//        Identifier domainId = new Identifier(dpnsContractId);
//        List<Document> docs = dashsdk.platformMobileFetchDocumentGetDocuments(domainId, "domain", BigInteger.ZERO, BigInteger.ZERO);
//        assertFalse(docs.isEmpty());
//    }


    @Test
    public void documentListTest() {
        // don't have a way to create a document yet
    }

    @Test
    public void getDocumentWithIDTest() throws Exception {
        Identifier dpnsContractIdentifier = new Identifier(dpnsContractId);
        Result<List<Document>, String> all = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                sdk,
                dpnsContractIdentifier,
                "domain",
                new ArrayList<>(),
                new ArrayList<>(),
                100,
                null
        );
        Identifier id = null;
        for (Document d : all.unwrap()) {
            PlatformValue value = d.getV0().get_0().getProperties().get("records").getMap().get_0().get(new PlatformValue("identity"));
            if (value != null) {
                id = new Identifier(value.getIdentifier().getBytes());
                break;
            }
        }
        assertNotNull(id);
        List<WhereClause> where = new ArrayList<>();
        where.add(new WhereClause("$id", WhereOperator.Equal, new PlatformValue(new Hash256(id.getBytes()))));
        Result<List<Document>, String> result = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                sdk,
                new Identifier(dpnsContractId),
                "domain",
                where,
                new ArrayList<>(),
                100,
                null
        );
        assertFalse(result.unwrap().isEmpty());
    }

    @Test
    public void getDocumentStartsWithTest() throws Exception {
        List<WhereClause> where = new ArrayList<>();
        where.add(new WhereClause("normalizedLabel", WhereOperator.StartsWith, new PlatformValue("test")));
        List<OrderClause> order = new ArrayList<>();
        order.add(new OrderClause("normalizedLabel", true));
        Result<List<Document>, String> result = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                sdk,
                new Identifier(dpnsContractId),
                "domain",
                where,
                order,
                100,
                null
        );
        List<Document> docs = result.unwrap();
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
                    dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                            sdk,
                    dpnsId,
                    "domain",
                    where,
                    orderBy,
                    5,
                    null
            );
            docsResult.unwrap().forEach(document -> {
                Map<String, PlatformValue> props = document.getV0().get_0().getProperties();
                System.out.println(props.get("label").getText());
            });
            where.add(new WhereClause("normalizedLabel", WhereOperator.StartsWith, new PlatformValue("test")));
            where.add(new WhereClause("normalizedParentDomainName", WhereOperator.Equal, new PlatformValue("dash")));
            orderBy.add(new OrderClause("normalizedLabel"));
            Result<List<Document>, String> docs2 = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                    sdk,
                    dpnsId,
                    "domain",
                    where,
                    orderBy,
                    100,
                    null
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
                    dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                            sdk,
                            dpnsId,
                            "domain",
                            where,
                            orderBy,
                            5,
                            null
                    );
            docsResult.unwrap().forEach(document -> {
                Map<String, PlatformValue> props = document.getV0().get_0().getProperties();
                System.out.println(props.get("label").getText());
            });
            where.add(new WhereClause("normalizedLabel", WhereOperator.StartsWith, new PlatformValue("zys-aaa")));
            where.add(new WhereClause("normalizedParentDomainName", WhereOperator.Equal, new PlatformValue("dash")));
            orderBy.add(new OrderClause("normalizedLabel"));
            Result<List<Document>, String> docs2 = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                    sdk,
                    dpnsId,
                    "domain",
                    where,
                    orderBy,
                    100,
                    null
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
    public void queryAllTest() {
        try {
            Identifier dpnsId = new Identifier(dpnsContractId);
            ArrayList<WhereClause> where = new ArrayList<>();
            ArrayList<OrderClause> orderBy = new ArrayList<>();
            // where.add(new WhereClause("normalizedLabel", WhereOperator.StartsWith, new PlatformValue("test")));
            where.add(new WhereClause("normalizedParentDomainName", WhereOperator.Equal, new PlatformValue("dash")));
            orderBy.add(new OrderClause("normalizedLabel"));
            int count = 0;
            Start startAfter = null;
            do {

                Result<List<Document>, String> result = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                        sdk,
                        dpnsId,
                        "domain",
                        where,
                        orderBy,
                        100,
                        startAfter
                );
                List<Document> docs = result.unwrap();
                System.out.println("------ query ------");
                System.out.println("items: " + docs.size());
                docs.forEach(document -> {
                    Map<String, PlatformValue> props = document.getV0().get_0().getProperties();
                    System.out.println(props.get("label").getText());
                    System.out.println("  id:     :  " + Base58.encode(document.getV0().get_0().getId().getBytes()));
                    System.out.println("  identity:  " + Base58.encode(props.get("records").getMap().get_0().get(new PlatformValue("identity")).getIdentifier().getBytes()));

                });
                count = docs.size();
                if (count > 0) {
                    startAfter = new Start(docs.get(docs.size() -1).getV0().get_0().getId().getBytes(), true);
                    System.out.println("start after: " + Base58.encode(startAfter.getStart_after().get_0()));
                }
            } while (count == 100);
        } catch (Exception e) {
            e.printStackTrace();
            fail(e.getMessage());
        }
    }

    @Test
    public void startsWithQueryTest() {
        try {
            Identifier dpnsId = new Identifier(dpnsContractId);
            ArrayList<WhereClause> where = new ArrayList<>();
            ArrayList<OrderClause> orderBy = new ArrayList<>();

            where.add(new WhereClause("normalizedLabel", WhereOperator.StartsWith, new PlatformValue("tut")));
            where.add(new WhereClause("normalizedParentDomainName", WhereOperator.Equal, new PlatformValue("dash")));
            orderBy.add(new OrderClause("normalizedLabel"));
            Result<List<Document>, String> docs2 = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                    sdk,
                    dpnsId,
                    "domain",
                    where,
                    orderBy,
                    100,
                    null
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
    public void startsWithQueryWithSDKTest() {
        try {
            Identifier dpnsId = new Identifier(dpnsContractId);
            ArrayList<WhereClause> where = new ArrayList<>();
            ArrayList<OrderClause> orderBy = new ArrayList<>();

            where.add(new WhereClause("normalizedLabel", WhereOperator.StartsWith, new PlatformValue("tut")));
            where.add(new WhereClause("normalizedParentDomainName", WhereOperator.Equal, new PlatformValue("dash")));
            orderBy.add(new OrderClause("normalizedLabel"));

            Result<List<Document>, String> docs2 = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                    sdk,
                    dpnsId,
                    "domain",
                    where,
                    orderBy,
                    100,
                    null
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
        List<WhereClause> list = new ArrayList<>();
        List<OrderClause> list2 = new ArrayList<>();
        Result<List<Document>, String> result = dashsdk.platformMobileFetchDocumentFetchDocumentsWithQueryAndSdk(
                sdk, new Identifier(dpnsContractId),
                "domain", list, list2, 100, null);
        List<Document> documentList = result.unwrap();
        documentList.forEach(doc -> {
            System.out.println(doc.getV0().get_0().getId().getBytes().length);
        });
    }
}
