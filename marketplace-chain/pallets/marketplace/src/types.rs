use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::BoundedVec;

// Constants for bounded vectors
pub const MAX_ORDER_NOTES_LENGTH: u32 = 512;
pub const MAX_DISPUTE_REASON_LENGTH: u32 = 1024;
pub const MAX_ORDERS_PER_BUYER: u32 = 1000;
pub const MAX_ORDERS_PER_SELLER: u32 = 1000;
pub const MAX_FULFILLMENT_DATA_LENGTH: u32 = 256;

/// Unique identifier for an order
pub type OrderId = [u8; 32];

/// Listing ID reference
pub type ListingId = [u8; 32];

/// Order notes
pub type OrderNotes = BoundedVec<u8, ConstU32<MAX_ORDER_NOTES_LENGTH>>;

/// Dispute reason
pub type DisputeReason = BoundedVec<u8, ConstU32<MAX_DISPUTE_REASON_LENGTH>>;

/// Fulfillment data (access keys, URLs, etc.)
pub type FulfillmentData = BoundedVec<u8, ConstU32<MAX_FULFILLMENT_DATA_LENGTH>>;

/// Status of an order
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum OrderStatus {
	/// Order created, payment pending
	Pending,
	/// Payment received, escrowed
	Paid,
	/// Seller is fulfilling the order
	Fulfilling,
	/// Order fulfilled, awaiting buyer confirmation
	AwaitingConfirmation,
	/// Order completed successfully
	Completed,
	/// Order cancelled before payment
	Cancelled,
	/// Order refunded after payment
	Refunded,
	/// Order disputed
	Disputed,
	/// Dispute resolved
	DisputeResolved,
}

/// Type of order
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum OrderType {
	/// One-time data purchase
	OneTimePurchase,
	/// Subscription to data
	Subscription,
	/// Pay-per-access
	PayPerAccess,
}

/// Dispute status
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum DisputeStatus {
	/// No dispute
	None,
	/// Dispute raised by buyer
	RaisedByBuyer,
	/// Dispute raised by seller
	RaisedBySeller,
	/// Under review
	UnderReview,
	/// Resolved in favor of buyer
	ResolvedForBuyer,
	/// Resolved in favor of seller
	ResolvedForSeller,
	/// Resolved with partial refund
	ResolvedPartial,
}

/// An order in the marketplace
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Order<T: crate::Config> {
	/// Unique order identifier
	pub order_id: OrderId,

	/// Buyer account
	pub buyer: T::AccountId,

	/// Seller account
	pub seller: T::AccountId,

	/// Listing being purchased
	pub listing_id: ListingId,

	/// Order type
	pub order_type: OrderType,

	/// Order status
	pub status: OrderStatus,

	/// Price paid
	pub price: u128,

	/// Escrowed amount
	pub escrowed_amount: u128,

	/// Quantity ordered
	pub quantity: u32,

	/// Fulfillment data (access credentials, etc.)
	pub fulfillment_data: Option<FulfillmentData>,

	/// Order notes from buyer
	pub notes: Option<OrderNotes>,

	/// Dispute status
	pub dispute_status: DisputeStatus,

	/// Dispute reason
	pub dispute_reason: Option<DisputeReason>,

	/// Subscription expiry (if applicable)
	pub subscription_expires_at: Option<BlockNumberFor<T>>,

	/// When order was created
	pub created_at: BlockNumberFor<T>,

	/// When order was paid
	pub paid_at: Option<BlockNumberFor<T>>,

	/// When order was fulfilled
	pub fulfilled_at: Option<BlockNumberFor<T>>,

	/// When order was completed
	pub completed_at: Option<BlockNumberFor<T>>,
}

impl<T: crate::Config> Order<T> {
	/// Check if order can be fulfilled
	pub fn can_fulfill(&self) -> bool {
		self.status == OrderStatus::Paid
	}

	/// Check if order can be confirmed
	pub fn can_confirm(&self) -> bool {
		self.status == OrderStatus::AwaitingConfirmation
	}

	/// Check if order can be cancelled
	pub fn can_cancel(&self) -> bool {
		matches!(self.status, OrderStatus::Pending | OrderStatus::Paid)
	}

	/// Check if order can be disputed
	pub fn can_dispute(&self) -> bool {
		matches!(
			self.status,
			OrderStatus::Fulfilling | OrderStatus::AwaitingConfirmation | OrderStatus::Completed
		)
	}

	/// Check if order is active (not terminal state)
	pub fn is_active(&self) -> bool {
		!matches!(
			self.status,
			OrderStatus::Completed | OrderStatus::Cancelled | OrderStatus::Refunded
		)
	}

	/// Check if escrow should be released
	pub fn should_release_escrow(&self) -> bool {
		matches!(self.status, OrderStatus::Completed | OrderStatus::DisputeResolved)
	}
}

/// Payment escrow information
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Escrow<T: crate::Config> {
	/// Order ID
	pub order_id: OrderId,

	/// Buyer
	pub buyer: T::AccountId,

	/// Seller
	pub seller: T::AccountId,

	/// Escrowed amount
	pub amount: u128,

	/// When escrow was created
	pub created_at: BlockNumberFor<T>,

	/// When escrow expires (auto-release)
	pub expires_at: BlockNumberFor<T>,
}

/// Marketplace statistics
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
pub struct MarketplaceStats {
	/// Total orders created
	pub total_orders: u64,

	/// Total completed orders
	pub completed_orders: u64,

	/// Total cancelled orders
	pub cancelled_orders: u64,

	/// Total disputed orders
	pub disputed_orders: u64,

	/// Total volume traded
	pub total_volume: u128,

	/// Total escrow amount
	pub total_escrowed: u128,
}
