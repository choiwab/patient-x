//! # Provenance Pallet
//!
//! This pallet provides data provenance and lineage tracking for Patient X HealthData Chain.
//!
//! ## Overview
//!
//! The Provenance pallet enables:
//! - Data lineage tracking (parent-child relationships)
//! - Transformation logging
//! - Usage history tracking
//! - Complete audit trail of data lifecycle
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `record_lineage`: Record data derivation lineage
//! - `record_transformation`: Log data transformation
//! - `record_usage`: Track data usage activity
//! - `query_lineage`: Query lineage for a record

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
		/// The overarching event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum number of lineage entries per record
		#[pallet::constant]
		type MaxLineageEntriesPerRecord: Get<u32>;

		/// Maximum number of transformation logs per record
		#[pallet::constant]
		type MaxTransformationLogsPerRecord: Get<u32>;

		/// Maximum number of usage activities per record
		#[pallet::constant]
		type MaxUsageActivitiesPerRecord: Get<u32>;

		/// Weight information for extrinsics
		type WeightInfo: crate::weights::WeightInfo;
	}

	/// Storage for lineage entries
	/// Maps LineageId -> LineageEntry
	#[pallet::storage]
	#[pallet::getter(fn lineage_entries)]
	pub type LineageEntries<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		LineageId,
		LineageEntry<T>,
		OptionQuery,
	>;

	/// Storage for record lineage index
	/// Maps RecordId -> Vec<LineageId>
	#[pallet::storage]
	#[pallet::getter(fn record_lineage_index)]
	pub type RecordLineageIndex<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		RecordId,
		BoundedVec<LineageId, T::MaxLineageEntriesPerRecord>,
		ValueQuery,
	>;

	/// Storage for transformation logs
	/// Maps TransformationId -> TransformationLog
	#[pallet::storage]
	#[pallet::getter(fn transformation_logs)]
	pub type TransformationLogs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		TransformationId,
		TransformationLog<T>,
		OptionQuery,
	>;

	/// Storage for record transformation index
	/// Maps RecordId -> Vec<TransformationId>
	#[pallet::storage]
	#[pallet::getter(fn record_transformations)]
	pub type RecordTransformations<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		RecordId,
		BoundedVec<TransformationId, T::MaxTransformationLogsPerRecord>,
		ValueQuery,
	>;

	/// Storage for usage activities
	/// Maps (RecordId, Index) -> UsageActivity
	#[pallet::storage]
	#[pallet::getter(fn usage_activities)]
	pub type UsageActivities<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		RecordId,
		Blake2_128Concat,
		u32, // Activity index
		UsageActivity<T>,
		OptionQuery,
	>;

	/// Storage for usage activity count per record
	#[pallet::storage]
	#[pallet::getter(fn usage_activity_count)]
	pub type UsageActivityCount<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		RecordId,
		u32,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Lineage was recorded
		LineageRecorded {
			lineage_id: LineageId,
			child_record: RecordId,
			parent_count: u32,
		},
		/// Transformation was logged
		TransformationLogged {
			transformation_id: TransformationId,
			record_id: RecordId,
			step_count: u32,
		},
		/// Usage activity was recorded
		UsageRecorded {
			record_id: RecordId,
			activity_type: u8,
			agent: T::AccountId,
		},
		/// Lineage was queried
		LineageQueried {
			record_id: RecordId,
			entry_count: u32,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Lineage entry not found
		LineageNotFound,
		/// Transformation log not found
		TransformationNotFound,
		/// Record not found
		RecordNotFound,
		/// Too many lineage entries
		TooManyLineageEntries,
		/// Too many transformation logs
		TooManyTransformationLogs,
		/// Too many usage activities
		TooManyUsageActivities,
		/// Invalid parent record
		InvalidParentRecord,
		/// No transformation steps provided
		NoTransformationSteps,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Record data lineage (derivation relationship)
		///
		/// # Arguments
		/// * `lineage_id` - Unique lineage identifier
		/// * `child_record` - Derived data record
		/// * `parent_records` - Source data records
		/// * `transformation_type` - Type of transformation (0-8)
		/// * `agent_type` - Agent type (0-3)
		/// * `description` - Optional description
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::record_lineage())]
		pub fn record_lineage(
			origin: OriginFor<T>,
			lineage_id: LineageId,
			child_record: RecordId,
			parent_records: BoundedVec<RecordId, ConstU32<MAX_PARENT_RECORDS>>,
			transformation_type: u8,
			agent_type: u8,
			description: Option<BoundedVec<u8, ConstU32<MAX_DESCRIPTION_LENGTH>>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Verify child record exists
			ensure!(
				pallet_health_records::Pallet::<T>::records(&child_record).is_some(),
				Error::<T>::RecordNotFound
			);

			// Verify all parent records exist
			for parent in &parent_records {
				ensure!(
					pallet_health_records::Pallet::<T>::records(parent).is_some(),
					Error::<T>::InvalidParentRecord
				);
			}

			let now = frame_system::Pallet::<T>::block_number();
			let parent_count = parent_records.len() as u32;

			let lineage_entry = LineageEntry {
				lineage_id,
				child_record,
				parent_records,
				transformation_type: transformation_type.into(),
				agent: who.clone(),
				agent_type: agent_type.into(),
				description,
				created_at: now,
			};

			LineageEntries::<T>::insert(&lineage_id, lineage_entry);

			// Add to record lineage index
			RecordLineageIndex::<T>::try_mutate(&child_record, |lineages| {
				lineages.try_push(lineage_id)
					.map_err(|_| Error::<T>::TooManyLineageEntries)
			})?;

			Self::deposit_event(Event::LineageRecorded {
				lineage_id,
				child_record,
				parent_count,
			});

			Ok(())
		}

		/// Record transformation log
		///
		/// # Arguments
		/// * `transformation_id` - Unique transformation identifier
		/// * `record_id` - Record being transformed
		/// * `transformation_type` - Type of transformation (0-8)
		/// * `parameters` - Optional transformation parameters
		/// * `description` - Optional description
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::record_transformation())]
		pub fn record_transformation(
			origin: OriginFor<T>,
			transformation_id: TransformationId,
			record_id: RecordId,
			transformation_type: u8,
			parameters: Option<BoundedVec<u8, ConstU32<128>>>,
			description: Option<BoundedVec<u8, ConstU32<MAX_DESCRIPTION_LENGTH>>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Verify record exists
			ensure!(
				pallet_health_records::Pallet::<T>::records(&record_id).is_some(),
				Error::<T>::RecordNotFound
			);

			let now = frame_system::Pallet::<T>::block_number();

			// Create transformation step
			let step = TransformationStep {
				step_index: 0,
				transformation_type: transformation_type.into(),
				parameters,
				description,
			};

			let mut steps = BoundedVec::default();
			steps.try_push(step)
				.map_err(|_| Error::<T>::NoTransformationSteps)?;

			let transformation_log = TransformationLog {
				transformation_id,
				record_id,
				steps,
				agent: who,
				timestamp: now,
			};

			TransformationLogs::<T>::insert(&transformation_id, transformation_log);

			// Add to record transformation index
			RecordTransformations::<T>::try_mutate(&record_id, |transformations| {
				transformations.try_push(transformation_id)
					.map_err(|_| Error::<T>::TooManyTransformationLogs)
			})?;

			Self::deposit_event(Event::TransformationLogged {
				transformation_id,
				record_id,
				step_count: 1,
			});

			Ok(())
		}

		/// Record usage activity
		///
		/// # Arguments
		/// * `record_id` - Record being accessed/used
		/// * `activity_type` - Type of activity (0-5)
		/// * `agent_type` - Agent type (0-3)
		/// * `purpose` - Optional purpose description
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::record_usage())]
		pub fn record_usage(
			origin: OriginFor<T>,
			record_id: RecordId,
			activity_type: u8,
			agent_type: u8,
			purpose: Option<BoundedVec<u8, ConstU32<MAX_DESCRIPTION_LENGTH>>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Verify record exists
			ensure!(
				pallet_health_records::Pallet::<T>::records(&record_id).is_some(),
				Error::<T>::RecordNotFound
			);

			let now = frame_system::Pallet::<T>::block_number();
			let activity_index = UsageActivityCount::<T>::get(&record_id);

			let usage_activity = UsageActivity {
				record_id,
				activity_type: activity_type.into(),
				agent: who.clone(),
				agent_type: agent_type.into(),
				purpose,
				timestamp: now,
			};

			UsageActivities::<T>::insert(&record_id, activity_index, usage_activity);
			UsageActivityCount::<T>::insert(&record_id, activity_index + 1);

			Self::deposit_event(Event::UsageRecorded {
				record_id,
				activity_type,
				agent: who,
			});

			Ok(())
		}

		/// Query lineage for a record
		///
		/// # Arguments
		/// * `record_id` - Record to query lineage for
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::query_lineage())]
		pub fn query_lineage(
			origin: OriginFor<T>,
			record_id: RecordId,
		) -> DispatchResult {
			ensure_signed(origin)?;

			// Verify record exists
			ensure!(
				pallet_health_records::Pallet::<T>::records(&record_id).is_some(),
				Error::<T>::RecordNotFound
			);

			let lineages = RecordLineageIndex::<T>::get(&record_id);
			let entry_count = lineages.len() as u32;

			Self::deposit_event(Event::LineageQueried {
				record_id,
				entry_count,
			});

			Ok(())
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		/// Get full lineage tree for a record
		pub fn get_lineage_tree(record_id: &RecordId) -> Vec<LineageEntry<T>> {
			let lineage_ids = RecordLineageIndex::<T>::get(record_id);
			let mut entries = Vec::new();

			for lineage_id in lineage_ids.iter() {
				if let Some(entry) = LineageEntries::<T>::get(lineage_id) {
					entries.push(entry);
				}
			}

			entries
		}

		/// Get all transformations for a record
		pub fn get_transformations(record_id: &RecordId) -> Vec<TransformationLog<T>> {
			let transformation_ids = RecordTransformations::<T>::get(record_id);
			let mut logs = Vec::new();

			for transformation_id in transformation_ids.iter() {
				if let Some(log) = TransformationLogs::<T>::get(transformation_id) {
					logs.push(log);
				}
			}

			logs
		}

		/// Get usage history for a record
		pub fn get_usage_history(record_id: &RecordId) -> Vec<UsageActivity<T>> {
			let count = UsageActivityCount::<T>::get(record_id);
			let mut activities = Vec::new();

			for i in 0..count {
				if let Some(activity) = UsageActivities::<T>::get(record_id, i) {
					activities.push(activity);
				}
			}

			activities
		}

		/// Trace lineage backwards (find all ancestors)
		pub fn trace_ancestors(record_id: &RecordId) -> Vec<RecordId> {
			let mut ancestors = Vec::new();
			let lineages = Self::get_lineage_tree(record_id);

			for lineage in lineages {
				for parent in lineage.parent_records.iter() {
					if !ancestors.contains(parent) {
						ancestors.push(*parent);
						// Recursively trace parent's ancestors
						let parent_ancestors = Self::trace_ancestors(parent);
						for ancestor in parent_ancestors {
							if !ancestors.contains(&ancestor) {
								ancestors.push(ancestor);
							}
						}
					}
				}
			}

			ancestors
		}
	}
}
