use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};

use security_bounty::{Bounty, Submission};

#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct SecurityBountyState {
    /// Counter for bounty IDs
    pub bounty_counter: RegisterView<u64>,

    /// Counter for submission IDs
    pub submission_counter: RegisterView<u64>,

    /// Map of bounty_id -> Bounty
    pub bounties: MapView<u64, Bounty>,

    /// Map of submission_id -> Submission
    pub submissions: MapView<u64, Submission>,
}
