use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::BoundedVec;

// Constants for bounded vectors
pub const MAX_LISTING_NAME_LENGTH: u32 = 128;
pub const MAX_DESCRIPTION_LENGTH: u32 = 1024;
pub const MAX_TAGS_PER_LISTING: u32 = 10;
pub const MAX_TAG_LENGTH: u32 = 32;
pub const MAX_LISTINGS_PER_PROVIDER: u32 = 100;
pub const MAX_LISTINGS_PER_CATEGORY: u32 = 10000;
pub const MAX_ACCESS_TERMS_LENGTH: u32 = 512;

/// Unique identifier for a data listing
pub type ListingId = [u8; 32];

/// Listing name
pub type ListingName = BoundedVec<u8, ConstU32<MAX_LISTING_NAME_LENGTH>>;

/// Listing description
pub type Description = BoundedVec<u8, ConstU32<MAX_DESCRIPTION_LENGTH>>;

/// Tag for categorization
pub type Tag = BoundedVec<u8, ConstU32<MAX_TAG_LENGTH>>;

/// Access terms text
pub type AccessTerms = BoundedVec<u8, ConstU32<MAX_ACCESS_TERMS_LENGTH>>;

/// Status of a data listing
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum ListingStatus {
	/// Listing is active and available for purchase
	Active,
	/// Listing is temporarily paused by provider
	Paused,
	/// Listing has expired based on expiration time
	Expired,
	/// Listing has been fulfilled (all units sold)
	Fulfilled,
	/// Listing has been cancelled by provider
	Cancelled,
}

/// Pricing model for data access
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum PricingModel {
	/// One-time fixed price
	Fixed,
	/// Recurring subscription (price per time period in blocks)
	Subscription,
	/// Pay per access/query
	PayPerAccess,
	/// Free (no payment required)
	Free,
}

/// Category of health data
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum DataCategory {
	/// Genomic and genetic data
	Genomic,
	/// Clinical records and medical history
	Clinical,
	/// Diagnostic imaging (X-rays, MRI, CT scans)
	Imaging,
	/// Laboratory test results
	Laboratory,
	/// Wearable device data (fitness, vitals)
	Wearable,
	/// Patient-reported outcomes
	PatientReported,
	/// Pharmaceutical and medication data
	Pharmaceutical,
	/// Negative health outcomes (adverse events)
	NegativeOutcomes,
	/// Research and trial data
	Research,
	/// Other/custom category
	Other,
}

/// Type of data being offered
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum DataType {
	/// Raw data records
	Raw,
	/// Anonymized/de-identified data
	Anonymized,
	/// Aggregated statistics
	Aggregated,
	/// Derived/processed data
	Derived,
	/// Synthetic data
	Synthetic,
}

/// Privacy level of the data
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum PrivacyLevel {
	/// Fully identifiable data
	Identifiable,
	/// Pseudonymized (identifiers replaced with pseudonyms)
	Pseudonymized,
	/// Anonymized (cannot be re-identified)
	Anonymized,
	/// Aggregated (statistical summaries only)
	Aggregated,
}

/// A data listing in the marketplace
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct DataListing<T: crate::Config> {
	/// Unique listing identifier
	pub listing_id: ListingId,

	/// Provider of the data
	pub provider: T::AccountId,

	/// Name/title of the listing
	pub name: ListingName,

	/// Detailed description
	pub description: Description,

	/// Category of data
	pub category: DataCategory,

	/// Type of data
	pub data_type: DataType,

	/// Privacy level
	pub privacy_level: PrivacyLevel,

	/// Pricing model
	pub pricing_model: PricingModel,

	/// Price (meaning depends on pricing model)
	pub price: u128,

	/// Access terms and conditions
	pub access_terms: AccessTerms,

	/// Tags for search/discovery
	pub tags: BoundedVec<Tag, ConstU32<MAX_TAGS_PER_LISTING>>,

	/// Current status
	pub status: ListingStatus,

	/// Total quantity available (None = unlimited)
	pub total_quantity: Option<u32>,

	/// Quantity remaining (None = unlimited)
	pub remaining_quantity: Option<u32>,

	/// Expiration block number (None = no expiration)
	pub expires_at: Option<BlockNumberFor<T>>,

	/// Block when listing was created
	pub created_at: BlockNumberFor<T>,

	/// Block when listing was last updated
	pub updated_at: BlockNumberFor<T>,
}

impl<T: crate::Config> DataListing<T> {
	/// Check if listing is available for purchase
	pub fn is_available(&self) -> bool {
		self.status == ListingStatus::Active
	}

	/// Check if listing has expired
	pub fn is_expired(&self, current_block: BlockNumberFor<T>) -> bool {
		if let Some(expires_at) = self.expires_at {
			current_block >= expires_at
		} else {
			false
		}
	}

	/// Check if listing has remaining quantity
	pub fn has_quantity(&self) -> bool {
		if let Some(remaining) = self.remaining_quantity {
			remaining > 0
		} else {
			true // Unlimited quantity
		}
	}

	/// Decrement remaining quantity
	pub fn decrement_quantity(&mut self) -> Result<(), &'static str> {
		if let Some(ref mut remaining) = self.remaining_quantity {
			if *remaining > 0 {
				*remaining -= 1;
				Ok(())
			} else {
				Err("No quantity remaining")
			}
		} else {
			Ok(()) // Unlimited quantity
		}
	}

	/// Check if listing can be purchased
	pub fn can_purchase(&self, current_block: BlockNumberFor<T>) -> bool {
		self.is_available() && !self.is_expired(current_block) && self.has_quantity()
	}
}

/// Search criteria for listings
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct SearchCriteria {
	/// Filter by category
	pub category: Option<DataCategory>,

	/// Filter by data type
	pub data_type: Option<DataType>,

	/// Filter by privacy level
	pub privacy_level: Option<PrivacyLevel>,

	/// Filter by pricing model
	pub pricing_model: Option<PricingModel>,

	/// Maximum price
	pub max_price: Option<u128>,

	/// Minimum price
	pub min_price: Option<u128>,
}
