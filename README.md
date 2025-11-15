# Patient X - Decentralized Medical Data Marketplace on Polkadot

> A complete medical data marketplace ecosystem built on Polkadot SDK, enabling secure, compliant, and transparent sharing of clinical research data including negative and abandoned trial results.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Polkadot SDK](https://img.shields.io/badge/Polkadot-SDK-E6007A)](https://github.com/paritytech/polkadot-sdk)
[![Substrate](https://img.shields.io/badge/Substrate-v1.0-brightgreen)](https://substrate.io/)

## Table of Contents

- [Overview](#overview)
- [Vision & Innovation](#vision--innovation)
- [Architecture](#architecture)
- [Core Features](#core-features)
- [New Features Roadmap](#new-features-roadmap)
- [Technology Stack](#technology-stack)
- [Getting Started](#getting-started)
- [Development Guide](#development-guide)
- [Security & Compliance](#security--compliance)
- [Directory Structure](#directory-structure)
- [Contributing](#contributing)
- [License](#license)

## Overview

Patient X is a decentralized medical data marketplace built as a fork of the [Polkadot SDK](https://github.com/paritytech/polkadot-sdk), implementing three interconnected parachains that enable secure, transparent, and compliant sharing of clinical research data.

### The Problem

- **Publication Bias**: Negative and abandoned clinical trial data is often lost, leading to redundant research and wasted resources
- **Data Silos**: Medical research data is fragmented across institutions with no standardized sharing mechanism
- **Compliance Complexity**: Cross-border data sharing faces conflicting regulations (GDPR, HIPAA, regional laws)
- **Trust Deficit**: Lack of verifiable data provenance and auditability undermines research integrity
- **Reproducibility Crisis**: Scientific results cannot be validated due to inaccessible underlying data

### The Solution

Patient X creates a blockchain-based ecosystem that:

1. **Incentivizes sharing of negative/abandoned data** through tokenized rewards
2. **Ensures cryptographic provenance** for all data transformations and access
3. **Automates cross-border compliance** with embedded regulatory logic
4. **Enables federated analytics** for privacy-preserving multi-institutional research
5. **Supports open science** with pre-publication embargoes and reproducibility windows
6. **Empowers patients** with granular consent control and participation incentives

## Vision & Innovation

### Negative Data Registry - A First-of-its-Kind Feature

The cornerstone innovation of Patient X is the **Negative/Abandoned Data Registry**, addressing one of healthcare research's most significant gaps. Failed trials and discontinued studies contain invaluable information that could:

- Prevent redundant research costing millions
- Identify safety signals earlier
- Accelerate drug development by ruling out dead ends
- Improve research design through lessons learned

**Our approach**:
- Structured metadata schema for negative results (endpoints, reason for failure, context)
- Immutable provenance linking to original protocols and consent records
- Token-based incentives for data contributors
- Publisher and regulatory recognition mechanisms
- Analytics credits for accessing negative datasets

### Global Compliance by Design

Rather than treating compliance as an afterthought, Patient X embeds regulatory logic directly into the blockchain:

- **Dynamic jurisdiction mapping**: Automatically applies GDPR, HIPAA, or regional rules based on patient residency and data location
- **Real-time compliance checking**: Each cross-border transaction is validated before execution
- **Data passport architecture**: Balances on-chain provenance with off-chain data residency requirements
- **Automated reporting**: Generates compliance attestations for regulators

### Open Science Infrastructure

Patient X provides the technical foundation for collaborative research:

- **Consortium modules**: Multi-party data pooling with fine-grained access control
- **Time-locked embargoes**: Support pre-publication data sharing with IP protection
- **Reproducibility windows**: Cryptographically verified access for peer reviewers
- **Version control**: Track dataset evolution with immutable audit trails

## Architecture

Patient X consists of three interconnected Polkadot parachains, each serving a specific function in the data marketplace ecosystem.

### System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        Polkadot Relay Chain                      │
│                    (Shared Security & Consensus)                 │
└────────────┬──────────────────┬──────────────────┬──────────────┘
             │                  │                  │
             │ XCM              │ XCM              │ XCM
             │                  │                  │
┌────────────▼─────────┐ ┌──────▼──────────┐ ┌────▼──────────────┐
│  IdentityConsent     │ │   HealthData    │ │   Marketplace     │
│  Chain (Para 2000)   │ │ Chain (Para 2001)│ │ Chain (Para 2002) │
├──────────────────────┤ ├─────────────────┤ ├───────────────────┤
│ • Identity Registry  │ │ • Health Records│ │ • Data Listings   │
│ • Consent Manager    │ │ • IPFS Integration│ • Negative Data   │
│ • Authentication     │ │ • Access Control│ │   Registry        │
│ • Role Management    │ │ • Encryption    │ │ • Marketplace Ops │
│ • Jurisdiction Info  │ │ • Audit Trail   │ │ • Compliance      │
│                      │ │ • Provenance    │ │   Engine          │
│                      │ │                 │ │ • Reputation      │
│                      │ │                 │ │ • Analytics       │
│                      │ │                 │ │ • Federated ML    │
└──────────────────────┘ └─────────────────┘ └───────────────────┘
         │                       │                      │
         │                       │                      │
         └───────────────────────┴──────────────────────┘
                                 │
                    ┌────────────▼─────────────┐
                    │    External Integrations  │
                    ├──────────────────────────┤
                    │ • IPFS Storage           │
                    │ • Regulatory APIs        │
                    │ • Publisher Systems      │
                    │ • ML/AI Platforms        │
                    │ • Cross-chain Bridges    │
                    │   (Ethereum, Cosmos)     │
                    └──────────────────────────┘
```

### 1. IdentityConsent Chain (Para ID: 2000)

**Purpose**: Identity, authentication, consent management, and jurisdiction tracking

#### Core Pallets

##### `pallet-identity-registry`
Manages user identities using Decentralized Identifiers (DIDs):
- **User Types**: Patients, Researchers, Institutions, Auditors, Publishers
- **Attributes**: Name, DID, verification status, jurisdiction, institution affiliation
- **Functions**:
  - `register_identity(origin, did, user_type, jurisdiction)`
  - `update_jurisdiction(origin, new_jurisdiction)`
  - `verify_identity(origin, user_did, verifier_signature)`
  - `get_user_info(did) -> UserInfo`

##### `pallet-consent-manager`
Handles granular consent policies:
- **Consent Attributes**:
  - Purpose (research, commercial, specific study)
  - Duration (start date, end date, auto-renewal)
  - Data types (demographics, diagnostics, genomics, imaging)
  - Allowed parties (specific researchers, institutions, categories)
  - Cross-border restrictions
  - Compensation preferences
- **Functions**:
  - `grant_consent(origin, data_owner, purpose, duration, data_types, allowed_parties, jurisdictions)`
  - `revoke_consent(origin, consent_id)`
  - `update_consent(origin, consent_id, new_terms)`
  - `check_consent_validity(consent_id, requester, purpose) -> bool`
  - `set_consent_embargo(origin, consent_id, embargo_until)`

##### `pallet-authentication`
Authentication and authorization:
- **Role-Based Access Control (RBAC)**
- **Multi-factor authentication support**
- **Session management**
- **Functions**:
  - `authenticate(origin, credentials) -> SessionToken`
  - `authorize_action(origin, action, resource) -> bool`
  - `assign_role(origin, user_did, role)`

##### `pallet-jurisdiction-manager` (New)
Tracks user and data jurisdictions:
- **Functions**:
  - `register_jurisdiction(origin, jurisdiction_code, regulations)`
  - `get_applicable_regulations(user_jurisdiction, data_jurisdiction) -> Vec<Regulation>`
  - `check_cross_border_compliance(from_jurisdiction, to_jurisdiction, data_type) -> ComplianceStatus`

**Storage**:
```rust
// Identity Registry
Identities: map AccountId => UserInfo
DIDs: map DID => AccountId
VerifiedUsers: map DID => VerificationStatus

// Consent Manager
Consents: map ConsentId => ConsentPolicy
UserConsents: double_map AccountId, ConsentId => ConsentStatus
ConsentIndex: map AccountId => Vec<ConsentId>

// Jurisdiction
UserJurisdictions: map AccountId => JurisdictionCode
Regulations: map JurisdictionCode => Vec<Regulation>
```

**Events**:
```rust
IdentityRegistered(AccountId, DID, UserType)
ConsentGranted(AccountId, ConsentId, Purpose)
ConsentRevoked(ConsentId, Reason)
ConsentUpdated(ConsentId)
JurisdictionChanged(AccountId, OldJurisdiction, NewJurisdiction)
```

### 2. HealthData Chain (Para ID: 2001)

**Purpose**: Medical record anchoring, encryption, IPFS integration, and access enforcement

#### Core Pallets

##### `pallet-health-records`
Manages medical record metadata and anchoring:
- **Record Attributes**:
  - Record ID (hash-based)
  - Owner (patient DID)
  - IPFS CID (content identifier)
  - Data format (FHIR, DICOM, HL7, custom)
  - Encryption metadata
  - Creation timestamp
  - Version history
- **Functions**:
  - `create_record(origin, ipfs_cid, data_format, encryption_key_id)`
  - `update_record(origin, record_id, new_ipfs_cid, version_note)`
  - `get_record_metadata(record_id) -> RecordMetadata`
  - `list_user_records(user_did) -> Vec<RecordId>`

##### `pallet-ipfs-integration`
Manages IPFS content addressing and pinning:
- **Functions**:
  - `pin_content(origin, ipfs_cid)`
  - `unpin_content(origin, ipfs_cid)`
  - `verify_ipfs_hash(ipfs_cid, expected_hash) -> bool`
  - `get_pinning_status(ipfs_cid) -> PinStatus`

##### `pallet-access-control`
Enforces access based on consent (queries IdentityConsent Chain):
- **Access Patterns**:
  - One-time access
  - Time-limited access
  - Recurring access (with limits)
  - Federated query access (for ML)
- **Functions**:
  - `request_access(origin, record_id, purpose)`
  - `grant_temporary_access(origin, requester, record_id, duration)`
  - `check_access_permission(requester, record_id) -> AccessStatus`
  - `revoke_access(origin, access_grant_id)`

##### `pallet-encryption`
Manages encryption keys and policies:
- **Encryption Strategy**: ChaCha20-Poly1305 for data, X25519 for key exchange
- **Functions**:
  - `generate_encryption_key(origin) -> KeyId`
  - `share_key(origin, key_id, recipient, encrypted_key)`
  - `rotate_key(origin, old_key_id) -> KeyId`
  - `get_encrypted_key(key_id, requester) -> Option<EncryptedKey>`

##### `pallet-provenance` (New)
Immutable audit trail with blind signatures:
- **Tracked Events**:
  - Data acquisition
  - Transformations (anonymization, aggregation)
  - Export operations
  - Reuse in derivative works
  - Negative data submission
- **Functions**:
  - `log_data_event(origin, record_id, event_type, metadata, blind_signature)`
  - `get_provenance_chain(record_id) -> Vec<ProvenanceEvent>`
  - `verify_provenance(record_id, event_id, signature) -> bool`
  - `export_audit_trail(record_id) -> AuditTrail`

**Storage**:
```rust
// Health Records
Records: map RecordId => RecordMetadata
RecordVersions: double_map RecordId, VersionNumber => VersionMetadata
UserRecords: map AccountId => Vec<RecordId>

// Access Control
AccessGrants: map GrantId => AccessGrant
ActiveAccesses: double_map RecordId, AccountId => AccessStatus

// Provenance
ProvenanceChain: map RecordId => Vec<ProvenanceEvent>
EventSignatures: map EventId => BlindSignature
```

**Events**:
```rust
RecordCreated(RecordId, Owner, IPFS_CID)
RecordUpdated(RecordId, VersionNumber)
AccessRequested(RecordId, Requester, Purpose)
AccessGranted(GrantId, RecordId, Requester, Duration)
AccessRevoked(GrantId, Reason)
ProvenanceLogged(RecordId, EventType, Timestamp)
```

### 3. Marketplace Chain (Para ID: 2002)

**Purpose**: Data discovery, listing, economic transactions, and compliance automation

#### Core Pallets

##### `pallet-data-listings`
Manages data set listings and discovery:
- **Listing Attributes**:
  - Dataset ID
  - Owner
  - Title & description
  - Data types included
  - Sample size
  - Quality metrics
  - Price (fixed or dynamic)
  - Availability status
  - Tags/categories
  - License terms
  - **Negative data flag** (new)
  - **IP status** (new)
- **Functions**:
  - `create_listing(origin, metadata, price, license)`
  - `update_listing(origin, listing_id, new_metadata)`
  - `flag_as_negative(origin, listing_id, failure_reason, context)`
  - `search_listings(query, filters) -> Vec<ListingId>`
  - `get_listing_details(listing_id) -> ListingMetadata`

##### `pallet-negative-registry` (New)
Dedicated registry for negative/abandoned data:
- **Negative Data Metadata**:
  - Original protocol reference
  - Reason for discontinuation
  - Endpoints measured
  - Context and learnings
  - Safety signals
  - Link to original consent records
  - Cryptographic provenance hash
- **Functions**:
  - `submit_negative_data(origin, listing_id, protocol_ref, reason, endpoints, context)`
  - `verify_negative_data(origin, submission_id, verifier_signature)`
  - `search_negative_data(disease_area, drug_class, endpoints) -> Vec<SubmissionId>`
  - `claim_reward(origin, submission_id)`

##### `pallet-marketplace`
Handles transactions and payments:
- **Transaction Types**:
  - One-time purchase
  - Subscription
  - Pay-per-query (federated)
  - Bounty fulfillment
- **Payment Features**:
  - Escrow
  - Multi-party split
  - Automatic royalties
  - Token rewards for negative data
- **Functions**:
  - `purchase_access(origin, listing_id, payment)`
  - `create_escrow(origin, listing_id, amount)`
  - `release_payment(origin, transaction_id)`
  - `refund_payment(origin, transaction_id, reason)`
  - `distribute_rewards(origin, submission_id, recipients)`

##### `pallet-compliance-engine` (New)
Automated compliance checking:
- **Compliance Rules**:
  - GDPR (EU): Right to erasure, consent requirements, data minimization
  - HIPAA (US): De-identification standards, access controls, audit logs
  - PDPA (Singapore): Consent, purpose limitation, cross-border transfer rules
  - Custom institutional policies
- **Functions**:
  - `check_transaction_compliance(origin, transaction_id) -> ComplianceReport`
  - `register_regulation(origin, jurisdiction, rules)`
  - `flag_noncompliance(transaction_id, violation_type)`
  - `generate_compliance_attestation(record_id) -> Attestation`
  - `check_cross_border_transfer(from_jurisdiction, to_jurisdiction, data_type) -> bool`

##### `pallet-reputation`
User reputation and ratings:
- **Reputation Factors**:
  - Data quality scores
  - Negative data contribution
  - Compliance history
  - Peer reviews
  - Citation count (for datasets)
- **Functions**:
  - `rate_data_quality(origin, listing_id, rating, review)`
  - `update_reputation(user_did, reputation_delta)`
  - `get_reputation_score(user_did) -> ReputationScore`
  - `award_badge(origin, user_did, badge_type)`

##### `pallet-analytics`
Usage analytics and metrics:
- **Tracked Metrics**:
  - Access frequency
  - Data utilization patterns
  - Market trends
  - Compliance violations
  - Negative data impact
- **Functions**:
  - `log_usage(origin, listing_id, query_type)`
  - `generate_analytics_report(time_range, filters) -> Report`
  - `get_market_insights() -> MarketInsights`

##### `pallet-federated-ml` (New)
Federated learning and analytics support:
- **Features**:
  - Multi-institution model training coordination
  - On-chain logging of all federated operations
  - Privacy-preserving aggregation
  - Reward distribution for data contributors
- **Functions**:
  - `create_federated_project(origin, institutions, model_spec, privacy_budget)`
  - `submit_local_update(origin, project_id, encrypted_gradient)`
  - `aggregate_updates(origin, project_id) -> GlobalModel`
  - `distribute_federated_rewards(origin, project_id, contribution_scores)`

##### `pallet-consortium` (New)
Multi-party collaboration and data pooling:
- **Consortium Types**:
  - Pre-publication consortia (with embargoes)
  - Industry collaborations
  - Academic networks
  - Regulatory sandboxes
- **Functions**:
  - `create_consortium(origin, members, governance_rules, ip_policy)`
  - `add_data_to_pool(origin, consortium_id, listing_id, access_terms)`
  - `set_embargo(origin, consortium_id, embargo_until, allowed_reviewers)`
  - `grant_reproducibility_access(origin, consortium_id, reviewer, time_window)`

**Storage**:
```rust
// Listings
Listings: map ListingId => ListingMetadata
NegativeDataRegistry: map SubmissionId => NegativeDataMetadata
ListingsByOwner: map AccountId => Vec<ListingId>
ListingsByCategory: map Category => Vec<ListingId>

// Marketplace
Transactions: map TransactionId => Transaction
Escrows: map EscrowId => EscrowDetails
Payments: map PaymentId => PaymentInfo

// Compliance
ComplianceRules: map JurisdictionCode => Vec<ComplianceRule>
ComplianceReports: map TransactionId => ComplianceReport
Violations: map ViolationId => ViolationDetails

// Reputation
UserReputation: map AccountId => ReputationScore
DataQualityRatings: double_map ListingId, AccountId => Rating
Badges: map AccountId => Vec<Badge>

// Federated ML
FederatedProjects: map ProjectId => ProjectMetadata
ModelUpdates: double_map ProjectId, Round => Vec<EncryptedUpdate>
RewardPool: map ProjectId => Balance

// Consortia
Consortia: map ConsortiumId => ConsortiumMetadata
ConsortiumMembers: map ConsortiumId => Vec<AccountId>
PooledData: double_map ConsortiumId, ListingId => AccessTerms
```

**Events**:
```rust
ListingCreated(ListingId, Owner, Price)
ListingFlaggedAsNegative(ListingId, Reason)
NegativeDataSubmitted(SubmissionId, Owner, ProtocolRef)
NegativeDataVerified(SubmissionId, Verifier)
PurchaseCompleted(TransactionId, Buyer, Seller, Amount)
ComplianceCheckPassed(TransactionId)
ComplianceViolation(TransactionId, ViolationType)
ReputationUpdated(AccountId, OldScore, NewScore)
FederatedProjectCreated(ProjectId, Institutions)
ModelUpdateSubmitted(ProjectId, Round, Contributor)
ConsortiumCreated(ConsortiumId, Members)
EmbargoSet(ConsortiumId, EmbargoUntil)
```

## Cross-Chain Communication (XCM)

Patient X leverages Polkadot's XCM (Cross-Consensus Messaging) for seamless inter-chain communication.

### XCM Message Flows

#### 1. Data Access Request with Compliance Check

```
┌──────────────┐         ┌────────────────────┐         ┌─────────────┐         ┌───────────────┐
│  Researcher  │         │  Marketplace Chain │         │  Identity   │         │  HealthData   │
│              │         │                    │         │  Chain      │         │  Chain        │
└──────┬───────┘         └─────────┬──────────┘         └──────┬──────┘         └───────┬───────┘
       │                           │                           │                        │
       │ Purchase Access           │                           │                        │
       ├──────────────────────────>│                           │                        │
       │                           │                           │                        │
       │                           │ XCM: Check Consent        │                        │
       │                           ├──────────────────────────>│                        │
       │                           │                           │                        │
       │                           │ XCM: Check Jurisdiction   │                        │
       │                           ├──────────────────────────>│                        │
       │                           │                           │                        │
       │                           │<──────────────────────────┤                        │
       │                           │ Consent Valid + Compliance│                        │
       │                           │        Check Passed       │                        │
       │                           │                           │                        │
       │                           │ XCM: Request Data Access                           │
       │                           ├───────────────────────────────────────────────────>│
       │                           │                           │                        │
       │                           │                           │ XCM: Verify Consent    │
       │                           │                           │<───────────────────────┤
       │                           │                           │                        │
       │                           │                           ├───────────────────────>│
       │                           │                           │ Consent Confirmed      │
       │                           │                           │                        │
       │                           │                           │        Log Access Event│
       │                           │<──────────────────────────────────────────────────┤│
       │                           │           Encrypted Data Pointer                   ││
       │<──────────────────────────┤                           │                        ││
       │    Access Granted         │                           │                        ││
       │                           │                           │                        ││
       │                           │ Release Payment           │                        ││
       │                           │ (after access verified)   │                        ││
       │                           │                           │                        ││
```

#### 2. Negative Data Submission with Provenance

```
┌──────────────┐         ┌────────────────────┐         ┌─────────────┐         ┌───────────────┐
│ Researcher/  │         │  Marketplace Chain │         │  Identity   │         │  HealthData   │
│ Institution  │         │                    │         │  Chain      │         │  Chain        │
└──────┬───────┘         └─────────┬──────────┘         └──────┬──────┘         └───────┬───────┘
       │                           │                           │                        │
       │ Submit Negative Data      │                           │                        │
       ├──────────────────────────>│                           │                        │
       │                           │                           │                        │
       │                           │ XCM: Verify Protocol Ref  │                        │
       │                           ├──────────────────────────>│                        │
       │                           │                           │                        │
       │                           │<──────────────────────────┤                        │
       │                           │  Original Consent Verified│                        │
       │                           │                           │                        │
       │                           │ XCM: Log Provenance Event                          │
       │                           ├───────────────────────────────────────────────────>│
       │                           │                           │                        │
       │                           │<──────────────────────────────────────────────────┤│
       │                           │          Provenance Hash                           ││
       │<──────────────────────────┤                           │                        ││
       │ Submission Confirmed      │                           │                        ││
       │ + Reward Tokens           │                           │                        ││
       │                           │                           │                        ││
```

#### 3. Consent Update Propagation

```
┌──────────────┐         ┌────────────────────┐         ┌───────────────┐
│   Patient    │         │  IdentityConsent   │         │  HealthData & │
│              │         │  Chain             │         │  Marketplace  │
└──────┬───────┘         └─────────┬──────────┘         └───────┬───────┘
       │                           │                            │
       │ Revoke Consent            │                            │
       ├──────────────────────────>│                            │
       │                           │                            │
       │                           │ Update Consent Status      │
       │                           │ (on-chain)                 │
       │                           │                            │
       │                           │ XCM: Notify Consent Change │
       │                           ├───────────────────────────>│
       │                           │                            │
       │                           │   • HealthData: Revoke     │
       │                           │     Active Access Grants   │
       │                           │   • Marketplace: Update    │
       │                           │     Listing Availability   │
       │                           │                            │
       │<──────────────────────────┤                            │
       │ Confirmation              │                            │
       │                           │                            │
```

#### 4. Federated Learning Coordination

```
┌────────────┐  ┌────────────┐  ┌────────────────────┐  ┌─────────────┐  ┌───────────────┐
│Institution │  │Institution │  │  Marketplace Chain │  │  Identity   │  │  HealthData   │
│     A      │  │     B      │  │                    │  │  Chain      │  │  Chain        │
└─────┬──────┘  └─────┬──────┘  └─────────┬──────────┘  └──────┬──────┘  └───────┬───────┘
      │               │                   │                     │                 │
      │ Create Federated Project          │                     │                 │
      ├──────────────────────────────────>│                     │                 │
      │               │                   │ XCM: Verify All     │                 │
      │               │                   │ Members Consented   │                 │
      │               │                   ├────────────────────>│                 │
      │               │                   │                     │                 │
      │               │                   │<────────────────────┤                 │
      │               │                   │  All Verified       │                 │
      │               │                   │                     │                 │
      │<──────────────┼───────────────────┤                     │                 │
      │ Project Created (ID: Fed-001)     │                     │                 │
      │               │                   │                     │                 │
      │ Submit Local  │                   │                     │                 │
      │ Model Update  │                   │                     │                 │
      ├──────────────────────────────────>│                     │                 │
      │               │                   │ Log Usage           │                 │
      │               │                   ├────────────────────────────────────────>│
      │               │                   │                     │                 │
      │               │ Submit Local      │                     │                 │
      │               │ Model Update      │                     │                 │
      │               ├──────────────────>│                     │                 │
      │               │                   │ Log Usage           │                 │
      │               │                   ├────────────────────────────────────────>│
      │               │                   │                     │                 │
      │               │                   │ Aggregate & Distribute Rewards         │
      │<──────────────┼───────────────────┤                     │                 │
      │ Reward Tokens │                   │                     │                 │
      │               │<──────────────────┤                     │                 │
      │               │ Reward Tokens     │                     │                 │
```

### XCM Configuration

All chains are configured with:
- **XCM Version**: v3
- **Asset Registration**: Native tokens from each chain
- **Trusted Parachains**: Mutual trust between all three Patient X chains
- **Message Filtering**: Type-safe message validation

## Core Features

### 1. Self-Sovereign Identity (SSI)

- **Decentralized Identifiers (DIDs)**: W3C-compliant DIDs for all users
- **Verifiable Credentials**: Institutional affiliations, medical licenses, researcher credentials
- **Multi-factor Authentication**: Support for hardware keys, biometrics
- **Privacy-Preserving**: Selective disclosure of identity attributes

### 2. Granular Consent Management

- **Purpose Specification**: Different consents for different research purposes
- **Temporal Control**: Start dates, end dates, auto-renewal, time-limited access
- **Data Type Selection**: Opt-in/out for specific data categories
- **Dynamic Revocation**: Instant consent withdrawal with blockchain propagation
- **Compensation Preferences**: Specify preferred reward mechanisms

### 3. IPFS-Based Encrypted Storage

- **Content Addressing**: Immutable CIDs for data integrity
- **Encryption at Rest**: ChaCha20-Poly1305 encryption before IPFS upload
- **Distributed Pinning**: Redundant storage across IPFS nodes
- **Format Support**: FHIR, DICOM, HL7, PDF, CSV, custom formats

### 4. Cryptographic Provenance

Every data event is cryptographically logged:
- **Data Acquisition**: Initial upload with source verification
- **Transformations**: Anonymization, aggregation, format conversion
- **Access Events**: Who accessed what, when, and why
- **Derivative Works**: Tracking data reuse in publications or models
- **Blind Signatures**: Privacy-preserving audit trails

### 5. Dynamic Pricing Mechanisms

- **Fixed Pricing**: Set by data owner
- **Auction-Based**: Highest bidder gets access
- **Subscription Model**: Recurring access for institutions
- **Pay-Per-Query**: For federated analytics
- **Reputation-Based Discounts**: High-reputation users get better rates

### 6. Reputation System

Multi-dimensional reputation scoring:
- **Data Quality**: Peer reviews, citation counts, usage frequency
- **Compliance History**: Clean record vs. violations
- **Negative Data Contribution**: Bonus for sharing failed trials
- **Community Participation**: Forum activity, mentorship
- **Badges**: Special recognition (e.g., "Top Negative Data Contributor 2025")

### 7. Audit Trails & Transparency

- **Immutable Logs**: All access events on-chain
- **Real-Time Monitoring**: Dashboard for data owners
- **Export Capabilities**: Generate reports for regulators
- **Breach Notifications**: Automated alerts for unauthorized access attempts

## New Features Roadmap

### Phase 1: Foundation (Q1-Q2 2025)

#### 1.1 Negative/Abandoned Data Registry ✨

**Priority**: CRITICAL (Flagship Feature)

**Implementation Details**:

**Pallet**: `pallet-negative-registry`

**Storage Schema**:
```rust
pub struct NegativeDataSubmission {
    submission_id: SubmissionId,
    submitter: AccountId,
    listing_id: ListingId,
    protocol_reference: ProtocolRef,
    trial_phase: TrialPhase, // Phase 1, 2, 3, 4
    disease_area: DiseaseArea,
    intervention_type: InterventionType, // Drug, Device, Procedure
    discontinuation_reason: DiscontinuationReason,
    endpoints_measured: Vec<Endpoint>,
    sample_size: u32,
    duration: Duration,
    safety_signals: Vec<SafetySignal>,
    efficacy_data: Option<EfficacyData>,
    learnings: Vec<String>,
    context: String,
    provenance_hash: Hash,
    consent_proof: ConsentProof,
    verification_status: VerificationStatus,
    rewards_claimed: bool,
    citations: Vec<PublicationRef>,
    submitted_at: Timestamp,
}

pub enum DiscontinuationReason {
    LackOfEfficacy,
    SafetyConcerns,
    FutilityAnalysis,
    EnrollmentFailure,
    FundingWithdrawn,
    RegulatoryHold,
    CompetitorSuccess,
    Other(String),
}

pub enum VerificationStatus {
    Pending,
    Verified(AccountId, Timestamp), // Verifier, verification timestamp
    Rejected(String), // Reason
}
```

**Functions**:
```rust
// Submit negative data
pub fn submit_negative_data(
    origin: OriginFor<T>,
    listing_id: ListingId,
    protocol_ref: ProtocolRef,
    reason: DiscontinuationReason,
    endpoints: Vec<Endpoint>,
    context: String,
    safety_signals: Vec<SafetySignal>,
) -> DispatchResult;

// Verify submission (for reviewers/institutions)
pub fn verify_submission(
    origin: OriginFor<T>,
    submission_id: SubmissionId,
    verification_signature: Signature,
) -> DispatchResult;

// Search negative data
pub fn search_negative_data(
    disease_area: Option<DiseaseArea>,
    intervention_type: Option<InterventionType>,
    endpoints: Vec<Endpoint>,
) -> Vec<SubmissionId>;

// Claim rewards
pub fn claim_negative_data_reward(
    origin: OriginFor<T>,
    submission_id: SubmissionId,
) -> DispatchResult;

// Update with citations (post-publication)
pub fn add_citation(
    origin: OriginFor<T>,
    submission_id: SubmissionId,
    publication_ref: PublicationRef,
) -> DispatchResult;
```

**Reward Mechanism**:
- **Base Reward**: 1000 tokens per verified submission
- **Quality Bonus**: +500 tokens for comprehensive safety/efficacy data
- **Citation Bonus**: +200 tokens per peer-reviewed publication citing the data
- **Time Bonus**: +300 tokens for submissions within 6 months of discontinuation
- **Rarity Bonus**: +1000 tokens for data in underrepresented disease areas

**Integration Points**:
- XCM query to IdentityConsent Chain to verify original protocol consent
- XCM message to HealthData Chain to log provenance event
- Analytics tracking for impact measurement

#### 1.2 Enhanced Auditability and Provenance

**Pallet**: `pallet-provenance` (in HealthData Chain)

**Implementation**:

```rust
pub struct ProvenanceEvent {
    event_id: EventId,
    record_id: RecordId,
    event_type: EventType,
    actor: AccountId,
    timestamp: Timestamp,
    metadata: BTreeMap<String, String>,
    blind_signature: BlindSignature,
    parent_event: Option<EventId>, // For chaining events
    hash_link: Hash, // Links to previous event for immutability
}

pub enum EventType {
    Acquisition { source: DataSource },
    Transform { operation: TransformOp, parameters: Vec<u8> },
    Export { destination: ExportDest, format: DataFormat },
    Reuse { derivative_work: WorkRef, purpose: Purpose },
    NegativeDataSubmission { submission_id: SubmissionId },
    AccessGrant { requester: AccountId, purpose: Purpose },
    ConsentUpdate { consent_id: ConsentId, change: ConsentChange },
}

pub struct BlindSignature {
    commitment: Hash,
    response: Vec<u8>,
    verifier_pubkey: PublicKey,
}
```

**Provenance API** (for external regulators):
```rust
// REST-like RPC interface
pub trait ProvenanceAPI {
    fn get_full_provenance_chain(record_id: RecordId) -> Vec<ProvenanceEvent>;
    fn verify_event_signature(event_id: EventId) -> bool;
    fn export_audit_trail(record_id: RecordId, format: AuditFormat) -> Vec<u8>;
    fn search_events_by_actor(actor: AccountId) -> Vec<EventId>;
    fn get_compliance_report(record_id: RecordId, jurisdiction: JurisdictionCode) -> ComplianceReport;
}
```

**Audit Triggers**:
```rust
// Real-time compliance monitoring
pub struct AuditTrigger {
    trigger_id: TriggerId,
    condition: TriggerCondition,
    action: TriggerAction,
    enabled: bool,
}

pub enum TriggerCondition {
    CrossBorderAccess { from: JurisdictionCode, to: JurisdictionCode },
    HighValueData { threshold: Balance },
    SensitiveDataType { data_type: DataType },
    UnusualAccessPattern { frequency_threshold: u32 },
    ConsentExpiry { days_before_expiry: u32 },
}

pub enum TriggerAction {
    NotifyRegulator { regulator_id: AccountId },
    RequireManualApproval,
    BlockTransaction,
    LogAlert,
    RequestAdditionalConsent,
}
```

### Phase 2: Compliance & Scale (Q3-Q4 2025)

#### 2.1 Cross-Border and Compliance Middleware

**Pallet**: `pallet-compliance-engine` (in Marketplace Chain)

**Compliance Rule Engine**:

```rust
pub struct ComplianceRule {
    rule_id: RuleId,
    jurisdiction: JurisdictionCode,
    regulation: RegulationType,
    conditions: Vec<Condition>,
    requirements: Vec<Requirement>,
    penalties: Vec<Penalty>,
}

pub enum RegulationType {
    GDPR,
    HIPAA,
    PDPA_Singapore,
    LGPD_Brazil,
    PIPEDA_Canada,
    Custom(String),
}

pub struct Condition {
    field: ConditionField,
    operator: Operator,
    value: Value,
}

pub enum ConditionField {
    DataOwnerResidency,
    DataRequesterResidency,
    DataStorageLocation,
    DataType,
    DataSensitivity,
    TransactionValue,
}

pub struct Requirement {
    requirement_type: RequirementType,
    parameters: BTreeMap<String, String>,
}

pub enum RequirementType {
    ExplicitConsent { granularity: ConsentGranularity },
    DataMinimization { allowed_fields: Vec<String> },
    PurposeLimitation { allowed_purposes: Vec<Purpose> },
    StorageLocalization { required_jurisdiction: JurisdictionCode },
    DataProtectionOfficer { required: bool },
    BreachNotification { max_hours: u32 },
    RightToErasure { supported: bool },
    DataPortability { format: Vec<DataFormat> },
}
```

**Compliance Checking Flow**:

```rust
pub fn check_transaction_compliance(
    transaction_id: TransactionId,
) -> Result<ComplianceReport, ComplianceError> {
    // 1. Get transaction details
    let transaction = Transactions::<T>::get(transaction_id)?;
    
    // 2. Determine applicable jurisdictions
    let buyer_jurisdiction = get_user_jurisdiction(transaction.buyer)?;
    let seller_jurisdiction = get_user_jurisdiction(transaction.seller)?;
    let data_location = get_data_location(transaction.listing_id)?;
    
    // 3. Get applicable regulations
    let regulations = get_applicable_regulations(
        buyer_jurisdiction,
        seller_jurisdiction,
        data_location,
    );
    
    // 4. Check each regulation
    let mut violations = Vec::new();
    for regulation in regulations {
        if let Some(violation) = check_regulation_compliance(transaction, regulation) {
            violations.push(violation);
        }
    }
    
    // 5. Generate compliance report
    Ok(ComplianceReport {
        transaction_id,
        passed: violations.is_empty(),
        violations,
        applicable_regulations: regulations,
        timestamp: current_timestamp(),
    })
}
```

**Data Passport Architecture**:

```rust
pub struct DataPassport {
    passport_id: PassportId,
    data_id: RecordId,
    origin_jurisdiction: JurisdictionCode,
    current_jurisdiction: JurisdictionCode,
    allowed_jurisdictions: Vec<JurisdictionCode>,
    transfer_history: Vec<Transfer>,
    compliance_attestations: Vec<Attestation>,
    on_chain_provenance_hash: Hash,
    off_chain_storage_refs: Vec<StorageRef>,
}

pub struct Transfer {
    from_jurisdiction: JurisdictionCode,
    to_jurisdiction: JurisdictionCode,
    timestamp: Timestamp,
    compliance_check: ComplianceReport,
    authorized_by: AccountId,
}
```

#### 2.2 Open Science & Collaboration Integration

**Pallet**: `pallet-consortium` (in Marketplace Chain)

**Consortium Structure**:

```rust
pub struct Consortium {
    consortium_id: ConsortiumId,
    name: String,
    consortium_type: ConsortiumType,
    members: Vec<Member>,
    governance: GovernanceRules,
    ip_policy: IPPolicy,
    data_pool: Vec<PooledData>,
    embargo: Option<Embargo>,
    created_at: Timestamp,
}

pub enum ConsortiumType {
    PrePublication,
    IndustryCollaboration,
    AcademicNetwork,
    RegulatorySandbox,
    OpenScience,
}

pub struct Member {
    account_id: AccountId,
    institution: Option<String>,
    role: MemberRole,
    voting_power: u32,
    joined_at: Timestamp,
}

pub enum MemberRole {
    PrincipalInvestigator,
    CoInvestigator,
    DataContributor,
    Reviewer,
    Observer,
}

pub struct GovernanceRules {
    decision_threshold: Percentage,
    voting_period: Duration,
    amendment_rules: AmendmentRules,
    dispute_resolution: DisputeResolution,
}

pub struct IPPolicy {
    ownership_model: OwnershipModel,
    publication_rights: PublicationRights,
    commercialization_terms: CommercializationTerms,
    credit_attribution: CreditAttribution,
}

pub enum OwnershipModel {
    SharedOwnership { distribution: BTreeMap<AccountId, Percentage> },
    InstitutionalOwnership { institution: String },
    PublicDomain,
    CreativeCommons { license: CCLicense },
}

pub struct Embargo {
    embargo_id: EmbargoId,
    embargo_until: Timestamp,
    allowed_reviewers: Vec<AccountId>,
    reviewer_access_terms: AccessTerms,
    automatic_release: bool,
}
```

**Multi-Party Access Control**:

```rust
pub fn create_consortium(
    origin: OriginFor<T>,
    name: String,
    consortium_type: ConsortiumType,
    initial_members: Vec<(AccountId, MemberRole)>,
    governance: GovernanceRules,
    ip_policy: IPPolicy,
) -> DispatchResult;

pub fn add_data_to_pool(
    origin: OriginFor<T>,
    consortium_id: ConsortiumId,
    listing_id: ListingId,
    access_terms: AccessTerms,
) -> DispatchResult;

pub fn set_embargo(
    origin: OriginFor<T>,
    consortium_id: ConsortiumId,
    embargo_until: Timestamp,
    allowed_reviewers: Vec<AccountId>,
) -> DispatchResult;

pub fn grant_reproducibility_access(
    origin: OriginFor<T>,
    consortium_id: ConsortiumId,
    reviewer: AccountId,
    time_window: Duration,
    access_level: AccessLevel,
) -> DispatchResult;

pub enum AccessLevel {
    ViewMetadata,
    ViewData,
    RunAnalyses { allowed_scripts: Vec<ScriptHash> },
    FullAccess,
}
```

**Reproducibility Window**:

```rust
pub struct ReproducibilityWindow {
    window_id: WindowId,
    consortium_id: ConsortiumId,
    reviewer: AccountId,
    granted_at: Timestamp,
    expires_at: Timestamp,
    access_level: AccessLevel,
    verified_access: bool,
    access_log: Vec<AccessEvent>,
}

// Cryptographically verified access for peer reviewers
pub fn verify_reproducibility_access(
    reviewer: AccountId,
    data_hash: Hash,
    analysis_script_hash: Hash,
    result_hash: Hash,
) -> Result<VerificationProof, VerificationError>;
```

### Phase 3: Advanced Analytics (Q1-Q2 2026)

#### 3.1 Federated/Aggregated Analytics & ML Support

**Pallet**: `pallet-federated-ml` (in Marketplace Chain)

**Federated Learning Architecture**:

```rust
pub struct FederatedProject {
    project_id: ProjectId,
    coordinator: AccountId,
    participating_institutions: Vec<Institution>,
    model_spec: ModelSpecification,
    privacy_budget: PrivacyBudget,
    current_round: u32,
    status: ProjectStatus,
    reward_pool: Balance,
}

pub struct Institution {
    account_id: AccountId,
    name: String,
    data_size: u32,
    contribution_score: f64,
    local_updates: Vec<LocalUpdate>,
}

pub struct ModelSpecification {
    model_type: ModelType,
    architecture: Vec<u8>, // Serialized model architecture
    hyperparameters: BTreeMap<String, String>,
    aggregation_method: AggregationMethod,
}

pub enum ModelType {
    NeuralNetwork,
    LogisticRegression,
    RandomForest,
    GradientBoosting,
    Custom(String),
}

pub enum AggregationMethod {
    FederatedAveraging,
    SecureAggregation,
    DifferentiallyPrivate { epsilon: f64, delta: f64 },
}

pub struct PrivacyBudget {
    epsilon: f64,
    delta: f64,
    spent: f64,
    remaining: f64,
}

pub struct LocalUpdate {
    round: u32,
    encrypted_gradient: Vec<u8>,
    contribution_proof: ContributionProof,
    privacy_guarantee: PrivacyGuarantee,
    submitted_at: Timestamp,
}
```

**Federated Learning Flow**:

```rust
// 1. Create federated project
pub fn create_federated_project(
    origin: OriginFor<T>,
    institutions: Vec<AccountId>,
    model_spec: ModelSpecification,
    privacy_budget: PrivacyBudget,
    reward_pool: Balance,
) -> DispatchResult;

// 2. Institutions submit local updates
pub fn submit_local_update(
    origin: OriginFor<T>,
    project_id: ProjectId,
    encrypted_gradient: Vec<u8>,
    contribution_proof: ContributionProof,
) -> DispatchResult;

// 3. Coordinator aggregates updates
pub fn aggregate_updates(
    origin: OriginFor<T>,
    project_id: ProjectId,
) -> Result<GlobalModel, AggregationError>;

// 4. Distribute rewards based on contribution
pub fn distribute_federated_rewards(
    origin: OriginFor<T>,
    project_id: ProjectId,
    contribution_scores: BTreeMap<AccountId, f64>,
) -> DispatchResult;
```

**Analytics Contract Templates**:

```rust
pub struct AnalyticsContract {
    contract_id: ContractId,
    template_type: TemplateType,
    parties: Vec<AccountId>,
    data_sources: Vec<ListingId>,
    computation_spec: ComputationSpec,
    result_sharing: ResultSharing,
    usage_limits: UsageLimits,
}

pub enum TemplateType {
    MultiInstitutionTraining,
    CrossBorderAnalytics,
    RareDiseaseConsortium,
    PharmacovigilanceNetwork,
    Custom,
}

pub struct ComputationSpec {
    allowed_operations: Vec<Operation>,
    privacy_constraints: PrivacyConstraints,
    output_format: OutputFormat,
}

pub struct ResultSharing {
    full_results: Vec<AccountId>,
    summary_only: Vec<AccountId>,
    aggregate_only: Vec<AccountId>,
}
```

**Reward Distribution for Valuable/Rare Datasets**:

```rust
pub fn calculate_data_value_score(
    listing_id: ListingId,
    ml_performance_improvement: f64,
    rarity_score: f64,
    negative_data_bonus: f64,
) -> f64 {
    let base_score = ml_performance_improvement * 100.0;
    let rarity_multiplier = 1.0 + (rarity_score * 0.5);
    let negative_bonus = if negative_data_bonus > 0.0 { 1.5 } else { 1.0 };
    
    base_score * rarity_multiplier * negative_bonus
}
```

### Phase 4: Incentives & IP (Q3 2026)

#### 4.1 Patient and Participant Incentives

**Enhanced Reward System**:

```rust
pub struct IncentiveProgram {
    program_id: ProgramId,
    program_type: ProgramType,
    reward_structure: RewardStructure,
    eligibility_criteria: Vec<Criterion>,
    budget: Balance,
    distributed: Balance,
}

pub enum ProgramType {
    NegativeDataBounty,
    ComplianceSandboxParticipation,
    RareDiseaseDataSharing,
    LongitudinalStudyParticipation,
    DataQualityVerification,
}

pub struct RewardStructure {
    base_reward: Balance,
    quality_multiplier: f64,
    time_bonus: TimeBonus,
    rarity_bonus: RarityBonus,
    compliance_bonus: Balance,
}

pub struct TimeBonus {
    enabled: bool,
    decay_rate: f64, // Reward decreases over time
    max_bonus: Balance,
}

pub struct RarityBonus {
    disease_area_multipliers: BTreeMap<DiseaseArea, f64>,
    data_type_multipliers: BTreeMap<DataType, f64>,
}
```

**Pay-for-Performance Model**:

```rust
pub fn create_bounty(
    origin: OriginFor<T>,
    bounty_type: BountyType,
    requirements: Vec<Requirement>,
    reward: Balance,
    deadline: Timestamp,
) -> DispatchResult;

pub enum BountyType {
    NegativeDataRelease { disease_area: DiseaseArea },
    ThirdPartyValidation { dataset_id: ListingId },
    ComplianceSandboxTesting { jurisdiction: JurisdictionCode },
    LongitudinalFollowUp { minimum_duration: Duration },
}

pub fn claim_bounty(
    origin: OriginFor<T>,
    bounty_id: BountyId,
    proof_of_completion: ProofOfCompletion,
) -> DispatchResult;
```

#### 4.2 IP, Licensing, and Versioning Layer

**Enhanced Data Listings**:

```rust
pub struct DataListing {
    listing_id: ListingId,
    owner: AccountId,
    title: String,
    description: String,
    version: Version,
    version_history: Vec<VersionMetadata>,
    ip_status: IPStatus,
    license: License,
    price: PriceStructure,
    quality_metrics: QualityMetrics,
}

pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

pub struct VersionMetadata {
    version: Version,
    ipfs_cid: CID,
    changes: String,
    released_at: Timestamp,
    deprecated: bool,
}

pub struct IPStatus {
    status_type: IPStatusType,
    owner: AccountId,
    co_owners: Vec<(AccountId, Percentage)>,
    patent_refs: Vec<PatentRef>,
    trademark_refs: Vec<TrademarkRef>,
}

pub enum IPStatusType {
    PublicDomain,
    Proprietary,
    SharedOwnership,
    Institutionally Owned { institution: String },
}

pub struct License {
    license_type: LicenseType,
    terms: LicenseTerms,
    restrictions: Vec<Restriction>,
}

pub enum LicenseType {
    CreativeCommons(CCLicense),
    Proprietary,
    Embargoed { release_date: Timestamp },
    Academic { commercial_use: bool },
    Custom(String),
}

pub enum CCLicense {
    BY,      // Attribution
    BY_SA,   // Attribution-ShareAlike
    BY_NC,   // Attribution-NonCommercial
    BY_NC_SA,
    BY_ND,   // Attribution-NoDerivs
    BY_NC_ND,
    CC0,     // Public Domain
}

pub struct LicenseTerms {
    attribution_required: bool,
    commercial_use_allowed: bool,
    derivative_works_allowed: bool,
    share_alike_required: bool,
    embargo_period: Option<Duration>,
    geographic_restrictions: Vec<JurisdictionCode>,
}

pub struct Restriction {
    restriction_type: RestrictionType,
    details: String,
}

pub enum RestrictionType {
    GeographicRestriction { jurisdictions: Vec<JurisdictionCode> },
    UseCase Restriction { disallowed_purposes: Vec<Purpose> },
    InstitutionRestriction { allowed_institutions: Vec<String> },
    TimeRestriction { available_until: Timestamp },
}
```

**Version Control Functions**:

```rust
pub fn create_new_version(
    origin: OriginFor<T>,
    listing_id: ListingId,
    version_type: VersionIncrement,
    new_ipfs_cid: CID,
    changes: String,
) -> DispatchResult;

pub enum VersionIncrement {
    Major, // Breaking changes
    Minor, // New features, backwards compatible
    Patch, // Bug fixes
}

pub fn deprecate_version(
    origin: OriginFor<T>,
    listing_id: ListingId,
    version: Version,
    reason: String,
) -> DispatchResult;

pub fn get_version_history(
    listing_id: ListingId,
) -> Vec<VersionMetadata>;
```

### Phase 5: Privacy & Interoperability (Q4 2026 - Q1 2027)

#### 5.1 Zero-Knowledge and Privacy Enhancements

**Zero-Knowledge Proof Integration**:

```rust
pub struct ZKProof {
    proof_type: ProofType,
    proof_data: Vec<u8>,
    public_inputs: Vec<u8>,
    verification_key: Vec<u8>,
}

pub enum ProofType {
    RangeProof, // Prove value is within range without revealing value
    MembershipProof, // Prove membership in set without revealing which member
    EqualityProof, // Prove two encrypted values are equal
    ComplianceProof, // Prove compliance without revealing data
}

// Example: Prove patient age is over 18 without revealing exact age
pub fn verify_age_compliance(
    encrypted_age: Vec<u8>,
    proof: ZKProof,
) -> Result<bool, VerificationError>;

// Example: Prove dataset meets size requirements without revealing exact count
pub fn verify_sample_size_requirement(
    encrypted_count: Vec<u8>,
    minimum_required: u32,
    proof: ZKProof,
) -> Result<bool, VerificationError>;

// Privacy-preserving query
pub fn query_with_zk_proof(
    origin: OriginFor<T>,
    listing_id: ListingId,
    query: EncryptedQuery,
    proof: ZKProof, // Proves requester has valid consent
) -> Result<EncryptedResult, QueryError>;
```

**Differential Privacy**:

```rust
pub struct DifferentialPrivacyParams {
    epsilon: f64, // Privacy loss budget
    delta: f64,   // Failure probability
    sensitivity: f64,
    noise_mechanism: NoiseMechanism,
}

pub enum NoiseMechanism {
    Laplace,
    Gaussian,
    Exponential,
}

pub fn apply_differential_privacy(
    true_result: f64,
    params: DifferentialPrivacyParams,
) -> f64;

// Track privacy budget consumption
pub fn update_privacy_budget(
    listing_id: ListingId,
    query_cost: f64,
) -> Result<RemainingBudget, BudgetExhaustedError>;
```

#### 5.2 Cross-Chain Bridges

**Ethereum Bridge**:

```rust
pub struct EthereumBridge {
    bridge_contract: H160, // Ethereum contract address
    relay_accounts: Vec<AccountId>,
    pending_transfers: Vec<Transfer>,
}

pub fn bridge_to_ethereum(
    origin: OriginFor<T>,
    ethereum_recipient: H160,
    amount: Balance,
    metadata: Vec<u8>,
) -> DispatchResult;

pub fn bridge_from_ethereum(
    origin: OriginFor<T>,
    ethereum_tx_hash: H256,
    proof: MerkleProof,
) -> DispatchResult;

// Transfer attestations for multi-chain provenance
pub fn export_attestation_to_ethereum(
    record_id: RecordId,
    ethereum_destination: H160,
) -> DispatchResult;
```

**Cosmos IBC Integration**:

```rust
pub fn create_ibc_channel(
    origin: OriginFor<T>,
    cosmos_chain_id: String,
    port_id: String,
) -> DispatchResult;

pub fn send_ibc_packet(
    origin: OriginFor<T>,
    channel_id: ChannelId,
    data: Vec<u8>,
    timeout: Timestamp,
) -> DispatchResult;

// Cross-chain compliance verification
pub fn verify_cross_chain_compliance(
    source_chain: ChainId,
    destination_chain: ChainId,
    data_passport: DataPassport,
) -> Result<bool, ComplianceError>;
```

## Technology Stack

### Core Framework
- **Blockchain Framework**: [Polkadot SDK](https://github.com/paritytech/polkadot-sdk) (forked)
- **Parachain Framework**: Cumulus
- **Consensus**: 
  - Aura (Authority Round) for block production
  - GRANDPA (from relay chain) for finality
- **Runtime**: FRAME (Framework for Runtime Aggregation of Modularized Entities)

### Cryptography
- **Data Encryption**: ChaCha20-Poly1305
- **Key Exchange**: X25519
- **Blind Signatures**: RSA-based blind signatures
- **Zero-Knowledge Proofs**: 
  - zkSNARKs (Groth16) for complex statements
  - Bulletproofs for range proofs
- **Hash Functions**: BLAKE2b

### Storage
- **Off-Chain Data**: IPFS (InterPlanetary File System)
- **On-Chain Storage**: RocksDB (via parity-db)
- **State Management**: Merkle-Patricia Trie

### Messaging
- **Cross-Chain Communication**: XCM (Cross-Consensus Messaging) v3
- **Message Queue**: XCMP (Cross-Chain Message Passing)

### Smart Contracts (Optional)
- **Language**: ink! (Rust-based smart contract language)
- **Use Cases**: Advanced consent logic, custom licensing terms

### Development Tools
- **Build System**: Cargo
- **Testing**: Rust test framework + zombienet for integration testing
- **Benchmarking**: FRAME benchmarking framework
- **Frontend**: Polkadot.js API, React

### External Integrations
- **IPFS**: go-ipfs or Kubo
- **Regulatory APIs**: Custom REST APIs for jurisdiction-specific compliance
- **ML Platforms**: TensorFlow, PyTorch (for federated learning)
- **Analytics**: Apache Superset, Grafana

## Getting Started

### Prerequisites

#### System Requirements
- **OS**: Linux (Ubuntu 22.04+ recommended), macOS, or Windows (WSL2)
- **RAM**: 8GB minimum, 16GB recommended
- **Storage**: 50GB available space
- **CPU**: 4 cores minimum

#### Software Dependencies

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install system dependencies

# macOS
brew install cmake pkg-config openssl git llvm protobuf

# Ubuntu/Debian
sudo apt update
sudo apt install -y \
  build-essential \
  cmake \
  pkg-config \
  libssl-dev \
  git \
  clang \
  libclang-dev \
  protobuf-compiler \
  curl

# Install Polkadot (for relay chain)
cargo install --git https://github.com/paritytech/polkadot --tag v1.0.0 polkadot

# Install zombienet (for local testnet)
wget https://github.com/paritytech/zombienet/releases/latest/download/zombienet-linux-x64
chmod +x zombienet-linux-x64
sudo mv zombienet-linux-x64 /usr/local/bin/zombienet
```

### Quick Start

The fastest way to get Patient X running is using the unified script:

```bash
# Clone the repository
git clone https://github.com/your-org/patient-x.git
cd patient-x

# Run everything (setup, build, test, launch)
./scripts/run-all.sh

# Or skip certain phases
./scripts/run-all.sh --skip-setup --skip-test

# Just check for type errors
./scripts/run-all.sh --check-only

# Clean build
./scripts/run-all.sh --clean
```

#### Available Script Options

```
--skip-setup      Skip environment setup
--skip-build      Skip building chains
--skip-test       Skip running tests
--skip-launch     Skip launching testnet
--check-only      Only run type checking (cargo check)
--clean           Remove build artifacts before building
-h, --help        Show help message
```

### Manual Setup

If you prefer step-by-step control:

#### 1. Environment Setup

```bash
cd patient-x
chmod +x scripts/*.sh
./scripts/setup.sh
```

This script:
- Verifies Rust installation
- Installs WebAssembly target
- Installs system dependencies
- Downloads Polkadot binary
- Installs zombienet
- Creates necessary directories (`data/`, `logs/`)

#### 2. Build All Parachains

```bash
./scripts/build-all.sh
```

Builds:
- IdentityConsent Chain
- HealthData Chain
- Marketplace Chain

Binaries will be in `target/release/`:
- `identity-consent-node`
- `health-data-node`
- `marketplace-node`

#### 3. Run Tests

```bash
# Test all chains
for chain in identity-consent-chain health-data-chain marketplace-chain; do
  cd $chain
  cargo test --workspace --all-features
  cd ..
done

# Test specific pallet
cd identity-consent-chain/pallets/identity-registry
cargo test
```

#### 4. Launch Local Testnet

```bash
./scripts/launch-testnet.sh
```

This uses zombienet to orchestrate:
- 1 relay chain (2 validators)
- 3 parachains (1 collator each)

### Access Endpoints

Once running, you can access:

| Chain                  | WebSocket Endpoint     | Purpose                          |
|------------------------|------------------------|----------------------------------|
| Relay Chain            | `ws://localhost:9944`  | Polkadot relay chain             |
| IdentityConsent Chain  | `ws://localhost:9988`  | Identity & consent management    |
| HealthData Chain       | `ws://localhost:9989`  | Medical records & provenance     |
| Marketplace Chain      | `ws://localhost:9990`  | Data marketplace & transactions  |

### Using Polkadot.js Apps

1. Open [Polkadot.js Apps](https://polkadot.js.org/apps)
2. Click top-left corner → Development → Local Node
3. Change endpoint to one of the above WebSocket URLs
4. Explore chain state, submit extrinsics, etc.

## Development Guide

### Project Structure

```
patient-x/
├── README.md                          # This file
├── Cargo.toml                         # Workspace manifest
├── LICENSE
├── .gitignore
│
├── scripts/                           # Automation scripts
│   ├── run-all.sh                     # Unified runner
│   ├── setup.sh                       # Environment setup
│   ├── build-all.sh                   # Build all chains
│   └── launch-testnet.sh              # Launch testnet
│
├── data/                              # Chain data (gitignored)
│   ├── relay/
│   ├── identity-consent/
│   ├── health-data/
│   └── marketplace/
│
├── logs/                              # Runtime logs (gitignored)
│
├── zombienet/                         # Zombienet configs
│   └── local-testnet.toml
│
├── identity-consent-chain/
│   ├── Cargo.toml
│   ├── node/                          # Node implementation
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── chain_spec.rs
│   │       ├── cli.rs
│   │       ├── command.rs
│   │       ├── rpc.rs
│   │       └── service.rs
│   ├── runtime/                       # Runtime configuration
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── build.rs
│   └── pallets/                       # Custom pallets
│       ├── identity-registry/
│       │   ├── Cargo.toml
│       │   └── src/
│       │       ├── lib.rs
│       │       ├── types.rs
│       │       ├── functions.rs
│       │       ├── tests.rs
│       │       └── benchmarking.rs
│       ├── consent-manager/
│       ├── authentication/
│       └── jurisdiction-manager/
│
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
│
└── marketplace-chain/
    ├── Cargo.toml
    ├── node/
    ├── runtime/
    └── pallets/
        ├── data-listings/
        ├── negative-registry/
        ├── marketplace/
        ├── compliance-engine/
        ├── reputation/
        ├── analytics/
        ├── federated-ml/
        └── consortium/
```

### Building Individual Chains

```bash
# IdentityConsent Chain
cd identity-consent-chain
cargo build --release

# HealthData Chain
cd health-data-chain
cargo build --release

# Marketplace Chain
cd marketplace-chain
cargo build --release
```

### Type Checking (Fast)

```bash
# Check all chains without building
./scripts/run-all.sh --check-only --skip-setup

# Check individual chain
cd identity-consent-chain
cargo check --workspace --all-features
```

### Running Tests

```bash
# Unit tests for specific pallet
cd identity-consent-chain/pallets/identity-registry
cargo test

# Integration tests for chain
cd identity-consent-chain
cargo test --workspace

# With verbose output
cargo test -- --nocapture

# Specific test
cargo test test_register_identity
```

### Benchmarking

```bash
cd identity-consent-chain

# Benchmark specific pallet
cargo build --release --features runtime-benchmarks
./target/release/identity-consent-node benchmark pallet \
  --chain=dev \
  --pallet=pallet_identity_registry \
  --extrinsic='*' \
  --steps=50 \
  --repeat=20 \
  --output=./pallets/identity-registry/src/weights.rs
```

### Adding a New Pallet

1. **Create pallet directory**:
```bash
cd marketplace-chain/pallets
cargo new my-pallet --lib
```

2. **Update `Cargo.toml`**:
```toml
[package]
name = "pallet-my-pallet"
version = "0.1.0"
edition = "2021"

[dependencies]
codec = { package = "parity-scale-codec", version = "3.0.0", default-features = false }
scale-info = { version = "2.0.0", default-features = false }
frame-support = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master" }
frame-system = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master" }
sp-std = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", branch = "master" }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
]
```

3. **Implement pallet logic** in `src/lib.rs`

4. **Add to runtime** (`marketplace-chain/runtime/src/lib.rs`):
```rust
impl pallet_my_pallet::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    // Add other associated types
}

construct_runtime!(
    pub enum Runtime {
        // ... existing pallets
        MyPallet: pallet_my_pallet,
    }
);
```

5. **Add to workspace** (`Cargo.toml` at repo root):
```toml
[workspace]
members = [
    # ... existing members
    "marketplace-chain/pallets/my-pallet",
]
```

### Debugging

#### Enable Detailed Logs

```bash
# Run with debug logs
RUST_LOG=debug ./target/release/marketplace-node \
  --dev \
  --tmp \
  --rpc-port 9933 \
  --port 30333
```

#### Use Polkadot.js for Debugging

1. Navigate to **Developer** → **Extrinsics**
2. Select pallet and function
3. Fill in parameters
4. Submit transaction
5. Check **Network** → **Explorer** for events

#### Use `log` macros in pallets

```rust
use frame_support::log::{info, debug, warn, error};

#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn my_function(origin: OriginFor<T>) -> DispatchResult {
        let who = ensure_signed(origin)?;
        
        info!("Function called by {:?}", who);
        debug!("Detailed debug info");
        
        Ok(())
    }
}
```

### XCM Testing

Testing cross-chain messages locally:

```bash
# Terminal 1: Relay chain
polkadot --chain=rococo-local --alice --tmp

# Terminal 2: IdentityConsent parachain
./target/release/identity-consent-node \
  --alice \
  --collator \
  --force-authoring \
  --chain identity-consent-local \
  --tmp \
  --port 40333 \
  --rpc-port 9988

# Terminal 3: Marketplace parachain
./target/release/marketplace-node \
  --alice \
  --collator \
  --force-authoring \
  --chain marketplace-local \
  --tmp \
  --port 40334 \
  --rpc-port 9990
```

Send XCM message from Marketplace to IdentityConsent:

```rust
// In pallet-marketplace
use xcm::prelude::*;

let message = Xcm(vec![
    UnpaidExecution { weight_limit: Unlimited, check_origin: None },
    Transact {
        origin_kind: OriginKind::SovereignAccount,
        require_weight_at_most: Weight::from_parts(1_000_000_000, 0),
        call: <IdentityConsentChain as Config>::RuntimeCall::from(
            pallet_consent_manager::Call::check_consent { consent_id }
        ).encode().into(),
    },
]);

T::XcmSender::send_xcm(
    MultiLocation::new(1, X1(Parachain(2000))),
    message,
)?;
```

## Security & Compliance

### Security Best Practices

#### 1. Data Encryption

All medical data MUST be encrypted before IPFS upload:

```rust
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};

pub fn encrypt_medical_data(
    plaintext: &[u8],
    key: &Key,
    nonce: &Nonce,
) -> Result<Vec<u8>, EncryptionError> {
    let cipher = ChaCha20Poly1305::new(key);
    cipher.encrypt(nonce, plaintext)
        .map_err(|_| EncryptionError::EncryptionFailed)
}
```

#### 2. Key Management

- Keys are generated per-record
- Keys are shared using X25519 key exchange
- Key rotation supported for long-lived records
- Keys never leave the blockchain unencrypted

#### 3. Access Control

Multi-layer verification:
1. **On-chain consent check** (IdentityConsent Chain)
2. **XCM verification** (double-check via cross-chain message)
3. **Audit logging** (all access attempts logged)

#### 4. Audit Trail

Every sensitive operation triggers an event:
```rust
Self::deposit_event(Event::DataAccessed {
    record_id,
    accessor: who,
    purpose,
    timestamp: <timestamp::Pallet<T>>::get(),
});
```

#### 5. Rate Limiting

Prevent abuse with on-chain rate limits:
```rust
let access_count = <AccessCount<T>>::get(&who, &record_id);
ensure!(access_count < T::MaxAccessPerDay::get(), Error::<T>::RateLimitExceeded);
<AccessCount<T>>::mutate(&who, &record_id, |count| *count += 1);
```

### Compliance Features

#### HIPAA Compliance

| Requirement                     | Implementation                                     |
|---------------------------------|----------------------------------------------------|
| Access Controls                 | Role-based permissions + consent verification     |
| Audit Controls                  | Immutable on-chain audit trail                    |
| Integrity Controls              | Cryptographic hashes for all data                 |
| Transmission Security           | End-to-end encryption (ChaCha20-Poly1305)         |
| Breach Notification             | Automated on-chain events for unauthorized access |

#### GDPR Compliance

| Right                           | Implementation                                     |
|---------------------------------|----------------------------------------------------|
| Right to Access                 | Patients can query their data via Polkadot.js     |
| Right to Rectification          | Update functions for record metadata             |
| Right to Erasure                | Remove IPFS CID + unpin from IPFS                 |
| Right to Data Portability       | Export in FHIR, DICOM, CSV formats               |
| Right to Object                 | Consent revocation with immediate effect          |
| Data Minimization               | Only necessary data fields stored on-chain        |

#### Cross-Border Data Transfer

Automated compliance checking before any cross-border transaction:

```rust
pub fn check_cross_border_compliance(
    from_jurisdiction: JurisdictionCode,
    to_jurisdiction: JurisdictionCode,
    data_type: DataType,
) -> Result<bool, ComplianceError> {
    // Check if transfer is allowed
    let rules = <CrossBorderRules<T>>::get(from_jurisdiction, to_jurisdiction);
    
    for rule in rules {
        if !rule.allows_transfer(data_type) {
            return Err(ComplianceError::TransferNotAllowed);
        }
    }
    
    // Log compliance check
    Self::deposit_event(Event::ComplianceCheckPassed {
        from: from_jurisdiction,
        to: to_jurisdiction,
        data_type,
        timestamp: <timestamp::Pallet<T>>::get(),
    });
    
    Ok(true)
}
```

### Penetration Testing

Regular security audits should include:
- Smart contract audits for custom pallets
- XCM message validation
- Cryptographic implementation review
- Access control bypass attempts
- DoS resistance testing

### Incident Response

In case of security incident:
1. **Immediate**: Pause affected pallets via governance
2. **Investigation**: Export full audit trail for forensics
3. **Notification**: Automated breach notifications via on-chain events
4. **Remediation**: Runtime upgrade to patch vulnerability
5. **Post-mortem**: Publish findings (privacy-preserving)

## Roadmap

### 2025

**Q1**
- ✅ Fork Polkadot SDK
- ✅ Implement basic three-chain architecture
- ⏳ Deploy Negative Data Registry
- ⏳ Implement Enhanced Provenance with blind signatures

**Q2**
- Compliance Engine development
- Data Passport architecture
- Open Science consortium module alpha

**Q3**
- Federated ML infrastructure
- Compliance middleware production release
- Cross-border testing with EU/US/Asia pilot partners

**Q4**
- Patient incentive programs launch
- IP/licensing layer enhancement
- Year-end security audit

### 2026

**Q1**
- Zero-knowledge proof integration
- Advanced federated analytics
- Reputation system v2

**Q2**
- Ethereum bridge deployment
- Publisher integrations (Nature, NEJM)
- Regulatory sandbox partnerships

**Q3**
- Cosmos IBC integration
- Multi-chain provenance
- Advanced privacy features (differential privacy)

**Q4**
- Mainnet launch preparation
- Parachain auction participation
- Community governance activation

### 2027+

- AI/ML model marketplace
- Decentralized clinical trial coordination
- Global regulatory harmonization efforts
- Real-world evidence (RWE) integration

## Contributing

We welcome contributions from the community! Here's how you can help:

### Types of Contributions

- **Bug Reports**: Open an issue with detailed reproduction steps
- **Feature Requests**: Propose new features via GitHub Discussions
- **Code Contributions**: Submit PRs for bug fixes or new features
- **Documentation**: Improve README, add examples, write tutorials
- **Testing**: Help test on different platforms, write integration tests

### Development Workflow

1. **Fork the repository**
2. **Create a feature branch**:
   ```bash
   git checkout -b feature/my-awesome-feature
   ```
3. **Make your changes**
4. **Run tests**:
   ```bash
   cargo test --workspace
   ```
5. **Run formatter**:
   ```bash
   cargo fmt --all
   ```
6. **Run clippy**:
   ```bash
   cargo clippy --workspace --all-features -- -D warnings
   ```
7. **Commit with clear messages**:
   ```bash
   git commit -m "feat: add negative data search by disease area"
   ```
8. **Push and open PR**:
   ```bash
   git push origin feature/my-awesome-feature
   ```

### Code Style

- Follow Rust standard style (enforced by `rustfmt`)
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Write unit tests for new functions
- Update integration tests for new features

### Review Process

1. Automated checks (CI/CD) must pass
2. At least one maintainer approval required
3. All comments must be resolved
4. Merge via squash commit

## License

This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for details.

## Acknowledgments

- **Polkadot/Substrate Team**: For the incredible blockchain framework
- **Parity Technologies**: For ongoing support and tooling
- **Web3 Foundation**: For grants and ecosystem support
- **Medical Research Community**: For insights into clinical data challenges
- **Open Science Advocates**: For inspiring the negative data registry concept

## Contact & Community

- **Website**: https://patientx.network (coming soon)
- **GitHub**: https://github.com/your-org/patient-x
- **Discord**: https://discord.gg/patientx
- **Twitter**: @PatientXNetwork
- **Email**: contact@patientx.network

## FAQ

**Q: Why Polkadot instead of Ethereum?**  
A: Polkadot's parachain architecture allows us to build specialized chains for identity, data, and marketplace functions while maintaining shared security. XCM enables seamless cross-chain communication without expensive bridging.

**Q: How is patient privacy ensured?**  
A: Multi-layered approach: (1) End-to-end encryption before IPFS storage, (2) Zero-knowledge proofs for sensitive queries, (3) Granular consent management, (4) Decentralized key management, (5) Immutable audit trails.

**Q: What makes negative data valuable?**  
A: Failed trials contain critical information that prevents redundant research, identifies safety signals, and improves future study design. Our token incentives make sharing profitable.

**Q: How does cross-border compliance work?**  
A: Our compliance engine automatically checks jurisdiction rules before any data transfer, generates attestations, and enforces data residency requirements via the Data Passport architecture.

**Q: Can I use Patient X for commercial research?**  
A: Yes! Dataset owners can set licensing terms (including commercial use) and prices. Our marketplace supports various licensing models from open science to proprietary.

**Q: How are rewards distributed in federated learning?**  
A: Contribution scores are calculated based on data size, quality, and impact on model performance. Rewards are distributed automatically via smart contracts.

**Q: Is Patient X production-ready?**  
A: Not yet. We're currently in active development (Q1 2025). Target mainnet launch is Q4 2026 after extensive testing and security audits.

---

**Built with ❤️ by the Patient X team and the Web3 medical research community.**
