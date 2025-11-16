//! # Jurisdiction Manager Pallet
//!
//! This pallet manages jurisdiction mapping and regulatory compliance for Patient X.
//!
//! ## Overview
//!
//! The Jurisdiction Manager enables:
//! - Jurisdiction registration and management (country/region codes)
//! - Compliance rule storage for different regulatory frameworks
//! - Cross-border data transfer validation
//! - Data residency enforcement
//! - Adequacy decision tracking
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `register_jurisdiction`: Register a new jurisdiction with compliance info
//! - `update_jurisdiction`: Update jurisdiction metadata
//! - `deactivate_jurisdiction`: Deactivate a jurisdiction
//! - `set_compliance_rule`: Set compliance rule for a specific regulation
//! - `update_compliance_rule`: Update existing compliance rule
//! - `add_approved_transfer_target`: Add approved jurisdiction for data transfer
//! - `remove_approved_transfer_target`: Remove approved transfer target
//! - `validate_transfer`: Validate cross-border data transfer request

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
	pub trait Config: frame_system::Config + pallet_identity_registry::Config + pallet_authentication::Config {
		/// The overarching event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum number of approved jurisdictions per jurisdiction
		#[pallet::constant]
		type MaxApprovedJurisdictions: Get<u32>;

		/// Maximum number of total jurisdictions in the system
		#[pallet::constant]
		type MaxJurisdictions: Get<u32>;

		/// Weight information for extrinsics
		type WeightInfo: crate::weights::WeightInfo;
	}

	/// Storage for jurisdiction information
	/// Maps JurisdictionCode -> JurisdictionInfo
	#[pallet::storage]
	#[pallet::getter(fn jurisdictions)]
	pub type Jurisdictions<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		JurisdictionCode,
		JurisdictionInfo<T>,
		OptionQuery,
	>;

	/// Storage for compliance rules
	/// Maps (JurisdictionCode, RegulationType) -> ComplianceRule
	#[pallet::storage]
	#[pallet::getter(fn compliance_rules)]
	pub type ComplianceRules<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		JurisdictionCode,
		Blake2_128Concat,
		RegulationType,
		ComplianceRule<T>,
		OptionQuery,
	>;

	/// Index of all registered jurisdictions
	#[pallet::storage]
	#[pallet::getter(fn jurisdiction_index)]
	pub type JurisdictionIndex<T: Config> =
		StorageValue<_, BoundedVec<JurisdictionCode, T::MaxJurisdictions>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Jurisdiction was registered
		JurisdictionRegistered {
			code: JurisdictionCode,
			registered_by: T::AccountId,
		},
		/// Jurisdiction was updated
		JurisdictionUpdated {
			code: JurisdictionCode,
			updated_by: T::AccountId,
		},
		/// Jurisdiction was deactivated
		JurisdictionDeactivated {
			code: JurisdictionCode,
			deactivated_by: T::AccountId,
		},
		/// Compliance rule was set
		ComplianceRuleSet {
			jurisdiction: JurisdictionCode,
			regulation_id: u8,
			set_by: T::AccountId,
		},
		/// Compliance rule was updated
		ComplianceRuleUpdated {
			jurisdiction: JurisdictionCode,
			regulation_id: u8,
			updated_by: T::AccountId,
		},
		/// Approved transfer target was added
		ApprovedTargetAdded {
			jurisdiction: JurisdictionCode,
			target: JurisdictionCode,
		},
		/// Approved transfer target was removed
		ApprovedTargetRemoved {
			jurisdiction: JurisdictionCode,
			target: JurisdictionCode,
		},
		/// Transfer was validated
		TransferValidated {
			from_jurisdiction: JurisdictionCode,
			to_jurisdiction: JurisdictionCode,
			validation_result: u8,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Jurisdiction not found
		JurisdictionNotFound,
		/// Jurisdiction already exists
		JurisdictionAlreadyExists,
		/// Jurisdiction is not active
		JurisdictionNotActive,
		/// Compliance rule not found
		ComplianceRuleNotFound,
		/// Invalid jurisdiction code
		InvalidJurisdictionCode,
		/// Too many jurisdictions
		TooManyJurisdictions,
		/// Too many approved transfer targets
		TooManyApprovedTargets,
		/// Not authorized to perform this action
		NotAuthorized,
		/// Cross-border transfer is prohibited
		TransferProhibited,
		/// Target jurisdiction not approved
		TargetNotApproved,
		/// Adequacy decision required
		AdequacyRequired,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register a new jurisdiction
		///
		/// # Arguments
		/// * `jurisdiction_code` - ISO 3166-1 alpha-2 country code (2 bytes)
		/// * `metadata` - Optional metadata (JSON or other format)
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::register_jurisdiction())]
		pub fn register_jurisdiction(
			origin: OriginFor<T>,
			jurisdiction_code: BoundedVec<u8, ConstU32<2>>,
			metadata: Option<BoundedVec<u8, ConstU32<MAX_JURISDICTION_METADATA_LENGTH>>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check if caller has permission (requires Administrator or Regulator role)
			ensure!(
				pallet_authentication::Pallet::<T>::has_permission(&who, pallet_authentication::Permission::ManageCompliance),
				Error::<T>::NotAuthorized
			);

			// Validate jurisdiction code (should be 2 bytes)
			ensure!(
				jurisdiction_code.len() == 2,
				Error::<T>::InvalidJurisdictionCode
			);

			// Check if jurisdiction already exists
			ensure!(
				!Jurisdictions::<T>::contains_key(&jurisdiction_code),
				Error::<T>::JurisdictionAlreadyExists
			);

			let now = frame_system::Pallet::<T>::block_number();

			let jurisdiction_info = JurisdictionInfo {
				code: jurisdiction_code.clone(),
				regulations: BoundedVec::default(),
				approved_transfer_targets: BoundedVec::default(),
				is_active: true,
				metadata,
				registered_at: now,
				updated_at: now,
			};

			Jurisdictions::<T>::insert(&jurisdiction_code, jurisdiction_info);

			// Add to index
			JurisdictionIndex::<T>::try_mutate(|index| {
				index.try_push(jurisdiction_code.clone())
					.map_err(|_| Error::<T>::TooManyJurisdictions)
			})?;

			Self::deposit_event(Event::JurisdictionRegistered {
				code: jurisdiction_code,
				registered_by: who,
			});

			Ok(())
		}

		/// Update jurisdiction metadata
		///
		/// # Arguments
		/// * `jurisdiction_code` - The jurisdiction to update
		/// * `metadata` - New metadata
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::update_jurisdiction())]
		pub fn update_jurisdiction(
			origin: OriginFor<T>,
			jurisdiction_code: BoundedVec<u8, ConstU32<2>>,
			metadata: Option<BoundedVec<u8, ConstU32<MAX_JURISDICTION_METADATA_LENGTH>>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check permission
			ensure!(
				pallet_authentication::Pallet::<T>::has_permission(&who, pallet_authentication::Permission::ManageCompliance),
				Error::<T>::NotAuthorized
			);

			Jurisdictions::<T>::try_mutate(&jurisdiction_code, |maybe_info| {
				let info = maybe_info.as_mut().ok_or(Error::<T>::JurisdictionNotFound)?;
				
				info.metadata = metadata;
				info.updated_at = frame_system::Pallet::<T>::block_number();

				Ok::<(), DispatchError>(())
			})?;

			Self::deposit_event(Event::JurisdictionUpdated {
				code: jurisdiction_code,
				updated_by: who,
			});

			Ok(())
		}

		/// Deactivate a jurisdiction
		///
		/// # Arguments
		/// * `jurisdiction_code` - The jurisdiction to deactivate
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::deactivate_jurisdiction())]
		pub fn deactivate_jurisdiction(
			origin: OriginFor<T>,
			jurisdiction_code: BoundedVec<u8, ConstU32<2>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check permission
			ensure!(
				pallet_authentication::Pallet::<T>::has_permission(&who, pallet_authentication::Permission::ManageCompliance),
				Error::<T>::NotAuthorized
			);

			Jurisdictions::<T>::try_mutate(&jurisdiction_code, |maybe_info| {
				let info = maybe_info.as_mut().ok_or(Error::<T>::JurisdictionNotFound)?;
				
				info.is_active = false;
				info.updated_at = frame_system::Pallet::<T>::block_number();

				Ok::<(), DispatchError>(())
			})?;

			Self::deposit_event(Event::JurisdictionDeactivated {
				code: jurisdiction_code,
				deactivated_by: who,
			});

			Ok(())
		}

		/// Set compliance rule for a specific regulation in a jurisdiction
		///
		/// # Arguments
		/// * `jurisdiction_code` - The jurisdiction
		/// * `regulation_id` - The regulation type (0-10)
		/// * `data_residency_type` - Data residency requirement (0-2)
		/// * `transfer_restriction_type` - Transfer restriction type (0-4)
		/// * `min_consent_age` - Minimum age for consent
		/// * `explicit_consent_required` - Whether explicit consent is required
		/// * `right_to_erasure` - Whether right to erasure is supported
		/// * `data_portability_required` - Whether data portability is required
		/// * `breach_notification_required` - Whether breach notification is mandatory
		/// * `max_sar_response_days` - Maximum days to respond to subject access requests
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::set_compliance_rule())]
		pub fn set_compliance_rule(
			origin: OriginFor<T>,
			jurisdiction_code: BoundedVec<u8, ConstU32<2>>,
			regulation_id: u8,
			data_residency_type: u8,
			transfer_restriction_type: u8,
			min_consent_age: u8,
			explicit_consent_required: bool,
			right_to_erasure: bool,
			data_portability_required: bool,
			breach_notification_required: bool,
			max_sar_response_days: u8,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check permission
			ensure!(
				pallet_authentication::Pallet::<T>::has_permission(&who, pallet_authentication::Permission::ManageCompliance),
				Error::<T>::NotAuthorized
			);

			// Ensure jurisdiction exists
			let mut jurisdiction = Jurisdictions::<T>::get(&jurisdiction_code)
				.ok_or(Error::<T>::JurisdictionNotFound)?;

			let regulation: RegulationType = regulation_id.into();
			let data_residency: DataResidencyRequirement = data_residency_type.into();
			let transfer_restriction: TransferRestriction = transfer_restriction_type.into();

			let rule = ComplianceRule {
				regulation: regulation.clone(),
				data_residency,
				transfer_restriction,
				min_consent_age,
				explicit_consent_required,
				right_to_erasure,
				data_portability_required,
				breach_notification_required,
				max_sar_response_days,
				updated_at: frame_system::Pallet::<T>::block_number(),
			};

			ComplianceRules::<T>::insert(&jurisdiction_code, &regulation, rule);

			// Add regulation to jurisdiction if not already present
			if !jurisdiction.regulations.contains(&regulation) {
				jurisdiction.regulations.try_push(regulation.clone())
					.map_err(|_| Error::<T>::TooManyJurisdictions)?;
				jurisdiction.updated_at = frame_system::Pallet::<T>::block_number();
				Jurisdictions::<T>::insert(&jurisdiction_code, jurisdiction);
			}

			Self::deposit_event(Event::ComplianceRuleSet {
				jurisdiction: jurisdiction_code,
				regulation_id,
				set_by: who,
			});

			Ok(())
		}

		/// Update existing compliance rule
		///
		/// # Arguments
		/// * `jurisdiction_code` - The jurisdiction
		/// * `regulation_id` - The regulation type
		/// * `max_sar_response_days` - New maximum days for SAR response
		#[pallet::call_index(4)]
		#[pallet::weight(<T as Config>::WeightInfo::update_compliance_rule())]
		pub fn update_compliance_rule(
			origin: OriginFor<T>,
			jurisdiction_code: BoundedVec<u8, ConstU32<2>>,
			regulation_id: u8,
			max_sar_response_days: u8,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check permission
			ensure!(
				pallet_authentication::Pallet::<T>::has_permission(&who, pallet_authentication::Permission::ManageCompliance),
				Error::<T>::NotAuthorized
			);

			let regulation: RegulationType = regulation_id.into();

			ComplianceRules::<T>::try_mutate(&jurisdiction_code, &regulation, |maybe_rule| {
				let rule = maybe_rule.as_mut().ok_or(Error::<T>::ComplianceRuleNotFound)?;
				
				rule.max_sar_response_days = max_sar_response_days;
				rule.updated_at = frame_system::Pallet::<T>::block_number();

				Ok::<(), DispatchError>(())
			})?;

			Self::deposit_event(Event::ComplianceRuleUpdated {
				jurisdiction: jurisdiction_code,
				regulation_id,
				updated_by: who,
			});

			Ok(())
		}

		/// Add an approved jurisdiction for data transfer
		///
		/// # Arguments
		/// * `jurisdiction_code` - The source jurisdiction
		/// * `target_jurisdiction` - The target jurisdiction to approve
		#[pallet::call_index(5)]
		#[pallet::weight(<T as Config>::WeightInfo::add_approved_transfer_target())]
		pub fn add_approved_transfer_target(
			origin: OriginFor<T>,
			jurisdiction_code: BoundedVec<u8, ConstU32<2>>,
			target_jurisdiction: BoundedVec<u8, ConstU32<2>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check permission
			ensure!(
				pallet_authentication::Pallet::<T>::has_permission(&who, pallet_authentication::Permission::ManageCompliance),
				Error::<T>::NotAuthorized
			);

			// Ensure both jurisdictions exist
			ensure!(
				Jurisdictions::<T>::contains_key(&jurisdiction_code),
				Error::<T>::JurisdictionNotFound
			);
			ensure!(
				Jurisdictions::<T>::contains_key(&target_jurisdiction),
				Error::<T>::JurisdictionNotFound
			);

			Jurisdictions::<T>::try_mutate(&jurisdiction_code, |maybe_info| {
				let info = maybe_info.as_mut().ok_or(Error::<T>::JurisdictionNotFound)?;
				
				// Add target if not already present
				if !info.approved_transfer_targets.contains(&target_jurisdiction) {
					info.approved_transfer_targets.try_push(target_jurisdiction.clone())
						.map_err(|_| Error::<T>::TooManyApprovedTargets)?;
					info.updated_at = frame_system::Pallet::<T>::block_number();
				}

				Ok::<(), DispatchError>(())
			})?;

			Self::deposit_event(Event::ApprovedTargetAdded {
				jurisdiction: jurisdiction_code,
				target: target_jurisdiction,
			});

			Ok(())
		}

		/// Remove an approved jurisdiction for data transfer
		///
		/// # Arguments
		/// * `jurisdiction_code` - The source jurisdiction
		/// * `target_jurisdiction` - The target jurisdiction to remove
		#[pallet::call_index(6)]
		#[pallet::weight(<T as Config>::WeightInfo::remove_approved_transfer_target())]
		pub fn remove_approved_transfer_target(
			origin: OriginFor<T>,
			jurisdiction_code: BoundedVec<u8, ConstU32<2>>,
			target_jurisdiction: BoundedVec<u8, ConstU32<2>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check permission
			ensure!(
				pallet_authentication::Pallet::<T>::has_permission(&who, pallet_authentication::Permission::ManageCompliance),
				Error::<T>::NotAuthorized
			);

			Jurisdictions::<T>::try_mutate(&jurisdiction_code, |maybe_info| {
				let info = maybe_info.as_mut().ok_or(Error::<T>::JurisdictionNotFound)?;
				
				// Remove target if present
				if let Some(pos) = info.approved_transfer_targets.iter().position(|x| x == &target_jurisdiction) {
					info.approved_transfer_targets.swap_remove(pos);
					info.updated_at = frame_system::Pallet::<T>::block_number();
				}

				Ok::<(), DispatchError>(())
			})?;

			Self::deposit_event(Event::ApprovedTargetRemoved {
				jurisdiction: jurisdiction_code,
				target: target_jurisdiction,
			});

			Ok(())
		}

		/// Validate a cross-border data transfer request
		///
		/// # Arguments
		/// * `from_jurisdiction` - Source jurisdiction
		/// * `to_jurisdiction` - Target jurisdiction
		#[pallet::call_index(7)]
		#[pallet::weight(<T as Config>::WeightInfo::validate_transfer())]
		pub fn validate_transfer(
			origin: OriginFor<T>,
			from_jurisdiction: BoundedVec<u8, ConstU32<2>>,
			to_jurisdiction: BoundedVec<u8, ConstU32<2>>,
		) -> DispatchResult {
			ensure_signed(origin)?;

			let result = Self::check_transfer_allowed(&from_jurisdiction, &to_jurisdiction)?;

			let result_u8 = match result {
				TransferValidationResult::Allowed => 0,
				TransferValidationResult::ConsentRequired => 1,
				TransferValidationResult::AdequacyRequired => 2,
				TransferValidationResult::SCCRequired => 3,
				TransferValidationResult::Prohibited => 4,
			};

			Self::deposit_event(Event::TransferValidated {
				from_jurisdiction,
				to_jurisdiction,
				validation_result: result_u8,
			});

			// Fail if transfer is prohibited
			if matches!(result, TransferValidationResult::Prohibited) {
				return Err(Error::<T>::TransferProhibited.into());
			}

			Ok(())
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		/// Check if a cross-border transfer is allowed
		pub fn check_transfer_allowed(
			from: &JurisdictionCode,
			to: &JurisdictionCode,
		) -> Result<TransferValidationResult, DispatchError> {
			// Get source jurisdiction
			let from_info = Jurisdictions::<T>::get(from)
				.ok_or(Error::<T>::JurisdictionNotFound)?;

			// Ensure source jurisdiction is active
			ensure!(from_info.is_active, Error::<T>::JurisdictionNotActive);

			// Get target jurisdiction
			let to_info = Jurisdictions::<T>::get(to)
				.ok_or(Error::<T>::JurisdictionNotFound)?;

			// Ensure target jurisdiction is active
			ensure!(to_info.is_active, Error::<T>::JurisdictionNotActive);

			// Check if target is in approved list
			if from_info.approved_transfer_targets.contains(to) {
				return Ok(TransferValidationResult::Allowed);
			}

			// Check compliance rules for source jurisdiction
			// If any regulation requires restrictions, apply most restrictive
			let mut most_restrictive = TransferValidationResult::Allowed;

			for regulation in &from_info.regulations {
				if let Some(rule) = ComplianceRules::<T>::get(from, regulation) {
					match rule.transfer_restriction {
						TransferRestriction::ConsentRequired => {
							if matches!(most_restrictive, TransferValidationResult::Allowed) {
								most_restrictive = TransferValidationResult::ConsentRequired;
							}
						},
						TransferRestriction::AdequacyRequired => {
							if !matches!(most_restrictive, TransferValidationResult::Prohibited) {
								most_restrictive = TransferValidationResult::AdequacyRequired;
							}
						},
						TransferRestriction::SCCRequired => {
							if !matches!(most_restrictive, TransferValidationResult::Prohibited) {
								most_restrictive = TransferValidationResult::SCCRequired;
							}
						},
						TransferRestriction::BCRRequired => {
							if !matches!(most_restrictive, TransferValidationResult::Prohibited) {
								most_restrictive = TransferValidationResult::SCCRequired;
							}
						},
						TransferRestriction::NoRestriction => {},
					}

					// Check data residency
					match rule.data_residency {
						DataResidencyRequirement::StrictLocal => {
							// Strict local means no transfer allowed
							return Ok(TransferValidationResult::Prohibited);
						},
						DataResidencyRequirement::ApprovedJurisdictions => {
							// Must be in approved list
							if !from_info.approved_transfer_targets.contains(to) {
								return Ok(TransferValidationResult::Prohibited);
							}
						},
						DataResidencyRequirement::NoRestriction => {},
					}
				}
			}

			Ok(most_restrictive)
		}

		/// Get compliance requirements for a jurisdiction
		pub fn get_compliance_requirements(
			jurisdiction: &JurisdictionCode,
		) -> Option<Vec<(RegulationType, ComplianceRule<T>)>> {
			if let Some(info) = Jurisdictions::<T>::get(jurisdiction) {
				let mut requirements = Vec::new();
				for regulation in &info.regulations {
					if let Some(rule) = ComplianceRules::<T>::get(jurisdiction, regulation) {
						requirements.push((regulation.clone(), rule));
					}
				}
				Some(requirements)
			} else {
				None
			}
		}

		/// Check if a jurisdiction has adequate data protection
		pub fn has_adequate_protection(
			from: &JurisdictionCode,
			to: &JurisdictionCode,
		) -> bool {
			if let Some(from_info) = Jurisdictions::<T>::get(from) {
				from_info.approved_transfer_targets.contains(to)
			} else {
				false
			}
		}
	}
}
