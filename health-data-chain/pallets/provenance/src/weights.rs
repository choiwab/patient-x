use frame_support::weights::Weight;

/// Weight functions needed for pallet_provenance
pub trait WeightInfo {
	fn record_lineage() -> Weight;
	fn record_transformation() -> Weight;
	fn record_usage() -> Weight;
	fn query_lineage() -> Weight;
}

/// Default weights (placeholder - should be replaced with benchmarking results)
impl WeightInfo for () {
	fn record_lineage() -> Weight {
		Weight::from_parts(70_000_000, 0)
	}

	fn record_transformation() -> Weight {
		Weight::from_parts(80_000_000, 0)
	}

	fn record_usage() -> Weight {
		Weight::from_parts(50_000_000, 0)
	}

	fn query_lineage() -> Weight {
		Weight::from_parts(60_000_000, 0)
	}
}
