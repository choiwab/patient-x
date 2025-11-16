use frame_support::weights::Weight;

/// Weight functions needed for pallet_access_control
pub trait WeightInfo {
	fn create_policy() -> Weight;
	fn update_policy() -> Weight;
	fn delete_policy() -> Weight;
	fn assign_attribute() -> Weight;
	fn revoke_attribute() -> Weight;
	fn evaluate_policy() -> Weight;
	fn attach_policy_to_record() -> Weight;
	fn detach_policy_from_record() -> Weight;
}

/// Default weights (placeholder - should be replaced with benchmarking results)
impl WeightInfo for () {
	fn create_policy() -> Weight {
		Weight::from_parts(70_000_000, 0)
	}

	fn update_policy() -> Weight {
		Weight::from_parts(60_000_000, 0)
	}

	fn delete_policy() -> Weight {
		Weight::from_parts(50_000_000, 0)
	}

	fn assign_attribute() -> Weight {
		Weight::from_parts(45_000_000, 0)
	}

	fn revoke_attribute() -> Weight {
		Weight::from_parts(40_000_000, 0)
	}

	fn evaluate_policy() -> Weight {
		Weight::from_parts(55_000_000, 0)
	}

	fn attach_policy_to_record() -> Weight {
		Weight::from_parts(50_000_000, 0)
	}

	fn detach_policy_from_record() -> Weight {
		Weight::from_parts(45_000_000, 0)
	}
}
