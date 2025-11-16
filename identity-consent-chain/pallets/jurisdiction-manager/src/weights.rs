use frame_support::weights::Weight;

/// Weight functions needed for pallet_jurisdiction_manager
pub trait WeightInfo {
	fn register_jurisdiction() -> Weight;
	fn update_jurisdiction() -> Weight;
	fn deactivate_jurisdiction() -> Weight;
	fn set_compliance_rule() -> Weight;
	fn update_compliance_rule() -> Weight;
	fn add_approved_transfer_target() -> Weight;
	fn remove_approved_transfer_target() -> Weight;
	fn validate_transfer() -> Weight;
}

/// Default weights (placeholder - should be replaced with benchmarking results)
impl WeightInfo for () {
	fn register_jurisdiction() -> Weight {
		Weight::from_parts(60_000_000, 0)
	}

	fn update_jurisdiction() -> Weight {
		Weight::from_parts(50_000_000, 0)
	}

	fn deactivate_jurisdiction() -> Weight {
		Weight::from_parts(40_000_000, 0)
	}

	fn set_compliance_rule() -> Weight {
		Weight::from_parts(70_000_000, 0)
	}

	fn update_compliance_rule() -> Weight {
		Weight::from_parts(60_000_000, 0)
	}

	fn add_approved_transfer_target() -> Weight {
		Weight::from_parts(45_000_000, 0)
	}

	fn remove_approved_transfer_target() -> Weight {
		Weight::from_parts(40_000_000, 0)
	}

	fn validate_transfer() -> Weight {
		Weight::from_parts(55_000_000, 0)
	}
}
