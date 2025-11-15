//! Type definitions for the Identity Registry pallet

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::*, BoundedVec};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Decentralized Identifier - max 64 bytes
pub type DID = BoundedVec<u8, ConstU32<64>>;

/// Jurisdiction code - max 8 bytes (e.g., "US", "EU", "SG")
pub type JurisdictionCode = BoundedVec<u8, ConstU32<8>>;

/// User information structure
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct UserInfo<T: frame_system::Config> {
	/// User's Decentralized Identifier
	pub did: DID,
	/// Type of user
	pub user_type: UserType,
	/// User's jurisdiction
	pub jurisdiction: JurisdictionCode,
	/// Optional institution affiliation
	pub institution: Option<BoundedVec<u8, ConstU32<128>>>,
	/// Whether the identity is verified
	pub verified: bool,
	/// Timestamp when identity was created
	pub created_at: <T as frame_system::Config>::BlockNumber,
}

/// Verification status structure
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct VerificationStatus<T: frame_system::Config> {
	/// Whether the user is verified
	pub verified: bool,
	/// Account that performed the verification
	pub verifier: T::AccountId,
	/// Timestamp when verification occurred
	pub verified_at: <T as frame_system::Config>::BlockNumber,
}

impl<T: frame_system::Config> Default for VerificationStatus<T> {
	fn default() -> Self {
		Self {
			verified: false,
			verifier: Default::default(),
			verified_at: Default::default(),
		}
	}
}

/// User type enumeration
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum UserType {
	/// Patient/data owner
	Patient,
	/// Medical researcher
	Researcher,
	/// Healthcare institution
	Institution,
	/// Data auditor
	Auditor,
	/// Academic publisher
	Publisher,
	/// Regulatory authority
	Regulator,
}

impl Default for UserType {
	fn default() -> Self {
		UserType::Patient
	}
}
