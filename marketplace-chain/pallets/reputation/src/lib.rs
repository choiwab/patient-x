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

		/// Maximum reviews per provider
		#[pallet::constant]
		type MaxReviewsPerProvider: Get<u32>;

		/// Maximum endorsements per provider
		#[pallet::constant]
		type MaxEndorsementsPerProvider: Get<u32>;
	}

	/// Storage for all reviews
	#[pallet::storage]
	#[pallet::getter(fn reviews)]
	pub type Reviews<T: Config> = StorageMap<_, Blake2_128Concat, ReviewId, Review<T>>;

	/// Reviews by provider
	#[pallet::storage]
	#[pallet::getter(fn provider_reviews)]
	pub type ProviderReviews<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<ReviewId, ConstU32<MAX_REVIEWS_PER_PROVIDER>>,
		ValueQuery,
	>;

	/// Reviews by reviewer
	#[pallet::storage]
	#[pallet::getter(fn reviewer_reviews)]
	pub type ReviewerReviews<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<ReviewId, ConstU32<MAX_REVIEWS_PER_PROVIDER>>,
		ValueQuery,
	>;

	/// Reputation scores
	#[pallet::storage]
	#[pallet::getter(fn reputation_scores)]
	pub type ReputationScores<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, ReputationScore, ValueQuery>;

	/// Endorsements
	#[pallet::storage]
	#[pallet::getter(fn endorsements)]
	pub type Endorsements<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId, // Endorsed provider
		Blake2_128Concat,
		T::AccountId, // Endorser
		Endorsement<T>,
	>;

	/// Provider badges
	#[pallet::storage]
	#[pallet::getter(fn provider_badges)]
	pub type ProviderBadges<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<ProviderBadge<T>, ConstU32<10>>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Review submitted
		ReviewSubmitted {
			review_id: ReviewId,
			reviewer: T::AccountId,
			provider: T::AccountId,
			rating: u8,
		},
		/// Provider endorsed
		ProviderEndorsed {
			endorser: T::AccountId,
			endorsed: T::AccountId,
		},
		/// Endorsement revoked
		EndorsementRevoked {
			endorser: T::AccountId,
			endorsed: T::AccountId,
		},
		/// Badge awarded
		BadgeAwarded {
			provider: T::AccountId,
			badge: u8,
		},
		/// Reputation updated
		ReputationUpdated {
			provider: T::AccountId,
			trust_score: u32,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Review already exists
		ReviewAlreadyExists,
		/// Review not found
		ReviewNotFound,
		/// Invalid rating (must be 1-5)
		InvalidRating,
		/// Already reviewed this order
		AlreadyReviewed,
		/// Cannot review own order
		CannotReviewSelf,
		/// Too many reviews for provider
		TooManyReviews,
		/// Endorsement already exists
		EndorsementAlreadyExists,
		/// Endorsement not found
		EndorsementNotFound,
		/// Cannot endorse self
		CannotEndorseSelf,
		/// Too many endorsements
		TooManyEndorsements,
		/// Invalid badge
		InvalidBadge,
		/// Badge already awarded
		BadgeAlreadyAwarded,
		/// Not authorized
		NotAuthorized,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Submit a review for a completed order
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::submit_review())]
		pub fn submit_review(
			origin: OriginFor<T>,
			review_id: ReviewId,
			provider: T::AccountId,
			order_id: OrderId,
			rating: u8,
			text: Option<BoundedVec<u8, ConstU32<MAX_REVIEW_TEXT_LENGTH>>>,
		) -> DispatchResult {
			let reviewer = ensure_signed(origin)?;

			// Ensure review doesn't already exist
			ensure!(!Reviews::<T>::contains_key(review_id), Error::<T>::ReviewAlreadyExists);

			// Validate rating
			ensure!(rating >= 1 && rating <= 5, Error::<T>::InvalidRating);

			// Ensure not reviewing self
			ensure!(reviewer != provider, Error::<T>::CannotReviewSelf);

			// Check review limit for provider
			let mut provider_reviews = ProviderReviews::<T>::get(&provider);
			ensure!(
				provider_reviews.len() < MAX_REVIEWS_PER_PROVIDER as usize,
				Error::<T>::TooManyReviews
			);

			// Check if already reviewed this order
			let reviewer_reviews = ReviewerReviews::<T>::get(&reviewer);
			for review_id in reviewer_reviews.iter() {
				if let Some(review) = Reviews::<T>::get(review_id) {
					if review.order_id == order_id {
						return Err(Error::<T>::AlreadyReviewed.into())
					}
				}
			}

			let current_block = frame_system::Pallet::<T>::block_number();

			let review = Review {
				review_id,
				reviewer: reviewer.clone(),
				provider: provider.clone(),
				order_id,
				rating,
				text,
				created_at: current_block,
			};

			// Store review
			Reviews::<T>::insert(review_id, review);

			// Update indices
			provider_reviews
				.try_push(review_id)
				.map_err(|_| Error::<T>::TooManyReviews)?;
			ProviderReviews::<T>::insert(&provider, provider_reviews);

			ReviewerReviews::<T>::mutate(&reviewer, |reviews| {
				let _ = reviews.try_push(review_id);
			});

			// Update reputation score
			ReputationScores::<T>::mutate(&provider, |score| {
				score.add_review(rating);
			});

			let trust_score = ReputationScores::<T>::get(&provider).trust_score;

			Self::deposit_event(Event::ReviewSubmitted {
				review_id,
				reviewer,
				provider: provider.clone(),
				rating,
			});

			Self::deposit_event(Event::ReputationUpdated { provider, trust_score });

			Ok(())
		}

		/// Endorse a provider
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::endorse_provider())]
		pub fn endorse_provider(
			origin: OriginFor<T>,
			provider: T::AccountId,
			weight: u8,
		) -> DispatchResult {
			let endorser = ensure_signed(origin)?;

			// Ensure not endorsing self
			ensure!(endorser != provider, Error::<T>::CannotEndorseSelf);

			// Ensure endorsement doesn't already exist
			ensure!(
				!Endorsements::<T>::contains_key(&provider, &endorser),
				Error::<T>::EndorsementAlreadyExists
			);

			// Validate weight (1-10)
			let weight = weight.min(10).max(1);

			let current_block = frame_system::Pallet::<T>::block_number();

			let endorsement = Endorsement {
				endorser: endorser.clone(),
				endorsed: provider.clone(),
				weight,
				created_at: current_block,
			};

			// Store endorsement
			Endorsements::<T>::insert(&provider, &endorser, endorsement);

			// Update reputation score
			ReputationScores::<T>::mutate(&provider, |score| {
				score.add_endorsement();
			});

			Self::deposit_event(Event::ProviderEndorsed { endorser, endorsed: provider });

			Ok(())
		}

		/// Revoke an endorsement
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::revoke_endorsement())]
		pub fn revoke_endorsement(
			origin: OriginFor<T>,
			provider: T::AccountId,
		) -> DispatchResult {
			let endorser = ensure_signed(origin)?;

			// Ensure endorsement exists
			ensure!(
				Endorsements::<T>::contains_key(&provider, &endorser),
				Error::<T>::EndorsementNotFound
			);

			// Remove endorsement
			Endorsements::<T>::remove(&provider, &endorser);

			// Update reputation score
			ReputationScores::<T>::mutate(&provider, |score| {
				score.endorsements = score.endorsements.saturating_sub(1);
				score.calculate_trust_score();
			});

			Self::deposit_event(Event::EndorsementRevoked { endorser, endorsed: provider });

			Ok(())
		}

		/// Award a badge to a provider (admin/governance function)
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::award_badge())]
		pub fn award_badge(
			origin: OriginFor<T>,
			provider: T::AccountId,
			badge: u8,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// In production, this would check admin/governance rights

			// Convert badge
			let badge_enum = Self::u8_to_badge(badge)?;

			let current_block = frame_system::Pallet::<T>::block_number();

			let provider_badge = ProviderBadge { badge: badge_enum, awarded_at: current_block };

			// Check if badge already awarded
			let badges = ProviderBadges::<T>::get(&provider);
			for existing_badge in badges.iter() {
				if existing_badge.badge == provider_badge.badge {
					return Err(Error::<T>::BadgeAlreadyAwarded.into())
				}
			}

			// Award badge
			ProviderBadges::<T>::mutate(&provider, |badges| {
				let _ = badges.try_push(provider_badge);
			});

			Self::deposit_event(Event::BadgeAwarded { provider, badge });

			Ok(())
		}

		/// Update reputation for completed/disputed order
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::update_reputation())]
		pub fn update_reputation_for_order(
			origin: OriginFor<T>,
			provider: T::AccountId,
			completed: bool,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// In production, this would be called by marketplace pallet

			ReputationScores::<T>::mutate(&provider, |score| {
				if completed {
					score.increment_completed_orders();
				} else {
					score.increment_disputed_orders();
				}
			});

			let trust_score = ReputationScores::<T>::get(&provider).trust_score;

			Self::deposit_event(Event::ReputationUpdated { provider, trust_score });

			Ok(())
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		/// Convert u8 to Badge
		fn u8_to_badge(value: u8) -> Result<Badge, Error<T>> {
			match value {
				0 => Ok(Badge::VerifiedProvider),
				1 => Ok(Badge::TrustedResearcher),
				2 => Ok(Badge::TopContributor),
				3 => Ok(Badge::Pioneer),
				4 => Ok(Badge::Expert),
				_ => Err(Error::<T>::InvalidBadge),
			}
		}

		/// Get all reviews for a provider
		pub fn get_provider_reviews(provider: &T::AccountId) -> Vec<ReviewId> {
			ProviderReviews::<T>::get(provider).to_vec()
		}

		/// Get all reviews by a reviewer
		pub fn get_reviewer_reviews(reviewer: &T::AccountId) -> Vec<ReviewId> {
			ReviewerReviews::<T>::get(reviewer).to_vec()
		}

		/// Get reputation score for a provider
		pub fn get_reputation_score(provider: &T::AccountId) -> ReputationScore {
			ReputationScores::<T>::get(provider)
		}

		/// Get trust level for a provider
		pub fn get_trust_level(provider: &T::AccountId) -> TrustLevel {
			ReputationScores::<T>::get(provider).get_trust_level()
		}
	}
}
