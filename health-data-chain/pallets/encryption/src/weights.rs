use frame_support::weights::Weight;

/// Weight functions needed for pallet_encryption
pub trait WeightInfo {
	fn register_key() -> Weight;
	fn revoke_key() -> Weight;
	fn rotate_key() -> Weight;
	fn share_key() -> Weight;
}

/// Default weights (placeholder)
impl WeightInfo for () {
	fn register_key() -> Weight {
		Weight::from_parts(60_000_000, 0)
	}

	fn revoke_key() -> Weight {
		Weight::from_parts(40_000_000, 0)
	}

	fn rotate_key() -> Weight {
		Weight::from_parts(70_000_000, 0)
	}

	fn share_key() -> Weight {
		Weight::from_parts(50_000_000, 0)
	}
}
