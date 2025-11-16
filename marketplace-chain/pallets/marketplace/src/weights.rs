use frame_support::weights::Weight;

/// Weight functions needed for pallet_marketplace.
pub trait WeightInfo {
	fn create_order() -> Weight;
	fn pay_order() -> Weight;
	fn fulfill_order() -> Weight;
	fn confirm_order() -> Weight;
	fn cancel_order() -> Weight;
	fn raise_dispute() -> Weight;
	fn resolve_dispute() -> Weight;
	fn release_escrow() -> Weight;
}

/// Default weights for pallet_marketplace.
pub struct SubstrateWeight;

impl WeightInfo for SubstrateWeight {
	fn create_order() -> Weight {
		Weight::from_parts(60_000_000, 0)
			.saturating_add(Weight::from_parts(0, 5000))
	}

	fn pay_order() -> Weight {
		Weight::from_parts(70_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3000))
	}

	fn fulfill_order() -> Weight {
		Weight::from_parts(65_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4000))
	}

	fn confirm_order() -> Weight {
		Weight::from_parts(60_000_000, 0)
			.saturating_add(Weight::from_parts(0, 2000))
	}

	fn cancel_order() -> Weight {
		Weight::from_parts(50_000_000, 0)
			.saturating_add(Weight::from_parts(0, 2000))
	}

	fn raise_dispute() -> Weight {
		Weight::from_parts(55_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3000))
	}

	fn resolve_dispute() -> Weight {
		Weight::from_parts(70_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3000))
	}

	fn release_escrow() -> Weight {
		Weight::from_parts(60_000_000, 0)
			.saturating_add(Weight::from_parts(0, 2000))
	}
}
