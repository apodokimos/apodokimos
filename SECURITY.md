# Security Policy

## Supported Versions

The following versions of Apodokimos are currently supported with security updates:

| Version | Supported          |
|---------|-------------------|
| main    | :white_check_mark: |
| < v1.0  | :x:                |

## Reporting Security Vulnerabilities

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, report security vulnerabilities via:

- **Email**: security@apodokimos.org (encrypted preferred)
- **Keybase**: @apodokimos-sec
- **PGP**: `0xAPODO_SECURITY_KEY_ID` (see below)

### What to Include

When reporting a vulnerability, please include:

1. **Description** of the vulnerability
2. **Steps to reproduce** (proof of concept if possible)
3. **Impact** assessment (what could an attacker do?)
4. **Affected components** (which pallets, modules, etc.)
5. **Suggested fix** (if you have one)

### Response Timeline

We aim to respond to security reports within **48 hours** and will keep you updated on our progress:

| Phase | Timeline | Action |
|-------|----------|--------|
| Initial Response | 48 hours | Acknowledge receipt |
| Assessment | 1 week | Validate and assess severity |
| Fix Development | 2-4 weeks | Develop and test fix |
| Disclosure | Coordinated | Public disclosure with fix |

## PGP Key

```
-----BEGIN PGP PUBLIC KEY BLOCK-----

[Replace with actual security team PGP key]

-----END PGP PUBLIC KEY BLOCK-----
```

## Security Considerations

### Critical Components

The following components require additional scrutiny:

- **pallet-sbt-reputation**: SBT non-transferability enforcement
- **pallet-governance**: Quadratic voting implementation
- **WeightFunction**: Economic security of claim scoring
- **Oracle connectors**: External data validation

### Known Security Properties

1. **SBT non-transferability**: Enforced at runtime level
2. **Sybil resistance**: Quadratic voting with field-specific SBTs
3. **Oracle manipulation**: Multi-source oracle aggregation with conflict detection

## Bug Bounty

Apodokimos operates a bug bounty program. Rewards are paid in governance SBTs (not transferable tokens) to align incentives with protocol values.

| Severity | Reward |
|----------|--------|
| Critical | 1000 governance SBT |
| High     | 500 governance SBT |
| Medium   | 200 governance SBT |
| Low      | 50 governance SBT |

See [BUG_BOUNTY.md](BUG_BOUNTY.md) for full program details and scope.

## Past Security Advisories

Security advisories will be published here once the protocol reaches mainnet.

## Disclosure Policy

We follow a **coordinated disclosure** policy:

1. Reporter submits vulnerability privately
2. We validate and develop fix
3. Fix is deployed to testnet/mainnet
4. Public disclosure 30 days after fix is available
5. Reporter is credited (unless they wish to remain anonymous)

## Security Audits

Planned security audits per [TODO.md](TODO.md) Phase 9:

- External audit of all four pallets (T-02)
- Formal verification of SBT non-transferability (T-03)

Audit reports will be published as Arweave-anchored Apodokimos claims.
