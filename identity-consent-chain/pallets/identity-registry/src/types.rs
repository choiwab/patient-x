//! Type definitions for the Identity Registry pallet

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::*, BoundedVec};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Decentralized Identifier - max 64 bytes
pub type DID = BoundedVec<u8, ConstU32<64>>;

/// Jurisdiction code - max 8 bytes (e.g., "US", "EU", "SG")
pub type JurisdictionCode = BoundedVec<u8, ConstU32<8>>;

/// User information structure
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct UserInfo<T: crate::Config> {
	/// User's Decentralized Identifier
	pub did: DID,
	/// Type of user
	pub user_type: UserType,
	/// User's jurisdiction
	pub jurisdiction: JurisdictionCode,
	/// Optional institution affiliation
	pub institution: Option<BoundedVec<u8, T::MaxInstitutionLength>>,
	/// Whether the identity is verified
	pub verified: bool,
	/// Block number when identity was created
	pub created_at: BlockNumberFor<T>,
}

/// Verification status structure
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct VerificationStatus<T: crate::Config> {
	/// Whether the user is verified
	pub verified: bool,
	/// Account that performed the verification
	pub verifier: T::AccountId,
	/// Block number when verification occurred
	pub verified_at: BlockNumberFor<T>,
}

impl<T: crate::Config> Default for VerificationStatus<T>
where
	T::AccountId: Default,
{
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
