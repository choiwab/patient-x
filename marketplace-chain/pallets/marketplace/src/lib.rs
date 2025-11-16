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

		/// Maximum orders per buyer
		#[pallet::constant]
		type MaxOrdersPerBuyer: Get<u32>;

		/// Maximum orders per seller
		#[pallet::constant]
		type MaxOrdersPerSeller: Get<u32>;

		/// Escrow timeout in blocks
		#[pallet::constant]
		type EscrowTimeout: Get<BlockNumberFor<Self>>;
	}

	/// Storage for all orders
	#[pallet::storage]
	#[pallet::getter(fn orders)]
	pub type Orders<T: Config> = StorageMap<_, Blake2_128Concat, OrderId, Order<T>>;

	/// Orders by buyer
	#[pallet::storage]
	#[pallet::getter(fn buyer_orders)]
	pub type BuyerOrders<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<OrderId, ConstU32<MAX_ORDERS_PER_BUYER>>,
		ValueQuery,
	>;

	/// Orders by seller
	#[pallet::storage]
	#[pallet::getter(fn seller_orders)]
	pub type SellerOrders<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<OrderId, ConstU32<MAX_ORDERS_PER_SELLER>>,
		ValueQuery,
	>;

	/// Active orders (pending, paid, fulfilling)
	#[pallet::storage]
	#[pallet::getter(fn active_orders)]
	pub type ActiveOrders<T: Config> =
		StorageValue<_, BoundedVec<OrderId, ConstU32<MAX_ORDERS_PER_BUYER>>, ValueQuery>;

	/// Escrow information
	#[pallet::storage]
	#[pallet::getter(fn escrows)]
	pub type Escrows<T: Config> = StorageMap<_, Blake2_128Concat, OrderId, Escrow<T>>;

	/// Disputed orders
	#[pallet::storage]
	#[pallet::getter(fn disputed_orders)]
	pub type DisputedOrders<T: Config> =
		StorageValue<_, BoundedVec<OrderId, ConstU32<MAX_ORDERS_PER_BUYER>>, ValueQuery>;

	/// Marketplace statistics
	#[pallet::storage]
	#[pallet::getter(fn marketplace_stats)]
	pub type MarketplaceStatistics<T: Config> = StorageValue<_, MarketplaceStats, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Order created
		OrderCreated {
			order_id: OrderId,
			buyer: T::AccountId,
			seller: T::AccountId,
			listing_id: ListingId,
			price: u128,
		},
		/// Order paid
		OrderPaid {
			order_id: OrderId,
			amount: u128,
		},
		/// Order fulfilled
		OrderFulfilled { order_id: OrderId },
		/// Order confirmed
		OrderConfirmed { order_id: OrderId },
		/// Order cancelled
		OrderCancelled { order_id: OrderId },
		/// Dispute raised
		DisputeRaised {
			order_id: OrderId,
			raised_by: T::AccountId,
		},
		/// Dispute resolved
		DisputeResolved {
			order_id: OrderId,
			resolution: u8,
		},
		/// Escrow released
		EscrowReleased {
			order_id: OrderId,
			to: T::AccountId,
			amount: u128,
		},
		/// Refund issued
		RefundIssued {
			order_id: OrderId,
			to: T::AccountId,
			amount: u128,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Order already exists
		OrderAlreadyExists,
		/// Order not found
		OrderNotFound,
		/// Not the buyer
		NotBuyer,
		/// Not the seller
		NotSeller,
		/// Invalid order status
		InvalidOrderStatus,
		/// Cannot fulfill order
		CannotFulfillOrder,
		/// Cannot confirm order
		CannotConfirmOrder,
		/// Cannot cancel order
		CannotCancelOrder,
		/// Cannot dispute order
		CannotDisputeOrder,
		/// Too many orders for buyer
		TooManyOrdersForBuyer,
		/// Too many orders for seller
		TooManyOrdersForSeller,
		/// Escrow not found
		EscrowNotFound,
		/// Insufficient balance
		InsufficientBalance,
		/// Invalid order type
		InvalidOrderType,
		/// Invalid dispute status
		InvalidDisputeStatus,
		/// Dispute already raised
		DisputeAlreadyRaised,
		/// No active dispute
		NoActiveDispute,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new order
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_order())]
		pub fn create_order(
			origin: OriginFor<T>,
			order_id: OrderId,
			seller: T::AccountId,
			listing_id: ListingId,
			order_type: u8,
			price: u128,
			quantity: u32,
			notes: Option<BoundedVec<u8, ConstU32<MAX_ORDER_NOTES_LENGTH>>>,
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?;

			// Ensure order doesn't already exist
			ensure!(!Orders::<T>::contains_key(order_id), Error::<T>::OrderAlreadyExists);

			// Check buyer order limit
			let mut buyer_orders = BuyerOrders::<T>::get(&buyer);
			ensure!(
				buyer_orders.len() < MAX_ORDERS_PER_BUYER as usize,
				Error::<T>::TooManyOrdersForBuyer
			);

			// Check seller order limit
			let mut seller_orders = SellerOrders::<T>::get(&seller);
			ensure!(
				seller_orders.len() < MAX_ORDERS_PER_SELLER as usize,
				Error::<T>::TooManyOrdersForSeller
			);

			// Convert order type
			let order_type_enum = Self::u8_to_order_type(order_type)?;

			let current_block = frame_system::Pallet::<T>::block_number();

			let order = Order {
				order_id,
				buyer: buyer.clone(),
				seller: seller.clone(),
				listing_id,
				order_type: order_type_enum,
				status: OrderStatus::Pending,
				price,
				escrowed_amount: 0,
				quantity,
				fulfillment_data: None,
				notes,
				dispute_status: DisputeStatus::None,
				dispute_reason: None,
				subscription_expires_at: None,
				created_at: current_block,
				paid_at: None,
				fulfilled_at: None,
				completed_at: None,
			};

			// Store order
			Orders::<T>::insert(order_id, order);

			// Update indices
			buyer_orders
				.try_push(order_id)
				.map_err(|_| Error::<T>::TooManyOrdersForBuyer)?;
			BuyerOrders::<T>::insert(&buyer, buyer_orders);

			seller_orders
				.try_push(order_id)
				.map_err(|_| Error::<T>::TooManyOrdersForSeller)?;
			SellerOrders::<T>::insert(&seller, seller_orders);

			// Add to active orders
			ActiveOrders::<T>::mutate(|orders| {
				let _ = orders.try_push(order_id);
			});

			// Update statistics
			MarketplaceStatistics::<T>::mutate(|stats| {
				stats.total_orders += 1;
			});

			Self::deposit_event(Event::OrderCreated { order_id, buyer, seller, listing_id, price });

			Ok(())
		}

		/// Pay for an order (escrow payment)
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::pay_order())]
		pub fn pay_order(origin: OriginFor<T>, order_id: OrderId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Orders::<T>::try_mutate(order_id, |maybe_order| -> DispatchResult {
				let order = maybe_order.as_mut().ok_or(Error::<T>::OrderNotFound)?;

				// Ensure caller is the buyer
				ensure!(order.buyer == who, Error::<T>::NotBuyer);

				// Ensure order is pending
				ensure!(order.status == OrderStatus::Pending, Error::<T>::InvalidOrderStatus);

				// In production, this would transfer funds from buyer to escrow account
				// For now, just update the order status and escrow amount

				let current_block = frame_system::Pallet::<T>::block_number();
				let escrow_timeout = T::EscrowTimeout::get();
				let expires_at = current_block + escrow_timeout;

				order.status = OrderStatus::Paid;
				order.escrowed_amount = order.price;
				order.paid_at = Some(current_block);

				// Create escrow
				let escrow = Escrow {
					order_id,
					buyer: order.buyer.clone(),
					seller: order.seller.clone(),
					amount: order.price,
					created_at: current_block,
					expires_at,
				};

				Escrows::<T>::insert(order_id, escrow);

				// Update statistics
				MarketplaceStatistics::<T>::mutate(|stats| {
					stats.total_escrowed += order.price;
					stats.total_volume += order.price;
				});

				Self::deposit_event(Event::OrderPaid { order_id, amount: order.price });

				Ok(())
			})
		}

		/// Fulfill an order (seller provides data/access)
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::fulfill_order())]
		pub fn fulfill_order(
			origin: OriginFor<T>,
			order_id: OrderId,
			fulfillment_data: BoundedVec<u8, ConstU32<MAX_FULFILLMENT_DATA_LENGTH>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Orders::<T>::try_mutate(order_id, |maybe_order| -> DispatchResult {
				let order = maybe_order.as_mut().ok_or(Error::<T>::OrderNotFound)?;

				// Ensure caller is the seller
				ensure!(order.seller == who, Error::<T>::NotSeller);

				// Ensure order can be fulfilled
				ensure!(order.can_fulfill(), Error::<T>::CannotFulfillOrder);

				let current_block = frame_system::Pallet::<T>::block_number();

				order.status = OrderStatus::AwaitingConfirmation;
				order.fulfillment_data = Some(fulfillment_data);
				order.fulfilled_at = Some(current_block);

				Self::deposit_event(Event::OrderFulfilled { order_id });

				Ok(())
			})
		}

		/// Confirm order receipt (buyer)
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::confirm_order())]
		pub fn confirm_order(origin: OriginFor<T>, order_id: OrderId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Orders::<T>::try_mutate(order_id, |maybe_order| -> DispatchResult {
				let order = maybe_order.as_mut().ok_or(Error::<T>::OrderNotFound)?;

				// Ensure caller is the buyer
				ensure!(order.buyer == who, Error::<T>::NotBuyer);

				// Ensure order can be confirmed
				ensure!(order.can_confirm(), Error::<T>::CannotConfirmOrder);

				let current_block = frame_system::Pallet::<T>::block_number();

				order.status = OrderStatus::Completed;
				order.completed_at = Some(current_block);

				// Release escrow to seller
				Self::release_escrow_internal(order_id, &order.seller, order.escrowed_amount)?;

				// Remove from active orders
				ActiveOrders::<T>::mutate(|orders| {
					orders.retain(|&id| id != order_id);
				});

				// Update statistics
				MarketplaceStatistics::<T>::mutate(|stats| {
					stats.completed_orders += 1;
				});

				Self::deposit_event(Event::OrderConfirmed { order_id });

				Ok(())
			})
		}

		/// Cancel an order
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::cancel_order())]
		pub fn cancel_order(origin: OriginFor<T>, order_id: OrderId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Orders::<T>::try_mutate(order_id, |maybe_order| -> DispatchResult {
				let order = maybe_order.as_mut().ok_or(Error::<T>::OrderNotFound)?;

				// Ensure caller is buyer or seller
				ensure!(
					order.buyer == who || order.seller == who,
					Error::<T>::NotBuyer
				);

				// Ensure order can be cancelled
				ensure!(order.can_cancel(), Error::<T>::CannotCancelOrder);

				let current_block = frame_system::Pallet::<T>::block_number();

				if order.status == OrderStatus::Paid {
					// Refund buyer
					Self::release_escrow_internal(order_id, &order.buyer, order.escrowed_amount)?;
					order.status = OrderStatus::Refunded;

					Self::deposit_event(Event::RefundIssued {
						order_id,
						to: order.buyer.clone(),
						amount: order.escrowed_amount,
					});
				} else {
					order.status = OrderStatus::Cancelled;
				}

				order.completed_at = Some(current_block);

				// Remove from active orders
				ActiveOrders::<T>::mutate(|orders| {
					orders.retain(|&id| id != order_id);
				});

				// Update statistics
				MarketplaceStatistics::<T>::mutate(|stats| {
					stats.cancelled_orders += 1;
				});

				Self::deposit_event(Event::OrderCancelled { order_id });

				Ok(())
			})
		}

		/// Raise a dispute
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::raise_dispute())]
		pub fn raise_dispute(
			origin: OriginFor<T>,
			order_id: OrderId,
			reason: BoundedVec<u8, ConstU32<MAX_DISPUTE_REASON_LENGTH>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Orders::<T>::try_mutate(order_id, |maybe_order| -> DispatchResult {
				let order = maybe_order.as_mut().ok_or(Error::<T>::OrderNotFound)?;

				// Ensure caller is buyer or seller
				ensure!(
					order.buyer == who || order.seller == who,
					Error::<T>::NotBuyer
				);

				// Ensure order can be disputed
				ensure!(order.can_dispute(), Error::<T>::CannotDisputeOrder);

				// Ensure no existing dispute
				ensure!(
					order.dispute_status == DisputeStatus::None,
					Error::<T>::DisputeAlreadyRaised
				);

				order.dispute_status = if order.buyer == who {
					DisputeStatus::RaisedByBuyer
				} else {
					DisputeStatus::RaisedBySeller
				};

				order.dispute_reason = Some(reason);

				// Add to disputed orders
				DisputedOrders::<T>::mutate(|orders| {
					let _ = orders.try_push(order_id);
				});

				// Update statistics
				MarketplaceStatistics::<T>::mutate(|stats| {
					stats.disputed_orders += 1;
				});

				Self::deposit_event(Event::DisputeRaised { order_id, raised_by: who });

				Ok(())
			})
		}

		/// Resolve a dispute (admin/governance function)
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::resolve_dispute())]
		pub fn resolve_dispute(
			origin: OriginFor<T>,
			order_id: OrderId,
			resolution: u8, // 0 = for buyer, 1 = for seller, 2 = partial
			refund_percentage: u8, // 0-100, for partial resolution
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// In production, this would check admin/governance rights

			Orders::<T>::try_mutate(order_id, |maybe_order| -> DispatchResult {
				let order = maybe_order.as_mut().ok_or(Error::<T>::OrderNotFound)?;

				// Ensure there's an active dispute
				ensure!(
					order.dispute_status != DisputeStatus::None,
					Error::<T>::NoActiveDispute
				);

				let current_block = frame_system::Pallet::<T>::block_number();

				match resolution {
					0 => {
						// Resolved for buyer - full refund
						order.dispute_status = DisputeStatus::ResolvedForBuyer;
						order.status = OrderStatus::Refunded;
						Self::release_escrow_internal(
							order_id,
							&order.buyer,
							order.escrowed_amount,
						)?;

						Self::deposit_event(Event::RefundIssued {
							order_id,
							to: order.buyer.clone(),
							amount: order.escrowed_amount,
						});
					},
					1 => {
						// Resolved for seller - payment released
						order.dispute_status = DisputeStatus::ResolvedForSeller;
						order.status = OrderStatus::Completed;
						Self::release_escrow_internal(
							order_id,
							&order.seller,
							order.escrowed_amount,
						)?;
					},
					2 => {
						// Partial resolution
						order.dispute_status = DisputeStatus::ResolvedPartial;
						order.status = OrderStatus::DisputeResolved;

						let refund_amount =
							(order.escrowed_amount * refund_percentage as u128) / 100;
						let seller_amount = order.escrowed_amount - refund_amount;

						// Partial refund to buyer
						if refund_amount > 0 {
							Self::release_escrow_internal(order_id, &order.buyer, refund_amount)?;
							Self::deposit_event(Event::RefundIssued {
								order_id,
								to: order.buyer.clone(),
								amount: refund_amount,
							});
						}

						// Remaining to seller
						if seller_amount > 0 {
							Self::release_escrow_internal(
								order_id,
								&order.seller,
								seller_amount,
							)?;
						}
					},
					_ => return Err(Error::<T>::InvalidDisputeStatus.into()),
				}

				order.completed_at = Some(current_block);

				// Remove from disputed orders
				DisputedOrders::<T>::mutate(|orders| {
					orders.retain(|&id| id != order_id);
				});

				// Remove from active orders
				ActiveOrders::<T>::mutate(|orders| {
					orders.retain(|&id| id != order_id);
				});

				Self::deposit_event(Event::DisputeResolved { order_id, resolution });

				Ok(())
			})
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		/// Convert u8 to OrderType
		fn u8_to_order_type(value: u8) -> Result<OrderType, Error<T>> {
			match value {
				0 => Ok(OrderType::OneTimePurchase),
				1 => Ok(OrderType::Subscription),
				2 => Ok(OrderType::PayPerAccess),
				_ => Err(Error::<T>::InvalidOrderType),
			}
		}

		/// Release escrow internally
		fn release_escrow_internal(
			order_id: OrderId,
			to: &T::AccountId,
			amount: u128,
		) -> DispatchResult {
			// Remove escrow
			Escrows::<T>::remove(order_id);

			// Update statistics
			MarketplaceStatistics::<T>::mutate(|stats| {
				stats.total_escrowed = stats.total_escrowed.saturating_sub(amount);
			});

			// In production, this would transfer funds from escrow account to recipient
			// For now, just emit event

			Self::deposit_event(Event::EscrowReleased { order_id, to: to.clone(), amount });

			Ok(())
		}

		/// Get all orders by buyer
		pub fn get_buyer_orders(buyer: &T::AccountId) -> Vec<OrderId> {
			BuyerOrders::<T>::get(buyer).to_vec()
		}

		/// Get all orders by seller
		pub fn get_seller_orders(seller: &T::AccountId) -> Vec<OrderId> {
			SellerOrders::<T>::get(seller).to_vec()
		}

		/// Get all active orders
		pub fn get_active_orders() -> Vec<OrderId> {
			ActiveOrders::<T>::get().to_vec()
		}

		/// Get all disputed orders
		pub fn get_disputed_orders() -> Vec<OrderId> {
			DisputedOrders::<T>::get().to_vec()
		}
	}
}
