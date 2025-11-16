use frame_support::weights::Weight;

/// Weight functions needed for pallet_data_listings.
pub trait WeightInfo {
	fn create_listing() -> Weight;
	fn update_listing() -> Weight;
	fn pause_listing() -> Weight;
	fn resume_listing() -> Weight;
	fn cancel_listing() -> Weight;
	fn extend_listing() -> Weight;
	fn search_by_category() -> Weight;
}

/// Default weights for pallet_data_listings.
pub struct SubstrateWeight;

impl WeightInfo for SubstrateWeight {
	fn create_listing() -> Weight {
		Weight::from_parts(50_000_000, 0)
			.saturating_add(Weight::from_parts(0, 5000))
	}

	fn update_listing() -> Weight {
		Weight::from_parts(40_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3000))
	}

	fn pause_listing() -> Weight {
		Weight::from_parts(30_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1000))
	}

	fn resume_listing() -> Weight {
		Weight::from_parts(30_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1000))
	}

	fn cancel_listing() -> Weight {
		Weight::from_parts(35_000_000, 0)
			.saturating_add(Weight::from_parts(0, 2000))
	}

	fn extend_listing() -> Weight {
		Weight::from_parts(30_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1000))
	}

	fn search_by_category() -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1000))
	}
}
