use frame_support::weights::Weight;

/// Weight functions needed for pallet_ipfs_integration
pub trait WeightInfo {
	fn request_pin() -> Weight;
	fn request_unpin() -> Weight;
	fn request_verify() -> Weight;
	fn cancel_operation() -> Weight;
	fn add_gateway() -> Weight;
	fn remove_gateway() -> Weight;
	fn update_gateway() -> Weight;
}

/// Default weights (placeholder - should be replaced with benchmarking results)
impl WeightInfo for () {
	fn request_pin() -> Weight {
		Weight::from_parts(50_000_000, 0)
	}

	fn request_unpin() -> Weight {
		Weight::from_parts(40_000_000, 0)
	}

	fn request_verify() -> Weight {
		Weight::from_parts(35_000_000, 0)
	}

	fn cancel_operation() -> Weight {
		Weight::from_parts(30_000_000, 0)
	}

	fn add_gateway() -> Weight {
		Weight::from_parts(45_000_000, 0)
	}

	fn remove_gateway() -> Weight {
		Weight::from_parts(40_000_000, 0)
	}

	fn update_gateway() -> Weight {
		Weight::from_parts(35_000_000, 0)
	}
}
