use crate::{self as pallet_consent_manager, *};
use frame_support::{
	assert_noop, assert_ok, parameter_types,
	traits::{ConstU128, ConstU16, ConstU32, ConstU64},
	BoundedVec,
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system,
		IdentityRegistry: pallet_identity_registry,
		ConsentManager: pallet_consent_manager,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
	type RuntimeTask = ();
	type ExtensionsWeightInfo = ();
	type SingleBlockMigrations = ();
	type MultiBlockMigrator = ();
	type PreInherents = ();
	type PostInherents = ();
	type PostTransactions = ();
}

impl pallet_identity_registry::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxInstitutionLength = ConstU32<100>;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxDataTypes: u32 = 20;
	pub const MaxAllowedParties: u32 = 50;
	pub const MaxJurisdictions: u32 = 10;
	pub const MaxConsentsPerUser: u32 = 100;
}

impl pallet_consent_manager::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxDataTypes = MaxDataTypes;
	type MaxAllowedParties = MaxAllowedParties;
	type MaxJurisdictions = MaxJurisdictions;
	type MaxConsentsPerUser = MaxConsentsPerUser;
	type WeightInfo = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	t.into()
}

// Helper function to register an identity
fn register_identity(account: u64, user_type: u8) {
	let did: pallet_identity_registry::DID =
		format!("did:patientx:user{}", account).into_bytes().try_into().unwrap();
	let jurisdiction: pallet_identity_registry::JurisdictionCode =
		b"US".to_vec().try_into().unwrap();

	assert_ok!(IdentityRegistry::register_identity(
		RuntimeOrigin::signed(account),
		did,
		user_type,
		jurisdiction,
		None,
	));
}

#[test]
fn grant_consent_works() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		register_identity(alice, 0); // Patient

		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1,
			end: Some(100),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, MaxDataTypes> =
			vec![DataType::Demographics, DataType::Genomics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, MaxJurisdictions> =
			vec![b"US".to_vec().try_into().unwrap()].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		assert_ok!(ConsentManager::grant_consent(
			RuntimeOrigin::signed(alice),
			purpose,
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		));

		// Verify consent index was updated
		let consents = ConsentManager::consent_index(alice);
		assert_eq!(consents.len(), 1);
	});
}

#[test]
fn grant_consent_fails_without_identity() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		// Don't register identity

		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1,
			end: Some(100),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, MaxJurisdictions> =
			vec![b"US".to_vec().try_into().unwrap()].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		assert_noop!(
			ConsentManager::grant_consent(
				RuntimeOrigin::signed(alice),
				purpose,
				duration,
				data_types,
				allowed_parties,
				jurisdictions,
				compensation,
			),
			Error::<Test>::IdentityNotRegistered
		);
	});
}

#[test]
fn grant_consent_fails_with_invalid_duration() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		register_identity(alice, 0);

		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 100,
			end: Some(50), // End before start
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, MaxJurisdictions> =
			vec![b"US".to_vec().try_into().unwrap()].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		assert_noop!(
			ConsentManager::grant_consent(
				RuntimeOrigin::signed(alice),
				purpose,
				duration,
				data_types,
				allowed_parties,
				jurisdictions,
				compensation,
			),
			Error::<Test>::InvalidDuration
		);
	});
}

#[test]
fn revoke_consent_works() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		register_identity(alice, 0);

		// Grant consent first
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1,
			end: Some(100),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, MaxJurisdictions> =
			vec![b"US".to_vec().try_into().unwrap()].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		assert_ok!(ConsentManager::grant_consent(
			RuntimeOrigin::signed(alice),
			purpose.clone(),
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		));

		// Get consent ID
		let consents = ConsentManager::consent_index(alice);
		let consent_id = consents[0];

		// Revoke it
		let reason = Some(b"Changed my mind".to_vec().try_into().unwrap());
		assert_ok!(ConsentManager::revoke_consent(
			RuntimeOrigin::signed(alice),
			consent_id,
			reason,
		));

		// Verify it's revoked
		let status = ConsentManager::user_consents(alice, consent_id).unwrap();
		assert!(!status.is_active());
	});
}

#[test]
fn revoke_consent_fails_for_non_owner() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let bob = 2u64;
		register_identity(alice, 0);
		register_identity(bob, 1);

		// Grant consent as alice
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1,
			end: Some(100),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, MaxJurisdictions> =
			vec![b"US".to_vec().try_into().unwrap()].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		assert_ok!(ConsentManager::grant_consent(
			RuntimeOrigin::signed(alice),
			purpose,
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		));

		let consents = ConsentManager::consent_index(alice);
		let consent_id = consents[0];

		// Try to revoke as bob
		assert_noop!(
			ConsentManager::revoke_consent(RuntimeOrigin::signed(bob), consent_id, None),
			Error::<Test>::NotConsentOwner
		);
	});
}

#[test]
fn update_consent_works() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		register_identity(alice, 0);

		// Grant consent
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1,
			end: Some(100),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, MaxJurisdictions> =
			vec![b"US".to_vec().try_into().unwrap()].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		assert_ok!(ConsentManager::grant_consent(
			RuntimeOrigin::signed(alice),
			purpose,
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		));

		let consents = ConsentManager::consent_index(alice);
		let consent_id = consents[0];

		// Update compensation
		let new_compensation = CompensationPreference::FixedPrice(1000);
		assert_ok!(ConsentManager::update_consent(
			RuntimeOrigin::signed(alice),
			consent_id,
			None,
			None,
			Some(new_compensation.clone()),
		));

		// Verify update
		let policy = ConsentManager::consents(consent_id).unwrap();
		assert_eq!(policy.compensation_preference, new_compensation);
	});
}

#[test]
fn update_consent_fails_for_non_owner() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let bob = 2u64;
		register_identity(alice, 0);
		register_identity(bob, 1);

		// Grant consent as alice
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1,
			end: Some(100),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, MaxJurisdictions> =
			vec![b"US".to_vec().try_into().unwrap()].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		assert_ok!(ConsentManager::grant_consent(
			RuntimeOrigin::signed(alice),
			purpose,
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		));

		let consents = ConsentManager::consent_index(alice);
		let consent_id = consents[0];

		// Try to update as bob
		assert_noop!(
			ConsentManager::update_consent(
				RuntimeOrigin::signed(bob),
				consent_id,
				None,
				None,
				Some(CompensationPreference::FixedPrice(1000)),
			),
			Error::<Test>::NotConsentOwner
		);
	});
}

#[test]
fn check_consent_validity_works() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let bob = 2u64;
		register_identity(alice, 0); // Patient
		register_identity(bob, 1); // Researcher

		// Grant consent
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1,
			end: Some(100),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, MaxJurisdictions> =
			vec![b"US".to_vec().try_into().unwrap()].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		assert_ok!(ConsentManager::grant_consent(
			RuntimeOrigin::signed(alice),
			purpose.clone(),
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		));

		let consents = ConsentManager::consent_index(alice);
		let consent_id = consents[0];

		// Set block number within range
		System::set_block_number(50);

		// Check consent validity
		assert_ok!(ConsentManager::check_consent_validity(
			RuntimeOrigin::signed(bob),
			consent_id,
			bob,
			purpose,
		));
	});
}

#[test]
fn check_consent_validity_fails_when_expired() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let bob = 2u64;
		register_identity(alice, 0);
		register_identity(bob, 1);

		// Grant consent
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1,
			end: Some(100),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, MaxJurisdictions> =
			vec![b"US".to_vec().try_into().unwrap()].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		assert_ok!(ConsentManager::grant_consent(
			RuntimeOrigin::signed(alice),
			purpose.clone(),
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		));

		let consents = ConsentManager::consent_index(alice);
		let consent_id = consents[0];

		// Set block number beyond end
		System::set_block_number(101);

		// Check should fail
		assert_noop!(
			ConsentManager::check_consent_validity(
				RuntimeOrigin::signed(bob),
				consent_id,
				bob,
				purpose,
			),
			Error::<Test>::ConsentExpired
		);
	});
}

#[test]
fn check_consent_validity_fails_with_wrong_purpose() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let bob = 2u64;
		register_identity(alice, 0);
		register_identity(bob, 1);

		// Grant consent for research
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1,
			end: Some(100),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, MaxJurisdictions> =
			vec![b"US".to_vec().try_into().unwrap()].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		assert_ok!(ConsentManager::grant_consent(
			RuntimeOrigin::signed(alice),
			purpose,
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		));

		let consents = ConsentManager::consent_index(alice);
		let consent_id = consents[0];

		System::set_block_number(50);

		// Try to check with commercial purpose
		assert_noop!(
			ConsentManager::check_consent_validity(
				RuntimeOrigin::signed(bob),
				consent_id,
				bob,
				Purpose::Commercial,
			),
			Error::<Test>::PurposeMismatch
		);
	});
}

#[test]
fn specific_allowed_parties_works() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let bob = 2u64;
		let charlie = 3u64;
		register_identity(alice, 0);
		register_identity(bob, 1);
		register_identity(charlie, 1);

		// Grant consent with specific allowed parties
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1,
			end: Some(100),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties: AllowedParties<Test> =
			AllowedParties::Specific(vec![bob].try_into().unwrap());
		let jurisdictions: BoundedVec<JurisdictionCode, MaxJurisdictions> =
			vec![b"US".to_vec().try_into().unwrap()].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		assert_ok!(ConsentManager::grant_consent(
			RuntimeOrigin::signed(alice),
			purpose.clone(),
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		));

		let consents = ConsentManager::consent_index(alice);
		let consent_id = consents[0];

		System::set_block_number(50);

		// Bob should be allowed
		assert_ok!(ConsentManager::check_consent_validity(
			RuntimeOrigin::signed(bob),
			consent_id,
			bob,
			purpose.clone(),
		));

		// Charlie should not be allowed
		assert_noop!(
			ConsentManager::check_consent_validity(
				RuntimeOrigin::signed(charlie),
				consent_id,
				charlie,
				purpose,
			),
			Error::<Test>::RequesterNotAllowed
		);
	});
}

#[test]
fn category_based_allowed_parties_works() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let bob = 2u64; // Researcher
		let charlie = 3u64; // Institution
		register_identity(alice, 0); // Patient
		register_identity(bob, 1); // Researcher
		register_identity(charlie, 2); // Institution

		// Grant consent allowing only researchers (user_type = 1)
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1,
			end: Some(100),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties: AllowedParties<Test> =
			AllowedParties::Categories(vec![1u8].try_into().unwrap()); // Researcher
		let jurisdictions: BoundedVec<JurisdictionCode, MaxJurisdictions> =
			vec![b"US".to_vec().try_into().unwrap()].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		assert_ok!(ConsentManager::grant_consent(
			RuntimeOrigin::signed(alice),
			purpose.clone(),
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		));

		let consents = ConsentManager::consent_index(alice);
		let consent_id = consents[0];

		System::set_block_number(50);

		// Researcher should be allowed
		assert_ok!(ConsentManager::check_consent_validity(
			RuntimeOrigin::signed(bob),
			consent_id,
			bob,
			purpose.clone(),
		));

		// Institution should not be allowed
		assert_noop!(
			ConsentManager::check_consent_validity(
				RuntimeOrigin::signed(charlie),
				consent_id,
				charlie,
				purpose,
			),
			Error::<Test>::RequesterNotAllowed
		);
	});
}

#[test]
fn consent_with_auto_renewal() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		register_identity(alice, 0);

		// Grant consent with auto-renewal and no end date
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1,
			end: None,
			auto_renewal: true,
		};
		let data_types: BoundedVec<DataType, MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, MaxJurisdictions> =
			vec![b"US".to_vec().try_into().unwrap()].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		assert_ok!(ConsentManager::grant_consent(
			RuntimeOrigin::signed(alice),
			purpose.clone(),
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		));

		let consents = ConsentManager::consent_index(alice);
		let consent_id = consents[0];

		// Should work even far in the future
		System::set_block_number(10000);

		assert_ok!(ConsentManager::check_consent_validity(
			RuntimeOrigin::signed(alice),
			consent_id,
			alice,
			purpose,
		));
	});
}
