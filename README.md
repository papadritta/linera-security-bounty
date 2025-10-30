# Linera Security Bounty Platform
> A next-generation decentralized bug bounty platform built for instant trust and transparency.
<img width="1128" height="191" alt="OG_github" src="https://github.com/user-attachments/assets/78585cde-4bb1-4194-8221-071d4485648a" />

[![CI](https://github.com/papadritta/linera-security-bounty/actions/workflows/ci.yml/badge.svg)](https://github.com/papadritta/linera-security-bounty/actions/workflows/ci.yml)
[![Security Audit](https://img.shields.io/badge/security-audited-brightgreen.svg)](./docs/SECURITY.md)
[![Linera Buildathon](https://img.shields.io/badge/Linera-Buildathon%20Wave%201-blue)](https://linera.io)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![Linera SDK](https://img.shields.io/badge/Linera%20SDK-0.16.0-green)](https://github.com/linera-io/linera-protocol)

---
> **Why Linera?** Microchains give isolated state, instant finality, and predictable execution—ideal for real-time bounty flows.


## TL;DR — Why it matters

**Linera Security Bounty** connects developers and security researchers directly — no waiting, no middlemen, no trust issues.

- **Instant finality:** vulnerabilities confirmed in milliseconds  
- **Real-time collaboration:** projects and researchers see updates instantly  
- **No platform fees (Wave 1):** direct researcher-to-project payouts  
- **Built on Linera microchains:** scalable, efficient, and secure  

## How We Stack Up

| Feature | HackerOne/Immunefi | Linera Security Bounty |
|---------|-------------------|------------------------|
| **Payout Time** | 21-45 days | < 1 second |
| **Platform Fees** | 20-30% | 0% (Wave 1) |
| **Transparency** | Opaque queue | Full on-chain |
| **Proof of Work** | Off-chain records | Immutable on-chain |
| **Congestion** | Single platform | Isolated microchains |
| **Trust Model** | Platform middleman | Smart contract escrow |
| **Researcher Control** | Platform decides | Direct verification |

In 2025, cyberattacks move fast — this platform helps you react faster.

---

## Documentation
See the [Project Overview](./docs/PROJECT_OVERVIEW.md) for a detailed description of the platform’s purpose, architecture, and long-term vision — intended for collaborators, contributors, and potential partners.

---

https://github.com/user-attachments/assets/2e9718c2-692f-493a-9f9d-45b9b2d4a6eb

- **For researchers:** Get paid in seconds, not weeks. Keep 100% of your earnings.
- **For projects:** Deploy bug bounties faster than you can tweet. Pay only for verified bugs.
- **For Linera team:** We showcase microchains' killer feature - instant finality for real-time coordination.
- **For investors:** $50B market. Zero-take-rate land grab. Network effects. This is Uniswap for security.

## See It In Action [▶️ Full Demo video](https://youtu.be/ytXKSmfHEsM) [▶️ Workflow Demo video](https://youtu.be/JfLQPf6GSUI)

---

## Contents
- [TL;DR — Why it matters](#tldr--why-it-matters)
- [Documentation](#documentation)
- [Problem & Solution](#problem--solution)
- [Key Features](#key-features)
- [Architecture](#architecture)
- [On-chain data structures](#on-chain-data-structures)
- [Getting Started](#getting-started)
- [Running it](#running-it)
- [Usage examples](#usage-examples)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Project Structure](#project-structure)
- [Troubleshooting](#troubleshooting)
- [What’s next — Roadmap](#whats-next--roadmap)
- [Security Notes](#security-notes)
- [Contributing](#contributing)
- [License](#license)
- [Author](#author)
- [Acknowledgments](#acknowledgments)
- [Support](#support)

---

## Problem & Solution
> Security moves in milliseconds — bounty verification shouldn’t take weeks.

**Problem.** Today’s bounty platforms are centralized, slow, and opaque: weeks-long payouts, opaque queues, and high fees.

**Solution.** A decentralized bounty protocol on Linera with instant finality and transparent state:
- **Instant verification:** no waiting for block confirmations
- **Trustless escrow:** smart contract holds rewards
- **Full transparency:** all submissions on-chain
- **Zero platform fees (Wave 1):** direct researcher-to-project payments

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
- **Linera microchains:** each bounty program runs on an isolated microchain  
- **GraphQL API:** rich query and mutation interface  
- **WASM compilation:** secure, deterministic execution  
- **Real-time updates:** instant state synchronization

---

## Architecture
```
┌─────────────────────────────────────────────────────────┐
│                    Frontend (HTML/JS)                   │
│                 GraphQL client interface                │
└─────────────────────┬───────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│             Linera GraphQL service (port 8080)          │
│  ┌────────────────────────────────────────────────────┐ │
│  │  queries: bounties, submissions, bountyById        │ │
│  │  mutations: createBounty, submitVulnerability,     │ │
│  │             verifySubmission, claimPayout          │ │
│  └────────────────────────────────────────────────────┘ │
└─────────────────────┬───────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│                  smart contract (wasm)                  │
│  ┌────────────────────────────────────────────────────┐ │
│  │  state management:                                 │ │
│  │  - MapView<u64, Bounty>                            │ │
│  │  - MapView<u64, Submission>                        │ │
│  │  - counters, ownership checks                      │ │
│  └────────────────────────────────────────────────────┘ │
└─────────────────────┬───────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│                  Linera microchains                     │
│                  (instant finality)                     │
└─────────────────────────────────────────────────────────┘
```

---

## On-chain data structures

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

See [`state.rs`](./security-bounty/src/state.rs) for storage layout and [`service.rs`](./security-bounty/src/service.rs) for the GraphQL schema.

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

# Clone repository
git clone https://github.com/papadritta/linera-security-bounty.git
cd linera-security-bounty
```

### Running it [▶️ Demo video](https://youtu.be/E7QBFVPIwT0)
> The workflow uses three terminals running in parallel. 
> Each terminal performs a separate role: network, deployment, and frontend.

#### Terminal 1 — network (keep running)
```bash
make network-up
# This will show 3 export commands — copy them
# Keep this terminal running (don’t close it!)
```
#### Terminal 2 — build & deploy
```bash
# Paste the 3 export commands here:
export LINERA_WALLET=/tmp/.tmpXXX/wallet.json
export LINERA_KEYSTORE=/tmp/.tmpXXX/keystore
export LINERA_STORAGE=rocksdb:/tmp/.tmpXXX/storage

# Verify they’re set:
make check-env

# Build and deploy:
make build && make deploy && make serve
```
#### Terminal 3 — frontend
```bash
make frontend
```

---

## Usage examples

After `make serve` and `make frontend`:

> Services: GraphQL at **http://localhost:8080**, Web UI at **http://localhost:3000**.

For complete testing workflows and detailed examples, see **[Tests.md](./docs/Tests.md)** which covers:

- **UI Testing** - Step-by-step browser-based testing
- **CLI Testing** - Command-line interface testing  
- **GraphQL Testing** - Direct API testing via GraphiQL
- **3 Complete Test Scenarios** - Critical approved, medium rejected, high-value payout

### Quick Start Examples

**UI:** Browse to **http://localhost:3000** to create bounties, submit findings, and verify/claim.

**CLI:** Use `linera query-application` to interact with the platform:
```bash
# List all bounties
linera query-application \
  --chain-id $CHAIN_ID \
  --application-id $APP_ID \
  'query { bounties { id contractAddress rewardPool } }'
```

**GraphQL:** Open the application-specific GraphQL IDE:
```
http://localhost:8080/chains/{CHAIN_ID}/applications/{APP_ID}
```

**Example mutation:**
```graphql
mutation {
  createBounty(
    contractAddress: "0x1234...",
    rewardPool: 10000000000,
    minSeverity: MEDIUM
  )
}
```
---

**[View complete testing guide](./docs/Tests.md)**

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

---

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

**Test scope:**
- Contract operations (create, submit, verify, claim)
- State persistence and queries
- Access control and ownership checks
- Error handling

---

## Project Structure
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
│   └── linera_logo.svg
├── scripts/                   # Build automation
│   ├── deploy.sh
├── docs/                      # Documentation
│   ├── PROJECT_OVERVIEW.md                      
│   ├── screenshots/
│   ├── CHANGELOG.md
│   ├── SECURITY.md
│   └── Tests.md  
├── Makefile                   # Build commands
├── README.md
└── LICENSE
```

---

## Troubleshooting

### Network won't start?
```bash
make network-status  # Check what's wrong
make network-down    # Stop any stuck processes
make network-up      # Try again
```

### Forgot to export variables?
> After make network-up, look for output like this:
```bash
export LINERA_WALLET=/tmp/.tmpXXX/wallet.json
export LINERA_KEYSTORE=/tmp/.tmpXXX/keystore  
export LINERA_STORAGE=rocksdb:/tmp/.tmpXXX/storage
```
Copy ALL THREE lines exactly as shown and paste them in your terminal.

### Check if they're set:
```bash
make check-env
```
If you see "All set!", you're good to go!

### Port already in use?
```bash
# Kill whatever's using port 8080
lsof -ti:8080 | xargs kill -9

# Kill whatever's using port 3000
lsof -ti:3000 | xargs kill -9
```
### Build errors?
```bash
make clean
make build
```
---

## What’s next — Roadmap

### Current version (Wave 1):
- ✅ **Bounty creation with reward pools**
- ✅ **Vulnerability submission system**
- ✅ **Manual verification workflow**
- ✅ **Payout claiming mechanism**
- ✅ **GraphQL API with full CRUD**
- ✅ **Web-based frontend interface**
- ✅ **Real-time updates via Linera microchains**

### Next iteration (Wave 2):
- 🔄 **Enhanced UI/UX:** better filtering, search, and dashboards
- 🔄 **Notification system:** webhook integration for status updates
- 🔄 **Submission analytics:** stats and reporting dashboard
- 🔄 **Public testnet deployment:** wider community testing

### Future enhancements:
- 💭 **Reputation System:** Track researcher credibility and history
- 💭 **Multi-token Support:** ERC-20 and custom token rewards
- 💭 **Dispute Resolution:** Community-driven arbitration
- 💭 **Private Programs:** Confidential vulnerability disclosure
- 💭 **TEE Oracle Integration:** Automated PoC verification
- 💭 **Multi-chain Support:** Cross-chain bounty programs
- 💭 **AI Classification:** ML-assisted severity scoring
- 💭 **DAO Governance:** Decentralized platform rules

---

## Security Notes

**Responsible disclosure:** See our [Security Policy](./docs/SECURITY.md) or report via [GitHub Security Advisories](../../security/advisories/new). We will acknowledge, triage, and coordinate fixes before public disclosure.

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

## Contributing

This is a buildathon submission. For production deployments:

1. Fork repository
2. Create feature branch
3. Add tests for new features
4. Ensure `make check-all` passes
5. Submit pull request

---

## License

MIT License © 2025 - see [LICENSE](LICENSE)

---

## Author

**papadritta**
- GitHub: [@papadritta](https://github.com/papadritta)
- Buildathon: Linera Wave 1

---

## Acknowledgments

- **Linera Team:** For the incredible microchain architecture
- **Rust Community:** For the best development tools
- **Bug Bounty Hunters:** For inspiring this platform

---

## Support

- Issues: [GitHub Issues](https://github.com/papadritta/linera-security-bounty/issues)
- Discussions: [GitHub Discussions](https://github.com/papadritta/linera-security-bounty/discussions)

---
**Built with ❤️ for Linera Buildathon Wave 1**

<img width="1128" height="191" alt="OG_github-2" src="https://github.com/user-attachments/assets/2b17ad32-9414-45fb-8407-2b47c5c65609" />


<!-- CI test -->