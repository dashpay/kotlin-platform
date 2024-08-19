%rename (Vote) dpp_voting_votes_Vote;
START_CLASS(Vote, dpp_voting_votes_Vote);
    dpp_voting_votes_Vote(dpp_voting_votes_resource_vote_ResourceVote * resourceVote) {
        return dpp_voting_votes_Vote_ResourceVote_ctor(clone(resourceVote));
    }
END_CLASS();

%rename (Voter) drive_proof_verifier_types_Voter;
START_CLASS(Voter, drive_proof_verifier_types_Voter);
    drive_proof_verifier_types_Voter(platform_value_types_identifier_Identifier *identifier) {
        return drive_proof_verifier_types_Voter_ctor(clone(identifier));
    }
END_CLASS();

%rename (VotePoll) dpp_voting_vote_polls_VotePoll;
START_CLASS(VotePoll, dpp_voting_vote_polls_VotePoll);
    dpp_voting_vote_polls_VotePoll(dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll * vote_poll) {
        return dpp_voting_vote_polls_VotePoll_ContestedDocumentResourceVotePoll_ctor(clone(vote_poll));
    }
END_CLASS();

%rename (Voters) drive_proof_verifier_types_Voters;
START_CLASS(Voters, drive_proof_verifier_types_Voters);
    drive_proof_verifier_types_Voters(std_collections_BTreeSet_drive_proof_verifier_types_Voter *setVoters) {
        return drive_proof_verifier_types_Voters_ctor(clone(setVoters));
    }
END_CLASS();

%rename (ResourceVote) dpp_voting_votes_resource_vote_ResourceVote;
START_CLASS(ResourceVote, dpp_voting_votes_resource_vote_ResourceVote);
    dpp_voting_votes_resource_vote_ResourceVote(dpp_voting_votes_resource_vote_v0_ResourceVoteV0 * resourceVoteV0) {
        return dpp_voting_votes_resource_vote_ResourceVote_V0_ctor(clone(resourceVoteV0));
    }
END_CLASS();

%rename (ResourceVoteByIdentity) drive_proof_verifier_types_ResourceVotesByIdentity;
START_CLASS(ResourceVoteByIdentity, drive_proof_verifier_types_ResourceVotesByIdentity);

END_CLASS();

%rename (ResourceVoteV0) dpp_voting_votes_resource_vote_v0_ResourceVoteV0;
START_CLASS(ResourceVoteV0, dpp_voting_votes_resource_vote_v0_ResourceVoteV0);
    dpp_voting_votes_resource_vote_v0_ResourceVoteV0(
        dpp_voting_vote_polls_VotePoll * vote_poll,
        dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice * resource_vote_choice
    ) {
        return dpp_voting_votes_resource_vote_v0_ResourceVoteV0_ctor(clone(vote_poll), clone(resource_vote_choice));
    }
END_CLASS();

%rename (ContenderWithSerializedDocument) dpp_voting_contender_structs_contender_ContenderWithSerializedDocument;
START_CLASS(ContenderWithSerializedDocument, dpp_voting_contender_structs_contender_ContenderWithSerializedDocument);
    dpp_voting_contender_structs_contender_ContenderWithSerializedDocument(dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 * docV0) {
        return dpp_voting_contender_structs_contender_ContenderWithSerializedDocument_V0_ctor(clone(docV0));
    }
END_CLASS();

%rename (ContenderWithSerializedDocumentV0) dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0;
%ignore dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0::vote_tally;
START_CLASS(ContenderWithSerializedDocumentV0, dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0);
    int getVoteTally() {
        if ($self->vote_tally == nullptr) {
            return -1;
        } else {
            return *$self->vote_tally;
        }
    }
END_CLASS();

%rename (ContestedDocumentResourceVotePoll) dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll;
START_CLASS(ContestedDocumentResourceVotePoll, dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll);
    dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll(
        platform_value_types_identifier_Identifier * contract_id,
        char * document_type_name,
        char * index_name,
        Vec_platform_value_Value * index_values
    ) {
        return dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll_ctor(
            clone(contract_id),
            memoryFactory.clone(document_type_name),
            memoryFactory.clone(index_name),
            clone(index_values)
        );
    }
END_CLASS();

%rename (Contenders) drive_proof_verifier_types_Contenders;
%ignore drive_proof_verifier_types_Contenders::abstain_vote_tally;
%ignore drive_proof_verifier_types_Contenders::lock_vote_tally;
START_CLASS(Contenders, drive_proof_verifier_types_Contenders);
    int getAbstainVoteTally() {
        if ($self->abstain_vote_tally == nullptr) {
            return -1;
        } else {
            return *$self->abstain_vote_tally;
        }
    }
    int getLockVoteTally() {
        if ($self->lock_vote_tally == nullptr) {
            return -1;
        } else {
            return *$self->lock_vote_tally;
        }
    }
END_CLASS();

%rename (ContestedResources) drive_proof_verifier_types_ContestedResources;
START_CLASS(ContestedResources, drive_proof_verifier_types_ContestedResources);

END_CLASS();


%rename (ContestedResource) drive_proof_verifier_types_ContestedResource;
%ignore drive_proof_verifier_types_ContestedResource::value;
START_CLASS(ContestedResource, drive_proof_verifier_types_ContestedResource);
    platform_value_Value * getValue() {
        return $self->value._0;
    }
END_CLASS();

%rename (ResourceVoteChoice) dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice;
START_CLASS(ResourceVoteChoice, dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice);
    dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice(dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::Tag tag, platform_value_types_identifier_Identifier * identifier) {
        switch(tag)  {
            case dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::Tag::TowardsIdentity:
                return dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice_TowardsIdentity_ctor(clone(identifier));
            case dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::Tag::Abstain:
                return dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice_Abstain_ctor();
            case dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::Tag::Lock:
                return dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice_Lock_ctor();
            default:
                throw std::invalid_argument("invalid ResourceVoteChoice");
        }
    }
END_CLASS();

MAP_STRUCT_TYPEMAP(
    std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote,
    platform_value_types_identifier_Identifier,
    Identifier,
    dpp_voting_votes_resource_vote_ResourceVote,
    ResourceVote
);

MAP_STRUCT_TYPEMAP(
    std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument,
    platform_value_types_identifier_Identifier,
    Identifier,
    dpp_voting_contender_structs_contender_ContenderWithSerializedDocument,
    ContenderWithSerializedDocument
);

SET_STRUCT_TYPEMAP(
    std_collections_BTreeSet_drive_proof_verifier_types_Voter,
    drive_proof_verifier_types_Voter,
    Voter,
    clone
);

LIST_STRUCT_TYPEMAP(Vec_drive_proof_verifier_types_ContestedResource, drive_proof_verifier_types_ContestedResource, ContestedResource, clone);

%newobject platform_mobile_voting_get_vote_contenders(
    RustSdk *rust_sdk,
    char *index_name,
    Vec_platform_value_Value *index_values,
    char *document_type_name,
    platform_value_types_identifier_Identifier *contract_id
);