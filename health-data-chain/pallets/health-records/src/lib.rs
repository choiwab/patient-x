//! # Health Records Pallet
//!
//! This pallet manages health record storage and access for Patient X HealthData Chain.
//!
//! ## Overview
//!
//! The Health Records pallet enables:
//! - Uploading and storing health record metadata on-chain
//! - Version tracking for record updates
//! - Access control and permission management
//! - Access logging for audit trails
//! - IPFS integration for encrypted data storage
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `upload_record`: Upload a new health record
//! - `update_record`: Update an existing health record (creates new version)
//! - `delete_record`: Mark a record as deleted (soft delete)
//! - `archive_record`: Archive a record (make read-only)
//! - `grant_access`: Grant access to a record
//! - `revoke_access`: Revoke access to a record
//! - `log_access`: Log an access event for audit
//! - `add_tags`: Add tags to a record for categorization

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
	pub trait Config: frame_system::Config {
		/// The overarching event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum number of records per user
		#[pallet::constant]
		type MaxRecordsPerUser: Get<u32>;

		/// Maximum number of access grants per record
		#[pallet::constant]
		type MaxAccessGrantsPerRecord: Get<u32>;

		/// Maximum number of versions per record
		#[pallet::constant]
		type MaxVersionsPerRecord: Get<u32>;

		/// Maximum number of access logs per record
		#[pallet::constant]
		type MaxAccessLogsPerRecord: Get<u32>;

		/// Weight information for extrinsics
		type WeightInfo: crate::weights::WeightInfo;
	}

	/// Storage for record metadata
	/// Maps RecordId -> RecordMetadata
	#[pallet::storage]
	#[pallet::getter(fn records)]
	pub type Records<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		RecordId,
		RecordMetadata<T>,
		OptionQuery,
	>;

	/// Storage for record versions
	/// Maps (RecordId, VersionNumber) -> VersionMetadata
	#[pallet::storage]
	#[pallet::getter(fn record_versions)]
	pub type RecordVersions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		RecordId,
		Blake2_128Concat,
		VersionNumber,
		VersionMetadata<T>,
		OptionQuery,
	>;

	/// Storage for user's records index
	/// Maps AccountId -> Vec<RecordId>
	#[pallet::storage]
	#[pallet::getter(fn user_records)]
	pub type UserRecords<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<RecordId, T::MaxRecordsPerUser>,
		ValueQuery,
	>;

	/// Storage for access grants
	/// Maps (RecordId, AccountId) -> AccessGrant
	#[pallet::storage]
	#[pallet::getter(fn access_grants)]
	pub type AccessGrants<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		RecordId,
		Blake2_128Concat,
		T::AccountId,
		AccessGrant<T>,
		OptionQuery,
	>;

	/// Storage for access grant index per record
	/// Maps RecordId -> Vec<AccountId>
	#[pallet::storage]
	#[pallet::getter(fn record_access_list)]
	pub type RecordAccessList<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		RecordId,
		BoundedVec<T::AccountId, T::MaxAccessGrantsPerRecord>,
		ValueQuery,
	>;

	/// Storage for access logs
	/// Maps (RecordId, LogIndex) -> AccessLog
	#[pallet::storage]
	#[pallet::getter(fn access_logs)]
	pub type AccessLogs<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		RecordId,
		Blake2_128Concat,
		u32, // Log index
		AccessLog<T>,
		OptionQuery,
	>;

	/// Storage for access log count per record
	#[pallet::storage]
	#[pallet::getter(fn access_log_count)]
	pub type AccessLogCount<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		RecordId,
		u32,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Record was uploaded
		RecordUploaded {
			record_id: RecordId,
			owner: T::AccountId,
			data_format: u8,
		},
		/// Record was updated
		RecordUpdated {
			record_id: RecordId,
			version: VersionNumber,
			updated_by: T::AccountId,
		},
		/// Record was deleted
		RecordDeleted {
			record_id: RecordId,
			deleted_by: T::AccountId,
		},
		/// Record was archived
		RecordArchived {
			record_id: RecordId,
			archived_by: T::AccountId,
		},
		/// Access was granted
		AccessGranted {
			record_id: RecordId,
			grantee: T::AccountId,
			granted_by: T::AccountId,
		},
		/// Access was revoked
		AccessRevoked {
			record_id: RecordId,
			revoked_from: T::AccountId,
			revoked_by: T::AccountId,
		},
		/// Access was logged
		AccessLogged {
			record_id: RecordId,
			accessor: T::AccountId,
			operation: u8,
		},
		/// Tags were added
		TagsAdded {
			record_id: RecordId,
			tag_count: u32,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Record not found
		RecordNotFound,
		/// Record already exists
		RecordAlreadyExists,
		/// Not the record owner
		NotRecordOwner,
		/// No access permission
		NoAccessPermission,
		/// Record is deleted
		RecordDeleted,
		/// Record is archived (read-only)
		RecordArchived,
		/// Too many records for user
		TooManyRecords,
		/// Too many access grants
		TooManyAccessGrants,
		/// Too many versions
		TooManyVersions,
		/// Access grant not found
		AccessGrantNotFound,
		/// Access has expired
		AccessExpired,
		/// Invalid IPFS CID
		InvalidIPFSCID,
		/// Invalid encryption key
		InvalidEncryptionKey,
		/// Too many tags
		TooManyTags,
		/// Cannot modify record
		CannotModifyRecord,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Upload a new health record
		///
		/// # Arguments
		/// * `record_id` - Unique identifier for the record
		/// * `ipfs_cid` - IPFS content identifier
		/// * `data_format_id` - Data format type (0-6)
		/// * `encryption_key_id` - Encryption key identifier
		/// * `tags` - Optional tags for categorization
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::upload_record())]
		pub fn upload_record(
			origin: OriginFor<T>,
			record_id: RecordId,
			ipfs_cid: CID,
			data_format_id: u8,
			encryption_key_id: KeyId,
			tags: Option<BoundedVec<BoundedVec<u8, ConstU32<MAX_TAG_LENGTH>>, ConstU32<MAX_TAGS_PER_RECORD>>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure record doesn't already exist
			ensure!(
				!Records::<T>::contains_key(&record_id),
				Error::<T>::RecordAlreadyExists
			);

			// Validate IPFS CID
			ensure!(!ipfs_cid.is_empty(), Error::<T>::InvalidIPFSCID);

			let now = frame_system::Pallet::<T>::block_number();
			let data_format: DataFormat = data_format_id.into();

			let metadata = RecordMetadata {
				record_id,
				owner: who.clone(),
				ipfs_cid: ipfs_cid.clone(),
				data_format: data_format.clone(),
				encryption_key_id,
				created_at: now,
				updated_at: now,
				current_version: 1,
				status: RecordStatus::Active,
				tags: tags.unwrap_or_default(),
			};

			// Store record metadata
			Records::<T>::insert(&record_id, metadata);

			// Create initial version
			let version_metadata = VersionMetadata {
				version: 1,
				ipfs_cid,
				encryption_key_id,
				created_at: now,
				created_by: who.clone(),
				data_format: data_format.clone(),
			};

			RecordVersions::<T>::insert(&record_id, 1, version_metadata);

			// Add to user's records index
			UserRecords::<T>::try_mutate(&who, |records| {
				records.try_push(record_id)
					.map_err(|_| Error::<T>::TooManyRecords)
			})?;

			Self::deposit_event(Event::RecordUploaded {
				record_id,
				owner: who,
				data_format: data_format_id,
			});

			Ok(())
		}

		/// Update an existing health record (creates new version)
		///
		/// # Arguments
		/// * `record_id` - Record to update
		/// * `new_ipfs_cid` - New IPFS content identifier
		/// * `new_encryption_key_id` - New encryption key identifier
		/// * `data_format_id` - Data format type
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::update_record())]
		pub fn update_record(
			origin: OriginFor<T>,
			record_id: RecordId,
			new_ipfs_cid: CID,
			new_encryption_key_id: KeyId,
			data_format_id: u8,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Records::<T>::try_mutate(&record_id, |maybe_record| {
				let record = maybe_record.as_mut().ok_or(Error::<T>::RecordNotFound)?;

				// Check if caller has permission to modify
				let can_modify = record.owner == who || 
					Self::check_modify_permission(&who, &record_id)?;

				ensure!(can_modify, Error::<T>::NoAccessPermission);

				// Cannot modify deleted or archived records
				ensure!(record.status == RecordStatus::Active, Error::<T>::CannotModifyRecord);

				let now = frame_system::Pallet::<T>::block_number();
				let data_format: DataFormat = data_format_id.into();
				let new_version = record.current_version + 1;

				// Create new version
				let version_metadata = VersionMetadata {
					version: new_version,
					ipfs_cid: new_ipfs_cid.clone(),
					encryption_key_id: new_encryption_key_id,
					created_at: now,
					created_by: who.clone(),
					data_format: data_format.clone(),
				};

				RecordVersions::<T>::insert(&record_id, new_version, version_metadata);

				// Update record metadata
				record.ipfs_cid = new_ipfs_cid;
				record.encryption_key_id = new_encryption_key_id;
				record.data_format = data_format;
				record.updated_at = now;
				record.current_version = new_version;

				Ok::<(), DispatchError>(())
			})?;

			Self::deposit_event(Event::RecordUpdated {
				record_id,
				version: Records::<T>::get(&record_id).unwrap().current_version,
				updated_by: who,
			});

			Ok(())
		}

		/// Delete a health record (soft delete)
		///
		/// # Arguments
		/// * `record_id` - Record to delete
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::delete_record())]
		pub fn delete_record(
			origin: OriginFor<T>,
			record_id: RecordId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Records::<T>::try_mutate(&record_id, |maybe_record| {
				let record = maybe_record.as_mut().ok_or(Error::<T>::RecordNotFound)?;

				// Only owner can delete
				ensure!(record.owner == who, Error::<T>::NotRecordOwner);

				let now = frame_system::Pallet::<T>::block_number();
				record.status = RecordStatus::Deleted;
				record.updated_at = now;

				Ok::<(), DispatchError>(())
			})?;

			Self::deposit_event(Event::RecordDeleted {
				record_id,
				deleted_by: who,
			});

			Ok(())
		}

		/// Archive a health record (make read-only)
		///
		/// # Arguments
		/// * `record_id` - Record to archive
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::archive_record())]
		pub fn archive_record(
			origin: OriginFor<T>,
			record_id: RecordId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Records::<T>::try_mutate(&record_id, |maybe_record| {
				let record = maybe_record.as_mut().ok_or(Error::<T>::RecordNotFound)?;

				// Only owner can archive
				ensure!(record.owner == who, Error::<T>::NotRecordOwner);

				let now = frame_system::Pallet::<T>::block_number();
				record.status = RecordStatus::Archived;
				record.updated_at = now;

				Ok::<(), DispatchError>(())
			})?;

			Self::deposit_event(Event::RecordArchived {
				record_id,
				archived_by: who,
			});

			Ok(())
		}

		/// Grant access to a health record
		///
		/// # Arguments
		/// * `record_id` - Record to grant access to
		/// * `grantee` - Account to grant access to
		/// * `duration_blocks` - Optional access duration (None = permanent)
		/// * `can_modify` - Whether grantee can modify the record
		/// * `can_share` - Whether grantee can re-share access
		#[pallet::call_index(4)]
		#[pallet::weight(<T as Config>::WeightInfo::grant_access())]
		pub fn grant_access(
			origin: OriginFor<T>,
			record_id: RecordId,
			grantee: T::AccountId,
			duration_blocks: Option<BlockNumberFor<T>>,
			can_modify: bool,
			can_share: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Get record and check permission
			let record = Records::<T>::get(&record_id)
				.ok_or(Error::<T>::RecordNotFound)?;

			// Only owner or accounts with share permission can grant access
			let can_grant = record.owner == who || 
				Self::check_share_permission(&who, &record_id)?;

			ensure!(can_grant, Error::<T>::NoAccessPermission);

			let now = frame_system::Pallet::<T>::block_number();
			let expires_at = duration_blocks.map(|duration| now + duration);

			let access_grant = AccessGrant {
				grantee: grantee.clone(),
				record_id,
				granted_at: now,
				expires_at,
				can_modify,
				can_share,
			};

			AccessGrants::<T>::insert(&record_id, &grantee, access_grant);

			// Add to access list
			RecordAccessList::<T>::try_mutate(&record_id, |list| {
				if !list.contains(&grantee) {
					list.try_push(grantee.clone())
						.map_err(|_| Error::<T>::TooManyAccessGrants)
				} else {
					Ok(())
				}
			})?;

			Self::deposit_event(Event::AccessGranted {
				record_id,
				grantee,
				granted_by: who,
			});

			Ok(())
		}

		/// Revoke access to a health record
		///
		/// # Arguments
		/// * `record_id` - Record to revoke access from
		/// * `grantee` - Account to revoke access from
		#[pallet::call_index(5)]
		#[pallet::weight(<T as Config>::WeightInfo::revoke_access())]
		pub fn revoke_access(
			origin: OriginFor<T>,
			record_id: RecordId,
			grantee: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Get record and check permission
			let record = Records::<T>::get(&record_id)
				.ok_or(Error::<T>::RecordNotFound)?;

			// Only owner can revoke access
			ensure!(record.owner == who, Error::<T>::NotRecordOwner);

			// Remove access grant
			AccessGrants::<T>::remove(&record_id, &grantee);

			// Remove from access list
			RecordAccessList::<T>::mutate(&record_id, |list| {
				if let Some(pos) = list.iter().position(|x| x == &grantee) {
					list.swap_remove(pos);
				}
			});

			Self::deposit_event(Event::AccessRevoked {
				record_id,
				revoked_from: grantee,
				revoked_by: who,
			});

			Ok(())
		}

		/// Log an access event for audit trail
		///
		/// # Arguments
		/// * `record_id` - Record that was accessed
		/// * `operation_id` - Access operation type (0-4)
		#[pallet::call_index(6)]
		#[pallet::weight(<T as Config>::WeightInfo::log_access())]
		pub fn log_access(
			origin: OriginFor<T>,
			record_id: RecordId,
			operation_id: u8,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Verify record exists
			ensure!(
				Records::<T>::contains_key(&record_id),
				Error::<T>::RecordNotFound
			);

			// Verify access permission (owner or has access grant)
			ensure!(
				Self::check_access_permission(&who, &record_id)?,
				Error::<T>::NoAccessPermission
			);

			let now = frame_system::Pallet::<T>::block_number();
			let operation: AccessOperation = operation_id.into();

			let log_entry = AccessLog {
				record_id,
				accessor: who.clone(),
				accessed_at: now,
				operation: operation.clone(),
			};

			let log_index = AccessLogCount::<T>::get(&record_id);
			AccessLogs::<T>::insert(&record_id, log_index, log_entry);
			AccessLogCount::<T>::insert(&record_id, log_index + 1);

			Self::deposit_event(Event::AccessLogged {
				record_id,
				accessor: who,
				operation: operation_id,
			});

			Ok(())
		}

		/// Add tags to a health record
		///
		/// # Arguments
		/// * `record_id` - Record to add tags to
		/// * `new_tags` - Tags to add
		#[pallet::call_index(7)]
		#[pallet::weight(<T as Config>::WeightInfo::add_tags())]
		pub fn add_tags(
			origin: OriginFor<T>,
			record_id: RecordId,
			new_tags: BoundedVec<BoundedVec<u8, ConstU32<MAX_TAG_LENGTH>>, ConstU32<MAX_TAGS_PER_RECORD>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Records::<T>::try_mutate(&record_id, |maybe_record| {
				let record = maybe_record.as_mut().ok_or(Error::<T>::RecordNotFound)?;

				// Only owner can add tags
				ensure!(record.owner == who, Error::<T>::NotRecordOwner);

				let mut tag_count = 0u32;
				for tag in new_tags.iter() {
					if !record.tags.contains(tag) {
						record.tags.try_push(tag.clone())
							.map_err(|_| Error::<T>::TooManyTags)?;
						tag_count += 1;
					}
				}

				record.updated_at = frame_system::Pallet::<T>::block_number();

				Ok::<u32, DispatchError>(tag_count)
			}).map(|tag_count| {
				Self::deposit_event(Event::TagsAdded {
					record_id,
					tag_count,
				});
			})
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		/// Check if an account has permission to access a record
		pub fn check_access_permission(
			account: &T::AccountId,
			record_id: &RecordId,
		) -> Result<bool, DispatchError> {
			if let Some(record) = Records::<T>::get(record_id) {
				// Owner always has access
				if &record.owner == account {
					return Ok(true);
				}

				// Check access grant
				if let Some(grant) = AccessGrants::<T>::get(record_id, account) {
					// Check if access has expired
					if let Some(expires_at) = grant.expires_at {
						let now = frame_system::Pallet::<T>::block_number();
						if now > expires_at {
							return Ok(false);
						}
					}
					return Ok(true);
				}

				Ok(false)
			} else {
				Err(Error::<T>::RecordNotFound.into())
			}
		}

		/// Check if an account has permission to modify a record
		pub fn check_modify_permission(
			account: &T::AccountId,
			record_id: &RecordId,
		) -> Result<bool, DispatchError> {
			if let Some(record) = Records::<T>::get(record_id) {
				// Owner always can modify
				if &record.owner == account {
					return Ok(true);
				}

				// Check if has access grant with modify permission
				if let Some(grant) = AccessGrants::<T>::get(record_id, account) {
					// Check expiration
					if let Some(expires_at) = grant.expires_at {
						let now = frame_system::Pallet::<T>::block_number();
						if now > expires_at {
							return Ok(false);
						}
					}
					return Ok(grant.can_modify);
				}

				Ok(false)
			} else {
				Err(Error::<T>::RecordNotFound.into())
			}
		}

		/// Check if an account has permission to share a record
		pub fn check_share_permission(
			account: &T::AccountId,
			record_id: &RecordId,
		) -> Result<bool, DispatchError> {
			if let Some(record) = Records::<T>::get(record_id) {
				// Owner always can share
				if &record.owner == account {
					return Ok(true);
				}

				// Check if has access grant with share permission
				if let Some(grant) = AccessGrants::<T>::get(record_id, account) {
					// Check expiration
					if let Some(expires_at) = grant.expires_at {
						let now = frame_system::Pallet::<T>::block_number();
						if now > expires_at {
							return Ok(false);
						}
					}
					return Ok(grant.can_share);
				}

				Ok(false)
			} else {
				Err(Error::<T>::RecordNotFound.into())
			}
		}

		/// Get all access grants for a record
		pub fn get_record_access_grants(
			record_id: &RecordId,
		) -> Vec<(T::AccountId, AccessGrant<T>)> {
			let access_list = RecordAccessList::<T>::get(record_id);
			let mut grants = Vec::new();

			for account in access_list.iter() {
				if let Some(grant) = AccessGrants::<T>::get(record_id, account) {
					grants.push((account.clone(), grant));
				}
			}

			grants
		}

		/// Get all versions of a record
		pub fn get_record_versions(
			record_id: &RecordId,
		) -> Vec<VersionMetadata<T>> {
			if let Some(record) = Records::<T>::get(record_id) {
				let mut versions = Vec::new();
				for version in 1..=record.current_version {
					if let Some(version_meta) = RecordVersions::<T>::get(record_id, version) {
						versions.push(version_meta);
					}
				}
				versions
			} else {
				Vec::new()
			}
		}
	}
}
