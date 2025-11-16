use frame_support::weights::Weight;

/// Weight functions needed for pallet_health_records
pub trait WeightInfo {
	fn upload_record() -> Weight;
	fn update_record() -> Weight;
	fn delete_record() -> Weight;
	fn archive_record() -> Weight;
	fn grant_access() -> Weight;
	fn revoke_access() -> Weight;
	fn log_access() -> Weight;
	fn add_tags() -> Weight;
}

/// Default weights (placeholder - should be replaced with benchmarking results)
impl WeightInfo for () {
	fn upload_record() -> Weight {
		Weight::from_parts(80_000_000, 0)
	}

	fn update_record() -> Weight {
		Weight::from_parts(70_000_000, 0)
	}

	fn delete_record() -> Weight {
		Weight::from_parts(50_000_000, 0)
	}

	fn archive_record() -> Weight {
		Weight::from_parts(45_000_000, 0)
	}

	fn grant_access() -> Weight {
		Weight::from_parts(60_000_000, 0)
	}

	fn revoke_access() -> Weight {
		Weight::from_parts(50_000_000, 0)
	}

	fn log_access() -> Weight {
		Weight::from_parts(40_000_000, 0)
	}

	fn add_tags() -> Weight {
		Weight::from_parts(35_000_000, 0)
	}
}
