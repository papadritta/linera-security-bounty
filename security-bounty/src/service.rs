#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Request, Response, Schema};
use linera_sdk::{
    graphql::GraphQLMutationRoot as _, linera_base_types::WithServiceAbi, views::View, Service,
    ServiceRuntime,
};

use security_bounty::Operation;

use self::state::SecurityBountyState;

pub struct SecurityBountyService {
    state: Arc<SecurityBountyState>,
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
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            self.state.clone(),
            Operation::mutation_root(self.runtime.clone()),
            EmptySubscription,
        )
        .finish();
        schema.execute(request).await
    }
}
