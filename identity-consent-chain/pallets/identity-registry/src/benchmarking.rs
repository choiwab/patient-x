//! Benchmarking setup for pallet-identity-registry

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as IdentityRegistry;
use frame_benchmarking::v2::*;
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use sp_std::vec;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn register_identity() {
		let caller: T::AccountId = whitelisted_caller();
		let did: DID = b"did:patientx:benchmark_user_12345"
			.to_vec()
			.try_into()
			.unwrap();
		let jurisdiction: JurisdictionCode = b"US".to_vec().try_into().unwrap();

		#[extrinsic_call]
		_(
			RawOrigin::Signed(caller.clone()),
			did.clone(),
			0, // Patient
			jurisdiction,
			None,
		);

		assert!(Identities::<T>::contains_key(&caller));
		assert_eq!(DIDs::<T>::get(&did), Some(caller));
	}

	#[benchmark]
	fn register_identity_with_institution() {
		let caller: T::AccountId = whitelisted_caller();
		let did: DID = b"did:patientx:researcher_benchmark"
			.to_vec()
			.try_into()
			.unwrap();
		let jurisdiction: JurisdictionCode = b"EU".to_vec().try_into().unwrap();
		let institution: BoundedVec<u8, T::MaxInstitutionLength> =
			b"University Hospital of Excellence"
				.to_vec()
				.try_into()
				.unwrap();

		#[extrinsic_call]
		register_identity(
			RawOrigin::Signed(caller.clone()),
			did.clone(),
			1, // Researcher
			jurisdiction,
			Some(institution),
		);

		assert!(Identities::<T>::contains_key(&caller));
	}

	#[benchmark]
	fn update_jurisdiction() {
		let caller: T::AccountId = whitelisted_caller();
		let did: DID = b"did:patientx:user_jurisdiction"
			.to_vec()
			.try_into()
			.unwrap();
		let old_jurisdiction: JurisdictionCode = b"US".to_vec().try_into().unwrap();
		let new_jurisdiction: JurisdictionCode = b"EU".to_vec().try_into().unwrap();

		// Setup: register identity first
		assert!(IdentityRegistry::<T>::register_identity(
			RawOrigin::Signed(caller.clone()).into(),
			did,
			0, // Patient
			old_jurisdiction,
			None,
		)
		.is_ok());

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), new_jurisdiction.clone());

		let user_info = Identities::<T>::get(&caller).unwrap();
		assert_eq!(user_info.jurisdiction, new_jurisdiction);
	}

	#[benchmark]
	fn verify_identity() {
		let patient: T::AccountId = whitelisted_caller();
		let verifier: T::AccountId = account("verifier", 0, 0);

		let patient_did: DID = b"did:patientx:patient_verify"
			.to_vec()
			.try_into()
			.unwrap();
		let verifier_did: DID = b"did:patientx:institution_verify"
			.to_vec()
			.try_into()
			.unwrap();
		let jurisdiction: JurisdictionCode = b"US".to_vec().try_into().unwrap();

		// Setup: register patient
		assert!(IdentityRegistry::<T>::register_identity(
			RawOrigin::Signed(patient).into(),
			patient_did.clone(),
			0, // Patient
			jurisdiction.clone(),
			None,
		)
		.is_ok());

		// Setup: register verifier as institution
		assert!(IdentityRegistry::<T>::register_identity(
			RawOrigin::Signed(verifier.clone()).into(),
			verifier_did,
			2, // Institution
			jurisdiction,
			None,
		)
		.is_ok());

		#[extrinsic_call]
		_(RawOrigin::Signed(verifier), patient_did.clone());

		let verification = VerifiedUsers::<T>::get(&patient_did).unwrap();
		assert!(verification.verified);
	}

	impl_benchmark_test_suite!(IdentityRegistry, crate::tests::new_test_ext(), crate::tests::Test);
}
