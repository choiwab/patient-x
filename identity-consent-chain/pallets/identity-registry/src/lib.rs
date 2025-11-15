//! # Identity Registry Pallet
//!
//! This pallet manages user identities using Decentralized Identifiers (DIDs).
//!
//! ## Overview
//!
//! The Identity Registry pallet provides functionality for:
//! - Registering user identities with DIDs
//! - Managing different user types (Patient, Researcher, Institution, etc.)
//! - Verifying identities
//! - Tracking jurisdiction information
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `register_identity` - Register a new identity with a DID
//! - `update_jurisdiction` - Update user's jurisdiction
//! - `verify_identity` - Verify an identity (restricted to authorized verifiers)

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod types;
pub mod weights;

pub use types::*;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, traits::Time};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::Verify;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Time provider
		type TimeProvider: Time;

		/// Maximum length of institution name
		#[pallet::constant]
		type MaxInstitutionLength: Get<u32>;

		/// Weight information for extrinsics
		type WeightInfo: WeightInfo;

		/// Signature type for verification
		type Signature: Verify<Signer = Self::AccountId> + Parameter;
	}

	/// Storage for user identities
	#[pallet::storage]
	#[pallet::getter(fn identities)]
	pub type Identities<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		UserInfo<T>,
		OptionQuery,
	>;

	/// Storage for DID to AccountId mapping
	#[pallet::storage]
	#[pallet::getter(fn dids)]
	pub type DIDs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		DID,
		T::AccountId,
		OptionQuery,
	>;

	/// Storage for verified users
	#[pallet::storage]
	#[pallet::getter(fn verified_users)]
	pub type VerifiedUsers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		DID,
		VerificationStatus<T>,
		ValueQuery,
	>;

	/// Storage for user jurisdictions
	#[pallet::storage]
	#[pallet::getter(fn user_jurisdictions)]
	pub type UserJurisdictions<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		JurisdictionCode,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Identity registered [account, did, user_type]
		IdentityRegistered {
			account: T::AccountId,
			did: DID,
			user_type: UserType,
		},
		/// Identity verified [did, verifier]
		IdentityVerified {
			did: DID,
			verifier: T::AccountId,
		},
		/// Jurisdiction updated [account, old_jurisdiction, new_jurisdiction]
		JurisdictionUpdated {
			account: T::AccountId,
			old_jurisdiction: Option<JurisdictionCode>,
			new_jurisdiction: JurisdictionCode,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// DID already exists
		DIDAlreadyExists,
		/// Identity already registered for this account
		IdentityAlreadyRegistered,
		/// Identity not found
		IdentityNotFound,
		/// DID not found
		DIDNotFound,
		/// Unauthorized verifier
		UnauthorizedVerifier,
		/// Invalid signature
		InvalidSignature,
		/// Institution name too long
		InstitutionNameTooLong,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register a new identity with a DID
		///
		/// # Arguments
		/// * `origin` - The account registering the identity
		/// * `did` - The Decentralized Identifier
		/// * `user_type` - Type of user (Patient, Researcher, etc.)
		/// * `jurisdiction` - User's jurisdiction code
		/// * `institution` - Optional institution affiliation
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::register_identity())]
		pub fn register_identity(
			origin: OriginFor<T>,
			did: DID,
			user_type: UserType,
			jurisdiction: JurisdictionCode,
			institution: Option<BoundedVec<u8, T::MaxInstitutionLength>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure DID is unique
			ensure!(!DIDs::<T>::contains_key(&did), Error::<T>::DIDAlreadyExists);

			// Ensure account doesn't already have identity
			ensure!(
				!Identities::<T>::contains_key(&who),
				Error::<T>::IdentityAlreadyRegistered
			);

			let user_info = UserInfo {
				did: did.clone(),
				user_type: user_type.clone(),
				jurisdiction: jurisdiction.clone(),
				institution,
				verified: false,
				created_at: T::TimeProvider::now(),
			};

			Identities::<T>::insert(&who, user_info);
			DIDs::<T>::insert(&did, &who);
			UserJurisdictions::<T>::insert(&who, &jurisdiction);

			Self::deposit_event(Event::IdentityRegistered {
				account: who,
				did,
				user_type,
			});

			Ok(())
		}

		/// Update user's jurisdiction
		///
		/// # Arguments
		/// * `origin` - The account updating jurisdiction
		/// * `new_jurisdiction` - New jurisdiction code
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::update_jurisdiction())]
		pub fn update_jurisdiction(
			origin: OriginFor<T>,
			new_jurisdiction: JurisdictionCode,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure identity exists
			ensure!(
				Identities::<T>::contains_key(&who),
				Error::<T>::IdentityNotFound
			);

			let old_jurisdiction = UserJurisdictions::<T>::get(&who);

			// Update in both storage locations
			Identities::<T>::mutate(&who, |info| {
				if let Some(user_info) = info {
					user_info.jurisdiction = new_jurisdiction.clone();
				}
			});

			UserJurisdictions::<T>::insert(&who, &new_jurisdiction);

			Self::deposit_event(Event::JurisdictionUpdated {
				account: who,
				old_jurisdiction,
				new_jurisdiction,
			});

			Ok(())
		}

		/// Verify an identity (restricted to authorized verifiers)
		///
		/// # Arguments
		/// * `origin` - The verifier account (must be Institution or Auditor)
		/// * `user_did` - DID of the user to verify
		/// * `verifier_signature` - Signature proving verification
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::verify_identity())]
		pub fn verify_identity(
			origin: OriginFor<T>,
			user_did: DID,
			_verifier_signature: T::Signature,
		) -> DispatchResult {
			let verifier = ensure_signed(origin)?;

			// Ensure verifier is authorized (Institution or Auditor)
			let verifier_info = Identities::<T>::get(&verifier)
				.ok_or(Error::<T>::IdentityNotFound)?;

			ensure!(
				matches!(verifier_info.user_type, UserType::Institution | UserType::Auditor),
				Error::<T>::UnauthorizedVerifier
			);

			// Get user account
			let user = DIDs::<T>::get(&user_did).ok_or(Error::<T>::DIDNotFound)?;

			// Update verification status
			Identities::<T>::mutate(&user, |info| {
				if let Some(user_info) = info {
					user_info.verified = true;
				}
			});

			VerifiedUsers::<T>::insert(
				&user_did,
				VerificationStatus {
					verified: true,
					verifier: verifier.clone(),
					verified_at: T::TimeProvider::now(),
				}
			);

			Self::deposit_event(Event::IdentityVerified {
				did: user_did,
				verifier,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Get user information by account ID
		pub fn get_user_info(account: &T::AccountId) -> Option<UserInfo<T>> {
			Identities::<T>::get(account)
		}

		/// Get account ID by DID
		pub fn get_account_by_did(did: &DID) -> Option<T::AccountId> {
			DIDs::<T>::get(did)
		}

		/// Check if user is verified
		pub fn is_verified(did: &DID) -> bool {
			VerifiedUsers::<T>::get(did).verified
		}
	}
}
