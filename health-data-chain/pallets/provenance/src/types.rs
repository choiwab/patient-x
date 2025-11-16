use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::*, BoundedVec};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use pallet_health_records::RecordId;

/// Type alias for lineage ID
pub type LineageId = [u8; 32];

/// Type alias for transformation ID
pub type TransformationId = [u8; 32];

/// Maximum description length
pub const MAX_DESCRIPTION_LENGTH: u32 = 256;

/// Maximum number of parent records
pub const MAX_PARENT_RECORDS: u32 = 10;

/// Maximum number of transformation steps
pub const MAX_TRANSFORMATION_STEPS: u32 = 20;

/// Transformation operation types
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum TransformationType {
	/// Data aggregation
	Aggregation,
	/// Data anonymization
	Anonymization,
	/// Data filtering
	Filtering,
	/// Data format conversion
	Conversion,
	/// Statistical analysis
	Analysis,
	/// Machine learning inference
	MLInference,
	/// Data merge
	Merge,
	/// Data split
	Split,
	/// Custom transformation
	Custom(BoundedVec<u8, ConstU32<32>>),
}

/// Agent type that performed an action
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum AgentType {
	/// Human user
	Human,
	/// Automated system
	System,
	/// Smart contract
	Contract,
	/// External service
	ExternalService,
}

/// Activity type for usage tracking
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum ActivityType {
	/// Data was accessed/read
	Access,
	/// Data was modified
	Modification,
	/// Data was derived/transformed
	Derivation,
	/// Data was shared
	Sharing,
	/// Data was exported
	Export,
	/// Data was deleted
	Deletion,
}

/// Data lineage entry
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct LineageEntry<T: crate::Config> {
	/// Lineage identifier
	pub lineage_id: LineageId,
	/// Child record (derived data)
	pub child_record: RecordId,
	/// Parent records (source data)
	pub parent_records: BoundedVec<RecordId, ConstU32<MAX_PARENT_RECORDS>>,
	/// Transformation applied
	pub transformation_type: TransformationType,
	/// Agent who performed the derivation
	pub agent: T::AccountId,
	/// Agent type
	pub agent_type: AgentType,
	/// Description of derivation
	pub description: Option<BoundedVec<u8, ConstU32<MAX_DESCRIPTION_LENGTH>>>,
	/// When derivation occurred
	pub created_at: BlockNumberFor<T>,
}

/// Transformation step details
#[derive(Encode, Decode, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct TransformationStep {
	/// Step index
	pub step_index: u32,
	/// Transformation type
	pub transformation_type: TransformationType,
	/// Parameters used (optional)
	pub parameters: Option<BoundedVec<u8, ConstU32<128>>>,
	/// Description
	pub description: Option<BoundedVec<u8, ConstU32<MAX_DESCRIPTION_LENGTH>>>,
}

/// Transformation log entry
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct TransformationLog<T: crate::Config> {
	/// Transformation identifier
	pub transformation_id: TransformationId,
	/// Record being transformed
	pub record_id: RecordId,
	/// Transformation steps
	pub steps: BoundedVec<TransformationStep, ConstU32<MAX_TRANSFORMATION_STEPS>>,
	/// Agent who performed transformation
	pub agent: T::AccountId,
	/// When transformation occurred
	pub timestamp: BlockNumberFor<T>,
}

/// Usage activity entry
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct UsageActivity<T: crate::Config> {
	/// Record that was used
	pub record_id: RecordId,
	/// Activity type
	pub activity_type: ActivityType,
	/// Agent who performed the activity
	pub agent: T::AccountId,
	/// Agent type
	pub agent_type: AgentType,
	/// Purpose of activity
	pub purpose: Option<BoundedVec<u8, ConstU32<MAX_DESCRIPTION_LENGTH>>>,
	/// When activity occurred
	pub timestamp: BlockNumberFor<T>,
}

/// Convert u8 to TransformationType
impl From<u8> for TransformationType {
	fn from(value: u8) -> Self {
		match value {
			0 => TransformationType::Aggregation,
			1 => TransformationType::Anonymization,
			2 => TransformationType::Filtering,
			3 => TransformationType::Conversion,
			4 => TransformationType::Analysis,
			5 => TransformationType::MLInference,
			6 => TransformationType::Merge,
			7 => TransformationType::Split,
			_ => TransformationType::Aggregation,
		}
	}
}

/// Convert u8 to AgentType
impl From<u8> for AgentType {
	fn from(value: u8) -> Self {
		match value {
			0 => AgentType::Human,
			1 => AgentType::System,
			2 => AgentType::Contract,
			3 => AgentType::ExternalService,
			_ => AgentType::Human,
		}
	}
}

/// Convert u8 to ActivityType
impl From<u8> for ActivityType {
	fn from(value: u8) -> Self {
		match value {
			0 => ActivityType::Access,
			1 => ActivityType::Modification,
			2 => ActivityType::Derivation,
			3 => ActivityType::Sharing,
			4 => ActivityType::Export,
			5 => ActivityType::Deletion,
			_ => ActivityType::Access,
		}
	}
}
