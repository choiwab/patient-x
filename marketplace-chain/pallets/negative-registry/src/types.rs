use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::BoundedVec;

// Constants for bounded vectors
pub const MAX_OUTCOME_DESCRIPTION_LENGTH: u32 = 2048;
pub const MAX_EVIDENCE_LENGTH: u32 = 512;
pub const MAX_EVIDENCE_PER_OUTCOME: u32 = 10;
pub const MAX_VERIFIERS_PER_OUTCOME: u32 = 5;
pub const MAX_OUTCOMES_PER_REPORTER: u32 = 1000;
pub const MAX_VERIFICATION_NOTES_LENGTH: u32 = 512;
pub const MAX_METADATA_LENGTH: u32 = 256;
pub const MAX_PRODUCT_NAME_LENGTH: u32 = 128;

/// Unique identifier for a negative outcome report
pub type OutcomeId = [u8; 32];

/// Evidence hash or IPFS CID
pub type EvidenceHash = BoundedVec<u8, ConstU32<MAX_EVIDENCE_LENGTH>>;

/// Description text
pub type Description = BoundedVec<u8, ConstU32<MAX_OUTCOME_DESCRIPTION_LENGTH>>;

/// Product/drug name
pub type ProductName = BoundedVec<u8, ConstU32<MAX_PRODUCT_NAME_LENGTH>>;

/// Verification notes
pub type VerificationNotes = BoundedVec<u8, ConstU32<MAX_VERIFICATION_NOTES_LENGTH>>;

/// Metadata
pub type Metadata = BoundedVec<u8, ConstU32<MAX_METADATA_LENGTH>>;

/// Type of negative outcome
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum OutcomeType {
	/// Adverse drug reaction
	AdverseDrugReaction,
	/// Treatment failure
	TreatmentFailure,
	/// Medication side effect
	SideEffect,
	/// Disease progression despite treatment
	DiseaseProgression,
	/// Medical device failure
	DeviceFailure,
	/// Surgical complication
	SurgicalComplication,
	/// Diagnostic error
	DiagnosticError,
	/// Contraindication violation
	ContraindicationViolation,
	/// Drug interaction
	DrugInteraction,
	/// Allergy reaction
	AllergyReaction,
	/// Other negative outcome
	Other,
}

/// Severity level of the outcome
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
pub enum SeverityLevel {
	#[default]
	/// Mild - minor inconvenience
	Mild,
	/// Moderate - significant discomfort
	Moderate,
	/// Severe - major health impact
	Severe,
	/// Critical - life-threatening
	Critical,
	/// Fatal - resulted in death
	Fatal,
}

/// Verification status of the outcome
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum VerificationStatus {
	/// Pending verification
	Pending,
	/// Under review by verifiers
	UnderReview,
	/// Verified by required parties
	Verified,
	/// Rejected as invalid
	Rejected,
	/// Disputed (conflicting verifications)
	Disputed,
}

/// Role of the verifier
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum VerifierRole {
	/// Healthcare provider who treated the patient
	HealthcareProvider,
	/// Independent medical expert
	MedicalExpert,
	/// Regulatory authority
	RegulatoryAuthority,
	/// Pharmaceutical company
	PharmaceuticalCompany,
	/// Patient advocate organization
	PatientAdvocate,
	/// Third-party auditor
	ThirdPartyAuditor,
}

/// Verification decision
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum VerificationDecision {
	/// Confirms the outcome report
	Confirmed,
	/// Disputes the outcome report
	Disputed,
	/// Needs more information
	NeedsMoreInfo,
	/// Cannot verify
	CannotVerify,
}

/// Reward tier based on outcome value
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum RewardTier {
	/// Basic tier - common outcomes
	Basic,
	/// Intermediate tier - moderately rare
	Intermediate,
	/// Advanced tier - rare and valuable
	Advanced,
	/// Premium tier - critically important/novel
	Premium,
}

/// A verification entry from a verifier
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Verification<T: crate::Config> {
	/// Verifier account
	pub verifier: T::AccountId,

	/// Role of the verifier
	pub role: VerifierRole,

	/// Verification decision
	pub decision: VerificationDecision,

	/// Verification notes/comments
	pub notes: Option<VerificationNotes>,

	/// When verification was submitted
	pub verified_at: BlockNumberFor<T>,
}

/// A negative health outcome report
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct NegativeOutcome<T: crate::Config> {
	/// Unique outcome identifier
	pub outcome_id: OutcomeId,

	/// Reporter (patient or healthcare provider)
	pub reporter: T::AccountId,

	/// Type of negative outcome
	pub outcome_type: OutcomeType,

	/// Severity level
	pub severity: SeverityLevel,

	/// Product/drug involved (optional)
	pub product_name: Option<ProductName>,

	/// Detailed description
	pub description: Description,

	/// Evidence hash (IPFS CID or similar)
	pub evidence: BoundedVec<EvidenceHash, ConstU32<MAX_EVIDENCE_PER_OUTCOME>>,

	/// Verification status
	pub verification_status: VerificationStatus,

	/// Verifications received
	pub verifications: BoundedVec<Verification<T>, ConstU32<MAX_VERIFIERS_PER_OUTCOME>>,

	/// Required number of verifications
	pub required_verifications: u8,

	/// Reward tier
	pub reward_tier: RewardTier,

	/// Reward amount (in native token)
	pub reward_amount: u128,

	/// Whether reward has been claimed
	pub reward_claimed: bool,

	/// Optional link to health record
	pub record_id: Option<[u8; 32]>,

	/// Optional link to data listing
	pub listing_id: Option<[u8; 32]>,

	/// Additional metadata
	pub metadata: Option<Metadata>,

	/// When outcome was reported
	pub reported_at: BlockNumberFor<T>,

	/// When verification was completed (if verified)
	pub verified_at: Option<BlockNumberFor<T>>,
}

impl<T: crate::Config> NegativeOutcome<T> {
	/// Check if outcome is verified
	pub fn is_verified(&self) -> bool {
		self.verification_status == VerificationStatus::Verified
	}

	/// Check if outcome has enough verifications
	pub fn has_enough_verifications(&self) -> bool {
		let confirmed_count = self
			.verifications
			.iter()
			.filter(|v| v.decision == VerificationDecision::Confirmed)
			.count();

		confirmed_count >= self.required_verifications as usize
	}

	/// Check if outcome is disputed
	pub fn is_disputed(&self) -> bool {
		let disputed_count = self
			.verifications
			.iter()
			.filter(|v| v.decision == VerificationDecision::Disputed)
			.count();

		// If any verifier disputes, mark as disputed
		disputed_count > 0
	}

	/// Get confirmed verification count
	pub fn confirmed_verification_count(&self) -> u8 {
		self.verifications
			.iter()
			.filter(|v| v.decision == VerificationDecision::Confirmed)
			.count() as u8
	}

	/// Check if reward can be claimed
	pub fn can_claim_reward(&self) -> bool {
		self.is_verified() && !self.reward_claimed && self.reward_amount > 0
	}
}

/// Reward pool configuration
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
pub struct RewardPoolConfig {
	/// Total pool balance
	pub total_balance: u128,

	/// Reserved (committed but not claimed)
	pub reserved_balance: u128,

	/// Reward amounts by tier
	pub tier_rewards: TierRewards,

	/// Minimum severity for rewards
	pub min_severity: SeverityLevel,
}

/// Reward amounts for each tier
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct TierRewards {
	/// Basic tier reward
	pub basic: u128,

	/// Intermediate tier reward
	pub intermediate: u128,

	/// Advanced tier reward
	pub advanced: u128,

	/// Premium tier reward
	pub premium: u128,
}

impl Default for TierRewards {
	fn default() -> Self {
		Self {
			basic: 100_000_000_000_000,       // 0.0001 token
			intermediate: 1_000_000_000_000_000, // 0.001 token
			advanced: 10_000_000_000_000_000,    // 0.01 token
			premium: 100_000_000_000_000_000,   // 0.1 token
		}
	}
}

impl TierRewards {
	/// Get reward amount for a tier
	pub fn get_reward(&self, tier: &RewardTier) -> u128 {
		match tier {
			RewardTier::Basic => self.basic,
			RewardTier::Intermediate => self.intermediate,
			RewardTier::Advanced => self.advanced,
			RewardTier::Premium => self.premium,
		}
	}
}

/// Statistics for the negative registry
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
pub struct RegistryStats {
	/// Total outcomes reported
	pub total_outcomes: u64,

	/// Total verified outcomes
	pub verified_outcomes: u64,

	/// Total rejected outcomes
	pub rejected_outcomes: u64,

	/// Total rewards distributed
	pub total_rewards_distributed: u128,

	/// Outcomes by severity (Mild, Moderate, Severe, Critical, Fatal)
	pub outcomes_by_severity: [u64; 5],

	/// Outcomes by type (11 types)
	pub outcomes_by_type: [u64; 11],
}
