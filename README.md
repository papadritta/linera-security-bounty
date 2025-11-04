# Linera Security Bounty Platform - Wave 2
> Production-ready decentralized bug bounty platform built for instant trust and transparency.

<img width="1128" height="191" alt="OG_github" src="https://github.com/user-attachments/assets/78585cde-4bb1-4194-8221-071d4485648a" />

[![Linera Buildathon](https://img.shields.io/badge/Linera-Buildathon%20Wave%202-blue)](https://linera.io)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![Linera SDK](https://img.shields.io/badge/Linera%20SDK-0.16.0-green)](https://github.com/linera-io/linera-protocol)

---

## Wave 2 Status

**Wave 1:** Functional prototype with core features (archived)  
**Wave 2:** Production deployment with improved architecture

- Live deployment on dedicated infrastructure
- React frontend with modern UX (port 5173)
- Modular architecture with production-grade patterns
- Real-world pilot projects (TODO: add details)
- Enhanced security and error handling

**Why Linera?** Microchains provide isolated state, instant finality, and predictable execution—critical for time-sensitive security workflows.

---

## Core Value Proposition

**Linera Security Bounty** connects developers and security researchers directly. No waiting, no middlemen, no trust issues.

- Instant finality: vulnerabilities confirmed in milliseconds
- Real-time collaboration: all parties see updates instantly
- Zero platform fees: direct researcher-to-project payouts
- Linera microchains: scalable, efficient, secure

## Comparison

| Feature | HackerOne/Immunefi | Linera Security Bounty |
|---------|-------------------|------------------------|
| Payout Time | 21-45 days | < 1 second |
| Platform Fees | 20-30% | 0% |
| Transparency | Opaque queue | Full on-chain |
| Proof of Work | Off-chain records | Immutable on-chain |
| Congestion | Single platform | Isolated microchains |
| Trust Model | Platform middleman | Smart contract escrow |
| Researcher Control | Platform decides | Direct verification |

Cyberattacks move fast. This platform helps you react faster.

---

## Documentation

- [Architecture Overview](./docs/ARCHITECTURE.md) - System design and technical decisions
- [Project Overview](./docs/PROJECT_OVERVIEW.md) - Vision, roadmap, business model
- [Security Policy](./docs/SECURITY.md) - Responsible disclosure guidelines
- [Changelog](./docs/CHANGELOG.md) - Development progress

---

## Live Demo

<!-- TODO: Add deployment URL and set a Product page-->
**Frontend:** `https://app.bug3.io`  
**GraphQL API:** `https://api.bug3.io`  
**Product page:** `https://bug3.io`

[Demo Video](TODO: NEW Demo with all new futures)

---

## Target Users

**Researchers:** Get paid instantly. Keep 100% of earnings.  
**Projects:** Deploy bug bounties in seconds. Pay only for verified vulnerabilities.  
**Validators:** Demonstrate microchain capabilities with real security use case.  
**Developers:** Reference implementation for production Linera applications.

---

## Quick Start

**Frontend:** `https://app.bug3.io`  

Full implementation available in private repository. Public repo contains architecture and reference materials for Wave 2 submission.

---

## Evolution

### Wave 1 (Complete)
- Core smart contract implementation
- GraphQL API with full CRUD
- Basic HTML frontend
- Local development workflow
- Working prototype

### Wave 2 (Current)
- Production server deployment
- React-based frontend
- Modular contract architecture
- Enhanced error handling and security
- Real-world pilot projects
- Performance monitoring

### Future
- Authentication system (wallet integration)
- Reputation system for researchers
- Multi-token support
- Automated PoC verification (TEE/Oracle)
- Cross-chain bounty aggregation
- Notification webhooks
- Analytics dashboard

---

## Technology Stack

- Smart Contracts: Rust + Linera SDK v0.16.0
- Frontend: React (Vite)
- API: GraphQL (Linera native)
- Deployment: Dedicated server infrastructure
- Storage: Linera microchain state

---

## Features

### Current
- Create bounty programs with reward pools
- Submit vulnerabilities with proof-of-concept
- Manual verification workflow
- Instant payout claims
- Severity-based rewards (Low/Medium/High/Critical)
- Real-time GraphQL subscriptions
- Isolated microchain execution

### In Development
- Enhanced UI/UX with filtering and search
- Researcher reputation tracking
- Advanced analytics and reporting
- Automated security checks
- Multi-project dashboard

---

## Architecture
```
Frontend (React) → GraphQL API → Smart Contract (WASM) → Linera Microchains
```

- Instant finality: sub-second transaction confirmation
- Isolated state: each bounty program on separate microchain
- Scalable: no congestion or gas competition
- Transparent: all operations on-chain and auditable

See [Architecture Documentation](./docs/ARCHITECTURE.md) for detailed design.

---

## Security

- Access control: owner-based permissions on all state mutations
- Immutable records: submissions cannot be altered post-creation
- Safe arithmetic: checked math prevents overflow
- Audit ready: production-grade error handling

For vulnerability disclosure: [SECURITY.md](./docs/SECURITY.md)

---

## Contributing

Wave 2 implementation is private to protect competitive advantages. For collaboration:

- Technical questions: open GitHub issue
- Partnership inquiries: contact via GitHub profile
- Security reports: see [SECURITY.md](./docs/SECURITY.md)

---

## Repository Structure
```
linera-security-bounty/
├── docs/                  # Architecture and documentation
│   ├── ARCHITECTURE.md
│   ├── PROJECT_OVERVIEW.md
│   ├── CHANGELOG.md
│   └── SECURITY.md
├── README.md
└── LICENSE
```

Implementation code in private repository. Contact for access.

---

## Status

**Wave 1:** Complete (archived in `wave1-archive` branch)  
**Wave 2:** Active development with production deployment  

---

## License

MIT License © 2025 - see [LICENSE](./LICENSE)

---

## Author

**papadritta**  
GitHub: [@papadritta](https://github.com/papadritta)  
Buildathon: Linera Wave 2

---

## Acknowledgments

- Linera Team: revolutionary microchain architecture
- Rust Community: exceptional tooling and ecosystem
- Wave 1 & Wave 2 Participants: early feedback and validation

---

## Links

- [Wave 1 Archive](https://github.com/papadritta/linera-security-bounty/tree/wave1-archive) - Prototype with simple features
- [Linera Protocol](https://linera.io) - Microchain documentation
- [Project Documentation](./docs/) - Technical details

---

Built for Linera Buildathon Wave 2

<img width="1128" alt="OG_github-2" src="https://github.com/user-attachments/assets/2b17ad32-9414-45fb-8407-2b47c5c65609" />
