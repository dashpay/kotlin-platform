//mod fermented;


use rs_sdk::dpp::identity::IdentityV0;
use rs_sdk::platform::Identity;

pub fn do_it() -> Identity {
    Identity::V0(IdentityV0::default())
}