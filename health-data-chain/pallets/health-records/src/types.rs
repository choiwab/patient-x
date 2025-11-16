use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::*, BoundedVec};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Type alias for record ID (32 bytes)
pub type RecordId = [u8; 32];

/// Type alias for IPFS Content Identifier (CID)
pub type CID = BoundedVec<u8, ConstU32<128>>;

/// Type alias for encryption key ID (32 bytes)
pub type KeyId = [u8; 32];

/// Type alias for version number
pub type VersionNumber = u32;

/// Maximum length for custom data format name
pub const MAX_CUSTOM_FORMAT_LENGTH: u32 = 32;

/// Maximum length for record tags
pub const MAX_TAG_LENGTH: u32 = 64;

/// Maximum number of tags per record
pub const MAX_TAGS_PER_RECORD: u32 = 10;

/// Maximum length for access reason
pub const MAX_ACCESS_REASON_LENGTH: u32 = 256;

/// Data format types for health records
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum DataFormat {
	/// Fast Healthcare Interoperability Resources
	FHIR,
	/// Digital Imaging and Communications in Medicine
	DICOM,
	/// Health Level 7
	HL7,
	/// Comma-Separated Values
	CSV,
	/// JavaScript Object Notation
	JSON,
	/// Portable Document Format
	PDF,
	/// Custom format
	Custom(BoundedVec<u8, ConstU32<MAX_CUSTOM_FORMAT_LENGTH>>),
}

/// Record status
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum RecordStatus {
	/// Record is active and accessible
	Active,
	/// Record is archived (read-only)
	Archived,
	/// Record is deleted (soft delete)
	Deleted,
	/// Record is under review
	UnderReview,
}

/// Record metadata stored on-chain
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct RecordMetadata<T: crate::Config> {
	/// Unique record identifier
	pub record_id: RecordId,
	/// Record owner (patient)
	pub owner: T::AccountId,
	/// IPFS content identifier for the encrypted data
	pub ipfs_cid: CID,
	/// Data format type
	pub data_format: DataFormat,
	/// Encryption key identifier
	pub encryption_key_id: KeyId,
	/// Record creation timestamp
	pub created_at: BlockNumberFor<T>,
	/// Last update timestamp
	pub updated_at: BlockNumberFor<T>,
	/// Current version number
	pub current_version: VersionNumber,
	/// Record status
	pub status: RecordStatus,
	/// Optional tags for categorization
	pub tags: BoundedVec<BoundedVec<u8, ConstU32<MAX_TAG_LENGTH>>, ConstU32<MAX_TAGS_PER_RECORD>>,
}

/// Version metadata for record history
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct VersionMetadata<T: crate::Config> {
	/// Version number
	pub version: VersionNumber,
	/// IPFS CID for this version
	pub ipfs_cid: CID,
	/// Encryption key used for this version
	pub encryption_key_id: KeyId,
	/// When this version was created
	pub created_at: BlockNumberFor<T>,
	/// Account that created this version
	pub created_by: T::AccountId,
	/// Data format for this version
	pub data_format: DataFormat,
}

/// Access grant information
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct AccessGrant<T: crate::Config> {
	/// Account granted access
	pub grantee: T::AccountId,
	/// Record being accessed
	pub record_id: RecordId,
	/// When access was granted
	pub granted_at: BlockNumberFor<T>,
	/// Access expiration (None = permanent)
	pub expires_at: Option<BlockNumberFor<T>>,
	/// Whether grantee can modify the record
	pub can_modify: bool,
	/// Whether grantee can re-share access
	pub can_share: bool,
}

/// Access log entry
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct AccessLog<T: crate::Config> {
	/// Record that was accessed
	pub record_id: RecordId,
	/// Account that accessed the record
	pub accessor: T::AccountId,
	/// When the access occurred
	pub accessed_at: BlockNumberFor<T>,
	/// Access operation type
	pub operation: AccessOperation,
}

/// Access operation types
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum AccessOperation {
	/// Record was read
	Read,
	/// Record was modified
	Modified,
	/// Record was shared
	Shared,
	/// Access was revoked
	AccessRevoked,
	/// Record was deleted
	Deleted,
}

/// Convert u8 to DataFormat
impl From<u8> for DataFormat {
	fn from(value: u8) -> Self {
		match value {
			0 => DataFormat::FHIR,
			1 => DataFormat::DICOM,
			2 => DataFormat::HL7,
			3 => DataFormat::CSV,
			4 => DataFormat::JSON,
			5 => DataFormat::PDF,
			_ => DataFormat::FHIR, // Default
		}
	}
}

/// Convert DataFormat to u8
impl From<DataFormat> for u8 {
	fn from(format: DataFormat) -> Self {
		match format {
			DataFormat::FHIR => 0,
			DataFormat::DICOM => 1,
			DataFormat::HL7 => 2,
			DataFormat::CSV => 3,
			DataFormat::JSON => 4,
			DataFormat::PDF => 5,
			DataFormat::Custom(_) => 6,
		}
	}
}

/// Convert u8 to RecordStatus
impl From<u8> for RecordStatus {
	fn from(value: u8) -> Self {
		match value {
			0 => RecordStatus::Active,
			1 => RecordStatus::Archived,
			2 => RecordStatus::Deleted,
			3 => RecordStatus::UnderReview,
			_ => RecordStatus::Active,
		}
	}
}

/// Convert u8 to AccessOperation
impl From<u8> for AccessOperation {
	fn from(value: u8) -> Self {
		match value {
			0 => AccessOperation::Read,
			1 => AccessOperation::Modified,
			2 => AccessOperation::Shared,
			3 => AccessOperation::AccessRevoked,
			4 => AccessOperation::Deleted,
			_ => AccessOperation::Read,
		}
	}
}
