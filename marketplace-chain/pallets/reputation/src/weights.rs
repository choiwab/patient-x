use frame_support::weights::Weight;

/// Weight functions needed for pallet_reputation.
pub trait WeightInfo {
	fn submit_review() -> Weight;
	fn endorse_provider() -> Weight;
	fn revoke_endorsement() -> Weight;
	fn award_badge() -> Weight;
	fn update_reputation() -> Weight;
}

/// Default weights for pallet_reputation.
pub struct SubstrateWeight;

impl WeightInfo for SubstrateWeight {
	fn submit_review() -> Weight {
		Weight::from_parts(50_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4000))
	}

	fn endorse_provider() -> Weight {
		Weight::from_parts(45_000_000, 0)
			.saturating_add(Weight::from_parts(0, 2000))
	}

	fn revoke_endorsement() -> Weight {
		Weight::from_parts(40_000_000, 0)
			.saturating_add(Weight::from_parts(0, 2000))
	}

	fn award_badge() -> Weight {
		Weight::from_parts(35_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1000))
	}

	fn update_reputation() -> Weight {
		Weight::from_parts(30_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1000))
	}
}
