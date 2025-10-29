# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in the Linera Security Bounty platform, please report it responsibly.

### How to Report

1. **Do not** open a public GitHub issue for security vulnerabilities
2. Report via [GitHub Security Advisories](https://github.com/papadritta/linera-security-bounty/security/advisories/new)
3. Or email directly with details about the vulnerability

### What to Include

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact assessment
- Suggested fix (if you have one)

### Response Timeline

- **Acknowledgment:** Within 48 hours
- **Initial Assessment:** Within 1 week
- **Fix Timeline:** Depends on severity, we will keep you updated

### Disclosure Policy

We follow coordinated disclosure:
- We will work with you to understand and fix the issue
- Public disclosure only after a fix is deployed
- Credit given to researchers who responsibly disclose

## Security Considerations

### Current Implementation (Wave 1)

This is a proof-of-concept buildathon submission. Current security measures:

- Access control on all state mutations
- Ownership verification for bounty creators
- Immutable submission records
- Safe arithmetic operations (overflow protection)
- Double-claim prevention

### Known Limitations

Wave 1 focuses on core functionality. Production deployment requires:

- External security audit of smart contracts
- Formal verification of critical logic
- Rate limiting on submissions
- Sybil resistance mechanisms
- Enhanced access control patterns

### Not Production Ready

This code is intended for demonstration and testing. Do not use in production without:

1. Comprehensive security audit
2. Formal verification
3. Extended testing period
4. Additional safety mechanisms

## Scope

Security reports should focus on:

- Smart contract vulnerabilities
- Access control bypasses
- State manipulation attacks
- GraphQL API vulnerabilities
- Denial of service vectors

Out of scope:

- Social engineering attacks
- Third-party dependencies (report to their maintainers)
- Physical security issues

## Recognition

We appreciate security researchers who help improve this platform. Responsible disclosure will be acknowledged in:

- Project documentation
- Security advisories
- Public thanks (with your permission)

## Questions

For non-security questions, use [GitHub Issues](https://github.com/papadritta/linera-security-bounty/issues) or [Discussions](https://github.com/papadritta/linera-security-bounty/discussions).