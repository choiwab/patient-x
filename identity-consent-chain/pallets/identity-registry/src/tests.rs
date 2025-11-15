//! Unit tests for the Identity Registry pallet

use crate::{self as pallet_identity_registry, *};
use frame_support::{
	assert_noop, assert_ok, parameter_types,
	traits::{ConstU32, ConstU64},
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Timestamp: pallet_timestamp,
		IdentityRegistry: pallet_identity_registry,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

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
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxInstitutionLength: u32 = 128;
}

impl pallet_identity_registry::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type TimeProvider = Timestamp;
	type MaxInstitutionLength = MaxInstitutionLength;
	type WeightInfo = ();
	type Signature = sp_runtime::MultiSignature;
}

// Helper function to create new test externalities
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

#[test]
fn register_identity_works() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let did: DID = b"did:patientx:alice123".to_vec().try_into().unwrap();
		let jurisdiction: JurisdictionCode = b"US".to_vec().try_into().unwrap();

		// Register identity
		assert_ok!(IdentityRegistry::register_identity(
			RuntimeOrigin::signed(alice),
			did.clone(),
			UserType::Patient,
			jurisdiction.clone(),
			None,
		));

		// Verify storage
		assert!(Identities::<Test>::contains_key(&alice));
		assert_eq!(DIDs::<Test>::get(&did), Some(alice));

		let user_info = Identities::<Test>::get(&alice).unwrap();
		assert_eq!(user_info.did, did);
		assert_eq!(user_info.user_type, UserType::Patient);
		assert_eq!(user_info.jurisdiction, jurisdiction);
		assert!(!user_info.verified);

		// Verify event
		System::assert_last_event(
			Event::IdentityRegistered {
				account: alice,
				did,
				user_type: UserType::Patient,
			}
			.into(),
		);
	});
}

#test]
fn register_identity_with_institution_works() {
	new_test_ext().execute_with(|| {
		let researcher = 2u64;
		let did: DID = b"did:patientx:researcher456".to_vec().try_into().unwrap();
		let jurisdiction: JurisdictionCode = b"EU".to_vec().try_into().unwrap();
		let institution = b"University Hospital".to_vec().try_into().unwrap();

		assert_ok!(IdentityRegistry::register_identity(
			RuntimeOrigin::signed(researcher),
			did.clone(),
			UserType::Researcher,
			jurisdiction,
			Some(institution),
		));

		let user_info = Identities::<Test>::get(&researcher).unwrap();
		assert_eq!(user_info.user_type, UserType::Researcher);
		assert!(user_info.institution.is_some());
	});
}

#[test]
fn duplicate_did_fails() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let bob = 2u64;
		let did: DID = b"did:patientx:duplicate".to_vec().try_into().unwrap();
		let jurisdiction: JurisdictionCode = b"US".to_vec().try_into().unwrap();

		// Alice registers first
		assert_ok!(IdentityRegistry::register_identity(
			RuntimeOrigin::signed(alice),
			did.clone(),
			UserType::Patient,
			jurisdiction.clone(),
			None,
		));

		// Bob tries to register with same DID - should fail
		assert_noop!(
			IdentityRegistry::register_identity(
				RuntimeOrigin::signed(bob),
				did,
				UserType::Researcher,
				jurisdiction,
				None,
			),
			Error::<Test>::DIDAlreadyExists
		);
	});
}

#[test]
fn duplicate_account_registration_fails() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let did1: DID = b"did:patientx:first".to_vec().try_into().unwrap();
		let did2: DID = b"did:patientx:second".to_vec().try_into().unwrap();
		let jurisdiction: JurisdictionCode = b"US".to_vec().try_into().unwrap();

		// First registration succeeds
		assert_ok!(IdentityRegistry::register_identity(
			RuntimeOrigin::signed(alice),
			did1,
			UserType::Patient,
			jurisdiction.clone(),
			None,
		));

		// Second registration with same account fails
		assert_noop!(
			IdentityRegistry::register_identity(
				RuntimeOrigin::signed(alice),
				did2,
				UserType::Patient,
				jurisdiction,
				None,
			),
			Error::<Test>::IdentityAlreadyRegistered
		);
	});
}

#[test]
fn update_jurisdiction_works() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let did: DID = b"did:patientx:alice".to_vec().try_into().unwrap();
		let old_jurisdiction: JurisdictionCode = b"US".to_vec().try_into().unwrap();
		let new_jurisdiction: JurisdictionCode = b"EU".to_vec().try_into().unwrap();

		// Register identity
		assert_ok!(IdentityRegistry::register_identity(
			RuntimeOrigin::signed(alice),
			did,
			UserType::Patient,
			old_jurisdiction.clone(),
			None,
		));

		// Update jurisdiction
		assert_ok!(IdentityRegistry::update_jurisdiction(
			RuntimeOrigin::signed(alice),
			new_jurisdiction.clone(),
		));

		// Verify update
		let user_info = Identities::<Test>::get(&alice).unwrap();
		assert_eq!(user_info.jurisdiction, new_jurisdiction);
		assert_eq!(UserJurisdictions::<Test>::get(&alice), Some(new_jurisdiction.clone()));

		// Verify event
		System::assert_last_event(
			Event::JurisdictionUpdated {
				account: alice,
				old_jurisdiction: Some(old_jurisdiction),
				new_jurisdiction,
			}
			.into(),
		);
	});
}

#[test]
fn update_jurisdiction_without_identity_fails() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let jurisdiction: JurisdictionCode = b"US".to_vec().try_into().unwrap();

		// Try to update without registering first
		assert_noop!(
			IdentityRegistry::update_jurisdiction(
				RuntimeOrigin::signed(alice),
				jurisdiction,
			),
			Error::<Test>::IdentityNotFound
		);
	});
}

#[test]
fn verify_identity_works() {
	new_test_ext().execute_with(|| {
		let patient = 1u64;
		let institution = 2u64;
		let patient_did: DID = b"did:patientx:patient".to_vec().try_into().unwrap();
		let institution_did: DID = b"did:patientx:hospital".to_vec().try_into().unwrap();
		let jurisdiction: JurisdictionCode = b"US".to_vec().try_into().unwrap();

		// Register patient
		assert_ok!(IdentityRegistry::register_identity(
			RuntimeOrigin::signed(patient),
			patient_did.clone(),
			UserType::Patient,
			jurisdiction.clone(),
			None,
		));

		// Register institution
		assert_ok!(IdentityRegistry::register_identity(
			RuntimeOrigin::signed(institution),
			institution_did,
			UserType::Institution,
			jurisdiction,
			None,
		));

		// Create dummy signature
		let signature = sp_runtime::MultiSignature::Sr25519(
			sp_core::sr25519::Signature([0u8; 64])
		);

		// Institution verifies patient
		assert_ok!(IdentityRegistry::verify_identity(
			RuntimeOrigin::signed(institution),
			patient_did.clone(),
			signature,
		));

		// Verify status
		let user_info = Identities::<Test>::get(&patient).unwrap();
		assert!(user_info.verified);

		let verification_status = VerifiedUsers::<Test>::get(&patient_did);
		assert!(verification_status.verified);
		assert_eq!(verification_status.verifier, institution);

		// Verify event
		System::assert_last_event(
			Event::IdentityVerified {
				did: patient_did,
				verifier: institution,
			}
			.into(),
		);
	});
}

#[test]
fn verify_identity_unauthorized_fails() {
	new_test_ext().execute_with(|| {
		let patient1 = 1u64;
		let patient2 = 2u64;
		let did1: DID = b"did:patientx:patient1".to_vec().try_into().unwrap();
		let did2: DID = b"did:patientx:patient2".to_vec().try_into().unwrap();
		let jurisdiction: JurisdictionCode = b"US".to_vec().try_into().unwrap();

		// Register both as patients
		assert_ok!(IdentityRegistry::register_identity(
			RuntimeOrigin::signed(patient1),
			did1,
			UserType::Patient,
			jurisdiction.clone(),
			None,
		));

		assert_ok!(IdentityRegistry::register_identity(
			RuntimeOrigin::signed(patient2),
			did2.clone(),
			UserType::Patient,
			jurisdiction,
			None,
		));

		let signature = sp_runtime::MultiSignature::Sr25519(
			sp_core::sr25519::Signature([0u8; 64])
		);

		// Patient trying to verify another patient should fail
		assert_noop!(
			IdentityRegistry::verify_identity(
				RuntimeOrigin::signed(patient1),
				did2,
				signature,
			),
			Error::<Test>::UnauthorizedVerifier
		);
	});
}

#[test]
fn verify_nonexistent_did_fails() {
	new_test_ext().execute_with(|| {
		let institution = 1u64;
		let institution_did: DID = b"did:patientx:hospital".to_vec().try_into().unwrap();
		let nonexistent_did: DID = b"did:patientx:ghost".to_vec().try_into().unwrap();
		let jurisdiction: JurisdictionCode = b"US".to_vec().try_into().unwrap();

		// Register institution
		assert_ok!(IdentityRegistry::register_identity(
			RuntimeOrigin::signed(institution),
			institution_did,
			UserType::Institution,
			jurisdiction,
			None,
		));

		let signature = sp_runtime::MultiSignature::Sr25519(
			sp_core::sr25519::Signature([0u8; 64])
		);

		// Try to verify non-existent DID
		assert_noop!(
			IdentityRegistry::verify_identity(
				RuntimeOrigin::signed(institution),
				nonexistent_did,
				signature,
			),
			Error::<Test>::DIDNotFound
		);
	});
}

#[test]
fn helper_functions_work() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let did: DID = b"did:patientx:alice".to_vec().try_into().unwrap();
		let jurisdiction: JurisdictionCode = b"US".to_vec().try_into().unwrap();

		// Register identity
		assert_ok!(IdentityRegistry::register_identity(
			RuntimeOrigin::signed(alice),
			did.clone(),
			UserType::Patient,
			jurisdiction,
			None,
		));

		// Test get_user_info
		let user_info = IdentityRegistry::get_user_info(&alice);
		assert!(user_info.is_some());
		assert_eq!(user_info.unwrap().did, did);

		// Test get_account_by_did
		let account = IdentityRegistry::get_account_by_did(&did);
		assert_eq!(account, Some(alice));

		// Test is_verified (should be false initially)
		assert!(!IdentityRegistry::is_verified(&did));
	});
}
