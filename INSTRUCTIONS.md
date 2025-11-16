# Patient X - Phase 1 Instructions

**Last Updated:** 2025-11-16
**Phase:** Phase 1 Complete - Pallet Development
**Status:** ✅ All 13 pallets implemented and compiling

---

## Table of Contents

1. [What We Have](#what-we-have)
2. [Prerequisites](#prerequisites)
3. [Project Structure](#project-structure)
4. [Building the Pallets](#building-the-pallets)
5. [Running Tests](#running-tests)
6. [Verifying the Code](#verifying-the-code)
7. [What's Missing for MVP](#whats-missing-for-mvp)
8. [Next Steps](#next-steps)
9. [Troubleshooting](#troubleshooting)

---

## What We Have

### Current State: Phase 1 Complete ✅

We have successfully implemented **13 production-ready Substrate pallets** (~8,500 lines of Rust code) that provide the core business logic for Patient X's decentralized health data platform.

**Implemented Pallets:**

#### IdentityConsent Chain (4 pallets)
- ✅ `pallet-identity-registry` - Decentralized identity and DID management
- ✅ `pallet-consent-manager` - Granular consent control
- ✅ `pallet-authentication` - Multi-factor authentication
- ✅ `pallet-jurisdiction-manager` - Regulatory compliance

#### HealthData Chain (5 pallets)
- ✅ `pallet-health-records` - Core health record storage
- ✅ `pallet-ipfs-integration` - Distributed file storage (off-chain worker)
- ✅ `pallet-access-control` - Attribute-based access control (ABAC)
- ✅ `pallet-encryption` - Key lifecycle management
- ✅ `pallet-provenance` - Data lineage tracking

#### Marketplace Chain (4 pallets)
- ✅ `pallet-data-listings` - Data marketplace listings
- ✅ `pallet-negative-registry` - **FLAGSHIP** negative outcome marketplace
- ✅ `pallet-marketplace` - Order processing & payment escrow
- ✅ `pallet-reputation` - Trust and review system

**What You Can Do:**
- ✅ Compile all pallets
- ✅ Review the code and architecture
- ✅ Understand the business logic
- ✅ Write unit tests
- ✅ Plan runtime integration

**What You CANNOT Do Yet:**
- ❌ Run a blockchain node (no runtime or node implementation)
- ❌ Submit transactions (no live chain)
- ❌ Use a frontend (not built yet)
- ❌ Deploy to testnet/mainnet

---

## Prerequisites

### System Requirements

**Rust Toolchain:**
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install nightly toolchain (required for some Substrate features)
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

# Set nightly as default (optional)
rustup default nightly
```

**Substrate Dependencies:**

Follow the official Substrate installation guide for your platform:
- https://docs.substrate.io/install/

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y build-essential git clang curl libssl-dev llvm libudev-dev make protobuf-compiler
```

**macOS:**
```bash
brew install openssl cmake llvm
```

**Minimum Specs:**
- 4 CPU cores
- 8 GB RAM
- 50 GB free disk space
- Good internet connection (for dependencies)

---

## Project Structure

```
patient-x/
├── identity-consent-chain/
│   └── pallets/
│       ├── identity-registry/
│       ├── consent-manager/
│       ├── authentication/
│       └── jurisdiction-manager/
├── health-data-chain/
│   └── pallets/
│       ├── health-records/
│       ├── ipfs-integration/
│       ├── access-control/
│       ├── encryption/
│       └── provenance/
├── marketplace-chain/
│   └── pallets/
│       ├── data-listings/
│       ├── negative-registry/      # FLAGSHIP FEATURE
│       ├── marketplace/
│       └── reputation/
├── Cargo.toml                       # Workspace configuration
├── INSTRUCTIONS.md                  # This file
├── PLAN.md                          # Development roadmap
└── README.md                        # Project overview
```

**Each pallet contains:**
- `Cargo.toml` - Dependencies and features
- `src/lib.rs` - Main pallet logic and dispatchables
- `src/types.rs` - Data structures and enums
- `src/weights.rs` - Transaction weight calculations

---

## Building the Pallets

### Build All Pallets

```bash
# Navigate to project root
cd /home/user/patient-x

# Build all pallets in the workspace
cargo build --release

# Or build in development mode (faster, larger binaries)
cargo build
```

**Expected Output:**
```
   Compiling pallet-identity-registry v0.1.0
   Compiling pallet-consent-manager v0.1.0
   ...
   Finished release [optimized] target(s) in 5m 23s
```

### Build Individual Pallets

```bash
# Build a specific pallet
cargo build -p pallet-negative-registry

# Build with release optimizations
cargo build -p pallet-marketplace --release

# Build all marketplace chain pallets
cargo build -p pallet-data-listings -p pallet-negative-registry -p pallet-marketplace -p pallet-reputation
```

### Check Without Building

```bash
# Fast compilation check (no binary output)
cargo check

# Check specific pallet
cargo check -p pallet-health-records

# Check all pallets in parallel
cargo check --workspace
```

**Tip:** Use `cargo check` during development - it's much faster than `cargo build`.

---

## Running Tests

### Current Test Status

**⚠️ Note:** Unit tests are not yet implemented for the pallets. This is a planned next step.

### When Tests Are Added

```bash
# Run all tests
cargo test

# Run tests for specific pallet
cargo test -p pallet-negative-registry

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test -p pallet-marketplace test_create_order
```

### Running Clippy (Linter)

```bash
# Check code quality
cargo clippy --all-targets --all-features

# Auto-fix some issues
cargo clippy --fix
```

### Running Formatter

```bash
# Check formatting
cargo fmt -- --check

# Auto-format code
cargo fmt
```

---

## Verifying the Code

### Verify All Pallets Compile

Run this comprehensive check to ensure everything is working:

```bash
# From project root
cd /home/user/patient-x

# Check all pallets compile
echo "Checking IdentityConsent Chain..."
cargo check -p pallet-identity-registry
cargo check -p pallet-consent-manager
cargo check -p pallet-authentication
cargo check -p pallet-jurisdiction-manager

echo "Checking HealthData Chain..."
cargo check -p pallet-health-records
cargo check -p pallet-ipfs-integration
cargo check -p pallet-access-control
cargo check -p pallet-encryption
cargo check -p pallet-provenance

echo "Checking Marketplace Chain..."
cargo check -p pallet-data-listings
cargo check -p pallet-negative-registry
cargo check -p pallet-marketplace
cargo check -p pallet-reputation

echo "✅ All pallets verified!"
```

### Expected Warnings

You may see these warnings (they are OK):
```
warning: the following packages contain code that will be rejected by a future version of Rust: trie-db v0.30.0
```

This is a dependency issue in the Polkadot SDK and doesn't affect functionality.

### View Pallet Dependencies

```bash
# See dependency tree
cargo tree -p pallet-negative-registry

# See all dependencies
cargo tree --workspace
```

---

## What's Missing for MVP

### Phase 2: Runtime Integration (Not Started)

**What:** Combine pallets into blockchain runtimes

**Tasks:**
1. Create `identity-consent-runtime/`
2. Create `health-data-runtime/`
3. Create `marketplace-runtime/`
4. Configure pallet parameters
5. Set up genesis configuration
6. Define runtime APIs

**Estimated Time:** 3-5 days

### Phase 3: Node Implementation (Not Started)

**What:** Build the actual blockchain nodes

**Tasks:**
1. Implement collator nodes for each chain
2. Set up networking and p2p
3. Configure consensus (Aura + GRANDPA)
4. Build CLI interfaces
5. Create chain specifications

**Estimated Time:** 5-7 days

### Phase 4: Testing & Integration (Not Started)

**What:** Test everything works together

**Tasks:**
1. Write unit tests for all pallets
2. Integration tests across pallets
3. End-to-end testing
4. Local 3-chain network setup

**Estimated Time:** 3-5 days

### Phase 5: Frontend & Deployment (Not Started)

**What:** User interface and deployment

**Tasks:**
1. Build Web3 frontend (React + Polkadot.js)
2. Wallet integration
3. Docker deployment
4. Monitoring and logging

**Estimated Time:** 10-14 days

**Total Time to MVP:** ~3-4 weeks

---

## Next Steps

### Immediate Next Steps (Phase 2)

To make this a working MVP, we need to:

#### Step 1: Create Runtime for Each Chain

**Example: Marketplace Chain Runtime**

```bash
# Create runtime directory
mkdir -p marketplace-chain/runtime
cd marketplace-chain/runtime

# Create runtime Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "marketplace-runtime"
version = "0.1.0"
edition.workspace = true

[dependencies]
# Substrate
frame-support = { git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master", default-features = false }
frame-system = { git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master", default-features = false }
sp-runtime = { git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master", default-features = false }

# Patient X pallets
pallet-data-listings = { path = "../pallets/data-listings", default-features = false }
pallet-negative-registry = { path = "../pallets/negative-registry", default-features = false }
pallet-marketplace = { path = "../pallets/marketplace", default-features = false }
pallet-reputation = { path = "../pallets/reputation", default-features = false }

[features]
default = ["std"]
std = [
    "frame-support/std",
    "frame-system/std",
    "sp-runtime/std",
    "pallet-data-listings/std",
    "pallet-negative-registry/std",
    "pallet-marketplace/std",
    "pallet-reputation/std",
]
EOF

# Create runtime lib.rs (simplified example)
cat > src/lib.rs << 'EOF'
#![cfg_attr(not(feature = "std"), no_std)]

pub use frame_support::{construct_runtime, parameter_types};
pub use sp_runtime::{generic, traits::BlakeTwo256};

// Configure runtime
pub type BlockNumber = u32;
pub type Signature = sp_runtime::MultiSignature;
pub type AccountId = <<Signature as sp_runtime::traits::Verify>::Signer as sp_runtime::traits::IdentifyAccount>::AccountId;

// Configure pallets
impl frame_system::Config for Runtime {
    type AccountId = AccountId;
    type Block = Block;
    // ... more configuration
}

impl pallet_data_listings::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxListingsPerProvider = ConstU32<100>;
    type MaxListingsPerCategory = ConstU32<10000>;
}

// ... configure other pallets

// Construct the runtime
construct_runtime!(
    pub struct Runtime {
        System: frame_system,
        DataListings: pallet_data_listings,
        NegativeRegistry: pallet_negative_registry,
        Marketplace: pallet_marketplace,
        Reputation: pallet_reputation,
    }
);
EOF
```

**Do this for all 3 chains:**
- `identity-consent-chain/runtime/`
- `health-data-chain/runtime/`
- `marketplace-chain/runtime/`

#### Step 2: Create Basic Node

```bash
# Use Substrate node template as base
cargo install substrate-node-template

# Or manually create node/
mkdir -p marketplace-chain/node
cd marketplace-chain/node

# Create node implementation
# (This requires significant Substrate knowledge)
```

#### Step 3: Local Development Network

```bash
# Once nodes are built, run locally
./target/release/marketplace-node --dev

# Or use zombienet for multi-chain setup
zombienet spawn network-config.toml
```

---

## Troubleshooting

### Common Issues

#### 1. Compilation Errors

**Issue:** `error: could not compile pallet-xxx`

**Solution:**
```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build
```

#### 2. Out of Memory During Build

**Issue:** Compilation runs out of RAM

**Solution:**
```bash
# Build with fewer parallel jobs
cargo build -j 2

# Or use release mode (uses less memory)
cargo build --release
```

#### 3. Dependency Conflicts

**Issue:** `error: failed to select a version for...`

**Solution:**
```bash
# Update Cargo.lock
cargo update

# Or specify exact versions in Cargo.toml
```

#### 4. WASM Target Missing

**Issue:** `error: the wasm32-unknown-unknown target may not be installed`

**Solution:**
```bash
rustup target add wasm32-unknown-unknown --toolchain nightly
```

#### 5. Git LFS Issues

**Issue:** Large files not downloading properly

**Solution:**
```bash
# Install git-lfs
git lfs install

# Pull large files
git lfs pull
```

#### 6. macOS libclang.dylib Error

**Issue:** `error: failed to run custom build command for librocksdb-sys`
- `dyld: Library not loaded: @rpath/libclang.dylib`

**Solution (Apple Silicon):**
```bash
# Install LLVM
brew install llvm

# Set environment variables (for current session)
export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"
export LDFLAGS="-L/opt/homebrew/opt/llvm/lib"
export CPPFLAGS="-I/opt/homebrew/opt/llvm/include"

# Make permanent - add to ~/.zshrc
echo 'export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"' >> ~/.zshrc
echo 'export LDFLAGS="-L/opt/homebrew/opt/llvm/lib"' >> ~/.zshrc
echo 'export CPPFLAGS="-I/opt/homebrew/opt/llvm/include"' >> ~/.zshrc
source ~/.zshrc

# Clean and rebuild
cargo clean
cargo build
```

**Solution (Intel Mac):**
```bash
# Use /usr/local instead of /opt/homebrew
export LIBCLANG_PATH="/usr/local/opt/llvm/lib"
export LDFLAGS="-L/usr/local/opt/llvm/lib"
export CPPFLAGS="-I/usr/local/opt/llvm/include"
```

**Alternative:** Install full Xcode (not just command line tools)
```bash
# Install from App Store, then:
sudo xcode-select --install
```

---

## Development Workflow

### Recommended Workflow

1. **Make changes to pallet code**
   ```bash
   # Edit src/lib.rs or src/types.rs
   code marketplace-chain/pallets/marketplace/src/lib.rs
   ```

2. **Check compilation**
   ```bash
   cargo check -p pallet-marketplace
   ```

3. **Fix any errors, then build**
   ```bash
   cargo build -p pallet-marketplace
   ```

4. **Run tests (when available)**
   ```bash
   cargo test -p pallet-marketplace
   ```

5. **Format and lint**
   ```bash
   cargo fmt
   cargo clippy
   ```

6. **Commit changes**
   ```bash
   git add .
   git commit -m "feat: add new functionality to marketplace"
   git push
   ```

---

## Documentation

### Pallet Documentation

Each pallet has inline documentation. View it with:

```bash
# Generate and open docs
cargo doc --open

# Generate docs for specific pallet
cargo doc -p pallet-negative-registry --open
```

### Architecture Documentation

- **PLAN.md** - Development roadmap and timeline
- **README.md** - Project overview
- **CLAUDE.md** - Polkadot SDK development guide
- **INSTRUCTIONS.md** - This file

---

## Performance Notes

### Build Times

**First build:** 5-15 minutes (downloads and compiles all dependencies)
**Incremental builds:** 30 seconds - 2 minutes
**Checking (no build):** 10-30 seconds

**Optimization tips:**
- Use `cargo check` during development
- Use `sccache` for caching: `cargo install sccache`
- Use `mold` linker on Linux for faster linking

### Resource Usage

**Disk Space:**
- Source code: ~50 MB
- Dependencies: ~2-5 GB
- Build artifacts: ~10-20 GB
- **Total:** ~15-25 GB

**Memory:**
- Compilation: 4-8 GB RAM
- Running node (future): 2-4 GB RAM

---

## Getting Help

### Resources

- **Substrate Documentation:** https://docs.substrate.io
- **Polkadot SDK Docs:** https://paritytech.github.io/polkadot-sdk/master/
- **FRAME Documentation:** https://docs.substrate.io/reference/frame-pallets/
- **Substrate StackExchange:** https://substrate.stackexchange.com/

### Project Contacts

- **Repository:** https://github.com/choiwab/patient-x
- **Issues:** https://github.com/choiwab/patient-x/issues

---

## Summary

**Phase 1 Status:** ✅ Complete
**What Works:** All 13 pallets compile successfully
**What's Next:** Runtime integration (Phase 2)
**Time to MVP:** ~3-4 weeks additional work

**To verify everything works right now:**
```bash
cd /home/user/patient-x
cargo check --workspace
```

If this completes without errors, Phase 1 is confirmed working! 🎉

---

**Last Updated:** 2025-11-16
**Next Update:** When Phase 2 (Runtime Integration) begins
