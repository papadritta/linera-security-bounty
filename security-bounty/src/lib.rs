use async_graphql::{Request, Response};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::{AccountOwner, ContractAbi, ServiceAbi, Timestamp},
};
use serde::{Deserialize, Serialize};

pub struct SecurityBountyAbi;

impl ContractAbi for SecurityBountyAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for SecurityBountyAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    /// Create a new bounty program for a contract
    CreateBounty {
        contract_address: String,
        reward_pool: u64,
        min_severity: Severity,
    },
    /// Submit a vulnerability report
    SubmitVulnerability {
        bounty_id: u64,
        title: String,
        description: String,
        severity: Severity,
        proof_of_concept: String,
    },
    /// Verify a submission (admin only)
    VerifySubmission {
        submission_id: u64,
        approved: bool,
        payout_amount: u64,
    },
    /// Claim payout for approved submission
    ClaimPayout { submission_id: u64 },
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, async_graphql::Enum, Copy,
)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct Bounty {
    pub id: u64,
    pub contract_address: String,
    pub creator: AccountOwner,
    pub reward_pool: u64,
    pub remaining_pool: u64,
    pub min_severity: Severity,
    pub created_at: Timestamp,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct Submission {
    pub id: u64,
    pub bounty_id: u64,
    pub hunter: AccountOwner,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub proof_of_concept: String,
    pub submitted_at: Timestamp,
    pub status: SubmissionStatus,
    pub payout_amount: u64,
    pub paid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, async_graphql::Enum, Copy)]
pub enum SubmissionStatus {
    Pending,
    Approved,
    Rejected,
}
