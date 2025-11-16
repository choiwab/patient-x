use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::BoundedVec;

// Constants for bounded vectors
pub const MAX_REVIEW_TEXT_LENGTH: u32 = 1024;
pub const MAX_REVIEWS_PER_PROVIDER: u32 = 1000;
pub const MAX_ENDORSEMENTS_PER_PROVIDER: u32 = 100;

/// Review text
pub type ReviewText = BoundedVec<u8, ConstU32<MAX_REVIEW_TEXT_LENGTH>>;

/// Rating value (1-5 stars)
pub type Rating = u8;

/// Order ID reference
pub type OrderId = [u8; 32];

/// Review ID
pub type ReviewId = [u8; 32];

/// A review of a data provider
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Review<T: crate::Config> {
	/// Review ID
	pub review_id: ReviewId,

	/// Reviewer (buyer)
	pub reviewer: T::AccountId,

	/// Provider being reviewed
	pub provider: T::AccountId,

	/// Associated order
	pub order_id: OrderId,

	/// Rating (1-5)
	pub rating: Rating,

	/// Review text
	pub text: Option<ReviewText>,

	/// When review was created
	pub created_at: BlockNumberFor<T>,
}

/// Reputation score for a provider
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
pub struct ReputationScore {
	/// Total number of reviews
	pub total_reviews: u32,

	/// Sum of all ratings
	pub total_rating: u32,

	/// Average rating (calculated: total_rating / total_reviews)
	pub average_rating: u32, // Stored as rating * 100 for precision

	/// Number of 5-star reviews
	pub five_star_count: u32,

	/// Number of 4-star reviews
	pub four_star_count: u32,

	/// Number of 3-star reviews
	pub three_star_count: u32,

	/// Number of 2-star reviews
	pub two_star_count: u32,

	/// Number of 1-star reviews
	pub one_star_count: u32,

	/// Total completed orders
	pub completed_orders: u32,

	/// Total disputed orders
	pub disputed_orders: u32,

	/// Total successful outcomes
	pub successful_outcomes: u32,

	/// Total endorsements received
	pub endorsements: u32,

	/// Trust score (0-1000, calculated from multiple factors)
	pub trust_score: u32,
}

impl ReputationScore {
	/// Add a review to the score
	pub fn add_review(&mut self, rating: Rating) {
		self.total_reviews += 1;
		self.total_rating += rating as u32;

		// Update rating distribution
		match rating {
			5 => self.five_star_count += 1,
			4 => self.four_star_count += 1,
			3 => self.three_star_count += 1,
			2 => self.two_star_count += 1,
			1 => self.one_star_count += 1,
			_ => {},
		}

		// Recalculate average (stored as rating * 100)
		if self.total_reviews > 0 {
			self.average_rating = (self.total_rating * 100) / self.total_reviews;
		}

		// Recalculate trust score
		self.calculate_trust_score();
	}

	/// Calculate trust score based on multiple factors
	pub fn calculate_trust_score(&mut self) {
		// Trust score calculation:
		// - 60% weight: average rating (0-500 points, normalized from 1-5 stars)
		// - 20% weight: completion rate (0-200 points)
		// - 10% weight: volume (0-100 points, capped at 100 reviews)
		// - 10% weight: endorsements (0-100 points, capped at 50 endorsements)

		let mut score: u32 = 0;

		// Average rating component (60% = 600 points)
		if self.total_reviews > 0 {
			// Map 1-5 stars to 0-600
			score += ((self.average_rating - 100) * 600) / 400;
		}

		// Completion rate component (20% = 200 points)
		let total_orders = self.completed_orders + self.disputed_orders;
		if total_orders > 0 {
			let completion_rate = (self.completed_orders * 100) / total_orders;
			score += (completion_rate * 200) / 100;
		}

		// Volume component (10% = 100 points)
		let review_volume = self.total_reviews.min(100);
		score += review_volume;

		// Endorsements component (10% = 100 points)
		let endorsement_score = (self.endorsements.min(50) * 100) / 50;
		score += endorsement_score;

		self.trust_score = score.min(1000);
	}

	/// Get trust level based on trust score
	pub fn get_trust_level(&self) -> TrustLevel {
		match self.trust_score {
			900..=1000 => TrustLevel::Excellent,
			750..=899 => TrustLevel::VeryGood,
			600..=749 => TrustLevel::Good,
			450..=599 => TrustLevel::Fair,
			300..=449 => TrustLevel::Poor,
			_ => TrustLevel::VeryPoor,
		}
	}

	/// Increment completed orders
	pub fn increment_completed_orders(&mut self) {
		self.completed_orders += 1;
		self.calculate_trust_score();
	}

	/// Increment disputed orders
	pub fn increment_disputed_orders(&mut self) {
		self.disputed_orders += 1;
		self.calculate_trust_score();
	}

	/// Add endorsement
	pub fn add_endorsement(&mut self) {
		self.endorsements += 1;
		self.calculate_trust_score();
	}
}

/// Trust level categories
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum TrustLevel {
	/// 900-1000: Excellent reputation
	Excellent,
	/// 750-899: Very good reputation
	VeryGood,
	/// 600-749: Good reputation
	Good,
	/// 450-599: Fair reputation
	Fair,
	/// 300-449: Poor reputation
	Poor,
	/// 0-299: Very poor reputation
	VeryPoor,
}

/// An endorsement from another provider or authority
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Endorsement<T: crate::Config> {
	/// Endorser account
	pub endorser: T::AccountId,

	/// Endorsed provider
	pub endorsed: T::AccountId,

	/// Endorsement weight (1-10, authorities can have higher weight)
	pub weight: u8,

	/// When endorsement was given
	pub created_at: BlockNumberFor<T>,
}

/// Reputation badge
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Badge {
	/// Verified healthcare provider
	VerifiedProvider,
	/// Trusted researcher
	TrustedResearcher,
	/// Top contributor (100+ verified outcomes)
	TopContributor,
	/// Pioneer (early adopter)
	Pioneer,
	/// Expert (500+ completed orders)
	Expert,
}

/// Provider badge record
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct ProviderBadge<T: crate::Config> {
	/// Badge type
	pub badge: Badge,

	/// When badge was awarded
	pub awarded_at: BlockNumberFor<T>,
}
