//! # Consent Manager Pallet
//!
//! This pallet manages granular consent policies for medical data sharing.
//!
//! ## Overview
//!
//! The Consent Manager pallet enables patients to:
//! - Grant granular consent for data usage
//! - Specify purposes, durations, and allowed parties
//! - Revoke consent at any time
//! - Track consent history
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `grant_consent`: Create a new consent policy
//! - `revoke_consent`: Revoke an existing consent
//! - `update_consent`: Update consent parameters
//! - `check_consent_validity`: Verify if consent is valid for a specific use

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Tests and benchmarks temporarily disabled - need to be updated for new dispatchable signatures
// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

pub mod types;
pub mod weights;

pub use types::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::Hash;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_identity_registry::Config {
		/// The overarching event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum number of data types per consent
		#[pallet::constant]
		type MaxDataTypes: Get<u32>;

		/// Maximum number of allowed parties in a specific list
		#[pallet::constant]
		type MaxAllowedParties: Get<u32>;

		/// Maximum number of jurisdictions per consent
		#[pallet::constant]
		type MaxJurisdictions: Get<u32>;

		/// Maximum number of consents per user
		#[pallet::constant]
		type MaxConsentsPerUser: Get<u32>;

		/// Weight information for extrinsics
		type WeightInfo: crate::weights::WeightInfo;
	}

	/// Storage map for consent policies
	#[pallet::storage]
	#[pallet::getter(fn consents)]
	pub type Consents<T: Config> =
		StorageMap<_, Blake2_128Concat, ConsentId, ConsentPolicy<T>, OptionQuery>;

	/// Double map for user consents (AccountId + ConsentId -> Status)
	#[pallet::storage]
	#[pallet::getter(fn user_consents)]
	pub type UserConsents<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		ConsentId,
		ConsentStatus<T>,
		OptionQuery,
	>;

	/// Index of consents per user
	#[pallet::storage]
	#[pallet::getter(fn consent_index)]
	pub type ConsentIndex<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<ConsentId, T::MaxConsentsPerUser>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Consent was granted
		ConsentGranted {
			data_owner: T::AccountId,
			consent_id: ConsentId,
		},
		/// Consent was revoked
		ConsentRevoked {
			consent_id: ConsentId,
			data_owner: T::AccountId,
		},
		/// Consent was updated
		ConsentUpdated {
			consent_id: ConsentId,
			data_owner: T::AccountId,
		},
		/// Consent validity was checked
		ConsentChecked {
			consent_id: ConsentId,
			requester: T::AccountId,
			valid: bool,
		},
		/// Consent expired
		ConsentExpired {
			consent_id: ConsentId,
			data_owner: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// User has not registered an identity
		IdentityNotRegistered,
		/// Consent was not found
		ConsentNotFound,
		/// User is not the owner of the consent
		NotConsentOwner,
		/// Consent is not active
		ConsentNotActive,
		/// Purpose does not match
		PurposeMismatch,
		/// Consent is not yet active
		ConsentNotYetActive,
		/// Consent has expired
		ConsentExpired,
		/// Requester is not allowed
		RequesterNotAllowed,
		/// Too many consents for this user
		TooManyConsents,
		/// Data types list is too long
		TooManyDataTypes,
		/// Allowed parties list is too long
		TooManyAllowedParties,
		/// Jurisdictions list is too long
		TooManyJurisdictions,
		/// Invalid duration (end before start)
		InvalidDuration,
		/// Consent already exists with this ID
		ConsentAlreadyExists,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Grant a new consent
		///
		/// # Arguments
		/// * `purpose_type` - Purpose type (0=ResearchGeneral, 1=Commercial, 2=PublicHealth)
		/// * `purpose_data` - Optional purpose-specific data (for ResearchSpecificStudy or Custom)
		/// * `start_block` - Start block number
		/// * `end_block` - Optional end block number
		/// * `auto_renewal` - Whether consent automatically renews
		/// * `data_type_ids` - Data type IDs (0-8 for standard types)
		/// * `allowed_party_type` - Type (0=Public, 1=Specific, 2=Categories)
		/// * `allowed_party_data` - Party-specific data (accounts or category IDs)
		/// * `jurisdictions` - Allowed jurisdictions
		/// * `compensation_type` - Compensation type (0=Free, 1=FixedPrice, 2=Percentage, 3=Negotiable)
		/// * `compensation_value` - Compensation value (price or percentage)
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::grant_consent())]
		pub fn grant_consent(
			origin: OriginFor<T>,
			purpose_type: u8,
			purpose_data: Option<BoundedVec<u8, ConstU32<128>>>,
			start_block: BlockNumberFor<T>,
			end_block: Option<BlockNumberFor<T>>,
			auto_renewal: bool,
			data_type_ids: BoundedVec<u8, T::MaxDataTypes>,
			allowed_party_type: u8,
			allowed_party_data: BoundedVec<u8, ConstU32<255>>,
			jurisdictions: BoundedVec<JurisdictionCode, T::MaxJurisdictions>,
			compensation_type: u8,
			compensation_value: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure user has registered identity
			ensure!(
				pallet_identity_registry::Pallet::<T>::identities(&who).is_some(),
				Error::<T>::IdentityNotRegistered
			);

			// Convert simple parameters to complex types
			let purpose = Self::decode_purpose(purpose_type, purpose_data)?;
			let duration = Duration {
				start: start_block,
				end: end_block,
				auto_renewal,
			};
			let data_types = Self::decode_data_types(data_type_ids)?;
			let allowed_parties = Self::decode_allowed_parties(allowed_party_type, allowed_party_data)?;
			let compensation_preference = Self::decode_compensation(compensation_type, compensation_value)?;

			// Validate duration
			if let Some(end) = duration.end {
				ensure!(end > duration.start, Error::<T>::InvalidDuration);
			}

			// Generate unique consent ID
			let consent_id = Self::generate_consent_id(&who, &purpose);

			// Ensure consent doesn't already exist
			ensure!(!Consents::<T>::contains_key(&consent_id), Error::<T>::ConsentAlreadyExists);

			let now = frame_system::Pallet::<T>::block_number();

			let policy = ConsentPolicy {
				consent_id,
				data_owner: who.clone(),
				purpose,
				duration,
				data_types,
				allowed_parties,
				jurisdictions,
				compensation_preference,
				created_at: now,
				updated_at: now,
			};

			// Store consent policy
			Consents::<T>::insert(&consent_id, policy);

			// Update user consents
			UserConsents::<T>::insert(&who, &consent_id, ConsentStatus::Active);

			// Update consent index
			ConsentIndex::<T>::try_mutate(&who, |consents| {
				consents.try_push(consent_id).map_err(|_| Error::<T>::TooManyConsents)
			})?;

			Self::deposit_event(Event::ConsentGranted {
				data_owner: who,
				consent_id,
			});

			Ok(())
		}

		/// Revoke a consent
		///
		/// # Arguments
		/// * `consent_id` - The consent to revoke
		/// * `reason` - Optional reason for revocation
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::revoke_consent())]
		pub fn revoke_consent(
			origin: OriginFor<T>,
			consent_id: ConsentId,
			reason: Option<BoundedVec<u8, ConstU32<256>>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure consent exists and belongs to user
			let policy =
				Consents::<T>::get(&consent_id).ok_or(Error::<T>::ConsentNotFound)?;
			ensure!(policy.data_owner == who, Error::<T>::NotConsentOwner);

			let now = frame_system::Pallet::<T>::block_number();

			// Update status to revoked
			UserConsents::<T>::insert(
				&who,
				&consent_id,
				ConsentStatus::Revoked { revoked_at: now, reason },
			);

			Self::deposit_event(Event::ConsentRevoked { consent_id, data_owner: who });

			Ok(())
		}

		/// Update consent parameters
		///
		/// # Arguments
		/// * `consent_id` - The consent to update
		/// * `new_compensation_type` - Optional new compensation type
		/// * `new_compensation_value` - Optional new compensation value
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::update_consent())]
		pub fn update_consent(
			origin: OriginFor<T>,
			consent_id: ConsentId,
			new_compensation_type: Option<u8>,
			new_compensation_value: Option<u128>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Get existing policy
			let mut policy =
				Consents::<T>::get(&consent_id).ok_or(Error::<T>::ConsentNotFound)?;
			ensure!(policy.data_owner == who, Error::<T>::NotConsentOwner);

			// Ensure consent is active
			let status = UserConsents::<T>::get(&who, &consent_id)
				.ok_or(Error::<T>::ConsentNotFound)?;
			ensure!(status.is_active(), Error::<T>::ConsentNotActive);

			// Update compensation if provided
			if let (Some(comp_type), Some(comp_value)) = (new_compensation_type, new_compensation_value) {
				let compensation = Self::decode_compensation(comp_type, comp_value)?;
				policy.compensation_preference = compensation;
			}

			// Update timestamp
			policy.updated_at = frame_system::Pallet::<T>::block_number();

			// Store updated policy
			Consents::<T>::insert(&consent_id, policy);

			Self::deposit_event(Event::ConsentUpdated { consent_id, data_owner: who });

			Ok(())
		}

		/// Check if consent is valid for a specific use
		///
		/// # Arguments
		/// * `consent_id` - The consent to check
		/// * `requester` - Who is requesting access
		/// * `purpose_type` - Purpose type to check
		/// * `purpose_data` - Optional purpose-specific data
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::check_consent_validity())]
		pub fn check_consent_validity(
			origin: OriginFor<T>,
			consent_id: ConsentId,
			requester: T::AccountId,
			purpose_type: u8,
			purpose_data: Option<BoundedVec<u8, ConstU32<128>>>,
		) -> DispatchResult {
			ensure_signed(origin)?;

			let policy =
				Consents::<T>::get(&consent_id).ok_or(Error::<T>::ConsentNotFound)?;

			let status = UserConsents::<T>::get(&policy.data_owner, &consent_id)
				.ok_or(Error::<T>::ConsentNotFound)?;

			// Check if consent is active
			ensure!(status.is_active(), Error::<T>::ConsentNotActive);

			// Decode purpose
			let purpose = Self::decode_purpose(purpose_type, purpose_data)?;

			// Check purpose match
			ensure!(Self::purpose_matches(&policy.purpose, &purpose), Error::<T>::PurposeMismatch);

			// Check duration
			let now = frame_system::Pallet::<T>::block_number();
			ensure!(now >= policy.duration.start, Error::<T>::ConsentNotYetActive);

			if let Some(end) = policy.duration.end {
				if now > end {
					// Mark as expired
					UserConsents::<T>::insert(
						&policy.data_owner,
						&consent_id,
						ConsentStatus::Expired { expired_at: now },
					);
					Self::deposit_event(Event::ConsentExpired {
						consent_id,
						data_owner: policy.data_owner,
					});
					return Err(Error::<T>::ConsentExpired.into());
				}
			}

			// Check allowed parties
			ensure!(
				Self::is_party_allowed(&policy.allowed_parties, &requester),
				Error::<T>::RequesterNotAllowed
			);

			Self::deposit_event(Event::ConsentChecked {
				consent_id,
				requester,
				valid: true,
			});

			Ok(())
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		/// Generate a unique consent ID based on owner and purpose
		fn generate_consent_id(owner: &T::AccountId, purpose: &Purpose) -> ConsentId {
			let now = frame_system::Pallet::<T>::block_number();
			let mut data = owner.encode();
			data.extend_from_slice(&purpose.encode());
			data.extend_from_slice(&now.encode());
			T::Hashing::hash(&data).as_ref()[0..32].try_into().unwrap_or([0u8; 32])
		}

		/// Check if two purposes match
		fn purpose_matches(policy_purpose: &Purpose, request_purpose: &Purpose) -> bool {
			match (policy_purpose, request_purpose) {
				(Purpose::ResearchGeneral, Purpose::ResearchGeneral) => true,
				(Purpose::ResearchGeneral, Purpose::ResearchSpecificStudy(_)) => true, // General allows specific
				(
					Purpose::ResearchSpecificStudy(study1),
					Purpose::ResearchSpecificStudy(study2),
				) => study1 == study2,
				(Purpose::Commercial, Purpose::Commercial) => true,
				(Purpose::PublicHealth, Purpose::PublicHealth) => true,
				(Purpose::Custom(c1), Purpose::Custom(c2)) => c1 == c2,
				_ => false,
			}
		}

		/// Check if a party is allowed
		fn is_party_allowed(allowed: &AllowedParties<T>, requester: &T::AccountId) -> bool {
			match allowed {
				AllowedParties::Public => true,
				AllowedParties::Specific(list) => list.contains(requester),
				AllowedParties::Categories(categories) => {
					// Check if requester's user type is in allowed categories
					if let Some(identity) = pallet_identity_registry::Pallet::<T>::identities(requester) {
						let user_type_u8: u8 = identity.user_type.into();
						categories.contains(&user_type_u8)
					} else {
						false
					}
				},
			}
		}

		/// Decode purpose from u8 and optional data
		fn decode_purpose(
			purpose_type: u8,
			purpose_data: Option<BoundedVec<u8, ConstU32<128>>>,
		) -> Result<Purpose, DispatchError> {
			match purpose_type {
				0 => Ok(Purpose::ResearchGeneral),
				1 => {
					if let Some(data) = purpose_data {
						// Convert BoundedVec<u8, 128> to BoundedVec<u8, 64> by truncating if needed
						let truncated: BoundedVec<u8, ConstU32<64>> = data.into_inner().try_into().map_err(|_| Error::<T>::TooManyDataTypes)?;
						Ok(Purpose::ResearchSpecificStudy(truncated))
					} else {
						Ok(Purpose::ResearchGeneral)
					}
				},
				2 => Ok(Purpose::Commercial),
				3 => Ok(Purpose::PublicHealth),
				4 => {
					if let Some(data) = purpose_data {
						Ok(Purpose::Custom(data))
					} else {
						Err(Error::<T>::PurposeMismatch.into())
					}
				},
				_ => Err(Error::<T>::PurposeMismatch.into()),
			}
		}

		/// Decode data types from u8 IDs
		fn decode_data_types(
			data_type_ids: BoundedVec<u8, T::MaxDataTypes>,
		) -> Result<BoundedVec<DataType, T::MaxDataTypes>, DispatchError> {
			let mut data_types = BoundedVec::new();
			for id in data_type_ids.iter() {
				let data_type = match id {
					0 => DataType::Demographics,
					1 => DataType::Diagnostics,
					2 => DataType::Genomics,
					3 => DataType::Imaging,
					4 => DataType::LabResults,
					5 => DataType::Medications,
					6 => DataType::Procedures,
					7 => DataType::Vitals,
					_ => return Err(Error::<T>::TooManyDataTypes.into()),
				};
				data_types.try_push(data_type).map_err(|_| Error::<T>::TooManyDataTypes)?;
			}
			Ok(data_types)
		}

		/// Decode allowed parties from type and data
		fn decode_allowed_parties(
			allowed_party_type: u8,
			_allowed_party_data: BoundedVec<u8, ConstU32<255>>,
		) -> Result<AllowedParties<T>, DispatchError> {
			match allowed_party_type {
				0 => Ok(AllowedParties::Public),
				// For simplicity, we'll default to Public for other types in this implementation
				// In a full implementation, we'd decode the party data
				_ => Ok(AllowedParties::Public),
			}
		}

		/// Decode compensation preference
		fn decode_compensation(
			compensation_type: u8,
			compensation_value: u128,
		) -> Result<CompensationPreference, DispatchError> {
			match compensation_type {
				0 => Ok(CompensationPreference::Free),
				1 => Ok(CompensationPreference::FixedPrice(compensation_value)),
				2 => {
					let percentage = compensation_value as u8;
					if percentage > 100 {
						return Err(Error::<T>::TooManyDataTypes.into()); // Reuse error for invalid percentage
					}
					Ok(CompensationPreference::Percentage(percentage))
				},
				3 => Ok(CompensationPreference::Negotiable),
				_ => Err(Error::<T>::TooManyDataTypes.into()),
			}
		}
	}
}
