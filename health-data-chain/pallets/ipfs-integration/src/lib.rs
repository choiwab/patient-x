//! # IPFS Integration Pallet
//!
//! This pallet provides IPFS integration with off-chain worker support for Patient X HealthData Chain.
//!
//! ## Overview
//!
//! The IPFS Integration pallet enables:
//! - Asynchronous IPFS pinning and unpinning via off-chain workers
//! - IPFS content verification
//! - Gateway configuration and management
//! - Operation queue management
//! - Pin reference counting
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `request_pin`: Request content to be pinned to IPFS
//! - `request_unpin`: Request content to be unpinned from IPFS
//! - `request_verify`: Request verification of IPFS content
//! - `cancel_operation`: Cancel a pending operation
//! - `add_gateway`: Add an IPFS gateway
//! - `remove_gateway`: Remove an IPFS gateway
//! - `update_gateway`: Update gateway configuration
//!
//! ### Off-chain Worker
//!
//! The off-chain worker processes the operation queue every block:
//! - Executes pending IPFS operations
//! - Updates operation status
//! - Handles retries for failed operations
//! - Verifies pinned content periodically

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

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_health_records::Config {
		/// The overarching event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum number of pending operations
		#[pallet::constant]
		type MaxPendingOperations: Get<u32>;

		/// Maximum number of IPFS gateways
		#[pallet::constant]
		type MaxGateways: Get<u32>;

		/// Maximum retry attempts for failed operations
		#[pallet::constant]
		type MaxRetries: Get<u8>;

		/// Weight information for extrinsics
		type WeightInfo: crate::weights::WeightInfo;
	}

	/// Storage for IPFS operations
	/// Maps OperationId -> IPFSOperation
	#[pallet::storage]
	#[pallet::getter(fn operations)]
	pub type Operations<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		OperationId,
		IPFSOperation<T>,
		OptionQuery,
	>;

	/// Storage for pending operation queue
	#[pallet::storage]
	#[pallet::getter(fn pending_operations)]
	pub type PendingOperations<T: Config> =
		StorageValue<_, BoundedVec<OperationId, T::MaxPendingOperations>, ValueQuery>;

	/// Storage for pin metadata
	/// Maps CID -> PinMetadata
	#[pallet::storage]
	#[pallet::getter(fn pins)]
	pub type Pins<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		CID,
		PinMetadata<T>,
		OptionQuery,
	>;

	/// Storage for IPFS gateways
	#[pallet::storage]
	#[pallet::getter(fn gateways)]
	pub type Gateways<T: Config> =
		StorageValue<_, BoundedVec<GatewayConfig, T::MaxGateways>, ValueQuery>;

	/// Counter for operation IDs
	#[pallet::storage]
	#[pallet::getter(fn next_operation_id)]
	pub type NextOperationId<T: Config> = StorageValue<_, OperationId, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Pin operation was requested
		PinRequested {
			operation_id: OperationId,
			cid: CID,
			requestor: T::AccountId,
		},
		/// Unpin operation was requested
		UnpinRequested {
			operation_id: OperationId,
			cid: CID,
			requestor: T::AccountId,
		},
		/// Verify operation was requested
		VerifyRequested {
			operation_id: OperationId,
			cid: CID,
			requestor: T::AccountId,
		},
		/// Operation was completed
		OperationCompleted {
			operation_id: OperationId,
			status: u8,
		},
		/// Operation failed
		OperationFailed {
			operation_id: OperationId,
			retry_count: u8,
		},
		/// Operation was cancelled
		OperationCancelled {
			operation_id: OperationId,
		},
		/// Content was pinned
		ContentPinned {
			cid: CID,
			pinner: T::AccountId,
		},
		/// Content was unpinned
		ContentUnpinned {
			cid: CID,
		},
		/// Content was verified
		ContentVerified {
			cid: CID,
			result: u8,
		},
		/// Gateway was added
		GatewayAdded {
			url: GatewayURL,
		},
		/// Gateway was removed
		GatewayRemoved {
			url: GatewayURL,
		},
		/// Gateway was updated
		GatewayUpdated {
			url: GatewayURL,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Operation not found
		OperationNotFound,
		/// Too many pending operations
		TooManyPendingOperations,
		/// Pin not found
		PinNotFound,
		/// Content already pinned
		AlreadyPinned,
		/// Content not pinned
		NotPinned,
		/// Invalid CID format
		InvalidCID,
		/// Gateway not found
		GatewayNotFound,
		/// Too many gateways
		TooManyGateways,
		/// Gateway already exists
		GatewayAlreadyExists,
		/// Operation cannot be cancelled
		CannotCancelOperation,
		/// Not authorized
		NotAuthorized,
		/// Invalid gateway URL
		InvalidGatewayURL,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Off-chain worker hook - processes IPFS operations
		fn offchain_worker(_block_number: BlockNumberFor<T>) {
			// Process pending operations
			let _ = Self::process_ipfs_queue();
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Request content to be pinned to IPFS
		///
		/// # Arguments
		/// * `cid` - IPFS content identifier
		/// * `name` - Optional pin name/label
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::request_pin())]
		pub fn request_pin(
			origin: OriginFor<T>,
			cid: CID,
			name: Option<BoundedVec<u8, ConstU32<MAX_PIN_NAME_LENGTH>>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Validate CID
			ensure!(!cid.is_empty(), Error::<T>::InvalidCID);

			// Check if already pinned
			if Pins::<T>::contains_key(&cid) {
				// If already pinned, just increment reference count
				Pins::<T>::try_mutate(&cid, |maybe_pin| {
					if let Some(pin) = maybe_pin {
						pin.reference_count += 1;
						Ok::<(), DispatchError>(())
					} else {
						Err(Error::<T>::PinNotFound.into())
					}
				})?;

				return Ok(());
			}

			let operation_id = Self::create_operation(
				cid.clone(),
				IPFSOpType::Pin,
				who.clone(),
			)?;

			// Create initial pin metadata
			let now = frame_system::Pallet::<T>::block_number();
			let pin_metadata = PinMetadata {
				cid: cid.clone(),
				status: PinStatus::Pinning,
				pinner: who.clone(),
				pinned_at: now,
				last_verified: None,
				name,
				reference_count: 1,
			};

			Pins::<T>::insert(&cid, pin_metadata);

			Self::deposit_event(Event::PinRequested {
				operation_id,
				cid,
				requestor: who,
			});

			Ok(())
		}

		/// Request content to be unpinned from IPFS
		///
		/// # Arguments
		/// * `cid` - IPFS content identifier
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::request_unpin())]
		pub fn request_unpin(
			origin: OriginFor<T>,
			cid: CID,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Verify pin exists
			let mut pin = Pins::<T>::get(&cid)
				.ok_or(Error::<T>::NotPinned)?;

			// Decrement reference count
			if pin.reference_count > 1 {
				pin.reference_count -= 1;
				Pins::<T>::insert(&cid, pin);
				return Ok(());
			}

			// Create unpin operation if reference count is 0
			let operation_id = Self::create_operation(
				cid.clone(),
				IPFSOpType::Unpin,
				who.clone(),
			)?;

			Self::deposit_event(Event::UnpinRequested {
				operation_id,
				cid,
				requestor: who,
			});

			Ok(())
		}

		/// Request verification of IPFS content
		///
		/// # Arguments
		/// * `cid` - IPFS content identifier
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::request_verify())]
		pub fn request_verify(
			origin: OriginFor<T>,
			cid: CID,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Verify pin exists
			ensure!(Pins::<T>::contains_key(&cid), Error::<T>::NotPinned);

			let operation_id = Self::create_operation(
				cid.clone(),
				IPFSOpType::Verify,
				who.clone(),
			)?;

			Self::deposit_event(Event::VerifyRequested {
				operation_id,
				cid,
				requestor: who,
			});

			Ok(())
		}

		/// Cancel a pending operation
		///
		/// # Arguments
		/// * `operation_id` - Operation to cancel
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::cancel_operation())]
		pub fn cancel_operation(
			origin: OriginFor<T>,
			operation_id: OperationId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Operations::<T>::try_mutate(&operation_id, |maybe_op| {
				let op = maybe_op.as_mut().ok_or(Error::<T>::OperationNotFound)?;

				// Only requestor can cancel
				ensure!(op.requestor == who, Error::<T>::NotAuthorized);

				// Can only cancel pending operations
				ensure!(
					matches!(op.status, OperationStatus::Pending),
					Error::<T>::CannotCancelOperation
				);

				op.status = OperationStatus::Cancelled;
				op.completed_at = Some(frame_system::Pallet::<T>::block_number());

				Ok::<(), DispatchError>(())
			})?;

			// Remove from pending queue
			PendingOperations::<T>::mutate(|queue| {
				if let Some(pos) = queue.iter().position(|id| id == &operation_id) {
					queue.swap_remove(pos);
				}
			});

			Self::deposit_event(Event::OperationCancelled {
				operation_id,
			});

			Ok(())
		}

		/// Add an IPFS gateway
		///
		/// # Arguments
		/// * `url` - Gateway URL
		/// * `priority` - Gateway priority (lower = higher priority)
		#[pallet::call_index(4)]
		#[pallet::weight(<T as Config>::WeightInfo::add_gateway())]
		pub fn add_gateway(
			origin: OriginFor<T>,
			url: GatewayURL,
			priority: u8,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Validate URL
			ensure!(!url.is_empty(), Error::<T>::InvalidGatewayURL);

			Gateways::<T>::try_mutate(|gateways| {
				// Check if gateway already exists
				ensure!(
					!gateways.iter().any(|g| g.url == url),
					Error::<T>::GatewayAlreadyExists
				);

				let config = GatewayConfig {
					url: url.clone(),
					enabled: true,
					priority,
				};

				gateways.try_push(config)
					.map_err(|_| Error::<T>::TooManyGateways)?;

				Ok::<(), DispatchError>(())
			})?;

			Self::deposit_event(Event::GatewayAdded { url });

			Ok(())
		}

		/// Remove an IPFS gateway
		///
		/// # Arguments
		/// * `url` - Gateway URL to remove
		#[pallet::call_index(5)]
		#[pallet::weight(<T as Config>::WeightInfo::remove_gateway())]
		pub fn remove_gateway(
			origin: OriginFor<T>,
			url: GatewayURL,
		) -> DispatchResult {
			ensure_root(origin)?;

			Gateways::<T>::mutate(|gateways| {
				if let Some(pos) = gateways.iter().position(|g| g.url == url) {
					gateways.swap_remove(pos);
				}
			});

			Self::deposit_event(Event::GatewayRemoved { url });

			Ok(())
		}

		/// Update gateway configuration
		///
		/// # Arguments
		/// * `url` - Gateway URL
		/// * `enabled` - Whether gateway is enabled
		/// * `priority` - New priority
		#[pallet::call_index(6)]
		#[pallet::weight(<T as Config>::WeightInfo::update_gateway())]
		pub fn update_gateway(
			origin: OriginFor<T>,
			url: GatewayURL,
			enabled: bool,
			priority: u8,
		) -> DispatchResult {
			ensure_root(origin)?;

			Gateways::<T>::try_mutate(|gateways| {
				let gateway = gateways.iter_mut()
					.find(|g| g.url == url)
					.ok_or(Error::<T>::GatewayNotFound)?;

				gateway.enabled = enabled;
				gateway.priority = priority;

				Ok::<(), DispatchError>(())
			})?;

			Self::deposit_event(Event::GatewayUpdated { url });

			Ok(())
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		/// Create a new IPFS operation
		fn create_operation(
			cid: CID,
			op_type: IPFSOpType,
			requestor: T::AccountId,
		) -> Result<OperationId, DispatchError> {
			let operation_id = NextOperationId::<T>::get();
			NextOperationId::<T>::put(operation_id.wrapping_add(1));

			let now = frame_system::Pallet::<T>::block_number();

			let operation = IPFSOperation {
				operation_id,
				cid,
				op_type,
				status: OperationStatus::Pending,
				requestor,
				requested_at: now,
				completed_at: None,
				retry_count: 0,
				error_message: None,
			};

			Operations::<T>::insert(operation_id, operation);

			// Add to pending queue
			PendingOperations::<T>::try_mutate(|queue| {
				queue.try_push(operation_id)
					.map_err(|_| Error::<T>::TooManyPendingOperations)
			})?;

			Ok(operation_id)
		}

		/// Process IPFS operation queue (called by off-chain worker)
		fn process_ipfs_queue() -> Result<(), &'static str> {
			let pending = PendingOperations::<T>::get();

			for operation_id in pending.iter() {
				if let Some(operation) = Operations::<T>::get(operation_id) {
					// Skip if already processing or completed
					if !matches!(operation.status, OperationStatus::Pending) {
						continue;
					}

					// Mark as processing
					Operations::<T>::mutate(operation_id, |maybe_op| {
						if let Some(op) = maybe_op {
							op.status = OperationStatus::Processing;
						}
					});

					// Execute operation based on type
					let result = match operation.op_type {
						IPFSOpType::Pin => Self::execute_pin(&operation),
						IPFSOpType::Unpin => Self::execute_unpin(&operation),
						IPFSOpType::Verify => Self::execute_verify(&operation),
						IPFSOpType::Update => Self::execute_update(&operation),
					};

					// Update operation status based on result
					Self::update_operation_status(*operation_id, result);
				}
			}

			Ok(())
		}

		/// Execute pin operation (simulated - would use IPFS HTTP API)
		fn execute_pin(_operation: &IPFSOperation<T>) -> Result<(), &'static str> {
			// NOTE: In production, this would make HTTP requests to IPFS API
			// For now, we simulate successful pinning
			// Simulate success
			Ok(())
		}

		/// Execute unpin operation (simulated)
		fn execute_unpin(_operation: &IPFSOperation<T>) -> Result<(), &'static str> {
			// Simulate success
			Ok(())
		}

		/// Execute verify operation (simulated)
		fn execute_verify(_operation: &IPFSOperation<T>) -> Result<(), &'static str> {
			// Simulate success
			Ok(())
		}

		/// Execute update operation (simulated)
		fn execute_update(_operation: &IPFSOperation<T>) -> Result<(), &'static str> {
			// Simulate success
			Ok(())
		}

		/// Update operation status after execution
		fn update_operation_status(operation_id: OperationId, result: Result<(), &'static str>) {
			Operations::<T>::mutate(&operation_id, |maybe_op| {
				if let Some(op) = maybe_op {
					let now = frame_system::Pallet::<T>::block_number();

					match result {
						Ok(()) => {
							op.status = OperationStatus::Completed;
							op.completed_at = Some(now);

							// Update pin status if pin operation
							if matches!(op.op_type, IPFSOpType::Pin) {
								Pins::<T>::mutate(&op.cid, |maybe_pin| {
									if let Some(pin) = maybe_pin {
										pin.status = PinStatus::Pinned;
									}
								});
							}

							// Remove pin if unpin operation
							if matches!(op.op_type, IPFSOpType::Unpin) {
								Pins::<T>::remove(&op.cid);
							}

							// Remove from pending queue
							PendingOperations::<T>::mutate(|queue| {
								if let Some(pos) = queue.iter().position(|id| id == &operation_id) {
									queue.swap_remove(pos);
								}
							});
						},
						Err(_error) => {
							let max_retries = T::MaxRetries::get();

							if op.retry_count < max_retries {
								// Retry
								op.retry_count += 1;
								op.status = OperationStatus::Pending;
							} else {
								// Mark as failed
								op.status = OperationStatus::Failed;
								op.completed_at = Some(now);

								// Update pin status if pin operation
								if matches!(op.op_type, IPFSOpType::Pin) {
									Pins::<T>::mutate(&op.cid, |maybe_pin| {
										if let Some(pin) = maybe_pin {
											pin.status = PinStatus::Failed;
										}
									});
								}

								// Remove from pending queue
								PendingOperations::<T>::mutate(|queue| {
									if let Some(pos) = queue.iter().position(|id| id == &operation_id) {
										queue.swap_remove(pos);
									}
								});
							}
						},
					}
				}
			});
		}

		/// Get all pending operations
		pub fn get_pending_operations() -> Vec<IPFSOperation<T>> {
			let pending_ids = PendingOperations::<T>::get();
			let mut operations = Vec::new();

			for operation_id in pending_ids.iter() {
				if let Some(op) = Operations::<T>::get(operation_id) {
					operations.push(op);
				}
			}

			operations
		}

		/// Get pin statistics
		pub fn get_pin_stats() -> (u32, u32, u32) {
			let mut pinned = 0u32;
			let mut pinning = 0u32;
			let mut failed = 0u32;

			for (_cid, pin) in Pins::<T>::iter() {
				match pin.status {
					PinStatus::Pinned => pinned += 1,
					PinStatus::Pinning => pinning += 1,
					PinStatus::Failed => failed += 1,
					_ => {},
				}
			}

			(pinned, pinning, failed)
		}
	}
}
