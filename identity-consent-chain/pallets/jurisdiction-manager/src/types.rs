use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::*, BoundedVec};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Type alias for jurisdiction code (ISO 3166-1 alpha-2 country codes)
pub type JurisdictionCode = BoundedVec<u8, ConstU32<2>>;

/// Maximum length for regulation name
pub const MAX_REGULATION_NAME_LENGTH: u32 = 32;

/// Maximum length for jurisdiction metadata
pub const MAX_JURISDICTION_METADATA_LENGTH: u32 = 256;

/// Maximum number of regulations per jurisdiction
pub const MAX_REGULATIONS_PER_JURISDICTION: u32 = 10;

/// Regulatory framework types
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum RegulationType {
	/// EU General Data Protection Regulation
	GDPR,
	/// US Health Insurance Portability and Accountability Act
	HIPAA,
	/// Singapore Personal Data Protection Act
	PDPA,
	/// California Consumer Privacy Act
	CCPA,
	/// UK Data Protection Act
	UKDPA,
	/// Brazil General Data Protection Law
	LGPD,
	/// Japan Act on the Protection of Personal Information
	APPI,
	/// South Korea Personal Information Protection Act
	PIPA,
	/// Australia Privacy Act
	PrivacyAct,
	/// Canada Personal Information Protection and Electronic Documents Act
	PIPEDA,
	/// Custom regulation
	Custom(BoundedVec<u8, ConstU32<MAX_REGULATION_NAME_LENGTH>>),
}

/// Data residency requirement
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum DataResidencyRequirement {
	/// Data must remain within the jurisdiction
	StrictLocal,
	/// Data can be stored in approved jurisdictions
	ApprovedJurisdictions,
	/// No residency restriction
	NoRestriction,
}

/// Cross-border transfer restriction
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum TransferRestriction {
	/// Transfers prohibited without explicit consent
	ConsentRequired,
	/// Transfers allowed to adequate protection jurisdictions
	AdequacyRequired,
	/// Standard contractual clauses required
	SCCRequired,
	/// Binding corporate rules required
	BCRRequired,
	/// No restriction
	NoRestriction,
}

/// Compliance requirements for a specific regulation
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct ComplianceRule<T: crate::Config> {
	/// The regulation type
	pub regulation: RegulationType,
	/// Data residency requirement
	pub data_residency: DataResidencyRequirement,
	/// Cross-border transfer restriction
	pub transfer_restriction: TransferRestriction,
	/// Minimum consent age (in years)
	pub min_consent_age: u8,
	/// Whether explicit consent is required
	pub explicit_consent_required: bool,
	/// Whether right to erasure is supported
	pub right_to_erasure: bool,
	/// Whether data portability is required
	pub data_portability_required: bool,
	/// Whether data breach notification is mandatory
	pub breach_notification_required: bool,
	/// Maximum days to respond to subject access requests
	pub max_sar_response_days: u8,
	/// When this rule was created/updated
	pub updated_at: BlockNumberFor<T>,
}

/// Jurisdiction information
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct JurisdictionInfo<T: crate::Config> {
	/// Jurisdiction code (ISO country code)
	pub code: JurisdictionCode,
	/// Active regulations in this jurisdiction
	pub regulations: BoundedVec<RegulationType, ConstU32<MAX_REGULATIONS_PER_JURISDICTION>>,
	/// Approved jurisdictions for data transfer
	pub approved_transfer_targets: BoundedVec<JurisdictionCode, T::MaxApprovedJurisdictions>,
	/// Is this jurisdiction active?
	pub is_active: bool,
	/// Optional metadata (JSON or other format)
	pub metadata: Option<BoundedVec<u8, ConstU32<MAX_JURISDICTION_METADATA_LENGTH>>>,
	/// When this jurisdiction was registered
	pub registered_at: BlockNumberFor<T>,
	/// Last update
	pub updated_at: BlockNumberFor<T>,
}

/// Transfer validation result
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum TransferValidationResult {
	/// Transfer is allowed
	Allowed,
	/// Transfer requires consent
	ConsentRequired,
	/// Transfer requires adequacy decision
	AdequacyRequired,
	/// Transfer requires standard contractual clauses
	SCCRequired,
	/// Transfer is prohibited
	Prohibited,
}

/// Convert u8 to RegulationType
impl From<u8> for RegulationType {
	fn from(value: u8) -> Self {
		match value {
			0 => RegulationType::GDPR,
			1 => RegulationType::HIPAA,
			2 => RegulationType::PDPA,
			3 => RegulationType::CCPA,
			4 => RegulationType::UKDPA,
			5 => RegulationType::LGPD,
			6 => RegulationType::APPI,
			7 => RegulationType::PIPA,
			8 => RegulationType::PrivacyAct,
			9 => RegulationType::PIPEDA,
			_ => RegulationType::GDPR, // Default
		}
	}
}

/// Convert RegulationType to u8
impl From<RegulationType> for u8 {
	fn from(regulation: RegulationType) -> Self {
		match regulation {
			RegulationType::GDPR => 0,
			RegulationType::HIPAA => 1,
			RegulationType::PDPA => 2,
			RegulationType::CCPA => 3,
			RegulationType::UKDPA => 4,
			RegulationType::LGPD => 5,
			RegulationType::APPI => 6,
			RegulationType::PIPA => 7,
			RegulationType::PrivacyAct => 8,
			RegulationType::PIPEDA => 9,
			RegulationType::Custom(_) => 10,
		}
	}
}

/// Convert u8 to DataResidencyRequirement
impl From<u8> for DataResidencyRequirement {
	fn from(value: u8) -> Self {
		match value {
			0 => DataResidencyRequirement::StrictLocal,
			1 => DataResidencyRequirement::ApprovedJurisdictions,
			_ => DataResidencyRequirement::NoRestriction,
		}
	}
}

/// Convert u8 to TransferRestriction
impl From<u8> for TransferRestriction {
	fn from(value: u8) -> Self {
		match value {
			0 => TransferRestriction::ConsentRequired,
			1 => TransferRestriction::AdequacyRequired,
			2 => TransferRestriction::SCCRequired,
			3 => TransferRestriction::BCRRequired,
			_ => TransferRestriction::NoRestriction,
		}
	}
}
