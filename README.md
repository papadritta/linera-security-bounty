# 🔒 Linera Security Bounty Platform

[![Linera Buildathon](https://img.shields.io/badge/Linera-Buildathon%20Wave%201-blue)](https://linera.io)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![Linera SDK](https://img.shields.io/badge/Linera%20SDK-0.16.0-green)](https://github.com/linera-io/linera-protocol)

**Real-time decentralized bug bounty platform on Linera microchains**

Submit vulnerabilities, verify findings, and claim rewards - all with instant finality powered by Linera's microchain architecture.

![Platform Demo](docs/screenshots/platform-overview.png)

---

## Problem & Solution

**Problem:** Traditional bug bounty platforms are centralized, slow, and opaque. Researchers wait weeks for payouts, disputes lack transparency, and platforms take high fees.

**Solution:** Decentralized bounty platform leveraging Linera's instant finality:
- **Instant verification** - No waiting for block confirmations
- **Trustless escrow** - Smart contract holds rewards
- **Full transparency** - All submissions on-chain
- **Zero platform fees** - Direct researcher-to-project payments

---

## Key Features

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

## Architecture
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

## Getting Started

### Prerequisites
```bash
# Install Rust 1.85+ (if you don't have it)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Linera CLI
cargo install linera-service --git https://github.com/linera-io/linera-protocol.git --tag v0.16.0

# Add WASM support
rustup target add wasm32-unknown-unknown
```

#### Running It
>The correct workflow uses 3 terminals:
#### Terminal 1 (Network - Keep Running):
```bash
make network-up
# This will show 3 export commands - copy them
# Keep this terminal running (don't close it!)
```
#### Terminal 2 (Build & Deploy):
```bash
# Clone repository
git clone https://github.com/papadritta/linera-security-bounty.git
cd linera-security-bounty

# Paste the 3 export commands here:
export LINERA_WALLET=/tmp/.tmpXXX/wallet.json
export LINERA_KEYSTORE=/tmp/.tmpXXX/keystore
export LINERA_STORAGE=rocksdb:/tmp/.tmpXXX/storage

# Verify they're set:
make check-env

# Build and deploy:
make build
make test
make deploy # Your app ID: 15dfa174fe0b1d824f855e96edfc26165c47f493deba15a72adc1ce1864d91b3
linera wallet show # Chain ID: 290abb2c4e719b4eff20f19414ef6305d5eea70075177fdd78b06bb6124a7b4e
# Start GraphQL service (keep running):
make serve
```
#### Terminal 3 (Frontend):
```bash
make frontend
```
> Then open your browser: http://localhost:3000

---

## 📖 Usage Examples
> Once it's running, you can interact with it via GraphQL at http://localhost:8080

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

### Verify a Submission (Program Owner)
```graphql
mutation {
  verifySubmission(
    submissionId: 1,
    approved: true,
    payoutAmount: "5000000000"
  )
}
```

### Claim Your Reward
```graphql
mutation {
  claimPayout(submissionId: 1)
}
```

---

## Development Workflow
```bash
# Make your changes in security-bounty/src/

# Format the code
make format-fix

# Run all checks
make check-all

# Rebuild
make build

# Redeploy
make deploy
```

## Testing
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
- Contract operations (create, submit, verify, claim)
- State persistence and queries
- Access control and ownership checks
- Error handling

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
│   └── linera_icon.png
├── scripts/                   # Build automation
│   ├── deploy.sh
├── docs/                      # Documentation
│   └── screenshots/
├── Makefile                   # Build commands
├── README.md
├── CHANGELOG.md
└── LICENSE
```
## Troubleshooting

#### Network won't start?
```bash
make network-status  # Check what's wrong
make network-down    # Stop any stuck processes
make network-up      # Try again
```

#### Forgot to export variables?
> After make network-up, look for output like this:
```bash
export LINERA_WALLET=/tmp/.tmpXXX/wallet.json
export LINERA_KEYSTORE=/tmp/.tmpXXX/keystore  
export LINERA_STORAGE=rocksdb:/tmp/.tmpXXX/storage
```
Copy ALL THREE lines exactly as shown and paste them in your terminal.

#### Check if they're set:
```bash
make check-env
```
If you see "All set!", you're good to go!

#### Port already in use?
```bash
# Kill whatever's using port 8080
lsof -ti:8080 | xargs kill -9

# Kill whatever's using port 3000
lsof -ti:3000 | xargs kill -9
```
#### Build errors?
```bash
make clean
make build
```
---

## What's Next? - Roadmap:

### Current version (Wave 1):
- ✅ **Bounty creation with reward pools**
- ✅ **Vulnerability submission system**
- ✅ **Manual verification workflow**
- ✅ **Payout claiming mechanism**
- ✅ **GraphQL API with full CRUD**
- ✅ **Web-based frontend interface**
- ✅ **Real-time updates via Linera microchains**

### Next Iteration (Wave 2):
- 🔄 **Enhanced UI/UX** - Better filtering, search, and dashboards
- 🔄 **Notification System** - Webhook integration for status updates
- 🔄 **Submission Analytics** - Stats and reporting dashboard
- 🔄 **Public Testnet Deployment** - Wider community testing

### Way to grow (Future Enhancements):
- 💭 **Reputation System** - Track researcher credibility and history
- 💭 **Multi-token Support** - ERC-20 and custom token rewards
- 💭 **Dispute Resolution** - Community-driven arbitration
- 💭 **Private Programs** - Confidential vulnerability disclosure
- 💭 **TEE Oracle Integration** - Automated PoC verification
- 💭 **Multi-chain Support** - Cross-chain bounty programs
- 💭 **AI Classification** - ML-assisted severity scoring
- 💭 **DAO Governance** - Decentralized platform rules

---

## Security Notes

**Current Implementation:**
- Access control on all state mutations
- Ownership verification for bounty creators
- Immutable submission records
- Safe arithmetic (no overflow)

**Production Requirements:**
- Formal verification of contract logic
- External audit of smart contracts
- Rate limiting on submissions
- Sybil resistance mechanisms

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
