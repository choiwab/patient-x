//! Benchmarking setup for pallet-consent-manager

use super::*;

#[allow(unused)]
use crate::Pallet as ConsentManager;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_std::vec;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn grant_consent() {
		let caller: T::AccountId = whitelisted_caller();

		// Register identity first
		let did: pallet_identity_registry::DID =
			b"did:patientx:benchmark_user_consent".to_vec().try_into().unwrap();
		let jurisdiction: pallet_identity_registry::JurisdictionCode =
			b"US".to_vec().try_into().unwrap();

		let _ = pallet_identity_registry::Pallet::<T>::register_identity(
			RawOrigin::Signed(caller.clone()).into(),
			did,
			0, // Patient
			jurisdiction.clone(),
			None,
		);

		// Prepare consent parameters
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1u32.into(),
			end: Some(1000u32.into()),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, T::MaxDataTypes> =
			vec![DataType::Demographics, DataType::Genomics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, T::MaxJurisdictions> =
			vec![jurisdiction].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		#[extrinsic_call]
		_(
			RawOrigin::Signed(caller.clone()),
			purpose,
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		);

		// Verify consent was created
		let consents = ConsentIndex::<T>::get(&caller);
		assert!(!consents.is_empty());
	}

	#[benchmark]
	fn revoke_consent() {
		let caller: T::AccountId = whitelisted_caller();

		// Register identity
		let did: pallet_identity_registry::DID =
			b"did:patientx:benchmark_revoke".to_vec().try_into().unwrap();
		let jurisdiction: pallet_identity_registry::JurisdictionCode =
			b"US".to_vec().try_into().unwrap();

		let _ = pallet_identity_registry::Pallet::<T>::register_identity(
			RawOrigin::Signed(caller.clone()).into(),
			did,
			0,
			jurisdiction.clone(),
			None,
		);

		// Grant consent first
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1u32.into(),
			end: Some(1000u32.into()),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, T::MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, T::MaxJurisdictions> =
			vec![jurisdiction].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		let _ = ConsentManager::<T>::grant_consent(
			RawOrigin::Signed(caller.clone()).into(),
			purpose,
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		);

		let consents = ConsentIndex::<T>::get(&caller);
		let consent_id = consents[0];

		let reason = Some(b"Benchmark revocation".to_vec().try_into().unwrap());

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), consent_id, reason);

		// Verify revocation
		let status = UserConsents::<T>::get(&caller, consent_id).unwrap();
		assert!(!status.is_active());
	}

	#[benchmark]
	fn update_consent() {
		let caller: T::AccountId = whitelisted_caller();

		// Register identity
		let did: pallet_identity_registry::DID =
			b"did:patientx:benchmark_update".to_vec().try_into().unwrap();
		let jurisdiction: pallet_identity_registry::JurisdictionCode =
			b"US".to_vec().try_into().unwrap();

		let _ = pallet_identity_registry::Pallet::<T>::register_identity(
			RawOrigin::Signed(caller.clone()).into(),
			did,
			0,
			jurisdiction.clone(),
			None,
		);

		// Grant consent first
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1u32.into(),
			end: Some(1000u32.into()),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, T::MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, T::MaxJurisdictions> =
			vec![jurisdiction].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		let _ = ConsentManager::<T>::grant_consent(
			RawOrigin::Signed(caller.clone()).into(),
			purpose,
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		);

		let consents = ConsentIndex::<T>::get(&caller);
		let consent_id = consents[0];

		let new_compensation = Some(CompensationPreference::FixedPrice(1000));

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), consent_id, None, None, new_compensation);

		// Verify update
		let policy = Consents::<T>::get(consent_id).unwrap();
		assert_eq!(policy.compensation_preference, CompensationPreference::FixedPrice(1000));
	}

	#[benchmark]
	fn check_consent_validity() {
		let caller: T::AccountId = whitelisted_caller();
		let requester: T::AccountId = account("requester", 0, 0);

		// Register both identities
		let did1: pallet_identity_registry::DID =
			b"did:patientx:benchmark_check".to_vec().try_into().unwrap();
		let did2: pallet_identity_registry::DID =
			b"did:patientx:benchmark_requester".to_vec().try_into().unwrap();
		let jurisdiction: pallet_identity_registry::JurisdictionCode =
			b"US".to_vec().try_into().unwrap();

		let _ = pallet_identity_registry::Pallet::<T>::register_identity(
			RawOrigin::Signed(caller.clone()).into(),
			did1,
			0,
			jurisdiction.clone(),
			None,
		);

		let _ = pallet_identity_registry::Pallet::<T>::register_identity(
			RawOrigin::Signed(requester.clone()).into(),
			did2,
			1,
			jurisdiction.clone(),
			None,
		);

		// Grant consent
		let purpose = Purpose::ResearchGeneral;
		let duration = Duration {
			start: 1u32.into(),
			end: Some(1000u32.into()),
			auto_renewal: false,
		};
		let data_types: BoundedVec<DataType, T::MaxDataTypes> =
			vec![DataType::Demographics].try_into().unwrap();
		let allowed_parties = AllowedParties::Public;
		let jurisdictions: BoundedVec<JurisdictionCode, T::MaxJurisdictions> =
			vec![jurisdiction].try_into().unwrap();
		let compensation = CompensationPreference::Free;

		let _ = ConsentManager::<T>::grant_consent(
			RawOrigin::Signed(caller.clone()).into(),
			purpose.clone(),
			duration,
			data_types,
			allowed_parties,
			jurisdictions,
			compensation,
		);

		let consents = ConsentIndex::<T>::get(&caller);
		let consent_id = consents[0];

		// Set block number in valid range
		frame_system::Pallet::<T>::set_block_number(100u32.into());

		#[extrinsic_call]
		_(RawOrigin::Signed(requester.clone()), consent_id, requester.clone(), purpose);
	}

	impl_benchmark_test_suite!(ConsentManager, crate::tests::new_test_ext(), crate::tests::Test);
}
