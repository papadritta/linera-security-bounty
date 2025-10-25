use async_graphql::Object;
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

#[Object]
impl SecurityBountyState {
    async fn bounties(&self) -> Vec<Bounty> {
        let count = *self.bounty_counter.get();
        let mut result = Vec::new();
        for id in 0..count {
            if let Ok(Some(bounty)) = self.bounties.get(&id).await {
                result.push(bounty);
            }
        }
        result
    }

    async fn submissions(&self) -> Vec<Submission> {
        let count = *self.submission_counter.get();
        let mut result = Vec::new();
        for id in 0..count {
            if let Ok(Some(submission)) = self.submissions.get(&id).await {
                result.push(submission);
            }
        }
        result
    }

    async fn bounty(&self, id: u64) -> Option<Bounty> {
        self.bounties.get(&id).await.ok().flatten()
    }

    async fn submission(&self, id: u64) -> Option<Submission> {
        self.submissions.get(&id).await.ok().flatten()
    }

    async fn bounty_submissions(&self, bounty_id: u64) -> Vec<Submission> {
        let count = *self.submission_counter.get();
        let mut result = Vec::new();
        for id in 0..count {
            if let Ok(Some(submission)) = self.submissions.get(&id).await {
                if submission.bounty_id == bounty_id {
                    result.push(submission);
                }
            }
        }
        result
    }
}
