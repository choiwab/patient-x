use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::*, BoundedVec};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Type alias for policy ID
pub type PolicyId = [u8; 32];

/// Type alias for attribute key
pub type AttributeKey = BoundedVec<u8, ConstU32<32>>;

/// Type alias for attribute value
pub type AttributeValue = BoundedVec<u8, ConstU32<64>>;

/// Maximum number of attributes per policy
pub const MAX_ATTRIBUTES_PER_POLICY: u32 = 10;

/// Maximum number of conditions per policy
pub const MAX_CONDITIONS_PER_POLICY: u32 = 5;

/// Maximum policy name length
pub const MAX_POLICY_NAME_LENGTH: u32 = 64;

/// Attribute types for access control
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum AttributeType {
	/// Role-based attribute (e.g., "role" = "doctor")
	Role,
	/// Organization-based attribute (e.g., "org" = "hospital_a")
	Organization,
	/// Department-based attribute (e.g., "dept" = "cardiology")
	Department,
	/// Clearance level (e.g., "clearance" = "level_3")
	ClearanceLevel,
	/// Geographic location (e.g., "location" = "us_east")
	Location,
	/// Time-based attribute (e.g., "time" = "business_hours")
	Time,
	/// Custom attribute
	Custom(BoundedVec<u8, ConstU32<32>>),
}

/// Access policy condition operators
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum ConditionOperator {
	/// Equals
	Equals,
	/// Not equals
	NotEquals,
	/// Contains (for list values)
	Contains,
	/// Greater than
	GreaterThan,
	/// Less than
	LessThan,
	/// In range
	InRange,
}

/// Access policy condition
#[derive(Encode, Decode, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PolicyCondition {
	/// Attribute key
	pub attribute_key: AttributeKey,
	/// Condition operator
	pub operator: ConditionOperator,
	/// Expected value
	pub value: AttributeValue,
}

/// Access policy effect
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum PolicyEffect {
	/// Allow access
	Allow,
	/// Deny access
	Deny,
}

/// Policy evaluation mode
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum PolicyMode {
	/// All conditions must be satisfied (AND)
	AllOf,
	/// At least one condition must be satisfied (OR)
	AnyOf,
	/// Exactly one condition must be satisfied (XOR)
	OneOf,
}

/// Access policy definition
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct AccessPolicy<T: crate::Config> {
	/// Unique policy ID
	pub policy_id: PolicyId,
	/// Policy name
	pub name: BoundedVec<u8, ConstU32<MAX_POLICY_NAME_LENGTH>>,
	/// Policy creator
	pub creator: T::AccountId,
	/// Policy effect (Allow/Deny)
	pub effect: PolicyEffect,
	/// Policy evaluation mode
	pub mode: PolicyMode,
	/// Policy conditions
	pub conditions: BoundedVec<PolicyCondition, ConstU32<MAX_CONDITIONS_PER_POLICY>>,
	/// When policy was created
	pub created_at: BlockNumberFor<T>,
	/// Policy expiration (None = never expires)
	pub expires_at: Option<BlockNumberFor<T>>,
	/// Whether policy is active
	pub is_active: bool,
}

/// User attribute assignment
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct UserAttribute<T: crate::Config> {
	/// Attribute key
	pub key: AttributeKey,
	/// Attribute value
	pub value: AttributeValue,
	/// Attribute type
	pub attribute_type: AttributeType,
	/// Who assigned this attribute
	pub assigned_by: T::AccountId,
	/// When assigned
	pub assigned_at: BlockNumberFor<T>,
	/// Expiration (None = never expires)
	pub expires_at: Option<BlockNumberFor<T>>,
}

/// Policy evaluation result
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum EvaluationResult {
	/// Access allowed
	Allow,
	/// Access denied
	Deny,
	/// Policy not applicable
	NotApplicable,
	/// Evaluation error
	Error,
}

/// Convert u8 to AttributeType
impl From<u8> for AttributeType {
	fn from(value: u8) -> Self {
		match value {
			0 => AttributeType::Role,
			1 => AttributeType::Organization,
			2 => AttributeType::Department,
			3 => AttributeType::ClearanceLevel,
			4 => AttributeType::Location,
			5 => AttributeType::Time,
			_ => AttributeType::Role, // Default
		}
	}
}

/// Convert u8 to ConditionOperator
impl From<u8> for ConditionOperator {
	fn from(value: u8) -> Self {
		match value {
			0 => ConditionOperator::Equals,
			1 => ConditionOperator::NotEquals,
			2 => ConditionOperator::Contains,
			3 => ConditionOperator::GreaterThan,
			4 => ConditionOperator::LessThan,
			5 => ConditionOperator::InRange,
			_ => ConditionOperator::Equals,
		}
	}
}

/// Convert u8 to PolicyEffect
impl From<u8> for PolicyEffect {
	fn from(value: u8) -> Self {
		match value {
			0 => PolicyEffect::Allow,
			1 => PolicyEffect::Deny,
			_ => PolicyEffect::Allow,
		}
	}
}

/// Convert u8 to PolicyMode
impl From<u8> for PolicyMode {
	fn from(value: u8) -> Self {
		match value {
			0 => PolicyMode::AllOf,
			1 => PolicyMode::AnyOf,
			2 => PolicyMode::OneOf,
			_ => PolicyMode::AllOf,
		}
	}
}

/// Convert u8 to EvaluationResult
impl From<u8> for EvaluationResult {
	fn from(value: u8) -> Self {
		match value {
			0 => EvaluationResult::Allow,
			1 => EvaluationResult::Deny,
			2 => EvaluationResult::NotApplicable,
			3 => EvaluationResult::Error,
			_ => EvaluationResult::Deny,
		}
	}
}
