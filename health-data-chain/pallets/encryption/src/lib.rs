//! # Encryption Pallet
//!
//! This pallet provides encryption key management for Patient X HealthData Chain.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod types;
pub mod weights;

pub use types::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use pallet_health_records::RecordId;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_health_records::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: crate::weights::WeightInfo;
	}

	#[pallet::storage]
	pub type Keys<T: Config> = StorageMap<_, Blake2_128Concat, KeyId, KeyMetadata<T>, OptionQuery>;

	#[pallet::storage]
	pub type UserKeys<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<KeyId, ConstU32<100>>, ValueQuery>;

	#[pallet::storage]
	pub type RecordKeys<T: Config> = StorageMap<_, Blake2_128Concat, RecordId, KeyId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KeyRegistered { key_id: KeyId, owner: T::AccountId },
		KeyRevoked { key_id: KeyId },
		KeyRotated { old_key_id: KeyId, new_key_id: KeyId },
		KeyShared { key_id: KeyId, recipient: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		KeyNotFound,
		KeyAlreadyExists,
		NotAuthorized,
		KeyRevoked,
		KeyExpired,
		TooManyKeys,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::register_key())]
		pub fn register_key(
			origin: OriginFor<T>,
			key_id: KeyId,
			key_type: u8,
			algorithm: u8,
			public_key: Option<PublicKey>,
			duration_blocks: Option<BlockNumberFor<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!Keys::<T>::contains_key(&key_id), Error::<T>::KeyAlreadyExists);

			let now = frame_system::Pallet::<T>::block_number();
			let metadata = KeyMetadata {
				key_id,
				owner: who.clone(),
				key_type: key_type.into(),
				algorithm: algorithm.into(),
				status: KeyStatus::Active,
				public_key,
				created_at: now,
				expires_at: duration_blocks.map(|d| now + d),
				last_rotated: None,
			};

			Keys::<T>::insert(&key_id, metadata);
			UserKeys::<T>::try_mutate(&who, |keys| keys.try_push(key_id).map_err(|_| Error::<T>::TooManyKeys))?;

			Self::deposit_event(Event::KeyRegistered { key_id, owner: who });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::revoke_key())]
		pub fn revoke_key(origin: OriginFor<T>, key_id: KeyId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Keys::<T>::try_mutate(&key_id, |maybe_key| {
				let key = maybe_key.as_mut().ok_or(Error::<T>::KeyNotFound)?;
				ensure!(key.owner == who, Error::<T>::NotAuthorized);
				key.status = KeyStatus::Revoked;
				Ok::<(), DispatchError>(())
			})?;

			Self::deposit_event(Event::KeyRevoked { key_id });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::rotate_key())]
		pub fn rotate_key(
			origin: OriginFor<T>,
			old_key_id: KeyId,
			new_key_id: KeyId,
			public_key: Option<PublicKey>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let old_key = Keys::<T>::get(&old_key_id).ok_or(Error::<T>::KeyNotFound)?;
			ensure!(old_key.owner == who, Error::<T>::NotAuthorized);

			let now = frame_system::Pallet::<T>::block_number();
			let new_metadata = KeyMetadata {
				key_id: new_key_id,
				owner: who.clone(),
				key_type: old_key.key_type,
				algorithm: old_key.algorithm,
				status: KeyStatus::Active,
				public_key,
				created_at: now,
				expires_at: old_key.expires_at,
				last_rotated: Some(now),
			};

			Keys::<T>::insert(&new_key_id, new_metadata);
			Keys::<T>::mutate(&old_key_id, |maybe_key| {
				if let Some(key) = maybe_key {
					key.status = KeyStatus::Revoked;
				}
			});

			Self::deposit_event(Event::KeyRotated { old_key_id, new_key_id });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::share_key())]
		pub fn share_key(origin: OriginFor<T>, key_id: KeyId, recipient: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let key = Keys::<T>::get(&key_id).ok_or(Error::<T>::KeyNotFound)?;
			ensure!(key.owner == who, Error::<T>::NotAuthorized);

			Self::deposit_event(Event::KeyShared { key_id, recipient });
			Ok(())
		}
	}
}
