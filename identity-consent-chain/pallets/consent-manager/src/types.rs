use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{BoundedVec, pallet_prelude::*};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Maximum length for custom purpose description
pub const MAX_PURPOSE_LENGTH: u32 = 128;

/// Maximum length for custom data type description
pub const MAX_DATA_TYPE_LENGTH: u32 = 64;

/// Type alias for consent identifier
pub type ConsentId = [u8; 32];

/// Type alias for jurisdiction code (ISO country codes)
pub type JurisdictionCode = BoundedVec<u8, ConstU32<8>>;

/// Consent purpose enumeration
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Purpose {
	/// General research purposes
	ResearchGeneral,
	/// Research for a specific study (with study identifier)
	ResearchSpecificStudy(BoundedVec<u8, ConstU32<64>>),
	/// Commercial use
	Commercial,
	/// Public health purposes
	PublicHealth,
	/// Custom purpose with description
	Custom(BoundedVec<u8, ConstU32<MAX_PURPOSE_LENGTH>>),
}

/// Duration configuration for consent
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Duration<T: crate::Config> {
	/// Start time in block number
	pub start: BlockNumberFor<T>,
	/// Optional end time in block number
	pub end: Option<BlockNumberFor<T>>,
	/// Whether consent automatically renews
	pub auto_renewal: bool,
}

/// Types of medical data
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum DataType {
	/// Demographic information
	Demographics,
	/// Diagnostic data
	Diagnostics,
	/// Genomic data
	Genomics,
	/// Medical imaging
	Imaging,
	/// Laboratory results
	LabResults,
	/// Medication records
	Medications,
	/// Medical procedures
	Procedures,
	/// Vital signs
	Vitals,
	/// Custom data type
	Custom(BoundedVec<u8, ConstU32<MAX_DATA_TYPE_LENGTH>>),
}

/// Allowed parties configuration
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub enum AllowedParties<T: crate::Config> {
	/// Specific list of accounts
	Specific(BoundedVec<T::AccountId, T::MaxAllowedParties>),
	/// Categories of users (by UserType from identity-registry)
	Categories(BoundedVec<u8, ConstU32<10>>), // Using u8 for UserType
	/// Public access
	Public,
}

/// Compensation preference
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum CompensationPreference {
	/// Free access (no compensation required)
	Free,
	/// Fixed price per access
	FixedPrice(u128),
	/// Percentage of sale price
	Percentage(u8), // 0-100
	/// Negotiable case-by-case
	Negotiable,
}

/// Complete consent policy
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct ConsentPolicy<T: crate::Config> {
	/// Unique consent identifier
	pub consent_id: ConsentId,
	/// Owner of the data
	pub data_owner: T::AccountId,
	/// Purpose of consent
	pub purpose: Purpose,
	/// Duration configuration
	pub duration: Duration<T>,
	/// Types of data covered
	pub data_types: BoundedVec<DataType, T::MaxDataTypes>,
	/// Who can access the data
	pub allowed_parties: AllowedParties<T>,
	/// Allowed jurisdictions
	pub jurisdictions: BoundedVec<JurisdictionCode, T::MaxJurisdictions>,
	/// Compensation preference
	pub compensation_preference: CompensationPreference,
	/// Creation timestamp
	pub created_at: BlockNumberFor<T>,
	/// Last update timestamp
	pub updated_at: BlockNumberFor<T>,
}

/// Consent status tracking
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub enum ConsentStatus<T: crate::Config> {
	/// Consent is active
	Active,
	/// Consent was revoked
	Revoked {
		/// When it was revoked
		revoked_at: BlockNumberFor<T>,
		/// Optional reason for revocation
		reason: Option<BoundedVec<u8, ConstU32<256>>>,
	},
	/// Consent expired
	Expired {
		/// When it expired
		expired_at: BlockNumberFor<T>,
	},
}

impl<T: crate::Config> ConsentStatus<T> {
	/// Check if consent is currently active
	pub fn is_active(&self) -> bool {
		matches!(self, ConsentStatus::Active)
	}
}

impl<T: crate::Config> Default for ConsentStatus<T> {
	fn default() -> Self {
		ConsentStatus::Active
	}
}
