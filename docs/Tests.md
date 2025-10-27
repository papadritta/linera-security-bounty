# Testing Guide

Complete testing documentation for the Linera Security Bounty Platform. This guide covers all testing methods and scenarios to validate platform functionality.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Quick Start](#quick-start)
3. [Testing Methods](#testing-methods)
4. [Test Scenarios](#test-scenarios)
5. [GraphQL Playground Testing](#graphql-playground-testing)
6. [UI Testing](#ui-testing)
7. [cURL Testing](#curl-testing)
8. [CLI Testing](#cli-testing)
9. [Command Reference](#command-reference)
10. [Troubleshooting](#troubleshooting)
11. [Testing Checklist](#testing-checklist)

---

## Prerequisites

Before testing, ensure your environment is set up:

```bash
# 1. Start the network (Terminal 1)
make network-up

# 2. Export the environment variables (Terminal 2)
export LINERA_WALLET=/tmp/.tmpXXX/wallet.json
export LINERA_KEYSTORE=/tmp/.tmpXXX/keystore
export LINERA_STORAGE=rocksdb:/tmp/.tmpXXX/storage

# 3. Build and deploy
make build
make deploy

# 4. Start GraphQL service (keep running)
make serve

# 5. Start frontend (Terminal 3)
make frontend
```

**Services running:**
- GraphQL: `http://localhost:8080`
- Web UI: `http://localhost:3000`

---

## Quick Start

```bash
# Extract Chain and App IDs from deployed frontend
CHAIN_ID=$(grep -o "chains/[a-f0-9]\{64\}" frontend/index.html | cut -d'/' -f2)
APP_ID=$(grep -o "applications/[a-f0-9]\{64\}" frontend/index.html | cut -d'/' -f2)
```

### Choose your preferred method:

- **[GraphQL Playground](#graphql-playground-testing)** - Interactive GraphQL IDE
- **[UI Testing](#ui-testing)** - Browser-based, visual
- **[cURL Testing](#curl-testing)** - Terminal commands, scriptable
- **[CLI Testing](#cli-testing)** - Linera native commands

---

## Testing Methods

### Comparison

| Method | Best For | Pros | Cons |
|--------|----------|------|------|
| **GraphQL Playground** | Development, testing | Interactive, autocomplete, easy debugging | Manual execution only |
| **UI** | Demos, training | Visual feedback, no CLI needed | Slower, not scriptable |
| **cURL** | Quick testing, CI/CD | Fast, scriptable, universal | Raw JSON output |
| **CLI** | Production | Native integration, reliable | Requires Linera CLI |

---

## Test Scenarios

We cover **3 complete scenarios** that test all platform operations:

### TEST 1: Critical Bug - Approved & Claimed
- Create bounty program (10B tokens)
- Submit critical vulnerability
- Admin approves with 5B payout
- Researcher claims reward

### TEST 2: Medium Bug - Rejected
- Create second bounty (5B tokens)
- Submit medium severity bug
- Admin rejects submission
- Verify rejection status

### TEST 3: Critical Bug - High Payout
- Create premium bounty (20B tokens)
- Submit critical reentrancy bug
- Admin approves with 15B payout
- Researcher claims high reward

**Total Coverage:** 20B tokens paid, 3 bounties, 3 submissions (2 approved, 1 rejected)

---

## GraphQL Playground Testing

The GraphQL Playground provides an interactive interface for testing mutations and queries with autocomplete and documentation.

### Access GraphQL Playground

1. Ensure GraphQL service is running: `make serve`
2. Open GraphQL Playground in your browser:
   ```
   http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID}
   ```
3. Replace `${CHAIN_ID}` and `${APP_ID}` with your actual IDs from deployment

### TEST 1: Critical Bug - Approved & Claimed

**Step 1: Create Bounty**
```graphql
mutation {
  createBounty(
    contractAddress: "0xALPHA_CRITICAL_CONTRACT_001",
    rewardPool: 10000000000,
    minSeverity: MEDIUM
  )
}
```

**Step 2: Submit Vulnerability**
```graphql
mutation {
  submitVulnerability(
    bountyId: 0,
    title: "Integer Overflow in Token Transfer",
    description: "Unchecked arithmetic in transfer function allows attacker to mint unlimited tokens",
    severity: CRITICAL,
    proofOfConcept: "1. Call transfer() with amount = MAX_UINT64\n2. Add small value causing overflow\n3. Balance wraps to small number\n4. Repeat to drain protocol"
  )
}
```

**Step 3: Verify Submission**
```graphql
mutation {
  verifySubmission(
    submissionId: 0,
    approved: true,
    payoutAmount: 5000000000
  )
}
```

**Step 4: Claim Payout**
```graphql
mutation {
  claimPayout(submissionId: 0)
}
```

**Verify Result**
```graphql
query {
  submission(id: 0) {
    id
    status
    payoutAmount
    paid
  }
  bounty(id: 0) {
    id
    rewardPool
    remainingPool
  }
}
```

**Expected:** Submission 0 with status `APPROVED`, paid `true`, payout `5000000000`

---

### TEST 2: Medium Bug - Rejected

**Step 1: Create Bounty**
```graphql
mutation {
  createBounty(
    contractAddress: "0xBETA_MEDIUM_CONTRACT_002",
    rewardPool: 5000000000,
    minSeverity: LOW
  )
}
```

**Step 2: Submit Vulnerability**
```graphql
mutation {
  submitVulnerability(
    bountyId: 1,
    title: "Missing Input Validation",
    description: "Function does not validate user input length, may cause DoS with large inputs",
    severity: MEDIUM,
    proofOfConcept: "1. Send input string with 1M characters\n2. Function consumes excessive gas\n3. Transaction fails but costs gas"
  )
}
```

**Step 3: Reject Submission**
```graphql
mutation {
  verifySubmission(
    submissionId: 1,
    approved: false,
    payoutAmount: 0
  )
}
```

**Verify Result**
```graphql
query {
  submission(id: 1) {
    id
    status
    payoutAmount
    paid
  }
}
```

**Expected:** Submission 1 with status `REJECTED`, paid `false`, payout `0`

---

### TEST 3: Critical Bug - High Payout

**Step 1: Create Bounty**
```graphql
mutation {
  createBounty(
    contractAddress: "0xGAMMA_ENTERPRISE_CONTRACT_003",
    rewardPool: 20000000000,
    minSeverity: HIGH
  )
}
```

**Step 2: Submit Vulnerability**
```graphql
mutation {
  submitVulnerability(
    bountyId: 2,
    title: "Reentrancy Vulnerability in Withdrawal",
    description: "Missing CEI pattern allows recursive calls to drain contract balance before state update",
    severity: CRITICAL,
    proofOfConcept: "1. Deploy malicious contract with fallback function\n2. Call withdraw() which sends ETH before updating balance\n3. Fallback calls withdraw() again\n4. Repeat until contract drained\n5. PoC repo: github.com/example/reentrancy-poc"
  )
}
```

**Step 3: Approve High Payout**
```graphql
mutation {
  verifySubmission(
    submissionId: 2,
    approved: true,
    payoutAmount: 15000000000
  )
}
```

**Step 4: Claim High Reward**
```graphql
mutation {
  claimPayout(submissionId: 2)
}
```

**Verify Result**
```graphql
query {
  submission(id: 2) {
    id
    status
    payoutAmount
    paid
  }
  bounty(id: 2) {
    id
    rewardPool
    remainingPool
  }
}
```

**Expected:** Submission 2 with status `APPROVED`, paid `true`, payout `15000000000`

---

### Useful GraphQL Queries

**List All Bounties**
```graphql
query {
  bounties {
    id
    contractAddress
    rewardPool
    remainingPool
    minSeverity
    active
  }
}
```

**List All Submissions**
```graphql
query {
  submissions {
    id
    bountyId
    title
    severity
    status
    payoutAmount
    paid
  }
}
```

**Get Specific Bounty**
```graphql
query {
  bounty(id: 0) {
    id
    contractAddress
    rewardPool
    remainingPool
    minSeverity
    active
  }
}
```

**Get Specific Submission**
```graphql
query {
  submission(id: 0) {
    id
    bountyId
    title
    description
    severity
    status
    payoutAmount
    paid
  }
}
```

**Get Submissions for Specific Bounty**
```graphql
query {
  bountySubmissions(bountyId: 0) {
    id
    title
    severity
    status
    payoutAmount
    paid
  }
}
```

---

## UI Testing

### Access Web Interface

1. Open browser: `http://localhost:3000`
2. Ensure GraphQL service is running in Terminal 2

### TEST 1: Critical Bug - Approved & Claimed

#### Step 1.1: Create Bounty Program
1. Locate "Create Bounty Program" section
2. Fill in:
   - Contract Address: `0xALPHA_CRITICAL_CONTRACT_001`
   - Reward Pool: `10000000000`
   - Min Severity: Select **MEDIUM**
3. Click "Create Bounty"
4. Verify Bounty ID 0 created in "Active Bug Bounty Programs" section

#### Step 1.2: Submit Vulnerability
1. Scroll to "Submit Vulnerability" section
2. Fill in:
   - Bounty ID: `0`
   - Title: `Integer Overflow in Token Transfer`
   - Description: `Unchecked arithmetic in transfer function allows attacker to mint unlimited tokens`
   - Severity: Select **CRITICAL**
   - Proof of Concept:
     ```
     1. Call transfer() with amount = MAX_UINT64
     2. Add small value causing overflow
     3. Balance wraps to small number
     4. Repeat to drain protocol
     ```
3. Click "Submit Vulnerability"
4. Verify Submission ID 0 created with status PENDING in "Vulnerability Submissions" section

#### Step 1.3: Verify Submission (Admin)
1. Locate "Verify Submission" section
2. Fill in:
   - Submission ID: `0`
   - Approved: Check YES
   - Payout Amount: `5000000000`
3. Click "Verify Submission"
4. Verify status changed to APPROVED in submissions list

#### Step 1.4: Claim Payout
1. Locate "Claim Payout" section
2. Fill in:
   - Submission ID: `0`
3. Click "Claim Payout"
4. Verify paid status shows true and bounty remaining pool reduced to 5,000,000,000

---

### TEST 2: Medium Bug - Rejected

#### Step 2.1: Create Second Bounty
1. Contract Address: `0xBETA_MEDIUM_CONTRACT_002`
2. Reward Pool: `5000000000`
3. Min Severity: **LOW**
4. Click "Create Bounty"
5. Verify Bounty ID 1 created

#### Step 2.2: Submit Vulnerability
1. Bounty ID: `1`
2. Title: `Missing Input Validation`
3. Description: `Function does not validate user input length, may cause DoS with large inputs`
4. Severity: **MEDIUM**
5. Proof of Concept:
   ```
   1. Send input string with 1M characters
   2. Function consumes excessive gas
   3. Transaction fails but costs gas
   ```
6. Click "Submit Vulnerability"
7. Verify Submission ID 1 created with status PENDING

#### Step 2.3: Reject Submission
1. Submission ID: `1`
2. Approved: Check NO
3. Payout Amount: `0`
4. Click "Verify Submission"
5. Verify status changed to REJECTED

---

### TEST 3: Critical Bug - High Payout

#### Step 3.1: Create Premium Bounty
1. Contract Address: `0xGAMMA_ENTERPRISE_CONTRACT_003`
2. Reward Pool: `20000000000`
3. Min Severity: **HIGH**
4. Click "Create Bounty"
5. Verify Bounty ID 2 created

#### Step 3.2: Submit Critical Finding
1. Bounty ID: `2`
2. Title: `Reentrancy Vulnerability in Withdrawal`
3. Description: `Missing CEI pattern allows recursive calls to drain contract balance before state update`
4. Severity: **CRITICAL**
5. Proof of Concept:
   ```
   1. Deploy malicious contract with fallback function
   2. Call withdraw() which sends ETH before updating balance
   3. Fallback calls withdraw() again
   4. Repeat until contract drained
   5. PoC repo: github.com/example/reentrancy-poc
   ```
6. Click "Submit Vulnerability"
7. Verify Submission ID 2 created with status PENDING

#### Step 3.3: Approve High Payout
1. Submission ID: `2`
2. Approved: Check YES
3. Payout Amount: `15000000000`
4. Click "Verify Submission"
5. Verify status changed to APPROVED

#### Step 3.4: Claim High Reward
1. Submission ID: `2`
2. Click "Claim Payout"
3. Verify paid status shows true and bounty remaining pool reduced to 5,000,000,000

---

## cURL Testing

### Setup

```bash
# Export Chain and App IDs
export CHAIN_ID="your_chain_id_here"
export APP_ID="your_app_id_here"
```

### TEST 1: Critical Bug - Approved & Claimed

```bash
# Step 1: Create Bounty
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { createBounty(contractAddress: \"0xALPHA_CRITICAL_CONTRACT_001\", rewardPool: 10000000000, minSeverity: MEDIUM) }"}'

# Step 2: Submit Vulnerability
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { submitVulnerability(bountyId: 0, title: \"Integer Overflow in Token Transfer\", description: \"Unchecked arithmetic in transfer function allows attacker to mint unlimited tokens\", severity: CRITICAL, proofOfConcept: \"1. Call transfer() with amount = MAX_UINT64\\n2. Add small value causing overflow\\n3. Balance wraps to small number\\n4. Repeat to drain protocol\") }"}'

# Step 3: Verify Submission
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { verifySubmission(submissionId: 0, approved: true, payoutAmount: 5000000000) }"}'

# Step 4: Claim Payout
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { claimPayout(submissionId: 0) }"}'

# Verify Result
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "{ submissions { id status payoutAmount paid } }"}'
```

**Expected:** Submission 0 with status `APPROVED`, paid `true`, payout `5000000000`

---

### TEST 2: Medium Bug - Rejected

```bash
# Step 1: Create Bounty
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { createBounty(contractAddress: \"0xBETA_MEDIUM_CONTRACT_002\", rewardPool: 5000000000, minSeverity: LOW) }"}'

# Step 2: Submit Vulnerability
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { submitVulnerability(bountyId: 1, title: \"Missing Input Validation\", description: \"Function does not validate user input length, may cause DoS with large inputs\", severity: MEDIUM, proofOfConcept: \"1. Send input string with 1M characters\\n2. Function consumes excessive gas\\n3. Transaction fails but costs gas\") }"}'

# Step 3: Reject Submission
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { verifySubmission(submissionId: 1, approved: false, payoutAmount: 0) }"}'

# Verify Result
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "{ submissions { id status payoutAmount paid } }"}'
```

**Expected:** Submission 1 with status `REJECTED`, paid `false`, payout `0`

---

### TEST 3: Critical Bug - High Payout

```bash
# Step 1: Create Bounty
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { createBounty(contractAddress: \"0xGAMMA_ENTERPRISE_CONTRACT_003\", rewardPool: 20000000000, minSeverity: HIGH) }"}'

# Step 2: Submit Vulnerability
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { submitVulnerability(bountyId: 2, title: \"Reentrancy Vulnerability in Withdrawal\", description: \"Missing CEI pattern allows recursive calls to drain contract balance before state update\", severity: CRITICAL, proofOfConcept: \"1. Deploy malicious contract with fallback function\\n2. Call withdraw() which sends ETH before updating balance\\n3. Fallback calls withdraw() again\\n4. Repeat until contract drained\\n5. PoC repo: github.com/example/reentrancy-poc\") }"}'

# Step 3: Approve High Payout
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { verifySubmission(submissionId: 2, approved: true, payoutAmount: 15000000000) }"}'

# Step 4: Claim High Reward
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { claimPayout(submissionId: 2) }"}'

# Verify Result
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "{ submissions { id status payoutAmount paid } bounties { id rewardPool remainingPool } }"}'
```

**Expected:** Submission 2 with status `APPROVED`, paid `true`, payout `15000000000`

---

## CLI Testing

### Setup

```bash
# Extract Chain and App IDs from frontend
CHAIN_ID=$(grep -o "chains/[a-f0-9]\{64\}" frontend/index.html | cut -d'/' -f2)
APP_ID=$(grep -o "applications/[a-f0-9]\{64\}" frontend/index.html | cut -d'/' -f2)
echo "Chain ID: ${CHAIN_ID}"
echo "App ID: ${APP_ID}"
```

### TEST 1: Critical Bug - Approved & Claimed

```bash
# Step 1: Create Bounty
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'mutation { createBounty(contractAddress: "0xALPHA_CRITICAL_CONTRACT_001", rewardPool: 10000000000, minSeverity: MEDIUM) }'

# Step 2: Submit Vulnerability
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'mutation { submitVulnerability(bountyId: 0, title: "Integer Overflow in Token Transfer", description: "Unchecked arithmetic in transfer function allows attacker to mint unlimited tokens", severity: CRITICAL, proofOfConcept: "1. Call transfer() with amount = MAX_UINT64\n2. Add small value causing overflow\n3. Balance wraps to small number\n4. Repeat to drain protocol") }'

# Step 3: Verify Submission
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'mutation { verifySubmission(submissionId: 0, approved: true, payoutAmount: 5000000000) }'

# Step 4: Claim Payout
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'mutation { claimPayout(submissionId: 0) }'

# Verify
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'query { submission(id: 0) { id status payoutAmount paid } }'
```

---

### TEST 2: Medium Bug - Rejected

```bash
# Step 1: Create Bounty
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'mutation { createBounty(contractAddress: "0xBETA_MEDIUM_CONTRACT_002", rewardPool: 5000000000, minSeverity: LOW) }'

# Step 2: Submit Vulnerability
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'mutation { submitVulnerability(bountyId: 1, title: "Missing Input Validation", description: "Function does not validate user input length, may cause DoS with large inputs", severity: MEDIUM, proofOfConcept: "1. Send input string with 1M characters\n2. Function consumes excessive gas\n3. Transaction fails but costs gas") }'

# Step 3: Reject Submission
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'mutation { verifySubmission(submissionId: 1, approved: false, payoutAmount: 0) }'

# Verify
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'query { submission(id: 1) { id status payoutAmount paid } }'
```

---

### TEST 3: Critical Bug - High Payout

```bash
# Step 1: Create Bounty
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'mutation { createBounty(contractAddress: "0xGAMMA_ENTERPRISE_CONTRACT_003", rewardPool: 20000000000, minSeverity: HIGH) }'

# Step 2: Submit Vulnerability
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'mutation { submitVulnerability(bountyId: 2, title: "Reentrancy Vulnerability in Withdrawal", description: "Missing CEI pattern allows recursive calls to drain contract balance before state update", severity: CRITICAL, proofOfConcept: "1. Deploy malicious contract with fallback function\n2. Call withdraw() which sends ETH before updating balance\n3. Fallback calls withdraw() again\n4. Repeat until contract drained\n5. PoC repo: github.com/example/reentrancy-poc") }'

# Step 3: Approve High Payout
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'mutation { verifySubmission(submissionId: 2, approved: true, payoutAmount: 15000000000) }'

# Step 4: Claim High Reward
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'mutation { claimPayout(submissionId: 2) }'

# Verify
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'query { submission(id: 2) { id status payoutAmount paid } bounty(id: 2) { id rewardPool remainingPool } }'
```

---

## Command Reference

### Query All Data

```bash
# List all bounties
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "{ bounties { id contractAddress rewardPool remainingPool minSeverity active } }"}'

# List all submissions
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "{ submissions { id bountyId title severity status payoutAmount paid } }"}'

# Get specific bounty
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "{ bounty(id: 0) { id contractAddress rewardPool remainingPool } }"}'

# Format with jq
curl -X POST http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID} \
  -H "Content-Type: application/json" \
  -d '{"query": "{ bounties { id rewardPool } }"}' | jq .
```

---

## Troubleshooting

### Chain/App ID Not Found

```bash
# Check if frontend is deployed
ls -la frontend/index.html

# Manually set IDs (from deployment output)
export CHAIN_ID="your_chain_id_here"
export APP_ID="your_app_id_here"
```

### Connection Refused

```bash
# Check if GraphQL service is running
curl http://localhost:8080

# Restart service
make serve
```

### GraphQL Errors

```bash
# Verify query syntax
# Use exact queries from this guide
# Check quotes are escaped properly (\")
```

### Port Already in Use

```bash
# Kill process using port 8080
lsof -ti:8080 | xargs kill -9

# Kill process using port 3000
lsof -ti:3000 | xargs kill -9
```

---

## Testing Checklist

- [ ] Environment variables exported correctly
- [ ] GraphQL service running on port 8080
- [ ] Frontend deployed and accessible
- [ ] TEST 1: Critical bug approved and paid
- [ ] TEST 2: Medium bug rejected
- [ ] TEST 3: High payout claimed
- [ ] All 3 bounties created successfully
- [ ] All 3 submissions processed correctly
- [ ] Total 20B tokens paid out
- [ ] Remaining pools calculated correctly

---

## Expected Results

After running all tests, verify:

- **3 Bounties created** (IDs: 0, 1, 2)
- **3 Submissions made** (IDs: 0, 1, 2)
- **Submission 0:** APPROVED, PAID (5B tokens)
- **Submission 1:** REJECTED
- **Submission 2:** APPROVED, PAID (15B tokens)
- **Total paid:** 20B tokens
- **Remaining pools:** Bounty 0 (5B), Bounty 1 (5B), Bounty 2 (5B)

---

**Testing complete!** All platform operations validated. For production deployment, run additional security audits and stress tests.