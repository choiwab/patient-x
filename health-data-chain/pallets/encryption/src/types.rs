use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::*, BoundedVec};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Type alias for encryption key ID (32 bytes)
pub type KeyId = [u8; 32];

/// Type alias for encrypted key material
pub type EncryptedKeyMaterial = BoundedVec<u8, ConstU32<256>>;

/// Type alias for public key
pub type PublicKey = BoundedVec<u8, ConstU32<64>>;

/// Encryption algorithm types
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum EncryptionAlgorithm {
	/// AES-256-GCM
	AES256GCM,
	/// ChaCha20-Poly1305
	ChaCha20Poly1305,
	/// Attribute-Based Encryption
	ABE,
	/// Hybrid (AES + RSA)
	Hybrid,
}

/// Key type
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum KeyType {
	/// Symmetric key
	Symmetric,
	/// Asymmetric key pair
	Asymmetric,
	/// ABE master key
	ABEMaster,
	/// ABE user key
	ABEUser,
}

/// Key status
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum KeyStatus {
	/// Key is active
	Active,
	/// Key is revoked
	Revoked,
	/// Key is expired
	Expired,
	/// Key is pending activation
	Pending,
}

/// Encryption key metadata
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct KeyMetadata<T: crate::Config> {
	/// Key identifier
	pub key_id: KeyId,
	/// Key owner
	pub owner: T::AccountId,
	/// Key type
	pub key_type: KeyType,
	/// Encryption algorithm
	pub algorithm: EncryptionAlgorithm,
	/// Key status
	pub status: KeyStatus,
	/// Public key (for asymmetric keys)
	pub public_key: Option<PublicKey>,
	/// When key was created
	pub created_at: BlockNumberFor<T>,
	/// Key expiration (None = never expires)
	pub expires_at: Option<BlockNumberFor<T>>,
	/// Last rotation timestamp
	pub last_rotated: Option<BlockNumberFor<T>>,
}

/// Convert u8 to EncryptionAlgorithm
impl From<u8> for EncryptionAlgorithm {
	fn from(value: u8) -> Self {
		match value {
			0 => EncryptionAlgorithm::AES256GCM,
			1 => EncryptionAlgorithm::ChaCha20Poly1305,
			2 => EncryptionAlgorithm::ABE,
			3 => EncryptionAlgorithm::Hybrid,
			_ => EncryptionAlgorithm::AES256GCM,
		}
	}
}

/// Convert u8 to KeyType
impl From<u8> for KeyType {
	fn from(value: u8) -> Self {
		match value {
			0 => KeyType::Symmetric,
			1 => KeyType::Asymmetric,
			2 => KeyType::ABEMaster,
			3 => KeyType::ABEUser,
			_ => KeyType::Symmetric,
		}
	}
}

/// Convert u8 to KeyStatus
impl From<u8> for KeyStatus {
	fn from(value: u8) -> Self {
		match value {
			0 => KeyStatus::Active,
			1 => KeyStatus::Revoked,
			2 => KeyStatus::Expired,
			3 => KeyStatus::Pending,
			_ => KeyStatus::Active,
		}
	}
}
