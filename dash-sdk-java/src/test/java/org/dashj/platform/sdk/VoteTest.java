package org.dashj.platform.sdk;

import org.bitcoinj.core.Base58;
import org.bitcoinj.core.DumpedPrivateKey;
import org.bitcoinj.core.Sha256Hash;
import org.bitcoinj.core.Utils;
import org.bitcoinj.params.TestNet3Params;
import org.dashj.platform.sdk.base.Result;
import org.dashj.platform.sdk.callbacks.Signer;
import org.jetbrains.annotations.NotNull;
import org.junit.AfterClass;
import org.junit.BeforeClass;
import org.junit.Test;

import java.io.ByteArrayOutputStream;
import java.math.BigInteger;
import java.text.DateFormat;
import java.util.ArrayList;
import java.util.Base64;
import java.util.List;
import java.util.Map;
import java.util.Optional;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertNotNull;

public class VoteTest extends SdkBaseTest {

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

    @Test
    public void getVoteContendorsTest() throws Exception {
        String name = "test100";
        ArrayList<PlatformValue> indexes = new ArrayList<>();
        indexes.add(new PlatformValue("dash"));
        indexes.add(new PlatformValue(name));
        Result<Contenders, String> result = dashsdk.platformMobileVotingGetVoteContenders(
                sdk,
                "parentNameAndLabel",
                indexes,
                "domain",
                new Identifier(dpnsContractId)
        );
        Contenders contenders = result.unwrap();
        assertNotNull(contenders);
        System.out.println("Username: " + name);
        System.out.println("Contenders: " + contenders.getContenders().size());
        TupleContestedDocumentVotePollWinnerInfoBlockInfo winner = contenders.getWinner();
        System.out.println("  Winner:" + (winner != null ? "" : "none"));
        if (winner != null) {
            System.out.print("  " + winner.getO_0().getTag());
            if (winner.getO_0().getTag() == ContestedDocumentVotePollWinnerInfo.Tag.WonByIdentity) {
                System.out.print(" " + Base58.encode(winner.getO_0().getWon_by_identity().get_0().get_0().get_0()));
            }
            System.out.println();
        }
        System.out.println("  Abstain: " + contenders.getAbstainVoteTally());
        System.out.println("  Lock: " + contenders.getLockVoteTally());
        System.out.println("  ---------------");
        for (Map.Entry<Identifier, ContenderWithSerializedDocument> entry : contenders.getContenders().entrySet()) {
            System.out.println("    Identifier: " + Base58.encode(entry.getKey().get_0().get_0()));
            byte [] serializedDocument = entry.getValue().getV0().get_0().getSerialized_document();
            System.out.println("    Serialized: " + (serializedDocument != null ? Base64.getEncoder().encodeToString(serializedDocument) : "null"));
            if (serializedDocument != null) {
                Result<Document, String> result2 = dashsdk.platformMobileFetchDocumentDeserializeDocumentSdk(
                        sdk, serializedDocument, new Identifier(dpnsContractId), "domain");
                Document document = result2.unwrap();
                long createdAt = document.getV0().get_0().getCreated_at().toLong();
                System.out.println("    createdAt: " + DateFormat.getDateInstance(DateFormat.LONG).format(createdAt));
            }
            System.out.println("    Votes: " + entry.getValue().getV0().get_0().getVoteTally());
            System.out.println("    ---------------");
        }

    }

    @Test
    public void getVoteContendorsForNonExistantTest() throws Exception {
        ArrayList<PlatformValue> indexes = new ArrayList<>();
        indexes.add(new PlatformValue("dash"));
        indexes.add(new PlatformValue("test11101010010110101010"));
        Result<Contenders, String> result = dashsdk.platformMobileVotingGetVoteContenders(
                sdk,
                "parentNameAndLabel",
                indexes,
                "domain",
                new Identifier(dpnsContractId)
        );
        Contenders contenders = result.unwrap();
        assertNotNull(contenders);
        System.out.println("Contenders: " + contenders.getContenders().size());
        System.out.println("  Abstain: " + contenders.getAbstainVoteTally());
        System.out.println("  Lock: " + contenders.getLockVoteTally());
        for (Map.Entry<Identifier, ContenderWithSerializedDocument> entry : contenders.getContenders().entrySet()) {
            System.out.println("  Identifier: " + Base58.encode(entry.getKey().get_0().get_0()));
            System.out.println("  " + entry.getValue().getV0().get_0().getVoteTally());
        }

    }

    @Test
    public void getContestedResources() throws Exception {
        Result<ContestedResources, String> result = dashsdk.platformMobileVotingGetContestedResources(
                sdk,
                "domain",
                new Identifier(dpnsContractId)
        );
        ContestedResources contestedResources = result.unwrap();
        assertNotNull(contestedResources);
        List<ContestedResource> list = contestedResources.get_0();
        for (ContestedResource item : list) {
            System.out.println(item.getValue().getText());
        }
    }

    @Test
    public void getVotePoolsTest() throws Exception {
        Identifier dpnsContractid = new Identifier(dpnsContractId);

        //SWIGTYPE_p_DashSdk mainnetSdk = dashsdk.platformMobileSdkCreateDashSdk(BigInteger.ZERO, BigInteger.ZERO, false);

        Result<VotePollsGroupedByTimeStamp, String> result = dashsdk.platformMobileVotingGetVotepolls(
                sdk,
                new TimestampMillis(System.currentTimeMillis()),
                true,
                new TimestampMillis(System.currentTimeMillis() + 14 * 24 * 3600 * 1000),
                true
        );

        VotePollsGroupedByTimeStamp votePolls = result.unwrap();
        assertNotNull(votePolls);
        List<TupleTimeStampMillisVotePoll> list = votePolls.get_0();
        System.out.println("results returned: " + list.size());

        for (TupleTimeStampMillisVotePoll item : list) {
            System.out.println("timestamp: " + item.getO_0().get_0().longValue());
            for (VotePoll votePoll : item.getO_1()) {
                System.out.println(votePoll.getContested_document_resource_vote_poll().get_0().getDocument_type_name());
            }
        }
    }

    @Test
    public void putToPlatformTest() throws Exception {
        ArrayList<PlatformValue> indexes = new ArrayList<>();
        String name = "rev1ew000";
        indexes.add(new PlatformValue("dash"));
        indexes.add(new PlatformValue(name));
        Identifier dpnsIdentifier = new Identifier(dpnsContractId);

        Result<Contenders, String> contendersResult = dashsdk.platformMobileVotingGetVoteContenders(sdk,"parentNameAndLabel", indexes,"domain", dpnsIdentifier);
        assertFalse(contendersResult.unwrap().getContenders().isEmpty());

        ContestedDocumentResourceVotePoll contestedDocumentResourceVotePoll = new ContestedDocumentResourceVotePoll(
                dpnsIdentifier,
                "domain",
                "parentNameAndLabel",
                indexes
        );
        assertEquals(dpnsIdentifier, contestedDocumentResourceVotePoll.getContract_id());
        assertEquals("domain", contestedDocumentResourceVotePoll.getDocument_type_name());
        assertEquals("parentNameAndLabel", contestedDocumentResourceVotePoll.getIndex_name());
        assertEquals(indexes, contestedDocumentResourceVotePoll.getIndex_values());

        VotePoll votePoll = new VotePoll(contestedDocumentResourceVotePoll);

        ResourceVoteChoice resourceVoteChoice = new ResourceVoteChoice(
                ResourceVoteChoice.Tag.TowardsIdentity,
                new Identifier(Base58.decode("87ehECsR368RgMtPcAgxBkHJUNvHyN6tSKS3AXw2TZbZ"))
        );
//        ResourceVoteChoice resourceVoteChoice = new ResourceVoteChoice(
//                ResourceVoteChoice.Tag.Abstain, null
//        );
//        ResourceVoteChoice resourceVoteChoice = new ResourceVoteChoice(
//                ResourceVoteChoice.Tag.Lock, null
//        );
//        assertEquals(ResourceVoteChoice.Tag.TowardsIdentity, resourceVoteChoice.getTag());
//        assertEquals(ResourceVoteChoice.Tag.Abstain, resourceVoteChoice.getTag());
//        assertEquals(ResourceVoteChoice.Tag.Lock, resourceVoteChoice.getTag());

        ResourceVoteV0 resourceVoteV0 = new ResourceVoteV0(votePoll, resourceVoteChoice);
        ResourceVote resourceVote = new ResourceVote(resourceVoteV0);
        Vote myVote = new Vote(resourceVote);

        DumpedPrivateKey privateKey = DumpedPrivateKey.fromBase58(TestNet3Params.get(), "cRiBg3AhEH7XhqZ6vQSyKFH8DTjQe8YiRirZ539TmfndNpNw6go1");
        byte[] proTxHash = Utils.HEX.decode("bc77a5a2cec455c79fb92fb683dbd87a2a92b663c9a46d0c50d11889b4aeb121");
        ByteArrayOutputStream boas = new ByteArrayOutputStream(32 + 20);
        boas.write(proTxHash);
        boas.write(privateKey.getKey().getPubKeyHash());
        Sha256Hash idBytes = Sha256Hash.of(boas.toByteArray());
        System.out.println(Base58.encode(idBytes.getBytes()));
        Result<Optional<Identity>, String> identityResult = dashsdk.platformMobileFetchIdentityFetchIdentityWithSdk(sdk, new Identifier(idBytes.getBytes()));

        try {
            Identity identity = identityResult.unwrap().get();
            IdentityPublicKey ipk = identity.getV0().get_0().getPublicKeys().values().stream().findFirst().get();
            Signer signer = new Signer() {
                @Override
                public byte[] sign(byte @NotNull [] key, byte @NotNull [] data) {
                    return privateKey.getKey().signHash(Sha256Hash.twiceOf(data));
                }
            };
            Result<Vote, String> result = dashsdk.platformMobileVotingPutVoteToPlatform(
                    sdk,
                    myVote,
                    new Identifier(proTxHash),
                    ipk,
                    signer.getNativeContext(),
                    BigInteger.valueOf(signer.getSignerCallback())
            );
            Vote resourceVoteFromPlatform = result.unwrap();
            assertNotNull(resourceVoteFromPlatform);
            System.out.println("Vote: " + resourceVoteFromPlatform.getResource_vote().get_0().getV0().get_0().getResource_vote_choice().getTag());
        } catch (Exception e) {
            System.out.println(e.getMessage());

            // Posting same vote twice:
            // Attempted to unwrap a Failure: Dapi client error: Transport(Status { code: AlreadyExists, message: "state transition already in chain", metadata: MetadataMap { headers: {"grpc-accept-encoding": "identity", "grpc-encoding": "identity", "content-type": "application/grpc+proto", "date": "Tue, 17 Sep 2024 22:15:36 GMT", "x-envoy-upstream-service-time": "3", "server": "envoy"} }, source: None }, Address { ban_count: 0, banned_until: None, uri: https://35.85.21.179:1443/ })
            ///Attempted to unwrap a Failure: Dapi client error: Transport(Status { code: InvalidArgument, message: "Masternode vote is already present for masternode DghTta8E4ySZsozAoF4WjnYxpADLw3i2B7trhYKQ2ovG voting for ContestedDocumentResourceVotePoll(ContestedDocumentResourceVotePoll { contract_id: GWRSAVFMjXx8HpQFaNJMqBV7MBgMK4br5UESsB4S31Ec, document_type_name: domain, index_name: parentNameAndLabel, index_values: [string dash, string rev1ew000] })", metadata: MetadataMap { headers: {"drive-error-data-bin": "oW9zZXJpYWxpemVkRXJyb3KYbwIYKBi8GHcYpRiiGM4YxBhVGMcYnxi5GC8YthiDGNsY2Bh6GCoYkhi2GGMYyRikGG0MGFAY0RgYGIkYtBiuGLEYIQAY5hhoGMYYWRivGGYYrhjhGOcYLBgYGG0Y3hh7GFsYfgoYHRhxGCoJGMQNGFcYIRj2GCIYvxhTGMUYMRhVBhhkGG8YbRhhGGkYbhIYcBhhGHIYZRhuGHQYThhhGG0YZRhBGG4YZBhMGGEYYhhlGGwCEgQYZBhhGHMYaBIJGHIYZRh2GDEYZRh3GDAYMBgw", "code": "40304", "grpc-accept-encoding": "identity", "grpc-encoding": "identity", "content-type": "application/grpc+proto", "date": "Wed, 25 Sep 2024 16:51:27 GMT", "x-envoy-upstream-service-time": "54", "server": "envoy"} }, source: None }, Address { ban_count: 0, banned_until: None, uri: https://54.149.33.167:1443/ })
            // Abstain, ForIdentity, Lock, Abstain, ForIdentity (failed)
        }
    }
}
