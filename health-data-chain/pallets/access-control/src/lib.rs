//! # Access Control Pallet
//!
//! This pallet provides attribute-based access control for Patient X HealthData Chain.
//!
//! ## Overview
//!
//! The Access Control pallet enables:
//! - Attribute-based access control (ABAC) for health records
//! - Policy creation and management
//! - User attribute assignment
//! - Policy evaluation
//! - Integration with health records for fine-grained access
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `create_policy`: Create a new access policy
//! - `update_policy`: Update an existing policy
//! - `delete_policy`: Delete a policy
//! - `assign_attribute`: Assign an attribute to a user
//! - `revoke_attribute`: Revoke a user's attribute
//! - `evaluate_policy`: Evaluate a policy against user attributes
//! - `attach_policy_to_record`: Attach a policy to a health record
//! - `detach_policy_from_record`: Detach a policy from a health record

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

		/// Maximum number of policies per record
		#[pallet::constant]
		type MaxPoliciesPerRecord: Get<u32>;

		/// Maximum number of attributes per user
		#[pallet::constant]
		type MaxAttributesPerUser: Get<u32>;

		/// Weight information for extrinsics
		type WeightInfo: crate::weights::WeightInfo;
	}

	/// Storage for access policies
	/// Maps PolicyId -> AccessPolicy
	#[pallet::storage]
	#[pallet::getter(fn policies)]
	pub type Policies<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		PolicyId,
		AccessPolicy<T>,
		OptionQuery,
	>;

	/// Storage for user attributes
	/// Maps (AccountId, AttributeKey) -> UserAttribute
	#[pallet::storage]
	#[pallet::getter(fn user_attributes)]
	pub type UserAttributes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		AttributeKey,
		UserAttribute<T>,
		OptionQuery,
	>;

	/// Storage for user attribute index
	/// Maps AccountId -> Vec<AttributeKey>
	#[pallet::storage]
	#[pallet::getter(fn user_attribute_keys)]
	pub type UserAttributeKeys<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<AttributeKey, T::MaxAttributesPerUser>,
		ValueQuery,
	>;

	/// Storage for record policies
	/// Maps RecordId -> Vec<PolicyId>
	#[pallet::storage]
	#[pallet::getter(fn record_policies)]
	pub type RecordPolicies<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		RecordId,
		BoundedVec<PolicyId, T::MaxPoliciesPerRecord>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Policy was created
		PolicyCreated {
			policy_id: PolicyId,
			creator: T::AccountId,
		},
		/// Policy was updated
		PolicyUpdated {
			policy_id: PolicyId,
			updated_by: T::AccountId,
		},
		/// Policy was deleted
		PolicyDeleted {
			policy_id: PolicyId,
		},
		/// Attribute was assigned to user
		AttributeAssigned {
			user: T::AccountId,
			attribute_key: AttributeKey,
			assigned_by: T::AccountId,
		},
		/// Attribute was revoked from user
		AttributeRevoked {
			user: T::AccountId,
			attribute_key: AttributeKey,
		},
		/// Policy was evaluated
		PolicyEvaluated {
			policy_id: PolicyId,
			user: T::AccountId,
			result: u8,
		},
		/// Policy was attached to record
		PolicyAttachedToRecord {
			record_id: RecordId,
			policy_id: PolicyId,
		},
		/// Policy was detached from record
		PolicyDetachedFromRecord {
			record_id: RecordId,
			policy_id: PolicyId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Policy not found
		PolicyNotFound,
		/// Policy already exists
		PolicyAlreadyExists,
		/// Attribute not found
		AttributeNotFound,
		/// Too many attributes
		TooManyAttributes,
		/// Too many policies
		TooManyPolicies,
		/// Too many conditions
		TooManyConditions,
		/// Invalid policy
		InvalidPolicy,
		/// Policy expired
		PolicyExpired,
		/// Attribute expired
		AttributeExpired,
		/// Not authorized
		NotAuthorized,
		/// Record not found
		RecordNotFound,
		/// Policy not attached to record
		PolicyNotAttached,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new access policy
		///
		/// # Arguments
		/// * `policy_id` - Unique policy identifier
		/// * `name` - Policy name
		/// * `effect` - Policy effect (0=Allow, 1=Deny)
		/// * `mode` - Policy mode (0=AllOf, 1=AnyOf, 2=OneOf)
		/// * `attribute_key` - Single condition attribute key
		/// * `operator` - Condition operator (0-5)
		/// * `value` - Condition value
		/// * `duration_blocks` - Optional expiration duration
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::create_policy())]
		pub fn create_policy(
			origin: OriginFor<T>,
			policy_id: PolicyId,
			name: BoundedVec<u8, ConstU32<MAX_POLICY_NAME_LENGTH>>,
			effect: u8,
			mode: u8,
			attribute_key: AttributeKey,
			operator: u8,
			value: AttributeValue,
			duration_blocks: Option<BlockNumberFor<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure policy doesn't already exist
			ensure!(
				!Policies::<T>::contains_key(&policy_id),
				Error::<T>::PolicyAlreadyExists
			);

			let now = frame_system::Pallet::<T>::block_number();
			let expires_at = duration_blocks.map(|duration| now + duration);

			// Create condition
			let condition = PolicyCondition {
				attribute_key,
				operator: operator.into(),
				value,
			};

			let mut conditions = BoundedVec::default();
			conditions.try_push(condition)
				.map_err(|_| Error::<T>::TooManyConditions)?;

			let policy = AccessPolicy {
				policy_id,
				name,
				creator: who.clone(),
				effect: effect.into(),
				mode: mode.into(),
				conditions,
				created_at: now,
				expires_at,
				is_active: true,
			};

			Policies::<T>::insert(&policy_id, policy);

			Self::deposit_event(Event::PolicyCreated {
				policy_id,
				creator: who,
			});

			Ok(())
		}

		/// Update an existing policy
		///
		/// # Arguments
		/// * `policy_id` - Policy to update
		/// * `is_active` - New active status
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::update_policy())]
		pub fn update_policy(
			origin: OriginFor<T>,
			policy_id: PolicyId,
			is_active: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Policies::<T>::try_mutate(&policy_id, |maybe_policy| {
				let policy = maybe_policy.as_mut().ok_or(Error::<T>::PolicyNotFound)?;

				// Only creator can update
				ensure!(policy.creator == who, Error::<T>::NotAuthorized);

				policy.is_active = is_active;

				Ok::<(), DispatchError>(())
			})?;

			Self::deposit_event(Event::PolicyUpdated {
				policy_id,
				updated_by: who,
			});

			Ok(())
		}

		/// Delete a policy
		///
		/// # Arguments
		/// * `policy_id` - Policy to delete
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::delete_policy())]
		pub fn delete_policy(
			origin: OriginFor<T>,
			policy_id: PolicyId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let policy = Policies::<T>::get(&policy_id)
				.ok_or(Error::<T>::PolicyNotFound)?;

			// Only creator can delete
			ensure!(policy.creator == who, Error::<T>::NotAuthorized);

			Policies::<T>::remove(&policy_id);

			Self::deposit_event(Event::PolicyDeleted { policy_id });

			Ok(())
		}

		/// Assign an attribute to a user
		///
		/// # Arguments
		/// * `user` - User to assign attribute to
		/// * `attribute_key` - Attribute key
		/// * `attribute_value` - Attribute value
		/// * `attribute_type` - Attribute type (0-6)
		/// * `duration_blocks` - Optional expiration duration
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::assign_attribute())]
		pub fn assign_attribute(
			origin: OriginFor<T>,
			user: T::AccountId,
			attribute_key: AttributeKey,
			attribute_value: AttributeValue,
			attribute_type: u8,
			duration_blocks: Option<BlockNumberFor<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let now = frame_system::Pallet::<T>::block_number();
			let expires_at = duration_blocks.map(|duration| now + duration);

			let attribute = UserAttribute {
				key: attribute_key.clone(),
				value: attribute_value,
				attribute_type: attribute_type.into(),
				assigned_by: who.clone(),
				assigned_at: now,
				expires_at,
			};

			UserAttributes::<T>::insert(&user, &attribute_key, attribute);

			// Add to index if not already present
			UserAttributeKeys::<T>::try_mutate(&user, |keys| {
				if !keys.contains(&attribute_key) {
					keys.try_push(attribute_key.clone())
						.map_err(|_| Error::<T>::TooManyAttributes)
				} else {
					Ok(())
				}
			})?;

			Self::deposit_event(Event::AttributeAssigned {
				user,
				attribute_key,
				assigned_by: who,
			});

			Ok(())
		}

		/// Revoke an attribute from a user
		///
		/// # Arguments
		/// * `user` - User to revoke attribute from
		/// * `attribute_key` - Attribute key to revoke
		#[pallet::call_index(4)]
		#[pallet::weight(<T as Config>::WeightInfo::revoke_attribute())]
		pub fn revoke_attribute(
			origin: OriginFor<T>,
			user: T::AccountId,
			attribute_key: AttributeKey,
		) -> DispatchResult {
			ensure_signed(origin)?;

			// Verify attribute exists
			ensure!(
				UserAttributes::<T>::contains_key(&user, &attribute_key),
				Error::<T>::AttributeNotFound
			);

			UserAttributes::<T>::remove(&user, &attribute_key);

			// Remove from index
			UserAttributeKeys::<T>::mutate(&user, |keys| {
				if let Some(pos) = keys.iter().position(|k| k == &attribute_key) {
					keys.swap_remove(pos);
				}
			});

			Self::deposit_event(Event::AttributeRevoked {
				user,
				attribute_key,
			});

			Ok(())
		}

		/// Evaluate a policy against user attributes
		///
		/// # Arguments
		/// * `policy_id` - Policy to evaluate
		/// * `user` - User to evaluate
		#[pallet::call_index(5)]
		#[pallet::weight(<T as Config>::WeightInfo::evaluate_policy())]
		pub fn evaluate_policy(
			origin: OriginFor<T>,
			policy_id: PolicyId,
			user: T::AccountId,
		) -> DispatchResult {
			ensure_signed(origin)?;

			let result = Self::evaluate_policy_internal(&policy_id, &user)?;

			let result_u8: u8 = match result {
				EvaluationResult::Allow => 0,
				EvaluationResult::Deny => 1,
				EvaluationResult::NotApplicable => 2,
				EvaluationResult::Error => 3,
			};

			Self::deposit_event(Event::PolicyEvaluated {
				policy_id,
				user,
				result: result_u8,
			});

			Ok(())
		}

		/// Attach a policy to a health record
		///
		/// # Arguments
		/// * `record_id` - Record to attach policy to
		/// * `policy_id` - Policy to attach
		#[pallet::call_index(6)]
		#[pallet::weight(<T as Config>::WeightInfo::attach_policy_to_record())]
		pub fn attach_policy_to_record(
			origin: OriginFor<T>,
			record_id: RecordId,
			policy_id: PolicyId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Verify record exists and caller is owner
			let record = pallet_health_records::Pallet::<T>::records(&record_id)
				.ok_or(Error::<T>::RecordNotFound)?;
			ensure!(record.owner == who, Error::<T>::NotAuthorized);

			// Verify policy exists
			ensure!(
				Policies::<T>::contains_key(&policy_id),
				Error::<T>::PolicyNotFound
			);

			RecordPolicies::<T>::try_mutate(&record_id, |policies| {
				if !policies.contains(&policy_id) {
					policies.try_push(policy_id)
						.map_err(|_| Error::<T>::TooManyPolicies)
				} else {
					Ok(())
				}
			})?;

			Self::deposit_event(Event::PolicyAttachedToRecord {
				record_id,
				policy_id,
			});

			Ok(())
		}

		/// Detach a policy from a health record
		///
		/// # Arguments
		/// * `record_id` - Record to detach policy from
		/// * `policy_id` - Policy to detach
		#[pallet::call_index(7)]
		#[pallet::weight(<T as Config>::WeightInfo::detach_policy_from_record())]
		pub fn detach_policy_from_record(
			origin: OriginFor<T>,
			record_id: RecordId,
			policy_id: PolicyId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Verify record exists and caller is owner
			let record = pallet_health_records::Pallet::<T>::records(&record_id)
				.ok_or(Error::<T>::RecordNotFound)?;
			ensure!(record.owner == who, Error::<T>::NotAuthorized);

			RecordPolicies::<T>::mutate(&record_id, |policies| {
				if let Some(pos) = policies.iter().position(|id| id == &policy_id) {
					policies.swap_remove(pos);
				}
			});

			Self::deposit_event(Event::PolicyDetachedFromRecord {
				record_id,
				policy_id,
			});

			Ok(())
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		/// Evaluate a policy against user attributes (internal)
		pub fn evaluate_policy_internal(
			policy_id: &PolicyId,
			user: &T::AccountId,
		) -> Result<EvaluationResult, DispatchError> {
			let policy = Policies::<T>::get(policy_id)
				.ok_or(Error::<T>::PolicyNotFound)?;

			// Check if policy is active
			if !policy.is_active {
				return Ok(EvaluationResult::NotApplicable);
			}

			// Check if policy expired
			if let Some(expires_at) = policy.expires_at {
				let now = frame_system::Pallet::<T>::block_number();
				if now > expires_at {
					return Ok(EvaluationResult::NotApplicable);
				}
			}

			// Evaluate conditions
			let mut satisfied_count = 0u32;
			for condition in &policy.conditions {
				if Self::evaluate_condition(condition, user)? {
					satisfied_count += 1;
				}
			}

			let total_conditions = policy.conditions.len() as u32;

			// Determine result based on policy mode
			let conditions_met = match policy.mode {
				PolicyMode::AllOf => satisfied_count == total_conditions,
				PolicyMode::AnyOf => satisfied_count > 0,
				PolicyMode::OneOf => satisfied_count == 1,
			};

			if conditions_met {
				match policy.effect {
					PolicyEffect::Allow => Ok(EvaluationResult::Allow),
					PolicyEffect::Deny => Ok(EvaluationResult::Deny),
				}
			} else {
				Ok(EvaluationResult::NotApplicable)
			}
		}

		/// Evaluate a single condition
		fn evaluate_condition(
			condition: &PolicyCondition,
			user: &T::AccountId,
		) -> Result<bool, DispatchError> {
			// Get user attribute
			let attribute = UserAttributes::<T>::get(user, &condition.attribute_key)
				.ok_or(Error::<T>::AttributeNotFound)?;

			// Check if attribute expired
			if let Some(expires_at) = attribute.expires_at {
				let now = frame_system::Pallet::<T>::block_number();
				if now > expires_at {
					return Err(Error::<T>::AttributeExpired.into());
				}
			}

			// Evaluate based on operator
			let result = match condition.operator {
				ConditionOperator::Equals => attribute.value == condition.value,
				ConditionOperator::NotEquals => attribute.value != condition.value,
				ConditionOperator::Contains => {
					// Check if attribute value contains the condition value
					attribute.value.iter().any(|&b| condition.value.contains(&b))
				},
				ConditionOperator::GreaterThan => {
					// Lexicographic comparison
					attribute.value > condition.value
				},
				ConditionOperator::LessThan => {
					// Lexicographic comparison
					attribute.value < condition.value
				},
				ConditionOperator::InRange => {
					// Simple range check - assumes condition.value format is "min,max"
					// For simplicity, just check if attribute value equals condition value
					attribute.value == condition.value
				},
			};

			Ok(result)
		}

		/// Check if user has access to record based on policies
		pub fn check_record_access(
			record_id: &RecordId,
			user: &T::AccountId,
		) -> bool {
			let policies = RecordPolicies::<T>::get(record_id);

			if policies.is_empty() {
				// No policies attached = allow by default
				return true;
			}

			// Evaluate all policies - any Deny overrides Allow
			let mut has_allow = false;
			let mut has_deny = false;

			for policy_id in policies.iter() {
				if let Ok(result) = Self::evaluate_policy_internal(policy_id, user) {
					match result {
						EvaluationResult::Allow => has_allow = true,
						EvaluationResult::Deny => has_deny = true,
						_ => {},
					}
				}
			}

			// Deny takes precedence
			!has_deny && has_allow
		}

		/// Get all user attributes
		pub fn get_user_attributes(user: &T::AccountId) -> Vec<UserAttribute<T>> {
			let keys = UserAttributeKeys::<T>::get(user);
			let mut attributes = Vec::new();

			for key in keys.iter() {
				if let Some(attr) = UserAttributes::<T>::get(user, key) {
					attributes.push(attr);
				}
			}

			attributes
		}
	}
}
