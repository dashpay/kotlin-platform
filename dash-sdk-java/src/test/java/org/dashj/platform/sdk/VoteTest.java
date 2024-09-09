package org.dashj.platform.sdk;

import org.bitcoinj.core.Base58;
import org.dashj.platform.sdk.base.Result;
import org.dashj.platform.sdk.callbacks.Signer;
import org.jetbrains.annotations.NotNull;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import java.math.BigInteger;
import java.util.ArrayList;
import java.util.Base64;
import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;

public class VoteTest extends BaseTest {

    static SWIGTYPE_p_DashSdk sdk;

    @BeforeAll
    static void startUp() {
        sdk = dashsdk.platformMobileSdkCreateDashSdk(BigInteger.ZERO, BigInteger.ZERO, true);
    }

    @AfterAll
    static void tearDown() {
        dashsdk.platformMobileSdkDestroyDashSdk(sdk);
    }

    @Test
    void getVoteContendorsTest() throws Exception {
        ArrayList<PlatformValue> indexes = new ArrayList<>();
        indexes.add(new PlatformValue("dash"));
        indexes.add(new PlatformValue("test111"));
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
            System.out.println("  Serialized:" + Base64.getEncoder().encodeToString(entry.getValue().getV0().get_0().getSerialized_document()));
            System.out.println("  Votes: " + entry.getValue().getV0().get_0().getVoteTally());
        }

    }

    @Test
    void getVoteContendorsForNonExistantTest() throws Exception {
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
    void getContestedResources() throws Exception {
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
    void putToPlatformTest() throws Exception {
        ArrayList<PlatformValue> indexes = new ArrayList<>();
        indexes.add(new PlatformValue("dash"));
        indexes.add(new PlatformValue("b0b1ee"));

        Identifier dpnsIdentifier = new Identifier(dpnsContractId);
        ContestedDocumentResourceVotePoll contestedDocumentResourceVotePoll = new ContestedDocumentResourceVotePoll(
                dpnsIdentifier,
                "domain",
                "parentDomainAndLabel",
                indexes
        );
        assertEquals(dpnsIdentifier, contestedDocumentResourceVotePoll.getContract_id());
        assertEquals("domain", contestedDocumentResourceVotePoll.getDocument_type_name());
        assertEquals("parentDomainAndLabel", contestedDocumentResourceVotePoll.getIndex_name());
        assertEquals(indexes, contestedDocumentResourceVotePoll.getIndex_values());

        VotePoll votePoll = new VotePoll(contestedDocumentResourceVotePoll);
        ResourceVoteChoice resourceVoteChoice = new ResourceVoteChoice(ResourceVoteChoice.Tag.TowardsIdentity, new Identifier(identifier));
        assertEquals(ResourceVoteChoice.Tag.TowardsIdentity, resourceVoteChoice.getTag());
        ResourceVoteV0 resourceVoteV0 = new ResourceVoteV0(votePoll, resourceVoteChoice);
        ResourceVote resourceVote = new ResourceVote(resourceVoteV0);
        Vote myVote = new Vote(resourceVote);

        IdentityPublicKeyV0 ipkv0 = new IdentityPublicKeyV0(
                new KeyID(1),
                Purpose.VOTING,
                SecurityLevel.HIGH,
                null,
                KeyType.BIP13_SCRIPT_HASH,
                false,
                new BinaryData(new byte[20]),
                null
        );

        Result<Vote, String> result = dashsdk.platformMobileVotingPutVoteToPlatform(
                sdk,
                myVote,
                new Identifier(new byte[32]),
                new IdentityPublicKey(ipkv0),
                    BigInteger.valueOf(new Signer() {
                        @Override
                        public byte[] sign(byte @NotNull [] key, byte @NotNull [] data) {
                            return new byte[0];
                        }
                    }.getSignerCallback()
                )
        );
        String error = result.unwrapError();
        assertNotNull(error);
    }
}
