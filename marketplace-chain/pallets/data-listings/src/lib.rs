#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod types;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::BoundedVec;
	use types::*;
	use weights::WeightInfo;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config<RuntimeEvent: From<Event<Self>>> {
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// Maximum number of listings per provider
		#[pallet::constant]
		type MaxListingsPerProvider: Get<u32>;

		/// Maximum number of listings per category
		#[pallet::constant]
		type MaxListingsPerCategory: Get<u32>;
	}

	/// Storage for all data listings
	#[pallet::storage]
	#[pallet::getter(fn listings)]
	pub type Listings<T: Config> = StorageMap<_, Blake2_128Concat, ListingId, DataListing<T>>;

	/// Listings by provider
	#[pallet::storage]
	#[pallet::getter(fn provider_listings)]
	pub type ProviderListings<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<ListingId, ConstU32<MAX_LISTINGS_PER_PROVIDER>>,
		ValueQuery,
	>;

	/// Listings by category (using u8 key to avoid DecodeWithMemTracking issues)
	#[pallet::storage]
	#[pallet::getter(fn category_listings_by_u8)]
	pub type CategoryListings<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u8,
		BoundedVec<ListingId, ConstU32<MAX_LISTINGS_PER_CATEGORY>>,
		ValueQuery,
	>;

	/// Active listings (for efficient discovery)
	#[pallet::storage]
	#[pallet::getter(fn active_listings)]
	pub type ActiveListings<T: Config> =
		StorageValue<_, BoundedVec<ListingId, ConstU32<MAX_LISTINGS_PER_CATEGORY>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new listing was created
		ListingCreated {
			listing_id: ListingId,
			provider: T::AccountId,
			category: u8,
		},
		/// A listing was updated
		ListingUpdated { listing_id: ListingId },
		/// A listing was paused
		ListingPaused { listing_id: ListingId },
		/// A listing was resumed
		ListingResumed { listing_id: ListingId },
		/// A listing was cancelled
		ListingCancelled { listing_id: ListingId },
		/// A listing was extended
		ListingExtended {
			listing_id: ListingId,
			new_expiration: BlockNumberFor<T>,
		},
		/// A listing has expired
		ListingExpired { listing_id: ListingId },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Listing already exists
		ListingAlreadyExists,
		/// Listing not found
		ListingNotFound,
		/// Not the listing owner
		NotListingOwner,
		/// Listing is not active
		ListingNotActive,
		/// Listing is already paused
		ListingAlreadyPaused,
		/// Listing has expired
		ListingExpired,
		/// Invalid status transition
		InvalidStatusTransition,
		/// Too many listings for provider
		TooManyListingsForProvider,
		/// Too many listings for category
		TooManyListingsForCategory,
		/// Invalid pricing model
		InvalidPricingModel,
		/// Invalid quantity
		InvalidQuantity,
		/// No quantity remaining
		NoQuantityRemaining,
		/// Listing cannot be cancelled
		CannotCancelListing,
		/// Invalid category
		InvalidCategory,
		/// Invalid data type
		InvalidDataType,
		/// Invalid privacy level
		InvalidPrivacyLevel,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new data listing
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_listing())]
		pub fn create_listing(
			origin: OriginFor<T>,
			listing_id: ListingId,
			name: BoundedVec<u8, ConstU32<MAX_LISTING_NAME_LENGTH>>,
			description: BoundedVec<u8, ConstU32<MAX_DESCRIPTION_LENGTH>>,
			category: u8,
			data_type: u8,
			privacy_level: u8,
			pricing_model: u8,
			price: u128,
			access_terms: BoundedVec<u8, ConstU32<MAX_ACCESS_TERMS_LENGTH>>,
			total_quantity: Option<u32>,
			expires_in_blocks: Option<BlockNumberFor<T>>,
		) -> DispatchResult {
			let provider = ensure_signed(origin)?;

			// Ensure listing doesn't already exist
			ensure!(!Listings::<T>::contains_key(listing_id), Error::<T>::ListingAlreadyExists);

			// Convert enums (keep originals for storage)
			let category_enum = Self::u8_to_category(category)?;
			let data_type_enum = Self::u8_to_data_type(data_type)?;
			let privacy_level_enum = Self::u8_to_privacy_level(privacy_level)?;
			let pricing_model_enum = Self::u8_to_pricing_model(pricing_model)?;

			// Check provider listing limit
			let mut provider_listings = ProviderListings::<T>::get(&provider);
			ensure!(
				provider_listings.len() < MAX_LISTINGS_PER_PROVIDER as usize,
				Error::<T>::TooManyListingsForProvider
			);

			// Check category listing limit
			let mut category_listings = CategoryListings::<T>::get(category);
			ensure!(
				category_listings.len() < MAX_LISTINGS_PER_CATEGORY as usize,
				Error::<T>::TooManyListingsForCategory
			);

			let current_block = frame_system::Pallet::<T>::block_number();
			let expires_at = expires_in_blocks.map(|blocks| current_block + blocks);

			let listing = DataListing {
				listing_id,
				provider: provider.clone(),
				name,
				description,
				category: category_enum,
				data_type: data_type_enum,
				privacy_level: privacy_level_enum,
				pricing_model: pricing_model_enum,
				price,
				access_terms,
				tags: BoundedVec::default(),
				status: ListingStatus::Active,
				total_quantity,
				remaining_quantity: total_quantity,
				expires_at,
				created_at: current_block,
				updated_at: current_block,
			};

			// Store listing
			Listings::<T>::insert(listing_id, listing);

			// Update indices
			provider_listings
				.try_push(listing_id)
				.map_err(|_| Error::<T>::TooManyListingsForProvider)?;
			ProviderListings::<T>::insert(&provider, provider_listings);

			category_listings
				.try_push(listing_id)
				.map_err(|_| Error::<T>::TooManyListingsForCategory)?;
			CategoryListings::<T>::insert(category, category_listings);

			// Add to active listings
			ActiveListings::<T>::mutate(|listings| {
				let _ = listings.try_push(listing_id);
			});

			Self::deposit_event(Event::ListingCreated { listing_id, provider, category });

			Ok(())
		}

		/// Update an existing listing
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::update_listing())]
		pub fn update_listing(
			origin: OriginFor<T>,
			listing_id: ListingId,
			name: Option<BoundedVec<u8, ConstU32<MAX_LISTING_NAME_LENGTH>>>,
			description: Option<BoundedVec<u8, ConstU32<MAX_DESCRIPTION_LENGTH>>>,
			price: Option<u128>,
			access_terms: Option<BoundedVec<u8, ConstU32<MAX_ACCESS_TERMS_LENGTH>>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Listings::<T>::try_mutate(listing_id, |maybe_listing| -> DispatchResult {
				let listing = maybe_listing.as_mut().ok_or(Error::<T>::ListingNotFound)?;

				// Ensure caller is the provider
				ensure!(listing.provider == who, Error::<T>::NotListingOwner);

				// Update fields
				if let Some(new_name) = name {
					listing.name = new_name;
				}
				if let Some(new_description) = description {
					listing.description = new_description;
				}
				if let Some(new_price) = price {
					listing.price = new_price;
				}
				if let Some(new_terms) = access_terms {
					listing.access_terms = new_terms;
				}

				listing.updated_at = frame_system::Pallet::<T>::block_number();

				Ok(())
			})?;

			Self::deposit_event(Event::ListingUpdated { listing_id });

			Ok(())
		}

		/// Pause a listing
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::pause_listing())]
		pub fn pause_listing(origin: OriginFor<T>, listing_id: ListingId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Listings::<T>::try_mutate(listing_id, |maybe_listing| -> DispatchResult {
				let listing = maybe_listing.as_mut().ok_or(Error::<T>::ListingNotFound)?;

				// Ensure caller is the provider
				ensure!(listing.provider == who, Error::<T>::NotListingOwner);

				// Ensure listing is active
				ensure!(listing.status == ListingStatus::Active, Error::<T>::ListingNotActive);

				listing.status = ListingStatus::Paused;
				listing.updated_at = frame_system::Pallet::<T>::block_number();

				Ok(())
			})?;

			// Remove from active listings
			ActiveListings::<T>::mutate(|listings| {
				listings.retain(|&id| id != listing_id);
			});

			Self::deposit_event(Event::ListingPaused { listing_id });

			Ok(())
		}

		/// Resume a paused listing
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::resume_listing())]
		pub fn resume_listing(origin: OriginFor<T>, listing_id: ListingId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Listings::<T>::try_mutate(listing_id, |maybe_listing| -> DispatchResult {
				let listing = maybe_listing.as_mut().ok_or(Error::<T>::ListingNotFound)?;

				// Ensure caller is the provider
				ensure!(listing.provider == who, Error::<T>::NotListingOwner);

				// Ensure listing is paused
				ensure!(listing.status == ListingStatus::Paused, Error::<T>::ListingAlreadyPaused);

				listing.status = ListingStatus::Active;
				listing.updated_at = frame_system::Pallet::<T>::block_number();

				Ok(())
			})?;

			// Add back to active listings
			ActiveListings::<T>::mutate(|listings| {
				let _ = listings.try_push(listing_id);
			});

			Self::deposit_event(Event::ListingResumed { listing_id });

			Ok(())
		}

		/// Cancel a listing
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::cancel_listing())]
		pub fn cancel_listing(origin: OriginFor<T>, listing_id: ListingId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Listings::<T>::try_mutate(listing_id, |maybe_listing| -> DispatchResult {
				let listing = maybe_listing.as_mut().ok_or(Error::<T>::ListingNotFound)?;

				// Ensure caller is the provider
				ensure!(listing.provider == who, Error::<T>::NotListingOwner);

				// Can only cancel Active or Paused listings
				ensure!(
					listing.status == ListingStatus::Active ||
						listing.status == ListingStatus::Paused,
					Error::<T>::CannotCancelListing
				);

				listing.status = ListingStatus::Cancelled;
				listing.updated_at = frame_system::Pallet::<T>::block_number();

				Ok(())
			})?;

			// Remove from active listings
			ActiveListings::<T>::mutate(|listings| {
				listings.retain(|&id| id != listing_id);
			});

			Self::deposit_event(Event::ListingCancelled { listing_id });

			Ok(())
		}

		/// Extend listing expiration
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::extend_listing())]
		pub fn extend_listing(
			origin: OriginFor<T>,
			listing_id: ListingId,
			additional_blocks: BlockNumberFor<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Listings::<T>::try_mutate(listing_id, |maybe_listing| -> DispatchResult {
				let listing = maybe_listing.as_mut().ok_or(Error::<T>::ListingNotFound)?;

				// Ensure caller is the provider
				ensure!(listing.provider == who, Error::<T>::NotListingOwner);

				let current_block = frame_system::Pallet::<T>::block_number();
				let new_expiration = if let Some(current_expiration) = listing.expires_at {
					// If already has expiration, extend from current expiration
					current_expiration + additional_blocks
				} else {
					// If no expiration, set from now
					current_block + additional_blocks
				};

				listing.expires_at = Some(new_expiration);
				listing.updated_at = current_block;

				Self::deposit_event(Event::ListingExtended { listing_id, new_expiration });

				Ok(())
			})?;

			Ok(())
		}

		/// Search listings by category (simple implementation)
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::search_by_category())]
		pub fn search_by_category(
			origin: OriginFor<T>,
			category: u8,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Verify category is valid
			let _category_enum = Self::u8_to_category(category)?;
			let _listings = CategoryListings::<T>::get(category);

			// In a real implementation, this would return results
			// For now, just verify the query is valid

			Ok(())
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		/// Convert u8 to DataCategory
		fn u8_to_category(value: u8) -> Result<DataCategory, Error<T>> {
			match value {
				0 => Ok(DataCategory::Genomic),
				1 => Ok(DataCategory::Clinical),
				2 => Ok(DataCategory::Imaging),
				3 => Ok(DataCategory::Laboratory),
				4 => Ok(DataCategory::Wearable),
				5 => Ok(DataCategory::PatientReported),
				6 => Ok(DataCategory::Pharmaceutical),
				7 => Ok(DataCategory::NegativeOutcomes),
				8 => Ok(DataCategory::Research),
				9 => Ok(DataCategory::Other),
				_ => Err(Error::<T>::InvalidCategory),
			}
		}

		/// Convert u8 to DataType
		fn u8_to_data_type(value: u8) -> Result<DataType, Error<T>> {
			match value {
				0 => Ok(DataType::Raw),
				1 => Ok(DataType::Anonymized),
				2 => Ok(DataType::Aggregated),
				3 => Ok(DataType::Derived),
				4 => Ok(DataType::Synthetic),
				_ => Err(Error::<T>::InvalidDataType),
			}
		}

		/// Convert u8 to PrivacyLevel
		fn u8_to_privacy_level(value: u8) -> Result<PrivacyLevel, Error<T>> {
			match value {
				0 => Ok(PrivacyLevel::Identifiable),
				1 => Ok(PrivacyLevel::Pseudonymized),
				2 => Ok(PrivacyLevel::Anonymized),
				3 => Ok(PrivacyLevel::Aggregated),
				_ => Err(Error::<T>::InvalidPrivacyLevel),
			}
		}

		/// Convert u8 to PricingModel
		fn u8_to_pricing_model(value: u8) -> Result<PricingModel, Error<T>> {
			match value {
				0 => Ok(PricingModel::Fixed),
				1 => Ok(PricingModel::Subscription),
				2 => Ok(PricingModel::PayPerAccess),
				3 => Ok(PricingModel::Free),
				_ => Err(Error::<T>::InvalidPricingModel),
			}
		}

		/// Get all active listings
		pub fn get_active_listings() -> Vec<ListingId> {
			ActiveListings::<T>::get().to_vec()
		}

		/// Get listings by provider
		pub fn get_provider_listings(provider: &T::AccountId) -> Vec<ListingId> {
			ProviderListings::<T>::get(provider).to_vec()
		}

		/// Get listings by category (pass category as u8: 0-9)
		pub fn get_category_listings(category: u8) -> Vec<ListingId> {
			CategoryListings::<T>::get(category).to_vec()
		}

		/// Check if listing is expired and update status if needed
		pub fn check_and_expire_listing(listing_id: ListingId) -> DispatchResult {
			Listings::<T>::try_mutate(listing_id, |maybe_listing| -> DispatchResult {
				let listing = maybe_listing.as_mut().ok_or(Error::<T>::ListingNotFound)?;

				let current_block = frame_system::Pallet::<T>::block_number();
				if listing.is_expired(current_block) &&
					listing.status == ListingStatus::Active
				{
					listing.status = ListingStatus::Expired;
					listing.updated_at = current_block;

					// Remove from active listings
					ActiveListings::<T>::mutate(|listings| {
						listings.retain(|&id| id != listing_id);
					});

					Self::deposit_event(Event::ListingExpired { listing_id });
				}

				Ok(())
			})
		}

		/// Decrement listing quantity (called when purchased)
		pub fn decrement_listing_quantity(listing_id: ListingId) -> DispatchResult {
			Listings::<T>::try_mutate(listing_id, |maybe_listing| -> DispatchResult {
				let listing = maybe_listing.as_mut().ok_or(Error::<T>::ListingNotFound)?;

				listing
					.decrement_quantity()
					.map_err(|_| Error::<T>::NoQuantityRemaining)?;

				// If quantity reaches 0, mark as fulfilled
				if let Some(remaining) = listing.remaining_quantity {
					if remaining == 0 {
						listing.status = ListingStatus::Fulfilled;

						// Remove from active listings
						ActiveListings::<T>::mutate(|listings| {
							listings.retain(|&id| id != listing_id);
						});
					}
				}

				listing.updated_at = frame_system::Pallet::<T>::block_number();

				Ok(())
			})
		}
	}
}
