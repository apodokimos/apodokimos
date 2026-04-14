# Contributing to Apodokimos

Thank you for your interest in contributing to Apodokimos! This document outlines the process for contributing to this project.

## License Overview

This repository uses a multi-license approach:

- **AGPL-3.0**: Core protocol, chain pallets, indexer, CLI (`apodokimos-core`, `apodokimos-chain`, `apodokimos-indexer`, `apodokimos-cli`)
- **Apache-2.0**: SDK crates (`apodokimos-sdk`, `sdk-ts`)
- **CC0**: Protocol specifications, schemas, documentation, whitepapers

By contributing, you agree that your contributions will be licensed under the appropriate license for the component you are modifying.

## Developer Certificate of Origin (DCO)

All contributions must include a sign-off indicating you accept the DCO. This is done by adding a `Signed-off-by` line to your commit message:

```
feat: add claim validation logic

Signed-off-by: Your Name <your.email@example.com>
```

You can automatically add this line using the `-s` flag with `git commit`:

```bash
git commit -s -m "your commit message"
```

### Developer Certificate of Origin 1.1

```
Copyright (C) 2004, 2006 The Linux Foundation and its contributors.
Everyone is permitted to copy and distribute verbatim copies of this
license document, but changing it is not allowed.

Developer's Certificate of Origin 1.1

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

## How to Contribute

### Reporting Issues

1. Check if the issue already exists in the issue tracker
2. If not, create a new issue with:
   - Clear title and description
   - Steps to reproduce (for bugs)
   - Expected vs actual behavior (for bugs)
   - Your environment details (OS, Rust version, etc.)

### Submitting Changes

1. **Fork the repository** and create your branch from `main`
2. **Install dependencies**:
   ```bash
   # Rust tooling
   cargo install cargo-audit
   cargo install cargo-deny
   cargo install cargo-expand
   ```
3. **Make your changes**:
   - Follow existing code style
   - Add tests for new functionality
   - Update documentation as needed
4. **Run checks**:
   ```bash
   cargo fmt --check
   cargo clippy --deny warnings
   cargo test
   cargo audit
   cargo deny check
   ```
5. **Commit your changes** with DCO sign-off:
   ```bash
   git commit -s -m "type: description"
   ```
   Commit types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
6. **Push to your fork** and submit a Pull Request

### Pull Request Process

1. Ensure all CI checks pass
2. Request review from CODEOWNERS
3. Address review feedback
4. PR will be merged by maintainers once approved

## Code Style

- **Rust**: Follow standard Rust formatting (`cargo fmt`)
- **TypeScript**: Follow project ESLint/Prettier configuration
- **Documentation**: Use clear, concise language; prefer examples

## Questions?

Open an issue or discussion if you have questions not covered here.

## Code of Conduct

All contributors are expected to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md).
