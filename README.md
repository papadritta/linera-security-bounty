# 🔒 Linera Security Bounty Platform

[![Linera Buildathon](https://img.shields.io/badge/Linera-Buildathon%20Wave%201-blue)](https://linera.io)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![Linera SDK](https://img.shields.io/badge/Linera%20SDK-0.16.0-green)](https://github.com/linera-io/linera-protocol)

**Real-time decentralized bug bounty platform on Linera microchains**

Submit vulnerabilities, verify findings, and claim rewards - all with instant finality powered by Linera's microchain architecture.

![Platform Demo](docs/screenshots/platform-overview.png)

---

## 🎯 Problem & Solution

**Problem:** Traditional bug bounty platforms are centralized, slow, and opaque. Researchers wait weeks for payouts, disputes lack transparency, and platforms take high fees.

**Solution:** Decentralized bounty platform leveraging Linera's instant finality:
- ⚡ **Instant verification** - No waiting for block confirmations
- 🔒 **Trustless escrow** - Smart contract holds rewards
- 🌐 **Full transparency** - All submissions on-chain
- 💰 **Zero platform fees** - Direct researcher-to-project payments

---

## ✨ Key Features

### For Security Researchers
- Submit vulnerabilities with proof-of-concept
- Track submission status in real-time
- Instant payout claims once approved
- Severity-based rewards (Low/Medium/High/Critical)

### For Project Owners
- Create bounty programs with custom reward pools
- Manual verification workflow
- Set minimum severity thresholds
- Close/pause programs anytime

### Technical Highlights
- **Linera Microchains** - Each bounty program can run on isolated chain
- **GraphQL API** - Rich query and mutation interface
- **WASM Compilation** - Secure, deterministic execution
- **Real-time Updates** - Instant state synchronization

---

## 🏗️ Architecture
```
┌─────────────────────────────────────────────────────────┐
│                    Frontend (HTML/JS)                    │
│                 GraphQL Client Interface                 │
└─────────────────────┬───────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│              Linera GraphQL Service (Port 8080)          │
│  ┌────────────────────────────────────────────────────┐ │
│  │  Queries: bounties, submissions, bountyById        │ │
│  │  Mutations: createBounty, submitVulnerability,     │ │
│  │             verifySubmission, claimPayout          │ │
│  └────────────────────────────────────────────────────┘ │
└─────────────────────┬───────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│              Smart Contract (WASM)                       │
│  ┌────────────────────────────────────────────────────┐ │
│  │  State Management:                                 │ │
│  │  - MapView<u64, Bounty>                           │ │
│  │  - MapView<u64, Submission>                       │ │
│  │  - Counters, ownership checks                     │ │
│  └────────────────────────────────────────────────────┘ │
└─────────────────────┬───────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│                  Linera Microchains                      │
│         (Instant finality, <1s block times)              │
└─────────────────────────────────────────────────────────┘
```

### Data Models

**Bounty:**
- `id`: Unique identifier
- `contract_address`: Target project
- `reward_pool`: Total funds available
- `min_severity`: Minimum accepted severity
- `creator`: Owner address
- `is_active`: Program status

**Submission:**
- `id`: Unique identifier
- `bounty_id`: Associated bounty
- `title`, `description`, `proof_of_concept`
- `severity`: Low | Medium | High | Critical
- `status`: Pending | Approved | Rejected
- `researcher`: Submitter address
- `payout_amount`: Reward if approved
- `paid`: Payout status

---

## 🚀 Quick Start

### Prerequisites
```bash
# Install Rust 1.85+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Linera CLI
cargo install linera-service --git https://github.com/linera-io/linera-protocol.git --tag v0.16.0

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Build & Deploy
```bash
# Clone repository
git clone https://github.com/papadritta/linera-security-bounty.git
cd linera-security-bounty

# Build application
make build

# Start local Linera network
linera net up

# Deploy application
make deploy

# Start GraphQL service (terminal 1)
make serve

# Start frontend (terminal 2)
make frontend

# Open browser
open http://localhost:3000
```

---

## 📖 Usage Examples

### Create Bounty Program
```graphql
mutation {
  createBounty(
    contractAddress: "0x1234...",
    rewardPool: "10000000000",
    minSeverity: "MEDIUM"
  )
}
```

### Submit Vulnerability
```graphql
mutation {
  submitVulnerability(
    bountyId: 1,
    title: "Integer Overflow in Transfer",
    description: "Unchecked arithmetic...",
    severity: "HIGH",
    proofOfConcept: "Steps to reproduce..."
  )
}
```

### Verify Submission
```graphql
mutation {
  verifySubmission(
    submissionId: 1,
    approved: true,
    payoutAmount: "5000000000"
  )
}
```

### Claim Payout
```graphql
mutation {
  claimPayout(submissionId: 1)
}
```

---

## 🧪 Testing
```bash
# Run all tests
make test

# Run linter
make lint

# Check formatting
make format

# Full validation
make check-all
```

**Test Coverage:**
- ✅ Contract operations (create, submit, verify, claim)
- ✅ State persistence and queries
- ✅ Access control and ownership checks
- ✅ Error handling

---

## 📁 Project Structure
```
linera-security-bounty/
├── security-bounty/           # Linera application
│   ├── src/
│   │   ├── lib.rs            # Types and interfaces
│   │   ├── state.rs          # State management
│   │   ├── contract.rs       # Core business logic
│   │   └── service.rs        # GraphQL API
│   ├── Cargo.toml
│   └── rust-toolchain.toml
├── frontend/                  # Web interface
│   └── index.html
├── scripts/                   # Build automation
│   ├── deploy.sh
│   └── test.sh
├── docs/                      # Documentation
│   └── screenshots/
├── Makefile                   # Build commands
├── README.md
├── CHANGELOG.md
└── LICENSE
```

---

## 🛣️ Roadmap

### Wave 1 (Current - MVP)
- ✅ Core bounty creation and submission
- ✅ Manual verification workflow
- ✅ GraphQL API
- ✅ Basic frontend interface

### Wave 2 (Next Phase)
- 🔄 **TEE Oracle Integration** - Automated PoC verification
- 🔄 **Multi-token Rewards** - Support ERC-20/native tokens
- 🔄 **Reputation System** - Researcher credibility scores
- 🔄 **Dispute Resolution** - Multi-sig arbitration

### Wave 3 (Future)
- 📋 Private vulnerability disclosure
- 📋 Multi-chain bounty programs
- 📋 AI-assisted severity classification
- 📋 DAO governance for platform rules

---

## 🔐 Security Considerations

**Current Implementation:**
- ✅ Access control on all state mutations
- ✅ Ownership verification for bounty creators
- ✅ Immutable submission records
- ✅ Safe arithmetic (no overflow)

**Production Requirements:**
- ⚠️ Formal verification of contract logic
- ⚠️ External audit of smart contracts
- ⚠️ Rate limiting on submissions
- ⚠️ Sybil resistance mechanisms

---

## 🤝 Contributing

This is a buildathon submission. For production deployments:

1. Fork repository
2. Create feature branch
3. Add tests for new features
4. Ensure `make check-all` passes
5. Submit pull request

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file

---

## 👤 Author

**papadritta**
- GitHub: [@papadritta](https://github.com/papadritta)
- Buildathon: Linera Wave 1

---

## 🙏 Acknowledgments

- **Linera Team** - For the incredible microchain architecture
- **Rust Community** - For the best development tools
- **Bug Bounty Hunters** - For inspiring this platform

---

## 📞 Support

- Issues: [GitHub Issues](https://github.com/papadritta/linera-security-bounty/issues)
- Discussions: [GitHub Discussions](https://github.com/papadritta/linera-security-bounty/discussions)

---

**Built with ❤️ for Linera Buildathon Wave 1**
