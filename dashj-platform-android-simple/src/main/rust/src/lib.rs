//mod fermented;


use rs_sdk::dpp::identity::IdentityV0;
use rs_sdk::platform::Identity;

pub fn get_default_identity() -> Identity {
    Identity::V0(IdentityV0::default())
}