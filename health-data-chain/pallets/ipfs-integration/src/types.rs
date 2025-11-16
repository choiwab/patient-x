use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::*, BoundedVec};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Type alias for IPFS Content Identifier (CID)
pub type CID = BoundedVec<u8, ConstU32<128>>;

/// Type alias for IPFS gateway URL
pub type GatewayURL = BoundedVec<u8, ConstU32<256>>;

/// Type alias for operation ID
pub type OperationId = u64;

/// Maximum length for pin name
pub const MAX_PIN_NAME_LENGTH: u32 = 64;

/// IPFS operation types
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum IPFSOpType {
	/// Pin content to IPFS
	Pin,
	/// Unpin content from IPFS
	Unpin,
	/// Verify content exists and is accessible
	Verify,
	/// Update pin (re-pin with new settings)
	Update,
}

/// IPFS operation status
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum OperationStatus {
	/// Operation is pending execution
	Pending,
	/// Operation is currently being processed
	Processing,
	/// Operation completed successfully
	Completed,
	/// Operation failed
	Failed,
	/// Operation was cancelled
	Cancelled,
}

/// IPFS pin status
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum PinStatus {
	/// Content is pinned
	Pinned,
	/// Content is being pinned
	Pinning,
	/// Content is not pinned
	Unpinned,
	/// Pin failed
	Failed,
}

/// IPFS operation metadata
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct IPFSOperation<T: crate::Config> {
	/// Unique operation ID
	pub operation_id: OperationId,
	/// IPFS CID
	pub cid: CID,
	/// Operation type
	pub op_type: IPFSOpType,
	/// Operation status
	pub status: OperationStatus,
	/// Requestor account
	pub requestor: T::AccountId,
	/// When operation was requested
	pub requested_at: BlockNumberFor<T>,
	/// When operation was completed (if completed)
	pub completed_at: Option<BlockNumberFor<T>>,
	/// Number of retry attempts
	pub retry_count: u8,
	/// Optional error message
	pub error_message: Option<BoundedVec<u8, ConstU32<256>>>,
}

/// IPFS pin metadata
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct PinMetadata<T: crate::Config> {
	/// IPFS CID
	pub cid: CID,
	/// Pin status
	pub status: PinStatus,
	/// Account that pinned the content
	pub pinner: T::AccountId,
	/// When content was pinned
	pub pinned_at: BlockNumberFor<T>,
	/// Last verification timestamp
	pub last_verified: Option<BlockNumberFor<T>>,
	/// Pin name/label
	pub name: Option<BoundedVec<u8, ConstU32<MAX_PIN_NAME_LENGTH>>>,
	/// Reference count (how many records reference this CID)
	pub reference_count: u32,
}

/// IPFS gateway configuration
#[derive(Encode, Decode, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct GatewayConfig {
	/// Gateway URL (e.g., "https://ipfs.io")
	pub url: GatewayURL,
	/// Whether this gateway is enabled
	pub enabled: bool,
	/// Priority (lower number = higher priority)
	pub priority: u8,
}

/// Verification result
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum VerificationResult {
	/// Content is accessible and valid
	Valid,
	/// Content is not accessible
	Inaccessible,
	/// Content hash mismatch
	HashMismatch,
	/// Verification failed due to error
	Error,
}

/// Convert u8 to IPFSOpType
impl From<u8> for IPFSOpType {
	fn from(value: u8) -> Self {
		match value {
			0 => IPFSOpType::Pin,
			1 => IPFSOpType::Unpin,
			2 => IPFSOpType::Verify,
			3 => IPFSOpType::Update,
			_ => IPFSOpType::Pin,
		}
	}
}

/// Convert IPFSOpType to u8
impl From<IPFSOpType> for u8 {
	fn from(op_type: IPFSOpType) -> Self {
		match op_type {
			IPFSOpType::Pin => 0,
			IPFSOpType::Unpin => 1,
			IPFSOpType::Verify => 2,
			IPFSOpType::Update => 3,
		}
	}
}

/// Convert u8 to OperationStatus
impl From<u8> for OperationStatus {
	fn from(value: u8) -> Self {
		match value {
			0 => OperationStatus::Pending,
			1 => OperationStatus::Processing,
			2 => OperationStatus::Completed,
			3 => OperationStatus::Failed,
			4 => OperationStatus::Cancelled,
			_ => OperationStatus::Pending,
		}
	}
}

/// Convert u8 to PinStatus
impl From<u8> for PinStatus {
	fn from(value: u8) -> Self {
		match value {
			0 => PinStatus::Pinned,
			1 => PinStatus::Pinning,
			2 => PinStatus::Unpinned,
			3 => PinStatus::Failed,
			_ => PinStatus::Unpinned,
		}
	}
}

/// Convert u8 to VerificationResult
impl From<u8> for VerificationResult {
	fn from(value: u8) -> Self {
		match value {
			0 => VerificationResult::Valid,
			1 => VerificationResult::Inaccessible,
			2 => VerificationResult::HashMismatch,
			3 => VerificationResult::Error,
			_ => VerificationResult::Error,
		}
	}
}
