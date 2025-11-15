# Patient X Development Plan

> Comprehensive development roadmap for building a decentralized medical data marketplace on Polkadot SDK

**Last Updated**: 2025-11-15
**Project Status**: Pre-Development Phase
**Target Mainnet Launch**: Q4 2026

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Project Foundation](#project-foundation)
3. [Development Phases](#development-phases)
4. [Phase 1: Foundation (Q1-Q2 2025)](#phase-1-foundation-q1-q2-2025)
5. [Phase 2: Compliance & Scale (Q3-Q4 2025)](#phase-2-compliance--scale-q3-q4-2025)
6. [Phase 3: Advanced Analytics (Q1-Q2 2026)](#phase-3-advanced-analytics-q1-q2-2026)
7. [Phase 4: Incentives & IP (Q3 2026)](#phase-4-incentives--ip-q3-2026)
8. [Phase 5: Privacy & Interoperability (Q4 2026 - Q1 2027)](#phase-5-privacy--interoperability-q4-2026---q1-2027)
9. [Technical Architecture Details](#technical-architecture-details)
10. [Team Structure & Resources](#team-structure--resources)
11. [Risk Management](#risk-management)
12. [Success Metrics](#success-metrics)

---

## Executive Summary

**Patient X** is an ambitious decentralized medical data marketplace built as a fork of the Polkadot SDK, implementing three interconnected parachains to enable secure, transparent, and compliant sharing of clinical research data, with a flagship focus on negative and abandoned trial results.

### Core Innovation
The **Negative/Abandoned Data Registry** addresses healthcare research's most significant gap by incentivizing researchers to share failed trial data, preventing redundant research, and accelerating medical discovery.

### Technical Approach
- **3 Parachains**: IdentityConsent (2000), HealthData (2001), Marketplace (2002)
- **14+ Custom Pallets**: Identity, Consent, Health Records, Negative Registry, Compliance Engine, etc.
- **Cross-Chain Communication**: XCM v3 for seamless parachain coordination
- **Storage**: IPFS for encrypted medical data, on-chain for provenance
- **Compliance**: Automated GDPR, HIPAA, PDPA enforcement

### Timeline Overview
- **Phase 1** (Q1-Q2 2025): Foundation + Negative Registry
- **Phase 2** (Q3-Q4 2025): Compliance & Scale
- **Phase 3** (Q1-Q2 2026): Advanced Analytics
- **Phase 4** (Q3 2026): Incentives & IP
- **Phase 5** (Q4 2026-Q1 2027): Privacy & Interoperability
- **Mainnet** (Q4 2026): Production launch

---

## Project Foundation

### Prerequisites & Environment Setup

#### 1.1 Development Environment

**System Requirements**:
- OS: Linux (Ubuntu 22.04+), macOS, or Windows (WSL2)
- RAM: 16GB minimum (32GB recommended)
- Storage: 100GB+ available space
- CPU: 8 cores minimum (16 recommended)

**Required Software**:
```bash
# Rust toolchain (Edition 2021)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

# System dependencies (Ubuntu/Debian)
sudo apt install -y \
  build-essential cmake pkg-config libssl-dev git \
  clang libclang-dev protobuf-compiler curl

# Polkadot SDK tools
cargo install psvm          # SDK version manager
cargo install parity-prdoc  # PR documentation
cargo install taplo-cli     # TOML formatter

# Polkadot binary
cargo install --git https://github.com/paritytech/polkadot --tag v1.0.0 polkadot

# Zombienet (network testing)
wget https://github.com/paritytech/zombienet/releases/latest/download/zombienet-linux-x64
chmod +x zombienet-linux-x64
sudo mv zombienet-linux-x64 /usr/local/bin/zombienet
```

#### 1.2 Repository Setup

**Fork Polkadot SDK**:
```bash
# Clone Polkadot SDK as base
git clone https://github.com/paritytech/polkadot-sdk.git patient-x-base
cd patient-x-base
git checkout master

# Create new repo for Patient X
cd ..
git clone https://github.com/your-org/patient-x.git
cd patient-x

# Copy relevant directories from Polkadot SDK
cp -r ../patient-x-base/substrate ./substrate-reference
cp -r ../patient-x-base/polkadot ./polkadot-reference
cp -r ../patient-x-base/cumulus ./cumulus-reference

# Keep these as reference; we'll build custom parachains
```

**Directory Structure**:
```
patient-x/
├── README.md
├── CLAUDE.md                      # AI assistant guide
├── plan.md                        # This file
├── Cargo.toml                     # Workspace manifest
├── LICENSE
├── .gitignore
├── .rustfmt.toml                  # Rust formatting rules
├── .config/
│   ├── taplo.toml                 # TOML formatting
│   ├── zepter.yaml                # Feature propagation
│   └── nextest.toml               # Test runner config
├── scripts/
│   ├── run-all.sh                 # Unified runner
│   ├── setup.sh                   # Environment setup
│   ├── build-all.sh               # Build all chains
│   ├── launch-testnet.sh          # Launch testnet
│   └── update-dependencies.sh     # Update SDK versions
├── zombienet/
│   ├── local-testnet.toml
│   ├── rococo-testnet.toml
│   └── mainnet-config.toml
├── docs/
│   ├── architecture/
│   ├── pallets/
│   ├── xcm/
│   └── compliance/
├── identity-consent-chain/
│   ├── Cargo.toml
│   ├── node/
│   ├── runtime/
│   └── pallets/
│       ├── identity-registry/
│       ├── consent-manager/
│       ├── authentication/
│       └── jurisdiction-manager/
├── health-data-chain/
│   ├── Cargo.toml
│   ├── node/
│   ├── runtime/
│   └── pallets/
│       ├── health-records/
│       ├── ipfs-integration/
│       ├── access-control/
│       ├── encryption/
│       └── provenance/
└── marketplace-chain/
    ├── Cargo.toml
    ├── node/
    ├── runtime/
    └── pallets/
        ├── data-listings/
        ├── negative-registry/       # FLAGSHIP FEATURE
        ├── marketplace/
        ├── compliance-engine/
        ├── reputation/
        ├── analytics/
        ├── federated-ml/
        └── consortium/
```

#### 1.3 Workspace Configuration

**Root `Cargo.toml`**:
```toml
[workspace]
resolver = "2"

members = [
    # IdentityConsent Chain
    "identity-consent-chain/node",
    "identity-consent-chain/runtime",
    "identity-consent-chain/pallets/*",

    # HealthData Chain
    "health-data-chain/node",
    "health-data-chain/runtime",
    "health-data-chain/pallets/*",

    # Marketplace Chain
    "marketplace-chain/node",
    "marketplace-chain/runtime",
    "marketplace-chain/pallets/*",
]

[workspace.package]
authors = ["Patient X Team <contact@patientx.network>"]
edition = "2021"
license = "MIT"
repository = "https://github.com/your-org/patient-x"

[workspace.dependencies]
# Polkadot SDK dependencies (use psvm to manage versions)
frame-support = { git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master", default-features = false }
frame-system = { git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master", default-features = false }
sp-core = { git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master", default-features = false }
sp-runtime = { git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master", default-features = false }
# ... other SDK dependencies

# Codec
parity-scale-codec = { version = "3.0.0", default-features = false, features = ["derive"] }
scale-info = { version = "2.0.0", default-features = false, features = ["derive"] }

# Cryptography
chacha20poly1305 = "0.10"
x25519-dalek = "2.0"
blake2 = "0.10"

# IPFS
ipfs-api = "0.17"
cid = "0.11"

[profile.release]
panic = "unwind"
opt-level = 3
lto = true
codegen-units = 1
```

---

## Development Phases

### Overview of Development Phases

| Phase | Timeline | Focus | Key Deliverables |
|-------|----------|-------|------------------|
| **Phase 1** | Q1-Q2 2025 | Foundation | 3 parachains, Negative Registry, Provenance |
| **Phase 2** | Q3-Q4 2025 | Compliance | Compliance Engine, Data Passport, Consortia |
| **Phase 3** | Q1-Q2 2026 | Analytics | Federated ML, Advanced Analytics |
| **Phase 4** | Q3 2026 | Incentives | Patient Rewards, IP/Licensing |
| **Phase 5** | Q4 2026-Q1 2027 | Privacy | ZK Proofs, Cross-chain Bridges |
| **Launch** | Q4 2026 | Production | Mainnet, Parachain Auction |

---

## Phase 1: Foundation (Q1-Q2 2025)

**Goal**: Establish core infrastructure with three operational parachains and flagship Negative Data Registry

### Month 1-2: IdentityConsent Chain

#### Week 1-2: Chain Scaffolding
- [ ] Fork cumulus parachain template
- [ ] Configure chain spec for Para ID 2000
- [ ] Set up node implementation (CLI, RPC, service)
- [ ] Configure runtime with basic pallets
- [ ] Implement genesis config

**Tasks**:
```bash
# Use cumulus parachain template as base
cd identity-consent-chain
cargo init --name identity-consent-node node
cargo init --name identity-consent-runtime runtime
```

**Node Implementation** (`node/src/main.rs`):
```rust
// Based on substrate/bin/node/cli structure
// See CLAUDE.md section on "Substrate → client/"
fn main() -> Result<()> {
    identity_consent_node::cli::run()
}
```

**Chain Spec** (`node/src/chain_spec.rs`):
```rust
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Identity Consent Development",
        "identity_consent_dev",
        ChainType::Development,
        move || {
            genesis_config(
                // Initial authorities
                vec![authority_keys_from_seed("Alice")],
                // Root key
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Endowed accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                ],
                2000u32.into(), // Para ID
            )
        },
        vec![],
        None,
        None,
        None,
        None,
        Extensions {
            relay_chain: "rococo-local".into(),
            para_id: 2000,
        },
    )
}
```

#### Week 3-4: pallet-identity-registry

**Priority**: CRITICAL
**Estimated Effort**: 40 hours

**Storage Schema**:
```rust
#[pallet::storage]
pub type Identities<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    UserInfo<T>,
    OptionQuery,
>;

#[pallet::storage]
pub type DIDs<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    DID,
    T::AccountId,
    OptionQuery,
>;

#[pallet::storage]
pub type VerifiedUsers<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    DID,
    VerificationStatus<T>,
    ValueQuery,
>;
```

**Types** (`pallets/identity-registry/src/types.rs`):
```rust
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct UserInfo<T: Config> {
    pub did: DID,
    pub user_type: UserType,
    pub jurisdiction: JurisdictionCode,
    pub institution: Option<BoundedVec<u8, T::MaxInstitutionLength>>,
    pub verified: bool,
    pub created_at: T::Moment,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum UserType {
    Patient,
    Researcher,
    Institution,
    Auditor,
    Publisher,
    Regulator,
}

pub type DID = BoundedVec<u8, ConstU32<64>>;
pub type JurisdictionCode = BoundedVec<u8, ConstU32<8>>;
```

**Dispatchables**:
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(T::WeightInfo::register_identity())]
    pub fn register_identity(
        origin: OriginFor<T>,
        did: DID,
        user_type: UserType,
        jurisdiction: JurisdictionCode,
        institution: Option<BoundedVec<u8, T::MaxInstitutionLength>>,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // Ensure DID is unique
        ensure!(!DIDs::<T>::contains_key(&did), Error::<T>::DIDAlreadyExists);

        // Ensure account doesn't already have identity
        ensure!(!Identities::<T>::contains_key(&who), Error::<T>::IdentityAlreadyRegistered);

        let user_info = UserInfo {
            did: did.clone(),
            user_type,
            jurisdiction,
            institution,
            verified: false,
            created_at: T::TimeProvider::now(),
        };

        Identities::<T>::insert(&who, user_info);
        DIDs::<T>::insert(&did, &who);

        Self::deposit_event(Event::IdentityRegistered {
            account: who,
            did,
            user_type,
        });

        Ok(())
    }

    #[pallet::weight(T::WeightInfo::verify_identity())]
    pub fn verify_identity(
        origin: OriginFor<T>,
        user_did: DID,
        verifier_signature: T::Signature,
    ) -> DispatchResult {
        let verifier = ensure_signed(origin)?;

        // Ensure verifier is authorized (Institution or Auditor)
        let verifier_info = Identities::<T>::get(&verifier)
            .ok_or(Error::<T>::IdentityNotFound)?;
        ensure!(
            matches!(verifier_info.user_type, UserType::Institution | UserType::Auditor),
            Error::<T>::UnauthorizedVerifier
        );

        // Get user account
        let user = DIDs::<T>::get(&user_did).ok_or(Error::<T>::DIDNotFound)?;

        // Verify signature
        ensure!(
            Self::verify_signature(&user, &verifier_signature),
            Error::<T>::InvalidSignature
        );

        // Update verification status
        Identities::<T>::mutate(&user, |info| {
            if let Some(user_info) = info {
                user_info.verified = true;
            }
        });

        VerifiedUsers::<T>::insert(
            &user_did,
            VerificationStatus {
                verified: true,
                verifier: verifier.clone(),
                verified_at: T::TimeProvider::now(),
            }
        );

        Self::deposit_event(Event::IdentityVerified {
            did: user_did,
            verifier,
        });

        Ok(())
    }
}
```

**Tests** (`pallets/identity-registry/src/tests.rs`):
```rust
#[test]
fn register_identity_works() {
    new_test_ext().execute_with(|| {
        let alice = 1u64;
        let did = b"did:patientx:alice123".to_vec().try_into().unwrap();

        assert_ok!(IdentityRegistry::register_identity(
            RuntimeOrigin::signed(alice),
            did.clone(),
            UserType::Patient,
            b"US".to_vec().try_into().unwrap(),
            None,
        ));

        assert!(Identities::<Test>::contains_key(&alice));
        assert_eq!(DIDs::<Test>::get(&did), Some(alice));

        System::assert_last_event(Event::IdentityRegistered {
            account: alice,
            did,
            user_type: UserType::Patient,
        }.into());
    });
}

#[test]
fn duplicate_did_fails() {
    new_test_ext().execute_with(|| {
        let alice = 1u64;
        let bob = 2u64;
        let did = b"did:patientx:duplicate".to_vec().try_into().unwrap();

        assert_ok!(IdentityRegistry::register_identity(
            RuntimeOrigin::signed(alice),
            did.clone(),
            UserType::Patient,
            b"US".to_vec().try_into().unwrap(),
            None,
        ));

        assert_noop!(
            IdentityRegistry::register_identity(
                RuntimeOrigin::signed(bob),
                did,
                UserType::Researcher,
                b"EU".to_vec().try_into().unwrap(),
                None,
            ),
            Error::<Test>::DIDAlreadyExists
        );
    });
}
```

**Benchmarking** (`pallets/identity-registry/src/benchmarking.rs`):
```rust
benchmarks! {
    register_identity {
        let caller: T::AccountId = whitelisted_caller();
        let did: DID = b"did:patientx:benchmark".to_vec().try_into().unwrap();
    }: _(
        RawOrigin::Signed(caller.clone()),
        did.clone(),
        UserType::Patient,
        b"US".to_vec().try_into().unwrap(),
        None
    )
    verify {
        assert!(Identities::<T>::contains_key(&caller));
    }
}
```

#### Week 5-6: pallet-consent-manager

**Priority**: CRITICAL
**Estimated Effort**: 50 hours

**Storage Schema**:
```rust
#[pallet::storage]
pub type Consents<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    ConsentId,
    ConsentPolicy<T>,
    OptionQuery,
>;

#[pallet::storage]
pub type UserConsents<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    Blake2_128Concat,
    ConsentId,
    ConsentStatus<T>,
    OptionQuery,
>;

#[pallet::storage]
pub type ConsentIndex<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    BoundedVec<ConsentId, T::MaxConsentsPerUser>,
    ValueQuery,
>;
```

**Types**:
```rust
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ConsentPolicy<T: Config> {
    pub consent_id: ConsentId,
    pub data_owner: T::AccountId,
    pub purpose: Purpose,
    pub duration: Duration<T>,
    pub data_types: BoundedVec<DataType, T::MaxDataTypes>,
    pub allowed_parties: AllowedParties<T>,
    pub jurisdictions: BoundedVec<JurisdictionCode, T::MaxJurisdictions>,
    pub compensation_preference: CompensationPreference,
    pub created_at: T::Moment,
    pub updated_at: T::Moment,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum Purpose {
    ResearchGeneral,
    ResearchSpecificStudy(BoundedVec<u8, ConstU32<64>>),
    Commercial,
    PublicHealth,
    Custom(BoundedVec<u8, ConstU32<128>>),
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Duration<T: Config> {
    pub start: T::Moment,
    pub end: Option<T::Moment>,
    pub auto_renewal: bool,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum DataType {
    Demographics,
    Diagnostics,
    Genomics,
    Imaging,
    LabResults,
    Medications,
    Procedures,
    Vitals,
    Custom(BoundedVec<u8, ConstU32<64>>),
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum AllowedParties<T: Config> {
    Specific(BoundedVec<T::AccountId, T::MaxAllowedParties>),
    Categories(BoundedVec<UserType, ConstU32<10>>),
    Public,
}
```

**Dispatchables**:
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(T::WeightInfo::grant_consent())]
    pub fn grant_consent(
        origin: OriginFor<T>,
        purpose: Purpose,
        duration: Duration<T>,
        data_types: BoundedVec<DataType, T::MaxDataTypes>,
        allowed_parties: AllowedParties<T>,
        jurisdictions: BoundedVec<JurisdictionCode, T::MaxJurisdictions>,
        compensation_preference: CompensationPreference,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // Ensure user has registered identity
        ensure!(
            pallet_identity_registry::Identities::<T>::contains_key(&who),
            Error::<T>::IdentityNotRegistered
        );

        // Generate unique consent ID
        let consent_id = Self::generate_consent_id(&who, &purpose);

        let policy = ConsentPolicy {
            consent_id: consent_id.clone(),
            data_owner: who.clone(),
            purpose: purpose.clone(),
            duration,
            data_types,
            allowed_parties,
            jurisdictions,
            compensation_preference,
            created_at: T::TimeProvider::now(),
            updated_at: T::TimeProvider::now(),
        };

        Consents::<T>::insert(&consent_id, policy);
        UserConsents::<T>::insert(
            &who,
            &consent_id,
            ConsentStatus::Active,
        );

        ConsentIndex::<T>::mutate(&who, |consents| {
            consents.try_push(consent_id.clone()).ok();
        });

        Self::deposit_event(Event::ConsentGranted {
            data_owner: who,
            consent_id,
            purpose,
        });

        Ok(())
    }

    #[pallet::weight(T::WeightInfo::revoke_consent())]
    pub fn revoke_consent(
        origin: OriginFor<T>,
        consent_id: ConsentId,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // Ensure consent exists and belongs to user
        let policy = Consents::<T>::get(&consent_id)
            .ok_or(Error::<T>::ConsentNotFound)?;
        ensure!(policy.data_owner == who, Error::<T>::NotConsentOwner);

        // Update status
        UserConsents::<T>::insert(&who, &consent_id, ConsentStatus::Revoked {
            revoked_at: T::TimeProvider::now(),
            reason: None,
        });

        // Emit XCM notification to HealthData and Marketplace chains
        Self::notify_consent_revocation(&consent_id)?;

        Self::deposit_event(Event::ConsentRevoked {
            consent_id,
            data_owner: who,
        });

        Ok(())
    }

    #[pallet::weight(T::WeightInfo::check_consent_validity())]
    pub fn check_consent_validity(
        origin: OriginFor<T>,
        consent_id: ConsentId,
        requester: T::AccountId,
        purpose: Purpose,
    ) -> DispatchResultWithPostInfo {
        ensure_signed(origin)?;

        let policy = Consents::<T>::get(&consent_id)
            .ok_or(Error::<T>::ConsentNotFound)?;

        let status = UserConsents::<T>::get(&policy.data_owner, &consent_id)
            .ok_or(Error::<T>::ConsentNotFound)?;

        // Check if consent is active
        ensure!(matches!(status, ConsentStatus::Active), Error::<T>::ConsentNotActive);

        // Check purpose match
        ensure!(Self::purpose_matches(&policy.purpose, &purpose), Error::<T>::PurposeMismatch);

        // Check duration
        let now = T::TimeProvider::now();
        ensure!(now >= policy.duration.start, Error::<T>::ConsentNotYetActive);
        if let Some(end) = policy.duration.end {
            ensure!(now <= end, Error::<T>::ConsentExpired);
        }

        // Check allowed parties
        ensure!(
            Self::is_party_allowed(&policy.allowed_parties, &requester),
            Error::<T>::RequesterNotAllowed
        );

        Self::deposit_event(Event::ConsentChecked {
            consent_id,
            requester,
            valid: true,
        });

        Ok(().into())
    }
}
```

#### Week 7: pallet-authentication

**Priority**: HIGH
**Estimated Effort**: 30 hours

**Implementation**: Role-based access control (RBAC) with session management

**Storage**:
```rust
#[pallet::storage]
pub type UserRoles<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    Blake2_128Concat,
    Role,
    (),
    OptionQuery,
>;

#[pallet::storage]
pub type Sessions<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    SessionToken,
    SessionInfo<T>,
    OptionQuery,
>;
```

#### Week 8: pallet-jurisdiction-manager

**Priority**: MEDIUM
**Estimated Effort**: 25 hours

**Implementation**: Jurisdiction mapping and regulation tracking

---

### Month 3-4: HealthData Chain

#### Week 9-10: Chain Scaffolding + pallet-health-records

**Priority**: CRITICAL
**Estimated Effort**: 60 hours

**Storage Schema**:
```rust
#[pallet::storage]
pub type Records<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    RecordId,
    RecordMetadata<T>,
    OptionQuery,
>;

#[pallet::storage]
pub type RecordVersions<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    RecordId,
    Blake2_128Concat,
    VersionNumber,
    VersionMetadata<T>,
    OptionQuery,
>;

#[pallet::storage]
pub type UserRecords<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    BoundedVec<RecordId, T::MaxRecordsPerUser>,
    ValueQuery,
>;
```

**Types**:
```rust
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct RecordMetadata<T: Config> {
    pub record_id: RecordId,
    pub owner: T::AccountId,
    pub ipfs_cid: CID,
    pub data_format: DataFormat,
    pub encryption_key_id: KeyId,
    pub created_at: T::Moment,
    pub current_version: VersionNumber,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum DataFormat {
    FHIR,
    DICOM,
    HL7,
    CSV,
    JSON,
    PDF,
    Custom(BoundedVec<u8, ConstU32<32>>),
}

pub type RecordId = [u8; 32];
pub type CID = BoundedVec<u8, ConstU32<128>>;
pub type KeyId = [u8; 32];
pub type VersionNumber = u32;
```

#### Week 11-12: pallet-ipfs-integration

**Priority**: HIGH
**Estimated Effort**: 35 hours

**Implementation**: Off-chain worker for IPFS pinning

```rust
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn offchain_worker(block_number: BlockNumberFor<T>) {
        // Process pending IPFS operations
        Self::process_ipfs_queue();
    }
}

impl<T: Config> Pallet<T> {
    fn process_ipfs_queue() {
        let queue = PendingIPFSOperations::<T>::get();

        for operation in queue {
            match operation.op_type {
                IPFSOpType::Pin => {
                    Self::pin_to_ipfs(&operation.cid);
                },
                IPFSOpType::Unpin => {
                    Self::unpin_from_ipfs(&operation.cid);
                },
                IPFSOpType::Verify => {
                    Self::verify_ipfs_content(&operation.cid);
                },
            }
        }
    }
}
```

#### Week 13-14: pallet-access-control + pallet-encryption

**Priority**: CRITICAL
**Estimated Effort**: 50 hours

**XCM Integration for Consent Verification**:
```rust
use xcm::prelude::*;

impl<T: Config> Pallet<T> {
    pub fn request_access_with_consent_check(
        origin: OriginFor<T>,
        record_id: RecordId,
        purpose: Purpose,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        let record = Records::<T>::get(&record_id)
            .ok_or(Error::<T>::RecordNotFound)?;

        // XCM message to IdentityConsent Chain (Para 2000)
        let message = Xcm(vec![
            UnpaidExecution { weight_limit: Unlimited, check_origin: None },
            Transact {
                origin_kind: OriginKind::SovereignAccount,
                require_weight_at_most: Weight::from_parts(1_000_000_000, 0),
                call: <T as Config>::ConsentCall::check_consent_validity(
                    record.owner.clone(),
                    who.clone(),
                    purpose.clone(),
                ).encode().into(),
            },
        ]);

        T::XcmSender::send_xcm(
            MultiLocation::new(1, X1(Parachain(2000))),
            message,
        )?;

        // Store pending access request
        PendingAccessRequests::<T>::insert(
            &record_id,
            AccessRequest {
                requester: who,
                purpose,
                requested_at: T::TimeProvider::now(),
                status: AccessRequestStatus::PendingConsent,
            }
        );

        Ok(())
    }
}
```

#### Week 15-16: pallet-provenance

**Priority**: CRITICAL (Flagship Feature Support)
**Estimated Effort**: 45 hours

**Implementation**: Immutable audit trail with blind signatures

```rust
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ProvenanceEvent<T: Config> {
    pub event_id: EventId,
    pub record_id: RecordId,
    pub event_type: EventType,
    pub actor: T::AccountId,
    pub timestamp: T::Moment,
    pub metadata: BoundedVec<u8, T::MaxMetadataSize>,
    pub blind_signature: BlindSignature,
    pub parent_event: Option<EventId>,
    pub hash_link: Hash,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum EventType {
    Acquisition { source: DataSource },
    Transform { operation: TransformOp },
    Export { destination: ExportDest },
    Reuse { derivative_work: WorkRef },
    NegativeDataSubmission { submission_id: SubmissionId },
    AccessGrant { requester: AccountId },
    ConsentUpdate { consent_id: ConsentId },
}
```

---

### Month 5-6: Marketplace Chain + Negative Registry

#### Week 17-18: Chain Scaffolding + pallet-data-listings

**Priority**: HIGH
**Estimated Effort**: 40 hours

#### Week 19-22: pallet-negative-registry (FLAGSHIP FEATURE)

**Priority**: CRITICAL
**Estimated Effort**: 80 hours

**This is the cornerstone innovation of Patient X**

**Storage Schema**:
```rust
#[pallet::storage]
pub type NegativeDataRegistry<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    SubmissionId,
    NegativeDataSubmission<T>,
    OptionQuery,
>;

#[pallet::storage]
pub type SubmissionsByDiseaseArea<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    DiseaseArea,
    BoundedVec<SubmissionId, T::MaxSubmissionsPerCategory>,
    ValueQuery,
>;

#[pallet::storage]
pub type RewardPool<T: Config> = StorageValue<_, Balance<T>, ValueQuery>;

#[pallet::storage]
pub type ClaimedRewards<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    SubmissionId,
    bool,
    ValueQuery,
>;
```

**Types**:
```rust
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct NegativeDataSubmission<T: Config> {
    pub submission_id: SubmissionId,
    pub submitter: T::AccountId,
    pub listing_id: ListingId,
    pub protocol_reference: ProtocolRef,
    pub trial_phase: TrialPhase,
    pub disease_area: DiseaseArea,
    pub intervention_type: InterventionType,
    pub discontinuation_reason: DiscontinuationReason,
    pub endpoints_measured: BoundedVec<Endpoint, T::MaxEndpoints>,
    pub sample_size: u32,
    pub duration: Duration,
    pub safety_signals: BoundedVec<SafetySignal, T::MaxSafetySignals>,
    pub efficacy_data: Option<EfficacyData>,
    pub learnings: BoundedVec<BoundedVec<u8, ConstU32<256>>, T::MaxLearnings>,
    pub context: BoundedVec<u8, T::MaxContextLength>,
    pub provenance_hash: Hash,
    pub consent_proof: ConsentProof,
    pub verification_status: VerificationStatus<T>,
    pub rewards_claimed: bool,
    pub citations: BoundedVec<PublicationRef, T::MaxCitations>,
    pub submitted_at: T::Moment,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum DiscontinuationReason {
    LackOfEfficacy,
    SafetyConcerns,
    FutilityAnalysis,
    EnrollmentFailure,
    FundingWithdrawn,
    RegulatoryHold,
    CompetitorSuccess,
    Other,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum TrialPhase {
    Phase1,
    Phase2,
    Phase3,
    Phase4,
    PreClinical,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum VerificationStatus<T: Config> {
    Pending,
    Verified {
        verifier: T::AccountId,
        verified_at: T::Moment,
    },
    Rejected {
        reason: BoundedVec<u8, ConstU32<256>>,
    },
}
```

**Dispatchables**:
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Submit negative/abandoned trial data
    #[pallet::weight(T::WeightInfo::submit_negative_data())]
    pub fn submit_negative_data(
        origin: OriginFor<T>,
        listing_id: ListingId,
        protocol_ref: ProtocolRef,
        trial_phase: TrialPhase,
        disease_area: DiseaseArea,
        intervention_type: InterventionType,
        discontinuation_reason: DiscontinuationReason,
        endpoints: BoundedVec<Endpoint, T::MaxEndpoints>,
        sample_size: u32,
        duration: Duration,
        safety_signals: BoundedVec<SafetySignal, T::MaxSafetySignals>,
        efficacy_data: Option<EfficacyData>,
        learnings: BoundedVec<BoundedVec<u8, ConstU32<256>>, T::MaxLearnings>,
        context: BoundedVec<u8, T::MaxContextLength>,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // Generate unique submission ID
        let submission_id = Self::generate_submission_id(&who, &listing_id);

        // Verify protocol reference and consent via XCM to IdentityConsent Chain
        Self::verify_protocol_consent(&protocol_ref)?;

        // Get provenance hash from HealthData Chain
        let provenance_hash = Self::get_provenance_hash(&listing_id)?;

        let submission = NegativeDataSubmission {
            submission_id: submission_id.clone(),
            submitter: who.clone(),
            listing_id,
            protocol_reference: protocol_ref,
            trial_phase,
            disease_area: disease_area.clone(),
            intervention_type,
            discontinuation_reason: discontinuation_reason.clone(),
            endpoints_measured: endpoints,
            sample_size,
            duration,
            safety_signals,
            efficacy_data,
            learnings,
            context,
            provenance_hash,
            consent_proof: ConsentProof::default(), // Set via XCM response
            verification_status: VerificationStatus::Pending,
            rewards_claimed: false,
            citations: BoundedVec::default(),
            submitted_at: T::TimeProvider::now(),
        };

        NegativeDataRegistry::<T>::insert(&submission_id, submission);

        // Index by disease area
        SubmissionsByDiseaseArea::<T>::mutate(&disease_area, |submissions| {
            submissions.try_push(submission_id.clone()).ok();
        });

        // Log provenance event on HealthData Chain
        Self::log_negative_data_event(&submission_id, &listing_id)?;

        Self::deposit_event(Event::NegativeDataSubmitted {
            submission_id,
            submitter: who,
            disease_area,
            discontinuation_reason,
        });

        Ok(())
    }

    /// Verify submission (for institutions/auditors)
    #[pallet::weight(T::WeightInfo::verify_submission())]
    pub fn verify_submission(
        origin: OriginFor<T>,
        submission_id: SubmissionId,
        verification_signature: T::Signature,
    ) -> DispatchResult {
        let verifier = ensure_signed(origin)?;

        // Ensure verifier is authorized (Institution or Auditor)
        ensure!(
            Self::is_authorized_verifier(&verifier),
            Error::<T>::UnauthorizedVerifier
        );

        // Get submission
        let mut submission = NegativeDataRegistry::<T>::get(&submission_id)
            .ok_or(Error::<T>::SubmissionNotFound)?;

        // Verify signature
        ensure!(
            Self::verify_signature(&submission, &verification_signature),
            Error::<T>::InvalidSignature
        );

        // Update verification status
        submission.verification_status = VerificationStatus::Verified {
            verifier: verifier.clone(),
            verified_at: T::TimeProvider::now(),
        };

        NegativeDataRegistry::<T>::insert(&submission_id, submission);

        Self::deposit_event(Event::NegativeDataVerified {
            submission_id,
            verifier,
        });

        Ok(())
    }

    /// Claim rewards for verified negative data submission
    #[pallet::weight(T::WeightInfo::claim_negative_data_reward())]
    pub fn claim_negative_data_reward(
        origin: OriginFor<T>,
        submission_id: SubmissionId,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        let mut submission = NegativeDataRegistry::<T>::get(&submission_id)
            .ok_or(Error::<T>::SubmissionNotFound)?;

        // Ensure submitter is claiming
        ensure!(submission.submitter == who, Error::<T>::NotSubmitter);

        // Ensure verified
        ensure!(
            matches!(submission.verification_status, VerificationStatus::Verified { .. }),
            Error::<T>::SubmissionNotVerified
        );

        // Ensure not already claimed
        ensure!(!submission.rewards_claimed, Error::<T>::RewardAlreadyClaimed);

        // Calculate reward
        let reward = Self::calculate_reward(&submission);

        // Transfer tokens
        T::Currency::transfer(
            &T::RewardAccount::get(),
            &who,
            reward,
            ExistenceRequirement::KeepAlive,
        )?;

        // Mark as claimed
        submission.rewards_claimed = true;
        NegativeDataRegistry::<T>::insert(&submission_id, submission);
        ClaimedRewards::<T>::insert(&submission_id, true);

        Self::deposit_event(Event::RewardClaimed {
            submission_id,
            submitter: who,
            amount: reward,
        });

        Ok(())
    }

    /// Add citation to submission (increases reputation)
    #[pallet::weight(T::WeightInfo::add_citation())]
    pub fn add_citation(
        origin: OriginFor<T>,
        submission_id: SubmissionId,
        publication_ref: PublicationRef,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        let mut submission = NegativeDataRegistry::<T>::get(&submission_id)
            .ok_or(Error::<T>::SubmissionNotFound)?;

        // Add citation
        submission.citations.try_push(publication_ref.clone())
            .map_err(|_| Error::<T>::TooManyCitations)?;

        NegativeDataRegistry::<T>::insert(&submission_id, submission);

        // Award citation bonus to submitter
        let citation_bonus = T::CitationBonus::get();
        T::Currency::transfer(
            &T::RewardAccount::get(),
            &submission.submitter,
            citation_bonus,
            ExistenceRequirement::KeepAlive,
        )?;

        Self::deposit_event(Event::CitationAdded {
            submission_id,
            publication_ref,
            bonus: citation_bonus,
        });

        Ok(())
    }
}

// Helper functions
impl<T: Config> Pallet<T> {
    fn calculate_reward(submission: &NegativeDataSubmission<T>) -> BalanceOf<T> {
        let mut reward = T::BaseReward::get();

        // Quality bonus
        if submission.safety_signals.len() > 0 && submission.efficacy_data.is_some() {
            reward = reward.saturating_add(T::QualityBonus::get());
        }

        // Time bonus (within 6 months of submission)
        let now = T::TimeProvider::now();
        let six_months = T::SixMonthsDuration::get();
        if now.saturating_sub(submission.submitted_at) <= six_months {
            reward = reward.saturating_add(T::TimeBonus::get());
        }

        // Rarity bonus (based on disease area)
        if Self::is_rare_disease(&submission.disease_area) {
            reward = reward.saturating_add(T::RarityBonus::get());
        }

        // Citation bonus
        let citation_count = submission.citations.len() as u32;
        let citation_reward = T::CitationBonus::get()
            .saturating_mul(citation_count.into());
        reward = reward.saturating_add(citation_reward);

        reward
    }

    fn verify_protocol_consent(protocol_ref: &ProtocolRef) -> DispatchResult {
        // XCM message to IdentityConsent Chain
        let message = Xcm(vec![
            UnpaidExecution { weight_limit: Unlimited, check_origin: None },
            Transact {
                origin_kind: OriginKind::SovereignAccount,
                require_weight_at_most: Weight::from_parts(1_000_000_000, 0),
                call: <T as Config>::ConsentCall::verify_protocol_reference(
                    protocol_ref.clone(),
                ).encode().into(),
            },
        ]);

        T::XcmSender::send_xcm(
            MultiLocation::new(1, X1(Parachain(2000))),
            message,
        )?;

        Ok(())
    }

    fn log_negative_data_event(
        submission_id: &SubmissionId,
        listing_id: &ListingId,
    ) -> DispatchResult {
        // XCM message to HealthData Chain
        let message = Xcm(vec![
            UnpaidExecution { weight_limit: Unlimited, check_origin: None },
            Transact {
                origin_kind: OriginKind::SovereignAccount,
                require_weight_at_most: Weight::from_parts(1_000_000_000, 0),
                call: <T as Config>::ProvenanceCall::log_negative_data_submission(
                    listing_id.clone(),
                    submission_id.clone(),
                ).encode().into(),
            },
        ]);

        T::XcmSender::send_xcm(
            MultiLocation::new(1, X1(Parachain(2001))),
            message,
        )?;

        Ok(())
    }
}
```

**Tests** (`pallets/negative-registry/src/tests.rs`):
```rust
#[test]
fn submit_negative_data_works() {
    new_test_ext().execute_with(|| {
        let researcher = 1u64;
        let listing_id = [1u8; 32];

        assert_ok!(NegativeRegistry::submit_negative_data(
            RuntimeOrigin::signed(researcher),
            listing_id,
            b"NCT12345678".to_vec().try_into().unwrap(),
            TrialPhase::Phase2,
            DiseaseArea::Oncology,
            InterventionType::Drug,
            DiscontinuationReason::LackOfEfficacy,
            vec![Endpoint::OverallSurvival].try_into().unwrap(),
            100,
            365,
            vec![].try_into().unwrap(),
            None,
            vec![b"Insufficient patient enrollment".to_vec().try_into().unwrap()].try_into().unwrap(),
            b"Phase 2 trial for advanced melanoma".to_vec().try_into().unwrap(),
        ));

        // Verify submission exists
        let submission_id = NegativeRegistry::generate_submission_id(&researcher, &listing_id);
        assert!(NegativeDataRegistry::<Test>::contains_key(&submission_id));
    });
}

#[test]
fn claim_reward_requires_verification() {
    new_test_ext().execute_with(|| {
        let researcher = 1u64;
        let submission_id = [2u8; 32];

        // Submit data
        // ... (setup code)

        // Try to claim before verification
        assert_noop!(
            NegativeRegistry::claim_negative_data_reward(
                RuntimeOrigin::signed(researcher),
                submission_id,
            ),
            Error::<Test>::SubmissionNotVerified
        );
    });
}

#[test]
fn reward_calculation_includes_bonuses() {
    new_test_ext().execute_with(|| {
        let submission = create_test_submission();
        let reward = NegativeRegistry::calculate_reward(&submission);

        // Should include base + quality + time + rarity bonuses
        assert_eq!(reward, 1000 + 500 + 300 + 1000);
    });
}
```

#### Week 23-24: pallet-marketplace + pallet-reputation

**Priority**: HIGH
**Estimated Effort**: 50 hours

---

### Month 7: Integration & XCM Testing

#### Week 25-26: XCM Message Flows

**Priority**: CRITICAL
**Estimated Effort**: 60 hours

**Test Scenarios**:

1. **Data Access Request with Compliance Check**
   - Marketplace → IdentityConsent (check consent)
   - Marketplace → IdentityConsent (check jurisdiction)
   - IdentityConsent → Marketplace (consent valid)
   - Marketplace → HealthData (request access)
   - HealthData → IdentityConsent (verify consent)
   - HealthData → Marketplace (grant access)

2. **Negative Data Submission with Provenance**
   - Marketplace → IdentityConsent (verify protocol)
   - Marketplace → HealthData (log provenance)
   - HealthData → Marketplace (provenance hash)

3. **Consent Update Propagation**
   - IdentityConsent → HealthData (update access)
   - IdentityConsent → Marketplace (update listings)

**XCM Test Framework** (`tests/xcm_integration.rs`):
```rust
use xcm_simulator::*;

decl_test_parachain! {
    pub struct IdentityConsentPara {
        Runtime = identity_consent_runtime::Runtime,
        XcmpMessageHandler = identity_consent_runtime::XcmpQueue,
        DmpMessageHandler = identity_consent_runtime::DmpQueue,
        new_ext = identity_para_ext(2000),
    }
}

decl_test_parachain! {
    pub struct MarketplacePara {
        Runtime = marketplace_runtime::Runtime,
        XcmpMessageHandler = marketplace_runtime::XcmpQueue,
        DmpMessageHandler = marketplace_runtime::DmpQueue,
        new_ext = marketplace_para_ext(2002),
    }
}

decl_test_relay_chain! {
    pub struct Relay {
        Runtime = polkadot_runtime::Runtime,
        RuntimeCall = polkadot_runtime::RuntimeCall,
        RuntimeEvent = polkadot_runtime::RuntimeEvent,
        XcmConfig = polkadot_runtime::XcmConfig,
        MessageQueue = polkadot_runtime::MessageQueue,
        System = polkadot_runtime::System,
        new_ext = relay_ext(),
    }
}

#[test]
fn test_consent_check_via_xcm() {
    MockNet::reset();

    MarketplacePara::execute_with(|| {
        // Request consent check
        assert_ok!(pallet_marketplace::Pallet::<marketplace_runtime::Runtime>::purchase_access(
            RuntimeOrigin::signed(ALICE),
            listing_id,
            1000,
        ));
    });

    // Process XCM messages
    MockNet::process_messages(1000);

    IdentityConsentPara::execute_with(|| {
        // Verify consent was checked
        assert!(pallet_consent_manager::ConsentChecks::<identity_consent_runtime::Runtime>::contains_key(&consent_id));
    });
}
```

#### Week 27-28: End-to-End Testing

**Priority**: CRITICAL
**Estimated Effort**: 50 hours

**Test Scenarios**:

1. **Patient Journey**
   - Register identity
   - Grant consent
   - Upload medical record
   - Revoke consent
   - Verify access is blocked

2. **Researcher Journey**
   - Register identity
   - Search for datasets
   - Purchase access
   - Submit negative data
   - Claim rewards

3. **Institution Journey**
   - Register identity
   - Verify user identities
   - Verify negative data submissions
   - Create consortium

---

## Phase 2: Compliance & Scale (Q3-Q4 2025)

### Month 8-9: Compliance Engine

#### Week 29-32: pallet-compliance-engine

**Priority**: CRITICAL
**Estimated Effort**: 80 hours

**Regulation Types**:
- GDPR (EU)
- HIPAA (US)
- PDPA (Singapore)
- LGPD (Brazil)
- PIPEDA (Canada)

**Implementation** (see README.md lines 867-1001 for detailed spec)

**Compliance Rule Engine**:
```rust
pub struct ComplianceRule<T: Config> {
    pub rule_id: RuleId,
    pub jurisdiction: JurisdictionCode,
    pub regulation: RegulationType,
    pub conditions: BoundedVec<Condition, T::MaxConditions>,
    pub requirements: BoundedVec<Requirement, T::MaxRequirements>,
    pub penalties: BoundedVec<Penalty, T::MaxPenalties>,
}

pub enum RegulationType {
    GDPR,
    HIPAA,
    PDPA_Singapore,
    LGPD_Brazil,
    PIPEDA_Canada,
    Custom(BoundedVec<u8, ConstU32<64>>),
}

pub fn check_transaction_compliance(
    transaction_id: TransactionId,
) -> Result<ComplianceReport<T>, ComplianceError> {
    // 1. Get transaction details
    // 2. Determine applicable jurisdictions
    // 3. Get applicable regulations
    // 4. Check each regulation
    // 5. Generate compliance report
}
```

**Data Passport Architecture**:
```rust
pub struct DataPassport<T: Config> {
    pub passport_id: PassportId,
    pub data_id: RecordId,
    pub origin_jurisdiction: JurisdictionCode,
    pub current_jurisdiction: JurisdictionCode,
    pub allowed_jurisdictions: BoundedVec<JurisdictionCode, T::MaxJurisdictions>,
    pub transfer_history: BoundedVec<Transfer<T>, T::MaxTransfers>,
    pub compliance_attestations: BoundedVec<Attestation<T>, T::MaxAttestations>,
    pub on_chain_provenance_hash: Hash,
    pub off_chain_storage_refs: BoundedVec<StorageRef, T::MaxStorageRefs>,
}
```

### Month 10-11: Open Science & Collaboration

#### Week 33-37: pallet-consortium

**Priority**: HIGH
**Estimated Effort**: 80 hours

**Consortium Types**:
- Pre-publication consortia (with embargoes)
- Industry collaborations
- Academic networks
- Regulatory sandboxes

**Implementation** (see README.md lines 989-1123 for detailed spec)

**Embargo Support**:
```rust
pub struct Embargo<T: Config> {
    pub embargo_id: EmbargoId,
    pub embargo_until: T::Moment,
    pub allowed_reviewers: BoundedVec<T::AccountId, T::MaxReviewers>,
    pub reviewer_access_terms: AccessTerms,
    pub automatic_release: bool,
}

pub fn grant_reproducibility_access(
    origin: OriginFor<T>,
    consortium_id: ConsortiumId,
    reviewer: T::AccountId,
    time_window: Duration,
    access_level: AccessLevel,
) -> DispatchResult;
```

### Month 12: Pilot Testing

#### Week 38-41: Pilot Programs

**Priority**: CRITICAL
**Estimated Effort**: Full team

**Pilot Partners**:
1. **EU Institution** (GDPR testing)
2. **US Research Hospital** (HIPAA testing)
3. **Singapore Medical Center** (PDPA testing)

**Test Scenarios**:
- Cross-border data transfer
- Compliance violation detection
- Negative data submission workflow
- Consortium creation and management

---

## Phase 3: Advanced Analytics (Q1-Q2 2026)

### Month 13-15: Federated Learning

#### Week 42-50: pallet-federated-ml

**Priority**: HIGH
**Estimated Effort**: 120 hours

**Implementation** (see README.md lines 1127-1273 for detailed spec)

**Federated Learning Flow**:
1. Create federated project
2. Institutions submit local updates
3. Coordinator aggregates updates
4. Distribute rewards based on contribution

**Privacy Budget Management**:
```rust
pub struct PrivacyBudget {
    pub epsilon: f64,
    pub delta: f64,
    pub spent: f64,
    pub remaining: f64,
}

pub fn update_privacy_budget(
    listing_id: ListingId,
    query_cost: f64,
) -> Result<RemainingBudget, BudgetExhaustedError>;
```

### Month 16-18: Analytics & Reputation

#### Week 51-60: pallet-analytics + Enhanced Reputation

**Priority**: MEDIUM
**Estimated Effort**: 100 hours

**Analytics Metrics**:
- Access frequency
- Data utilization patterns
- Market trends
- Compliance violations
- Negative data impact

**Reputation System v2**:
- Multi-dimensional scoring
- Time-weighted contributions
- Citation impact
- Compliance history

---

## Phase 4: Incentives & IP (Q3 2026)

### Month 19-21: Patient Incentives

#### Week 61-70: Enhanced Reward System

**Priority**: HIGH
**Estimated Effort**: 100 hours

**Implementation** (see README.md lines 1276-1342 for detailed spec)

**Bounty System**:
- Negative data release bounties
- Third-party validation bounties
- Compliance sandbox testing bounties
- Longitudinal follow-up bounties

### Month 22-24: IP & Licensing

#### Week 71-80: IP Layer + Version Control

**Priority**: HIGH
**Estimated Effort**: 100 hours

**Implementation** (see README.md lines 1344-1464 for detailed spec)

**License Types**:
- Creative Commons (CC-BY, CC-BY-SA, etc.)
- Proprietary
- Embargoed
- Academic (commercial use restrictions)

---

## Phase 5: Privacy & Interoperability (Q4 2026 - Q1 2027)

### Month 25-26: Zero-Knowledge Proofs

#### Week 81-90: ZK Integration

**Priority**: MEDIUM
**Estimated Effort**: 120 hours

**Implementation** (see README.md lines 1467-1535 for detailed spec)

**ZK Proof Types**:
- Range proofs (age compliance)
- Membership proofs (dataset criteria)
- Equality proofs
- Compliance proofs

### Month 27-28: Cross-Chain Bridges

#### Week 91-100: Ethereum + Cosmos Bridges

**Priority**: HIGH
**Estimated Effort**: 120 hours

**Ethereum Bridge**:
- Smart contract deployment
- Merkle proof verification
- Attestation transfer

**Cosmos IBC**:
- IBC channel creation
- Cross-chain compliance verification

---

## Technical Architecture Details

### XCM Configuration

**All chains configured with**:
```rust
// In runtime/src/lib.rs
impl cumulus_pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type ChannelInfo = ParachainSystem;
    type VersionWrapper = ();
    type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
    type ControllerOrigin = EnsureRoot<AccountId>;
    type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
    type WeightInfo = ();
    type PriceForSiblingDelivery = ();
}

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = XcmOriginToTransactDispatchOrigin;
    type IsReserve = ();
    type IsTeleporter = ();
    type UniversalLocation = UniversalLocation;
    type Barrier = Barrier;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    type Trader = ();
    type ResponseHandler = PolkadotXcm;
    type AssetTrap = PolkadotXcm;
    type AssetClaims = PolkadotXcm;
    type SubscriptionService = PolkadotXcm;
    type PalletInstancesInfo = AllPalletsWithSystem;
    type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
    type AssetLocker = ();
    type AssetExchanger = ();
    type FeeManager = ();
    type MessageExporter = ();
    type UniversalAliases = ();
    type CallDispatcher = RuntimeCall;
    type SafeCallFilter = ();
}
```

### Trusted Parachain Configuration

```rust
// Trust Patient X parachains
pub type Barrier = (
    TakeWeightCredit,
    AllowTopLevelPaidExecutionFrom<Everything>,
    AllowUnpaidExecutionFrom<ParentOrParasiblings>, // Trust relay + siblings
    // Allow XCMP from specific parachains
    AllowExplicitUnpaidExecutionFrom<(
        IsInVec<AllowedParachains>, // 2000, 2001, 2002
    )>,
);

parameter_types! {
    pub AllowedParachains: Vec<MultiLocation> = vec![
        MultiLocation::new(1, X1(Parachain(2000))), // IdentityConsent
        MultiLocation::new(1, X1(Parachain(2001))), // HealthData
        MultiLocation::new(1, X1(Parachain(2002))), // Marketplace
    ];
}
```

### IPFS Integration

**Off-Chain Worker Setup**:
```rust
// In pallets/ipfs-integration/src/lib.rs
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn offchain_worker(block_number: BlockNumberFor<T>) {
        log::info!("IPFS worker running at block {:?}", block_number);

        if block_number % T::ProcessingInterval::get() == 0u32.into() {
            Self::process_ipfs_queue();
        }
    }
}

fn process_ipfs_queue() {
    let queue = PendingIPFSOperations::<T>::get();

    for operation in queue.iter().take(MAX_OPERATIONS_PER_BLOCK) {
        match operation.op_type {
            IPFSOpType::Pin => {
                if let Ok(result) = Self::ipfs_pin(&operation.cid) {
                    Self::mark_operation_complete(operation.id);
                    Self::submit_pin_confirmation(operation.cid, result);
                }
            },
            IPFSOpType::Unpin => {
                if let Ok(_) = Self::ipfs_unpin(&operation.cid) {
                    Self::mark_operation_complete(operation.id);
                }
            },
            IPFSOpType::Verify => {
                if let Ok(hash) = Self::ipfs_verify(&operation.cid) {
                    Self::submit_verification_result(operation.cid, hash);
                }
            },
        }
    }
}

fn ipfs_pin(cid: &CID) -> Result<PinResult, IPFSError> {
    let ipfs_url = T::IPFSEndpoint::get();
    let url = format!("{}/api/v0/pin/add?arg={}", ipfs_url, cid);

    let request = http::Request::post(&url)
        .body(vec![])
        .send()
        .map_err(|_| IPFSError::NetworkError)?;

    let response = request
        .wait()
        .map_err(|_| IPFSError::RequestTimeout)?;

    if response.code != 200 {
        return Err(IPFSError::PinFailed);
    }

    Ok(PinResult::Success)
}
```

---

## Team Structure & Resources

### Core Team (Estimated 15-20 people)

#### Engineering (12 people)

1. **Tech Lead** (1)
   - Overall architecture
   - Code review
   - Technical decision-making

2. **Blockchain Engineers** (4)
   - Pallet development
   - Runtime configuration
   - XCM integration

3. **Backend Engineers** (2)
   - IPFS integration
   - Off-chain workers
   - API development

4. **Frontend Engineers** (2)
   - Polkadot.js UI
   - Custom dashboard
   - Mobile app (optional)

5. **DevOps Engineers** (2)
   - CI/CD setup
   - Infrastructure management
   - Monitoring & alerting

6. **Security Engineer** (1)
   - Smart contract audits
   - Penetration testing
   - Incident response

#### Research & Compliance (3 people)

7. **Compliance Expert** (1)
   - GDPR/HIPAA/PDPA expertise
   - Regulatory engagement
   - Policy development

8. **Medical Domain Expert** (1)
   - Clinical trial expertise
   - Negative data strategy
   - Stakeholder engagement

9. **Cryptography Researcher** (1)
   - ZK proof design
   - Privacy-preserving protocols
   - Security analysis

#### Product & Design (3 people)

10. **Product Manager** (1)
    - Roadmap planning
    - Stakeholder coordination
    - Feature prioritization

11. **UX/UI Designer** (1)
    - User experience design
    - Interface design
    - Usability testing

12. **Technical Writer** (1)
    - Documentation
    - Tutorials
    - Developer guides

#### Community & Operations (2 people)

13. **Community Manager** (1)
    - Discord/Telegram management
    - Developer relations
    - Event organization

14. **Operations Manager** (1)
    - Project coordination
    - Budget management
    - Vendor relationships

### Budget Estimate (Phase 1-5)

**Personnel Costs** (24 months):
- Engineering: $2.4M
- Research & Compliance: $480K
- Product & Design: $480K
- Community & Operations: $240K
- **Total Personnel**: $3.6M

**Infrastructure Costs**:
- Cloud services: $120K
- IPFS hosting: $60K
- Testing infrastructure: $40K
- **Total Infrastructure**: $220K

**External Services**:
- Security audits: $200K
- Legal/regulatory: $150K
- Marketing: $100K
- **Total Services**: $450K

**Contingency** (15%): $640K

**Grand Total**: ~$4.9M

---

## Risk Management

### Technical Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| XCM message failures | HIGH | Retry logic, timeout handling, comprehensive testing |
| IPFS data availability | HIGH | Multiple pinning services, redundancy, fallback mechanisms |
| Scalability bottlenecks | MEDIUM | Benchmarking, optimization, parathread option |
| Smart contract vulnerabilities | HIGH | Multiple audits, bug bounty program, gradual rollout |
| Cryptographic weaknesses | HIGH | Peer review, standard libraries, expert consultation |

### Regulatory Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| GDPR non-compliance | CRITICAL | Legal review, compliance testing, DPO appointment |
| HIPAA violations | CRITICAL | Security controls, audit trails, BAA agreements |
| Cross-border data transfer restrictions | HIGH | Data passport, jurisdiction mapping, legal guidance |
| Changing regulations | MEDIUM | Modular compliance engine, runtime upgrades |

### Business Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| Low adoption | HIGH | Pilot programs, partnerships, incentives |
| Competitor launches | MEDIUM | Unique features (negative registry), network effects |
| Funding shortfall | MEDIUM | Phased development, grant applications, token sale |
| Key personnel loss | MEDIUM | Documentation, knowledge sharing, succession planning |

### Operational Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| Development delays | MEDIUM | Buffer time, agile methodology, regular checkpoints |
| Infrastructure outages | MEDIUM | Redundancy, monitoring, incident response plan |
| Security breaches | HIGH | Defense in depth, penetration testing, incident response |
| Data loss | HIGH | Backups, IPFS redundancy, disaster recovery plan |

---

## Success Metrics

### Phase 1 (Foundation)

- [ ] 3 parachains operational on Rococo testnet
- [ ] 10+ custom pallets deployed
- [ ] XCM messages flowing between all chains
- [ ] 100+ negative data submissions (testnet)
- [ ] 50+ test users registered

### Phase 2 (Compliance & Scale)

- [ ] Compliance engine handles 5 jurisdictions
- [ ] 10+ pilot partners onboarded
- [ ] 1000+ test transactions processed
- [ ] 100% compliance check success rate
- [ ] 5+ active consortia

### Phase 3 (Advanced Analytics)

- [ ] 3+ federated learning projects
- [ ] 10+ institutions participating in FL
- [ ] Privacy budget tracking operational
- [ ] Analytics dashboard live
- [ ] Reputation system v2 deployed

### Phase 4 (Incentives & IP)

- [ ] 10,000+ tokens distributed as rewards
- [ ] 50+ bounties claimed
- [ ] 100+ datasets with versioning
- [ ] 5+ license types supported
- [ ] IP tracking for 1000+ datasets

### Phase 5 (Privacy & Interoperability)

- [ ] ZK proofs integrated for 3+ use cases
- [ ] Ethereum bridge operational
- [ ] Cosmos IBC channel open
- [ ] Cross-chain provenance working
- [ ] 100+ cross-chain transactions

### Mainnet Launch (Q4 2026)

- [ ] Security audits completed (3 firms)
- [ ] Parachain auction won
- [ ] 10,000+ user registrations
- [ ] 1,000+ datasets listed
- [ ] 100+ verified negative data submissions
- [ ] 50+ active institutions
- [ ] $1M+ in marketplace transactions

---

## Conclusion

This development plan provides a comprehensive roadmap for building Patient X, a groundbreaking decentralized medical data marketplace on Polkadot. The phased approach ensures:

1. **Strong Foundation**: Core infrastructure and flagship Negative Registry in Phase 1
2. **Compliance First**: Automated regulatory enforcement in Phase 2
3. **Advanced Features**: Federated ML and analytics in Phase 3
4. **Sustainable Economics**: Incentive mechanisms in Phase 4
5. **Future-Proof**: Privacy and interoperability in Phase 5

The project's success depends on:
- **Technical Excellence**: Following Polkadot SDK best practices (see CLAUDE.md)
- **Regulatory Rigor**: Embedding compliance from day one
- **User Focus**: Solving real problems for patients, researchers, and institutions
- **Innovation**: The Negative Data Registry as a unique value proposition

**Next Steps**:
1. Assemble core team
2. Set up development environment
3. Begin Phase 1, Week 1: IdentityConsent Chain scaffolding
4. Secure pilot partners
5. Apply for Web3 Foundation grants

---

**Document Version**: 1.0
**Last Updated**: 2025-11-15
**Maintained By**: Patient X Core Team
**Questions**: contact@patientx.network
