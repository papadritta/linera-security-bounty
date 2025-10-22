#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    graphql::GraphQLMutationRoot, linera_base_types::WithServiceAbi, views::View, Service,
    ServiceRuntime,
};

use security_bounty::{Bounty, Operation, Submission};

use self::state::SecurityBountyState;

pub struct SecurityBountyService {
    state: SecurityBountyState,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(SecurityBountyService);

impl WithServiceAbi for SecurityBountyService {
    type Abi = security_bounty::SecurityBountyAbi;
}

impl Service for SecurityBountyService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = SecurityBountyState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        SecurityBountyService {
            state,
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let bounty_count = *self.state.bounty_counter.get();
        let submission_count = *self.state.submission_counter.get();

        let mut bounties = Vec::new();
        for id in 0..bounty_count {
            if let Some(bounty) = self.state.bounties.get(&id).await.expect("Failed to read") {
                bounties.push(bounty);
            }
        }

        let mut submissions = Vec::new();
        for id in 0..submission_count {
            if let Some(submission) = self
                .state
                .submissions
                .get(&id)
                .await
                .expect("Failed to read")
            {
                submissions.push(submission);
            }
        }

        Schema::build(
            QueryRoot {
                bounties,
                submissions,
            },
            Operation::mutation_root(self.runtime.clone()),
            EmptySubscription,
        )
        .finish()
        .execute(query)
        .await
    }
}

struct QueryRoot {
    bounties: Vec<Bounty>,
    submissions: Vec<Submission>,
}

#[Object]
impl QueryRoot {
    async fn bounties(&self) -> &Vec<Bounty> {
        &self.bounties
    }

    async fn submissions(&self) -> &Vec<Submission> {
        &self.submissions
    }

    async fn bounty(&self, id: u64) -> Option<&Bounty> {
        self.bounties.iter().find(|b| b.id == id)
    }

    async fn submission(&self, id: u64) -> Option<&Submission> {
        self.submissions.iter().find(|s| s.id == id)
    }

    async fn bounty_submissions(&self, bounty_id: u64) -> Vec<&Submission> {
        self.submissions
            .iter()
            .filter(|s| s.bounty_id == bounty_id)
            .collect()
    }
}
