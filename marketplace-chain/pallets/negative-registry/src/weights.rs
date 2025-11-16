use frame_support::weights::Weight;

/// Weight functions needed for pallet_negative_registry.
pub trait WeightInfo {
	fn report_outcome() -> Weight;
	fn submit_verification() -> Weight;
	fn update_outcome() -> Weight;
	fn claim_reward() -> Weight;
	fn fund_reward_pool() -> Weight;
	fn update_reward_config() -> Weight;
	fn add_evidence() -> Weight;
	fn link_to_listing() -> Weight;
}

/// Default weights for pallet_negative_registry.
pub struct SubstrateWeight;

impl WeightInfo for SubstrateWeight {
	fn report_outcome() -> Weight {
		Weight::from_parts(80_000_000, 0)
			.saturating_add(Weight::from_parts(0, 8000))
	}

	fn submit_verification() -> Weight {
		Weight::from_parts(60_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4000))
	}

	fn update_outcome() -> Weight {
		Weight::from_parts(50_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3000))
	}

	fn claim_reward() -> Weight {
		Weight::from_parts(70_000_000, 0)
			.saturating_add(Weight::from_parts(0, 2000))
	}

	fn fund_reward_pool() -> Weight {
		Weight::from_parts(40_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1000))
	}

	fn update_reward_config() -> Weight {
		Weight::from_parts(35_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1000))
	}

	fn add_evidence() -> Weight {
		Weight::from_parts(45_000_000, 0)
			.saturating_add(Weight::from_parts(0, 2000))
	}

	fn link_to_listing() -> Weight {
		Weight::from_parts(40_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1000))
	}
}
