mod fermented;


use dash_sdk::dpp::identity::IdentityV0;
use dash_sdk::platform::Identity;

pub fn dummy_function() -> Identity {
    Identity::V0(IdentityV0::default())
}