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

		/// Maximum outcomes per reporter
		#[pallet::constant]
		type MaxOutcomesPerReporter: Get<u32>;

		/// Minimum verifications required
		#[pallet::constant]
		type MinVerifications: Get<u8>;

		/// Maximum verifications allowed
		#[pallet::constant]
		type MaxVerifications: Get<u8>;
	}

	/// Storage for all negative outcomes
	#[pallet::storage]
	#[pallet::getter(fn outcomes)]
	pub type Outcomes<T: Config> =
		StorageMap<_, Blake2_128Concat, OutcomeId, NegativeOutcome<T>>;

	/// Outcomes by reporter
	#[pallet::storage]
	#[pallet::getter(fn reporter_outcomes)]
	pub type ReporterOutcomes<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<OutcomeId, ConstU32<MAX_OUTCOMES_PER_REPORTER>>,
		ValueQuery,
	>;

	/// Outcomes by type (using u8 to avoid DecodeWithMemTracking)
	#[pallet::storage]
	#[pallet::getter(fn outcomes_by_type)]
	pub type OutcomesByType<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u8,
		BoundedVec<OutcomeId, ConstU32<MAX_OUTCOMES_PER_REPORTER>>,
		ValueQuery,
	>;

	/// Outcomes by severity (using u8)
	#[pallet::storage]
	#[pallet::getter(fn outcomes_by_severity)]
	pub type OutcomesBySeverity<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u8,
		BoundedVec<OutcomeId, ConstU32<MAX_OUTCOMES_PER_REPORTER>>,
		ValueQuery,
	>;

	/// Verified outcomes list
	#[pallet::storage]
	#[pallet::getter(fn verified_outcomes)]
	pub type VerifiedOutcomes<T: Config> =
		StorageValue<_, BoundedVec<OutcomeId, ConstU32<MAX_OUTCOMES_PER_REPORTER>>, ValueQuery>;

	/// Reward pool configuration
	#[pallet::storage]
	#[pallet::getter(fn reward_pool)]
	pub type RewardPool<T: Config> = StorageValue<_, RewardPoolConfig, ValueQuery>;

	/// Registry statistics
	#[pallet::storage]
	#[pallet::getter(fn registry_stats)]
	pub type RegistryStatistics<T: Config> = StorageValue<_, RegistryStats, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A negative outcome was reported
		OutcomeReported {
			outcome_id: OutcomeId,
			reporter: T::AccountId,
			outcome_type: u8,
			severity: u8,
		},
		/// A verification was submitted
		VerificationSubmitted {
			outcome_id: OutcomeId,
			verifier: T::AccountId,
			decision: u8,
		},
		/// An outcome was verified
		OutcomeVerified {
			outcome_id: OutcomeId,
			reward_amount: u128,
		},
		/// An outcome was rejected
		OutcomeRejected { outcome_id: OutcomeId },
		/// An outcome was disputed
		OutcomeDisputed { outcome_id: OutcomeId },
		/// A reward was claimed
		RewardClaimed {
			outcome_id: OutcomeId,
			reporter: T::AccountId,
			amount: u128,
		},
		/// Reward pool was funded
		RewardPoolFunded { amount: u128, new_balance: u128 },
		/// Reward configuration updated
		RewardConfigUpdated,
		/// Evidence added to outcome
		EvidenceAdded { outcome_id: OutcomeId },
		/// Outcome linked to data listing
		LinkedToListing {
			outcome_id: OutcomeId,
			listing_id: [u8; 32],
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Outcome already exists
		OutcomeAlreadyExists,
		/// Outcome not found
		OutcomeNotFound,
		/// Not the outcome reporter
		NotReporter,
		/// Already verified by this verifier
		AlreadyVerified,
		/// Outcome already verified
		AlreadyVerifiedOutcome,
		/// Outcome already rejected
		AlreadyRejected,
		/// Cannot verify (invalid status)
		CannotVerify,
		/// Too many outcomes for reporter
		TooManyOutcomes,
		/// Too many verifications
		TooManyVerifications,
		/// Reward already claimed
		RewardAlreadyClaimed,
		/// Outcome not verified
		OutcomeNotVerified,
		/// Insufficient reward pool
		InsufficientRewardPool,
		/// Invalid outcome type
		InvalidOutcomeType,
		/// Invalid severity
		InvalidSeverity,
		/// Invalid reward tier
		InvalidRewardTier,
		/// Invalid verifier role
		InvalidVerifierRole,
		/// Invalid verification decision
		InvalidVerificationDecision,
		/// Not enough funds
		InsufficientFunds,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Report a negative health outcome
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::report_outcome())]
		pub fn report_outcome(
			origin: OriginFor<T>,
			outcome_id: OutcomeId,
			outcome_type: u8,
			severity: u8,
			product_name: Option<BoundedVec<u8, ConstU32<MAX_PRODUCT_NAME_LENGTH>>>,
			description: BoundedVec<u8, ConstU32<MAX_OUTCOME_DESCRIPTION_LENGTH>>,
			evidence: BoundedVec<u8, ConstU32<MAX_EVIDENCE_LENGTH>>,
			reward_tier: u8,
			required_verifications: u8,
			record_id: Option<[u8; 32]>,
		) -> DispatchResult {
			let reporter = ensure_signed(origin)?;

			// Ensure outcome doesn't already exist
			ensure!(!Outcomes::<T>::contains_key(outcome_id), Error::<T>::OutcomeAlreadyExists);

			// Check reporter outcome limit
			let mut reporter_outcomes = ReporterOutcomes::<T>::get(&reporter);
			ensure!(
				reporter_outcomes.len() < MAX_OUTCOMES_PER_REPORTER as usize,
				Error::<T>::TooManyOutcomes
			);

			// Validate required verifications
			ensure!(
				required_verifications >= T::MinVerifications::get() &&
					required_verifications <= T::MaxVerifications::get(),
				Error::<T>::TooManyVerifications
			);

			// Convert enums
			let outcome_type_enum = Self::u8_to_outcome_type(outcome_type)?;
			let severity_enum = Self::u8_to_severity(severity)?;
			let reward_tier_enum = Self::u8_to_reward_tier(reward_tier)?;

			// Get reward amount
			let reward_pool = RewardPool::<T>::get();
			let reward_amount = reward_pool.tier_rewards.get_reward(&reward_tier_enum);

			let current_block = frame_system::Pallet::<T>::block_number();

			// Create evidence vector
			let mut evidence_vec = BoundedVec::default();
			evidence_vec
				.try_push(evidence)
				.map_err(|_| Error::<T>::OutcomeNotFound)?; // Use generic error for now

			let outcome = NegativeOutcome {
				outcome_id,
				reporter: reporter.clone(),
				outcome_type: outcome_type_enum,
				severity: severity_enum,
				product_name,
				description,
				evidence: evidence_vec,
				verification_status: VerificationStatus::Pending,
				verifications: BoundedVec::default(),
				required_verifications,
				reward_tier: reward_tier_enum,
				reward_amount,
				reward_claimed: false,
				record_id,
				listing_id: None,
				metadata: None,
				reported_at: current_block,
				verified_at: None,
			};

			// Store outcome
			Outcomes::<T>::insert(outcome_id, outcome);

			// Update indices
			reporter_outcomes
				.try_push(outcome_id)
				.map_err(|_| Error::<T>::TooManyOutcomes)?;
			ReporterOutcomes::<T>::insert(&reporter, reporter_outcomes);

			OutcomesByType::<T>::mutate(outcome_type, |outcomes| {
				let _ = outcomes.try_push(outcome_id);
			});

			OutcomesBySeverity::<T>::mutate(severity, |outcomes| {
				let _ = outcomes.try_push(outcome_id);
			});

			// Update statistics
			RegistryStatistics::<T>::mutate(|stats| {
				stats.total_outcomes += 1;
				stats.outcomes_by_type[outcome_type as usize] += 1;
				stats.outcomes_by_severity[severity as usize] += 1;
			});

			// Reserve reward from pool
			RewardPool::<T>::mutate(|pool| {
				pool.reserved_balance += reward_amount;
			});

			Self::deposit_event(Event::OutcomeReported {
				outcome_id,
				reporter,
				outcome_type,
				severity,
			});

			Ok(())
		}

		/// Submit verification for an outcome
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::submit_verification())]
		pub fn submit_verification(
			origin: OriginFor<T>,
			outcome_id: OutcomeId,
			role: u8,
			decision: u8,
			notes: Option<BoundedVec<u8, ConstU32<MAX_VERIFICATION_NOTES_LENGTH>>>,
		) -> DispatchResult {
			let verifier = ensure_signed(origin)?;

			Outcomes::<T>::try_mutate(outcome_id, |maybe_outcome| -> DispatchResult {
				let outcome = maybe_outcome.as_mut().ok_or(Error::<T>::OutcomeNotFound)?;

				// Ensure not already verified or rejected
				ensure!(
					outcome.verification_status == VerificationStatus::Pending ||
						outcome.verification_status == VerificationStatus::UnderReview,
					Error::<T>::CannotVerify
				);

				// Ensure not already verified by this verifier
				ensure!(
					!outcome.verifications.iter().any(|v| v.verifier == verifier),
					Error::<T>::AlreadyVerified
				);

				// Convert enums
				let role_enum = Self::u8_to_verifier_role(role)?;
				let decision_enum = Self::u8_to_verification_decision(decision)?;

				let current_block = frame_system::Pallet::<T>::block_number();

				let verification = Verification {
					verifier: verifier.clone(),
					role: role_enum,
					decision: decision_enum.clone(),
					notes,
					verified_at: current_block,
				};

				outcome
					.verifications
					.try_push(verification)
					.map_err(|_| Error::<T>::TooManyVerifications)?;

				// Update status to UnderReview
				outcome.verification_status = VerificationStatus::UnderReview;

				// Check if outcome should be verified or rejected
				if outcome.is_disputed() {
					outcome.verification_status = VerificationStatus::Disputed;
					Self::deposit_event(Event::OutcomeDisputed { outcome_id });
				} else if outcome.has_enough_verifications() {
					outcome.verification_status = VerificationStatus::Verified;
					outcome.verified_at = Some(current_block);

					// Add to verified outcomes
					VerifiedOutcomes::<T>::mutate(|outcomes| {
						let _ = outcomes.try_push(outcome_id);
					});

					// Update statistics
					RegistryStatistics::<T>::mutate(|stats| {
						stats.verified_outcomes += 1;
					});

					Self::deposit_event(Event::OutcomeVerified {
						outcome_id,
						reward_amount: outcome.reward_amount,
					});
				}

				Self::deposit_event(Event::VerificationSubmitted {
					outcome_id,
					verifier,
					decision,
				});

				Ok(())
			})
		}

		/// Claim reward for a verified outcome
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::claim_reward())]
		pub fn claim_reward(origin: OriginFor<T>, outcome_id: OutcomeId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Outcomes::<T>::try_mutate(outcome_id, |maybe_outcome| -> DispatchResult {
				let outcome = maybe_outcome.as_mut().ok_or(Error::<T>::OutcomeNotFound)?;

				// Ensure caller is the reporter
				ensure!(outcome.reporter == who, Error::<T>::NotReporter);

				// Ensure outcome can claim reward
				ensure!(outcome.can_claim_reward(), Error::<T>::OutcomeNotVerified);

				// Ensure not already claimed
				ensure!(!outcome.reward_claimed, Error::<T>::RewardAlreadyClaimed);

				// Check reward pool has sufficient balance
				let mut reward_pool = RewardPool::<T>::get();
				ensure!(
					reward_pool.total_balance >= outcome.reward_amount,
					Error::<T>::InsufficientRewardPool
				);

				// Transfer reward
				// Note: In production, this would use Currency trait transfer
				// For now, just mark as claimed and update balances

				outcome.reward_claimed = true;

				// Update reward pool
				reward_pool.total_balance -= outcome.reward_amount;
				reward_pool.reserved_balance -= outcome.reward_amount;
				RewardPool::<T>::put(reward_pool);

				// Update statistics
				RegistryStatistics::<T>::mutate(|stats| {
					stats.total_rewards_distributed += outcome.reward_amount;
				});

				Self::deposit_event(Event::RewardClaimed {
					outcome_id,
					reporter: who,
					amount: outcome.reward_amount,
				});

				Ok(())
			})
		}

		/// Fund the reward pool (admin/governance function)
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::fund_reward_pool())]
		pub fn fund_reward_pool(origin: OriginFor<T>, amount: u128) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// In production, this would:
			// 1. Check caller has admin rights
			// 2. Transfer funds from caller to pool account

			RewardPool::<T>::mutate(|pool| {
				pool.total_balance += amount;
			});

			let new_balance = RewardPool::<T>::get().total_balance;

			Self::deposit_event(Event::RewardPoolFunded { amount, new_balance });

			Ok(())
		}

		/// Update reward configuration (admin/governance function)
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::update_reward_config())]
		pub fn update_reward_config(
			origin: OriginFor<T>,
			basic: u128,
			intermediate: u128,
			advanced: u128,
			premium: u128,
			min_severity: u8,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// In production, this would check admin/governance rights

			let min_severity_enum = Self::u8_to_severity(min_severity)?;

			let tier_rewards = TierRewards { basic, intermediate, advanced, premium };

			RewardPool::<T>::mutate(|pool| {
				pool.tier_rewards = tier_rewards;
				pool.min_severity = min_severity_enum;
			});

			Self::deposit_event(Event::RewardConfigUpdated);

			Ok(())
		}

		/// Add additional evidence to an existing outcome
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::add_evidence())]
		pub fn add_evidence(
			origin: OriginFor<T>,
			outcome_id: OutcomeId,
			evidence: BoundedVec<u8, ConstU32<MAX_EVIDENCE_LENGTH>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Outcomes::<T>::try_mutate(outcome_id, |maybe_outcome| -> DispatchResult {
				let outcome = maybe_outcome.as_mut().ok_or(Error::<T>::OutcomeNotFound)?;

				// Ensure caller is the reporter
				ensure!(outcome.reporter == who, Error::<T>::NotReporter);

				// Add evidence
				outcome
					.evidence
					.try_push(evidence)
					.map_err(|_| Error::<T>::OutcomeNotFound)?; // Use generic error for now

				Self::deposit_event(Event::EvidenceAdded { outcome_id });

				Ok(())
			})
		}

		/// Link outcome to a data listing
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::link_to_listing())]
		pub fn link_to_listing(
			origin: OriginFor<T>,
			outcome_id: OutcomeId,
			listing_id: [u8; 32],
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Outcomes::<T>::try_mutate(outcome_id, |maybe_outcome| -> DispatchResult {
				let outcome = maybe_outcome.as_mut().ok_or(Error::<T>::OutcomeNotFound)?;

				// Ensure caller is the reporter
				ensure!(outcome.reporter == who, Error::<T>::NotReporter);

				// Link to listing
				outcome.listing_id = Some(listing_id);

				Self::deposit_event(Event::LinkedToListing { outcome_id, listing_id });

				Ok(())
			})
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		/// Convert u8 to OutcomeType
		fn u8_to_outcome_type(value: u8) -> Result<OutcomeType, Error<T>> {
			match value {
				0 => Ok(OutcomeType::AdverseDrugReaction),
				1 => Ok(OutcomeType::TreatmentFailure),
				2 => Ok(OutcomeType::SideEffect),
				3 => Ok(OutcomeType::DiseaseProgression),
				4 => Ok(OutcomeType::DeviceFailure),
				5 => Ok(OutcomeType::SurgicalComplication),
				6 => Ok(OutcomeType::DiagnosticError),
				7 => Ok(OutcomeType::ContraindicationViolation),
				8 => Ok(OutcomeType::DrugInteraction),
				9 => Ok(OutcomeType::AllergyReaction),
				10 => Ok(OutcomeType::Other),
				_ => Err(Error::<T>::InvalidOutcomeType),
			}
		}

		/// Convert u8 to SeverityLevel
		fn u8_to_severity(value: u8) -> Result<SeverityLevel, Error<T>> {
			match value {
				0 => Ok(SeverityLevel::Mild),
				1 => Ok(SeverityLevel::Moderate),
				2 => Ok(SeverityLevel::Severe),
				3 => Ok(SeverityLevel::Critical),
				4 => Ok(SeverityLevel::Fatal),
				_ => Err(Error::<T>::InvalidSeverity),
			}
		}

		/// Convert u8 to RewardTier
		fn u8_to_reward_tier(value: u8) -> Result<RewardTier, Error<T>> {
			match value {
				0 => Ok(RewardTier::Basic),
				1 => Ok(RewardTier::Intermediate),
				2 => Ok(RewardTier::Advanced),
				3 => Ok(RewardTier::Premium),
				_ => Err(Error::<T>::InvalidRewardTier),
			}
		}

		/// Convert u8 to VerifierRole
		fn u8_to_verifier_role(value: u8) -> Result<VerifierRole, Error<T>> {
			match value {
				0 => Ok(VerifierRole::HealthcareProvider),
				1 => Ok(VerifierRole::MedicalExpert),
				2 => Ok(VerifierRole::RegulatoryAuthority),
				3 => Ok(VerifierRole::PharmaceuticalCompany),
				4 => Ok(VerifierRole::PatientAdvocate),
				5 => Ok(VerifierRole::ThirdPartyAuditor),
				_ => Err(Error::<T>::InvalidVerifierRole),
			}
		}

		/// Convert u8 to VerificationDecision
		fn u8_to_verification_decision(value: u8) -> Result<VerificationDecision, Error<T>> {
			match value {
				0 => Ok(VerificationDecision::Confirmed),
				1 => Ok(VerificationDecision::Disputed),
				2 => Ok(VerificationDecision::NeedsMoreInfo),
				3 => Ok(VerificationDecision::CannotVerify),
				_ => Err(Error::<T>::InvalidVerificationDecision),
			}
		}

		/// Get all outcomes by reporter
		pub fn get_reporter_outcomes(reporter: &T::AccountId) -> Vec<OutcomeId> {
			ReporterOutcomes::<T>::get(reporter).to_vec()
		}

		/// Get all verified outcomes
		pub fn get_verified_outcomes() -> Vec<OutcomeId> {
			VerifiedOutcomes::<T>::get().to_vec()
		}

		/// Get outcomes by type
		pub fn get_outcomes_by_type(outcome_type: u8) -> Vec<OutcomeId> {
			OutcomesByType::<T>::get(outcome_type).to_vec()
		}

		/// Get outcomes by severity
		pub fn get_outcomes_by_severity(severity: u8) -> Vec<OutcomeId> {
			OutcomesBySeverity::<T>::get(severity).to_vec()
		}

		/// Get total rewards available in pool
		pub fn get_available_rewards() -> u128 {
			let pool = RewardPool::<T>::get();
			pool.total_balance.saturating_sub(pool.reserved_balance)
		}
	}
}
