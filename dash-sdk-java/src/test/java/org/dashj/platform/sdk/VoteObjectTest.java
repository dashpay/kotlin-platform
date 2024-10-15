package org.dashj.platform.sdk;

import org.bitcoinj.core.Base58;
import org.junit.Test;

import java.util.ArrayList;

import static org.junit.Assert.assertEquals;

public class VoteObjectTest extends BaseTest {

    @Test
    public void createAndDestroyVoteTest() {
        ArrayList<PlatformValue> indexes = new ArrayList<>();
        String name = "rev1ew000";
        indexes.add(new PlatformValue("dash"));
        indexes.add(new PlatformValue(name));
        Identifier dpnsIdentifier = new Identifier(dpnsContractId);

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

        ResourceVoteV0 resourceVoteV0 = new ResourceVoteV0(votePoll, resourceVoteChoice);
        ResourceVote resourceVote = new ResourceVote(resourceVoteV0);
        Vote myVote = new Vote(resourceVote);

       myVote.delete();
       resourceVote.delete();
       resourceVoteV0.delete();
       resourceVoteChoice.delete();
       votePoll.delete();
       contestedDocumentResourceVotePoll.delete();
       dpnsIdentifier.delete();
    }
}
