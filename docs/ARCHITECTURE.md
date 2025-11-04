# Architecture Overview - Wave 2

## System Design

Linera Security Bounty is built on microchain architecture for isolated state management and instant finality.

### Core Components
```
┌─────────────────────────────────────────┐
│          Frontend React (port 5173)     │
│     Modern UI with real-time updates    │
└──────────────┬──────────────────────────┘
               │ GraphQL
               ▼
┌─────────────────────────────────────────┐
│       Linera GraphQL Service            │
│  Queries & Mutations API (port 8080)    │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│      Smart Contract (WASM)              │
│   State Management & Business Logic     │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│         Linera Microchains (SDK v0.16.0)│
│      Instant Finality & Isolation       │
└─────────────────────────────────────────┘
```

### State Management

The platform uses Linera's `MapView` for efficient key-value storage:

- **Bounties**: Indexed by ID, stores reward pools and metadata
- **Submissions**: Indexed by ID, linked to bounty IDs
- **Counters**: Atomic incrementing for unique IDs

### Security Model

- **Access Control**: Owner-based permissions on all mutations
- **Immutable Records**: Submissions cannot be altered after creation
- **Safe Arithmetic**: Checked math operations prevent overflow
- **State Validation**: All state transitions validated before commit

### Scalability

Each bounty program can run on its own microchain:
- **Isolated State**: No cross-chain interference
- **Parallel Execution**: Multiple bounties process simultaneously
- **Predictable Performance**: No congestion or gas wars

## Wave 2 Improvements

### Architecture Evolution

<!-- TODO: Add specific technical improvements -->
1. **Modular Contract Design**: Separated concerns into distinct modules
2. **Enhanced Error Handling**: Comprehensive error types with context
3. **Production Patterns**: Proper logging, validation, and recovery
4. **Dedicated Server Deployment**: Production-grade hosting infrastructure
5. **TODO: Add performance optimizations**
6. **TODO: Add security improvements**
7. **TODO: Add storage improvements**

### Future Architecture

- **Multi-token Support**: Abstract payment layer for various tokens
- **Reputation System**: Decentralized researcher credibility tracking
- **Oracle Integration**: Automated PoC verification via TEE
- **Cross-chain Bridge**: Multi-chain bounty aggregation
- **Authentication System**: User login/logout with wallet integration

## Technical Decisions

### Why Linera?

- **Instant Finality**: Sub-second confirmation for time-sensitive security work
- **Microchains**: Isolated execution prevents DOS and congestion
- **GraphQL Native**: Rich query interface without custom indexers
- **WASM Runtime**: Secure, deterministic contract execution

### Trade-offs

**Chosen**: Simplicity and security over feature complexity
**Rationale**: Security bounties require trust - start minimal, scale carefully

## Deployment Architecture

### Wave 2 Deployment
```
Dedicated Server Infrastructure
├── Linera SDK v0.16.0
├── GraphQL Endpoints (public access)
└── Bounty Microchains (isolated programs)
```

<!-- TODO: Add deployment URL and access details -->

### Production (Future)

- Multi-region deployment
- HSM key management for high-value bounties
- Monitoring and alerting infrastructure
- Automated backup and recovery

## Integration Points

### External Systems

- **Notification Service**: Webhook integration for status updates
- **Analytics**: Event streaming for dashboard metrics  
- **Identity**: Optional researcher verification system

### API Surface

GraphQL schema provides:
- Queries: Fetch bounties, submissions, statistics
- Mutations: Create, verify, claim operations
- Subscriptions: Real-time state updates (future)

## Performance Characteristics

- **Bounty Creation**: < 1 second
- **Submission**: < 1 second  
- **Verification**: < 1 second (manual) / instant (future automated)
- **Payout**: < 1 second after approval

<!-- TODO: Add real performance metrics from deployment -->

## Security Considerations

Implementation details omitted for IP protection. Key principles:
- Defense in depth
- Principle of least privilege
- Fail-safe defaults
- Audit logging for all critical operations

<!-- TODO: Add Wave 2 security enhancements (high-level only) -->

---

*All detailed implementation on the private repository, contact for partnership inquiries.*
