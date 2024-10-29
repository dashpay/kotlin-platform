use std::collections::{BTreeMap, BTreeSet};
use dpp::document::Document;
use dpp::document::v0::DocumentV0;
use dpp::identity::identity_public_key::TimestampMillis;
use dpp::identity::identity_public_key::KeyID;
use dpp::prelude::{BlockHeight, CoreBlockHeight, Revision};
use dpp::voting::contender_structs::{ContenderWithSerializedDocument, ContenderWithSerializedDocumentV0};
use dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice;
use dpp::voting::vote_polls::contested_document_resource_vote_poll::ContestedDocumentResourceVotePoll;
use dpp::voting::vote_polls::VotePoll;
use dpp::voting::votes::resource_vote::ResourceVote;
use dpp::voting::votes::resource_vote::v0::ResourceVoteV0;
use dpp::voting::votes::Vote;
use drive::query::{OrderClause, WhereClause, WhereOperator};
use drive_proof_verifier::types::{Contenders, ContestedResource, ContestedResources, VotePollsGroupedByTimestamp, Voter, Voters};
use platform_value::{Hash256, Value, ValueMap};

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn KeyID_clone(id: KeyID) -> KeyID {
    id.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Revision_clone(revision: Revision) -> Revision {
    revision.clone()
}
#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn TimestampMillis_clone(time: TimestampMillis) -> TimestampMillis {
    time.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn prelude_TimestampMillis_clone(time: dpp::prelude::TimestampMillis) -> dpp::prelude::TimestampMillis {
    time.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn CoreBlockHeight_clone(height: CoreBlockHeight) -> CoreBlockHeight {
    height.clone()
}
#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn BlockHeight_clone(height: BlockHeight) -> BlockHeight {
    height.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Value_clone(value: Value) -> Value {
    value.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Vec_Value_clone(value_vec: Vec<Value>) -> Vec<Value> {
    value_vec.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn ValueMap_clone(value_map: ValueMap) -> ValueMap {
    value_map.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn std_collections_Map_keys_String_values_platform_value_Value_clone(map: BTreeMap<String, Value>) -> BTreeMap<String, Value> {
    map.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Vec_u8_clone(vec: Vec<u8>) -> Vec<u8> {
    vec.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Document_clone(document: Document) -> Document {
    document.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn DocumentV0_clone(document: DocumentV0) -> DocumentV0 {
    document.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Arr_u8_32_clone(slice: [u8; 32]) -> [u8; 32] {
    slice.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Arr_u8_20_clone(slice: [u8; 20]) -> [u8; 20] {
    slice.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Arr_u8_36_clone(slice: [u8; 36]) -> [u8; 36] {
    slice.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn WhereClause_clone(o: WhereClause) -> WhereClause {
    o.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn WhereOperator_clone(o: WhereOperator) -> WhereOperator {
    o.clone()
}
#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn OrderClause_clone(o: OrderClause) -> OrderClause {
    o.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Hash256_clone(o: Hash256) -> Hash256 {
    o.clone()
}


#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Contenders_clone(o: Contenders) -> Contenders {
    o.clone()
}
#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn ContenderWithSerializedDocument_clone(o: ContenderWithSerializedDocument) -> ContenderWithSerializedDocument {
    o.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn ContenderWithSerializedDocumentV0_clone(o: ContenderWithSerializedDocumentV0) -> ContenderWithSerializedDocumentV0 {
    o.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn ContestedResources_clone(o: ContestedResources) -> ContestedResources {
    o.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn ContestedResource_clone(o: ContestedResource) -> ContestedResource {
    o.clone()
}
#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Vote_clone(o: Vote) -> Vote {
    o.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Voter_clone(o: Voter) -> Voter {
    o.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Voters_clone(o: Voters) -> Voters {
    o.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn std_collections_BTreeSet_drive_proof_verifier_types_Voter_clone(o: BTreeSet<Voter>) -> BTreeSet<Voter> {
    o.clone()
}
#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn VotePoll_clone(o: VotePoll) -> VotePoll {
    o.clone()
}


#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn ContestedDocumentResourceVotePoll_clone(o: ContestedDocumentResourceVotePoll) -> ContestedDocumentResourceVotePoll {
    o.clone()
}
#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn ResourceVote_clone(o: ResourceVote) -> ResourceVote {
    o.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn ResourceVoteChoice_clone(o: ResourceVoteChoice) -> ResourceVoteChoice {
    o.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn ResourceVoteV0_clone(o: ResourceVoteV0) -> ResourceVoteV0 {
    o.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn VotePollsGroupedByTimestamp_clone(o: VotePollsGroupedByTimestamp) -> VotePollsGroupedByTimestamp {
    return o.clone()
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll_clone(o: (dpp::prelude::TimestampMillis, Vec<VotePoll>)) -> (dpp::prelude::TimestampMillis, Vec<VotePoll>) {
    o.clone()
}