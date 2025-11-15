# CLAUDE.md - AI Assistant Guide for Polkadot SDK

This document provides comprehensive guidance for AI assistants (like Claude) working with the Polkadot SDK repository. It covers repository structure, development workflows, coding conventions, and best practices.

---

## Table of Contents

1. [Repository Overview](#repository-overview)
2. [Repository Structure](#repository-structure)
3. [Development Setup](#development-setup)
4. [Coding Standards](#coding-standards)
5. [Development Workflow](#development-workflow)
6. [Testing](#testing)
7. [Documentation](#documentation)
8. [Release Process](#release-process)
9. [Common Tasks](#common-tasks)
10. [Important Files & Locations](#important-files--locations)

---

## Repository Overview

The **Polkadot SDK** is a large monorepo that provides all components needed to build on the Polkadot network. It is the amalgamation of three previously separate repositories:

- **Substrate**: Core blockchain framework for building custom blockchains
- **Polkadot**: Relay chain implementation that secures the network
- **Cumulus**: Parachain framework for building Polkadot parachains
- **Bridges**: Cross-chain bridge implementations (including Snowbridge for Ethereum)

**Key Facts:**
- License: GPL-3.0-only (Polkadot, Cumulus, Bridges); Apache 2.0 + GPL-3.0 split for Substrate
- Language: Rust (Edition 2021)
- Main Branch: `master`
- Release Cadence: Quarterly stable releases (every 3 months)
- Release Format: `stableYYMM` (e.g., `stable2412`)
- Documentation: https://paritytech.github.io/polkadot-sdk/master/
- Community: StackExchange, Matrix, Telegram, Discord

---

## Repository Structure

### Top-Level Directories

```
/home/user/patient-x/
├── substrate/          # Core blockchain framework (FRAME, primitives, client)
├── polkadot/          # Relay chain implementation (node, runtime, XCM)
├── cumulus/           # Parachain framework (client, pallets, runtimes)
├── bridges/           # Cross-chain bridges (modules, relays, Snowbridge)
├── templates/         # Project templates (minimal, parachain, solochain)
├── umbrella/          # Unified polkadot-sdk facade crate
├── docs/              # Official documentation and guidelines
├── prdoc/             # Pull request documentation database
├── scripts/           # Build and utility scripts
├── docker/            # Docker configurations
└── .github/           # CI/CD workflows and configurations
```

### Major Components

#### Substrate (`/substrate`)

**Core Framework for Blockchain Development**

- **frame/** (~100 pallets): FRAME runtime system
  - Consensus: aura, babe, grandpa
  - Governance: democracy, collective, conviction-voting
  - Assets: assets, asset-conversion
  - Staking: staking, delegated-staking
  - Identity, Treasury, Elections, etc.

- **primitives/** (43 crates): Core runtime primitives
  - `sp-core`: Core types and traits
  - `sp-runtime`: Runtime types and utilities
  - `sp-api`: Runtime API framework
  - `sp-consensus-*`: Consensus algorithms
  - `sp-state-machine`: State management

- **client/** (35+ modules): Node client implementation
  - Network: network, network-gossip, mixnet
  - Consensus: consensus subsystems
  - RPC: rpc, rpc-api, rpc-servers, rpc-spec-v2
  - Storage: db, state-db
  - Executor: executor modules
  - Transaction pool, keystore, block builder

#### Polkadot (`/polkadot`)

**Relay Chain Implementation**

- **node/** (19 subsystems): Core validator node
  - **core/**: PVF, backing, approval voting, dispute coordination
  - **network/**: 12 networking modules
  - **subsystem/**: Subsystem infrastructure

- **runtime/**: Multiple runtimes (rococo, westend, test-runtime)

- **xcm/** (11 subdirectories): Cross-Consensus Messaging
  - pallet-xcm, xcm-builder, xcm-executor
  - xcm-simulator for testing

- **cli/**: Command-line interface
- **rpc/**: JSON-RPC endpoints

**Key Binaries:**
- `polkadot`: Main relay chain node
- `polkadot-execute-worker`: PVF execution worker
- `polkadot-prepare-worker`: PVF preparation worker

#### Cumulus (`/cumulus`)

**Parachain Development Framework**

- **client/** (15 modules): Parachain client components
  - Collator, consensus (aura, relay-chain)
  - Relay chain interfaces
  - Parachain inherent, PoV recovery

- **pallets/** (10 pallets): Parachain-specific pallets
  - parachain-system, dmp-queue, xcmp-queue
  - collator-selection, aura-ext

- **parachains/**: Example implementations
  - **runtimes/**: asset-hub, bridge-hubs, collectives, coretime, people
  - **integration-tests/**: Emulated chain tests
  - **chain-specs/**: Chain specifications

#### Bridges (`/bridges`)

**Cross-Chain Bridge Infrastructure**

- **modules/** (9 pallets): Bridge pallets
  - beefy, grandpa, messages, parachains
  - relayers, xcm-bridge-hub

- **primitives/**: Bridge primitives
- **relays/**: Relay implementations
- **snowbridge/**: Ethereum bridge (8 subdirectories)

#### Templates (`/templates`)

**Project Starting Points**

- **minimal/**: Minimal node template
- **parachain/**: Parachain template
- **solochain/**: Solo chain template
- **zombienet/**: Network testing configurations

---

## Development Setup

### Prerequisites

Follow the installation guide at: https://docs.polkadot.com/develop/parachains/install-polkadot-sdk

**Key Requirements:**
- Rust toolchain (nightly required for some features)
- WASM target: `rustup target add wasm32-unknown-unknown --toolchain nightly`
- System dependencies (varies by platform)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/paritytech/polkadot-sdk.git
cd polkadot-sdk

# Quick start script (builds and runs example node)
curl --proto '=https' --tlsv1.2 -sSf \
  https://raw.githubusercontent.com/paritytech/polkadot-sdk/master/scripts/getting-started.sh | bash

# Build everything (takes a while)
cargo build --release

# Run tests
cargo test
```

### Useful Tools

- **psvm**: Polkadot SDK Version Manager - manages SDK dependencies in Cargo.toml
  ```bash
  cargo install psvm
  psvm update <version>
  ```

- **prdoc**: Pull request documentation tool
  ```bash
  cargo install parity-prdoc
  prdoc generate <PR_NUMBER>
  ```

- **zepter**: Feature propagation checker
  - Ensures cargo features are correctly propagated

- **taplo**: TOML formatter
  ```bash
  cargo install taplo-cli
  taplo format --config .config/taplo.toml
  ```

---

## Coding Standards

### Rust Formatting

**Configuration:** `.rustfmt.toml`

**Key Rules:**
- **Indentation**: Hard tabs (not spaces)
- **Line Length**: Max 100 characters (120 in exceptional cases)
- **Indent Depth**: Max 5 levels (8 in exceptional cases)
- **Edition**: 2021
- **Imports**: Crate-level granularity, reordered automatically
- **Comments**: Max 100 characters, wrapped

**Format Code:**
```bash
cargo +nightly fmt
```

### Style Guidelines

See `docs/contributor/STYLE_GUIDE.md` for complete details.

**Critical Rules:**

1. **Indentation & Line Length**
   ```rust
   fn calculation(some_long_variable_a: i8, some_long_variable_b: i8) -> bool {
       let x = some_long_variable_a * some_long_variable_b
           - some_long_variable_b / some_long_variable_a
           + sqrt(some_long_variable_a) - sqrt(some_long_variable_b);
       x > 10
   }
   ```

2. **Function Arguments**
   ```rust
   // OK - all parameters indented consistently
   fn foo(
       really_long_parameter_name_1: SomeLongTypeName,
       really_long_parameter_name_2: SomeLongTypeName,
       shrt_nm_1: u8,
       shrt_nm_2: u8,
   ) {
       // ...
   }
   ```

3. **Trailing Commas**
   - Always end multi-line comma-delimited items with `,`

4. **Panickers & Unwrap**
   - Avoid `unwrap()` - require explicit proofs
   - Exception: test code
   ```rust
   let path = self.path().expect(
       "self is instance of DiskDirectory; \
        DiskDirectory always returns path; \
        qed"
   );
   ```

5. **Unsafe Code**
   - Requires explicit justification and proof
   - Consider trade-offs: performance vs. reliability/security
   - Document safety invariants

### Cargo.toml Conventions

**Chain-Specific Crates** (not published to umbrella):
```toml
[package.metadata.polkadot-sdk]
exclude-from-umbrella = true
```

**Test/Example Crates** (not published to crates.io):
```toml
[package]
publish = false
```

### Licensing

**Substrate Split:**
- Apache 2.0: Primitives (`sp-*`), Frame (`frame-*`), Pallets, Binaries
- GPL-3.0 + Classpath: Client (`sc-*`)

**Others:**
- Polkadot: GPL-3.0-only
- Cumulus: GPL-3.0-only
- Bridges: GPL-3.0-only

---

## Development Workflow

### Branch Strategy

- **master**: Main development branch (all PRs target here)
- **stableYYMM**: Stable release branches (e.g., `stable2412`)
  - Created 1.5 months before release for QA
  - Only receives backports
  - No force pushes allowed

**Branch Naming:**
- Use prefixed branches: `<moniker>-<feature-name>`
- Example: `gav-my-feature`

### Pull Request Process

#### 1. Create PR

Use the PR template in `docs/contributor/PULL_REQUEST_TEMPLATE.md`

**Requirements:**
- Clear title and description
- Breaking changes documented with migration examples
- Tag with at least one `T*` label (component affected)
- All CI checks must pass
- Minimum 48-hour review period

**Labels:**
- `T*`: Topic/component (REQUIRED, multiple allowed)
- `D*`: Difficulty (optional, at most one)
- `I*`: Issue severity/type (for issues)
- `A1-insubstantial`: Fast-track (docs/comments only)
- `R0-no-crate-publish-required`: Exempt from prdoc

#### 2. Create PRDoc

**Every PR needs a prdoc file** (unless labeled `R0-no-crate-publish-required`)

**Auto-generate with command bot:**
```
/cmd prdoc --audience runtime_dev --bump patch
```

**Or generate locally:**
```bash
cargo install parity-prdoc
prdoc generate <PR_NUMBER>
# Edit the generated file in prdoc/
prdoc check -n <PR_NUMBER>
```

**PRDoc Structure:**
```yaml
title: "Brief description of changes"
doc:
  - audience: runtime_dev
    description: |
      Detailed description for this audience.

crates:
  - name: frame-example
    bump: major  # or minor, patch, none
  - name: frame-example-pallet
    bump: minor
```

**Audience Types:**
- `runtime_dev`: Parachain teams, runtime builders
- `runtime_user`: Frontend devs, exchanges, state readers
- `node_dev`: Client builders, RPC consumers
- `node_operator`: Validators, node runners

**Bump Levels (SemVer):**
- `none`: No observable change
- `patch`: Bug fixes, no breaking changes
- `minor`: New features, backward compatible
- `major`: Breaking changes

#### 3. Review Process

**Reviewers check for:**
- Buggy behavior
- Undue maintenance burden
- Coding style violations
- Performance regressions (pessimization)
- Feature reduction
- Breaking changes properly documented

**Reviews may NOT block on:**
- Existence of a slightly better approach
- Long-term vision disagreements

#### 4. Merge

- All review comments must be addressed
- CI must pass
- Maintainer approval required

### UI Tests

Used for macros to verify generated code format.

**Update UI tests:**
```bash
./scripts/update-ui-tests.sh           # current rust version
./scripts/update-ui-tests.sh 1.70      # specific version
```

**Or via command bot (for paritytech members):**
```
/cmd update-ui
/cmd update-ui --image docker.io/paritytech/ci-unified:bullseye-1.70.0-2023-05-23
```

### Feature Propagation

Use **zepter** to enforce correct feature propagation between crates.

Configuration: `.config/zepter.yaml`

### Command Bot

**For paritytech organization members:**

```
/cmd --help              # List available commands
/cmd prdoc --help        # PRDoc generation help
/cmd update-ui           # Update UI tests
```

---

## Testing

### Test Organization

Tests are colocated with source code in `tests/` or inline in `src/`.

**Key Test Locations:**

1. **Unit Tests**: In source files (`#[cfg(test)]` modules)

2. **Integration Tests**:
   - `/cumulus/parachains/integration-tests/` - Emulated chains
   - `/polkadot/zombienet-sdk-tests/` - SDK integration tests

3. **Benchmarks**:
   - Individual: `*/benches.rs` or `#[benchmark]` in pallets
   - Batch: `/substrate/scripts/run_all_benchmarks.sh`
   - Cumulus: `/cumulus/scripts/benchmarks.sh`

4. **Network Tests**:
   - **ZombieNet**: Automated network testing
   - Config files in `zombienet/` directories
   - Flaky tests: `.github/ZOMBIENET_FLAKY_TESTS.md`

### Running Tests

```bash
# All tests
cargo test

# Specific package
cargo test -p frame-system

# With nextest (faster)
cargo nextest run

# Benchmarks
cargo bench

# UI tests (requires RUN_UI_TESTS env var)
RUN_UI_TESTS=1 cargo test
```

**Nextest Configuration:** `.config/nextest.toml`

### CI/CD

**Location:** `.github/workflows/`

**Key Workflows:**
- `checks-quick.yml`: Fast CI checks
- `checks.yml`: Full CI suite
- `tests-linux-stable.yml`: Linux testing
- `check-semver.yml`: SemVer validation
- `check-runtime-compatibility.yml`: Runtime compatibility
- `bench-all-runtimes.yml`: Runtime benchmarking
- `zombienet_*.yml`: Network integration tests

**Custom Actions:** `.github/actions/`

---

## Documentation

### Documentation Structure

**Location:** `/docs/`

**Key Documentation:**
- `docs/contributor/CONTRIBUTING.md`: Contribution guidelines
- `docs/contributor/STYLE_GUIDE.md`: Coding standards
- `docs/contributor/DOCUMENTATION_GUIDELINES.md`: Doc writing
- `docs/contributor/SECURITY.md`: Security policy
- `docs/RELEASE.md`: Release process
- `docs/BACKPORT.md`: Backport process

**SDK Documentation:**
- **Source**: `docs/sdk/src/`
- **Output**: https://paritytech.github.io/polkadot-sdk/master/
- Includes Rust API docs, guides, examples

### Writing Documentation

See `docs/contributor/DOCUMENTATION_GUIDELINES.md` for full guidelines.

**What to Document:**
- All `pub` items in crates assigned to `docs-audit` in CODEOWNERS
- All public modules (with `//!` module docs)

**How to Document:**

1. **Start with a single sentence**
   ```rust
   /// Computes the square root of the input.
   ///
   /// Additional details go after a newline...
   ///
   /// # Examples
   /// ```
   /// assert_eq!(sqrt(4), Ok(2));
   /// ```
   ///
   /// # Errors
   /// Returns `Err(())` if input is negative.
   pub fn sqrt(x: i32) -> Result<u32, ()> { ... }
   ```

2. **Use special sections**:
   - `# Examples`: Always include when possible
   - `# Errors`: For `Result` returns
   - `# Panics`: If function can panic
   - `# Safety`: For unsafe functions

3. **Link to related items**:
   ```rust
   /// See also [`OtherStruct`] and [`related_function`].
   ```

4. **Rust Docs vs Comments**:
   - `///`: External documentation (appears in rust-docs)
   - `//`: Internal comments (not in rust-docs)

### Pallet Documentation

**Top-level pallet docs** (`lib.rs`):
- Overview of pallet purpose
- Usage examples
- Configuration guide
- Migration notes

**Dispatchables**: Document parameters, errors, weights

**Storage**: Document structure, invariants

**Events/Errors**: Clear descriptions of when they occur

---

## Release Process

### Release Cadence

**Stable Releases**: Every 3 months
- Format: `stableYYMM` (e.g., `stable2412`)
- Support: 1 year with patches
- QA Period: 1.5 months before release

**Patch Releases**: Monthly
- Format: `stableYYMM-PATCH` (e.g., `stable2412-4`)
- Bug fixes and security updates

**Release Registry**: https://github.com/paritytech/release-registry/

### Versioning

#### Umbrella Crate
- Format: `{YYMM}.0.0` (e.g., `2503.0.0`)
- Re-exports all public SDK components

#### Node Version
- Mostly `minor` increments
- `major` for special releases
- `patch` for monthly fixes
- Not strict SemVer for CLI

#### Crate Versioning
- Follows SemVer 2.0.0
- Pre-1.0.0: `0.y.z` where `y` is major, `z` is minor
- Public API excludes:
  - Items in `__private` modules
  - Items marked "unstable" or "experimental"
  - Experimental features in docs

### Backports

**master → stable:**
- Allowed: `patch`, `minor`, audited changes
- `major` only for internal API crates
- Security fixes prioritized

**stable → master:**
- Version bumps, spec version updates
- PRDoc reorganization
- Done by release team

---

## Common Tasks

### Adding a New Pallet

1. Create pallet in appropriate location:
   - Substrate: `substrate/frame/my-pallet/`
   - Cumulus: `cumulus/pallets/my-pallet/`

2. Add to workspace in root `Cargo.toml`:
   ```toml
   members = [
       # ...
       "substrate/frame/my-pallet",
   ]
   ```

3. Configure pallet `Cargo.toml`:
   ```toml
   [package]
   name = "pallet-my-pallet"
   version = "0.1.0"
   authors.workspace = true
   edition.workspace = true
   license.workspace = true
   ```

4. Write pallet documentation in `lib.rs`

5. Add tests and benchmarks

6. Update documentation if needed

### Running Benchmarks

```bash
# Substrate pallets
./substrate/scripts/run_all_benchmarks.sh

# Cumulus pallets
./cumulus/scripts/benchmarks.sh

# Specific pallet
cargo bench -p pallet-my-pallet
```

### Updating Dependencies

```bash
# Using psvm (recommended)
cargo install psvm
psvm update stable2412

# Manual (update all Polkadot SDK deps)
# Edit version in Cargo.toml files
```

### Deprecating Code

Follow `docs/contributor/DEPRECATION_CHECKLIST.md`:

1. Mark with `#[deprecated]` attribute
2. Document replacement in deprecation message
3. Update documentation
4. Add migration guide
5. Wait appropriate time before removal

### Creating Templates

When modifying templates (`templates/`):
- Ensure they stay minimal and focused
- Test template builds
- Update template documentation
- Sync with getting-started script if needed

---

## Important Files & Locations

### Configuration Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | Root workspace manifest (~1000 members) |
| `.rustfmt.toml` | Rust formatting rules |
| `.config/taplo.toml` | TOML formatting |
| `.config/zepter.yaml` | Feature propagation rules |
| `.config/nextest.toml` | Test runner config |
| `.config/lychee.toml` | Link checker config |
| `.prdoc.toml` | PRDoc configuration |

### CI/CD

| Location | Purpose |
|----------|---------|
| `.github/workflows/` | 50+ workflow files |
| `.github/actions/` | 10 custom actions |
| `.github/CODEOWNERS` | Code ownership mapping |
| `.github/dependabot.yml` | Dependency updates |

### Scripts

| Script | Purpose |
|--------|---------|
| `scripts/getting-started.sh` | Quick start for devs |
| `scripts/generate-umbrella.py` | Generate umbrella crate |
| `scripts/update-ui-tests.sh` | Update UI test outputs |
| `substrate/scripts/run_all_benchmarks.sh` | Batch benchmarking |

### Documentation

| Location | Content |
|----------|---------|
| `docs/contributor/` | Contributor guidelines |
| `docs/sdk/` | SDK documentation source |
| `docs/RELEASE.md` | Release process |
| `docs/AUDIT.md` | Security audit procedures |
| `prdoc/` | PR documentation database |

### Binaries

**Released Binaries:**
- `polkadot` - Relay chain node
- `polkadot-execute-worker` - PVF execution
- `polkadot-prepare-worker` - PVF preparation
- `polkadot-parachain` - Parachain collator
- `polkadot-omni-node` - Universal parachain node
- `chain-spec-builder` - Chain spec tool
- `frame-omni-bencher` - Benchmarking tool

---

## Best Practices for AI Assistants

### When Contributing Code

1. **Always run `cargo +nightly fmt`** before committing
2. **Check for existing similar functionality** before adding new code
3. **Document all public APIs** according to guidelines
4. **Include tests** for new functionality
5. **Add benchmarks** for runtime code (pallets)
6. **Consider SemVer implications** of changes
7. **Never use `unwrap()`** without explicit justification (except tests)
8. **Avoid `unsafe`** unless absolutely necessary with full justification

### When Reviewing Code

1. **Check PRDoc completeness** and accuracy
2. **Verify SemVer bumps** are appropriate
3. **Ensure breaking changes** are documented with migration guides
4. **Review test coverage** - tests should cover new functionality
5. **Check documentation** - all `pub` items documented
6. **Validate formatting** - code follows style guide
7. **Consider security implications** - especially for runtime code

### Common Pitfalls to Avoid

1. **Don't modify stable branches directly** - use backport process
2. **Don't force push** to any branch after sharing
3. **Don't skip PRDoc** unless genuinely `R0-no-crate-publish-required`
4. **Don't merge with failing CI** - fix all errors first
5. **Don't use `major` bumps lightly** - consider migration burden
6. **Don't ignore security concerns** - runtime code is critical
7. **Don't create unnecessary dependencies** - minimize dep tree

### Helpful Commands Reference

```bash
# Format code
cargo +nightly fmt

# Check formatting
cargo +nightly fmt --check

# Lint
cargo clippy --all-targets --all-features

# Test
cargo test
cargo nextest run  # faster alternative

# Build
cargo build --release

# Benchmarks
cargo bench

# Check features
zepter run check

# Update dependencies
psvm update stable2412

# Generate PRDoc
prdoc generate <PR_NUMBER>
prdoc check -n <PR_NUMBER>
```

---

## Additional Resources

- **Main Docs**: https://docs.polkadot.com
- **Rust Docs**: https://paritytech.github.io/polkadot-sdk/master/
- **StackExchange**: https://substrate.stackexchange.com/
- **Release Registry**: https://github.com/paritytech/release-registry/
- **Polkadot Fellowship**: https://polkadot-fellows.github.io/dashboard/
- **Issue Labels**: https://paritytech.github.io/labels/doc_polkadot-sdk.html

**Community Channels:**
- Telegram: https://t.me/substratedevs
- Matrix: https://matrix.to/#/#substratedevs:matrix.org
- Discord: Polkadot server, Substrate Developers channel

---

## Updates & Maintenance

This document should be updated when:
- Major repository restructuring occurs
- New development tools are adopted
- Workflow processes change significantly
- New release processes are established

**Last Updated**: 2025-11-15
**Repository State**: Based on commit `a216543` and recent commits

---

**Note for AI Assistants**: This guide provides the essential context needed to effectively contribute to the Polkadot SDK. Always cross-reference with the official documentation in the `docs/` directory for the most up-to-date and detailed information. When in doubt, ask for clarification or consult the CODEOWNERS for the relevant component.
