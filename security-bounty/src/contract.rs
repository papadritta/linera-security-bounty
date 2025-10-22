#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};

use security_bounty::{Bounty, Operation, Severity, Submission, SubmissionStatus};

use self::state::SecurityBountyState;

pub struct SecurityBountyContract {
    state: SecurityBountyState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(SecurityBountyContract);

impl WithContractAbi for SecurityBountyContract {
    type Abi = security_bounty::SecurityBountyAbi;
}

impl Contract for SecurityBountyContract {
    type Message = ();
    type Parameters = ();
    type InstantiationArgument = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = SecurityBountyState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        SecurityBountyContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        self.runtime.application_parameters();
        self.state.bounty_counter.set(0);
        self.state.submission_counter.set(0);
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::CreateBounty {
                contract_address,
                reward_pool,
                min_severity,
            } => {
                self.create_bounty(contract_address, reward_pool, min_severity)
                    .await;
            }
            Operation::SubmitVulnerability {
                bounty_id,
                title,
                description,
                severity,
                proof_of_concept,
            } => {
                self.submit_vulnerability(
                    bounty_id,
                    title,
                    description,
                    severity,
                    proof_of_concept,
                )
                .await;
            }
            Operation::VerifySubmission {
                submission_id,
                approved,
                payout_amount,
            } => {
                self.verify_submission(submission_id, approved, payout_amount)
                    .await;
            }
            Operation::ClaimPayout { submission_id } => {
                self.claim_payout(submission_id).await;
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl SecurityBountyContract {
    async fn create_bounty(
        &mut self,
        contract_address: String,
        reward_pool: u64,
        min_severity: Severity,
    ) {
        let bounty_id = *self.state.bounty_counter.get();
        let creator = self
            .runtime
            .authenticated_owner()
            .expect("Missing signer for bounty creation");

        let bounty = Bounty {
            id: bounty_id,
            contract_address,
            creator,
            reward_pool,
            remaining_pool: reward_pool,
            min_severity,
            created_at: self.runtime.system_time(),
            active: true,
        };

        self.state
            .bounties
            .insert(&bounty_id, bounty)
            .expect("Failed to insert bounty");
        self.state.bounty_counter.set(bounty_id + 1);
    }

    async fn submit_vulnerability(
        &mut self,
        bounty_id: u64,
        title: String,
        description: String,
        severity: Severity,
        proof_of_concept: String,
    ) {
        let bounty = self
            .state
            .bounties
            .get(&bounty_id)
            .await
            .expect("Failed to read bounty")
            .expect("Bounty not found");

        assert!(bounty.active, "Bounty is not active");
        assert!(
            severity >= bounty.min_severity,
            "Severity below minimum required"
        );

        let submission_id = *self.state.submission_counter.get();
        let hunter = self
            .runtime
            .authenticated_owner()
            .expect("Missing signer for submission");

        let submission = Submission {
            id: submission_id,
            bounty_id,
            hunter,
            title,
            description,
            severity,
            proof_of_concept,
            submitted_at: self.runtime.system_time(),
            status: SubmissionStatus::Pending,
            payout_amount: 0,
            paid: false,
        };

        self.state
            .submissions
            .insert(&submission_id, submission)
            .expect("Failed to insert submission");
        self.state.submission_counter.set(submission_id + 1);
    }

    async fn verify_submission(&mut self, submission_id: u64, approved: bool, payout_amount: u64) {
        let mut submission = self
            .state
            .submissions
            .get(&submission_id)
            .await
            .expect("Failed to read submission")
            .expect("Submission not found");

        let mut bounty = self
            .state
            .bounties
            .get(&submission.bounty_id)
            .await
            .expect("Failed to read bounty")
            .expect("Bounty not found");

        let signer = self
            .runtime
            .authenticated_owner()
            .expect("Missing signer for verification");

        assert_eq!(
            signer, bounty.creator,
            "Only bounty creator can verify submissions"
        );
        assert_eq!(
            submission.status,
            SubmissionStatus::Pending,
            "Submission already processed"
        );

        if approved {
            assert!(
                payout_amount <= bounty.remaining_pool,
                "Insufficient funds in bounty pool"
            );

            submission.status = SubmissionStatus::Approved;
            submission.payout_amount = payout_amount;
            bounty.remaining_pool -= payout_amount;

            let bounty_id = bounty.id;
            self.state
                .bounties
                .insert(&bounty_id, bounty)
                .expect("Failed to update bounty");
        } else {
            submission.status = SubmissionStatus::Rejected;
        }

        self.state
            .submissions
            .insert(&submission_id, submission)
            .expect("Failed to update submission");
    }

    async fn claim_payout(&mut self, submission_id: u64) {
        let mut submission = self
            .state
            .submissions
            .get(&submission_id)
            .await
            .expect("Failed to read submission")
            .expect("Submission not found");

        let signer = self
            .runtime
            .authenticated_owner()
            .expect("Missing signer for payout claim");

        assert_eq!(
            signer, submission.hunter,
            "Only submission author can claim payout"
        );
        assert_eq!(
            submission.status,
            SubmissionStatus::Approved,
            "Submission not approved"
        );
        assert!(!submission.paid, "Payout already claimed");

        submission.paid = true;

        self.state
            .submissions
            .insert(&submission_id, submission)
            .expect("Failed to update submission");

        // In production, this would trigger actual token transfer
        // For now, we just mark it as paid
    }
}

#[cfg(test)]
mod tests {
    use futures::FutureExt as _;
    use linera_sdk::{util::BlockingWait, views::View, Contract, ContractRuntime};

    use super::{SecurityBountyContract, SecurityBountyState};

    #[test]
    fn initialize_state() {
        let app = create_app();
        assert_eq!(*app.state.bounty_counter.get(), 0);
        assert_eq!(*app.state.submission_counter.get(), 0);
    }

    fn create_app() -> SecurityBountyContract {
        let runtime = ContractRuntime::new().with_application_parameters(());
        let mut contract = SecurityBountyContract {
            state: SecurityBountyState::load(runtime.root_view_storage_context())
                .blocking_wait()
                .expect("Failed to read from mock key value store"),
            runtime,
        };

        contract
            .instantiate(())
            .now_or_never()
            .expect("Initialization should not await");

        contract
    }
}
