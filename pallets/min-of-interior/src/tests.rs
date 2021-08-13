use crate::mock::*;
use crate::*;
use frame_support::traits::Hooks;
use frame_support::{assert_err, assert_ok};
use frame_system::ensure_signed;

#[test]
fn basic_identity_test() {
    new_test_ext().execute_with(|| {
        let account1 = Origin::signed(1);
        let id1 = [1; 32];
        let account2 = Origin::signed(2);
        let id2 = [2; 32];

        let reviewer_account = Origin::signed(3);
        let reviewer_id = [3; 32];

        // setup reviewer
        IdentityPallet::match_account_to_id(
            ensure_signed(reviewer_account.clone()).unwrap(),
            reviewer_id,
        );
        IdentityPallet::push_identity(reviewer_id, IdentityType::Citizen).unwrap();
        IdentityPallet::push_identity(reviewer_id, IdentityType::MinisterOfInterior).unwrap();

        assert_eq!(
            IdentityPallet::check_id_identity(id1, IdentityType::Citizen),
            false
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id2, IdentityType::Citizen),
            false
        );

        assert_eq!(MinInteriorPallet::get_all_requests(), Default::default());

        // request kyc
        assert_ok!(MinInteriorPallet::request_kyc(
            account1.clone(),
            KycData { id: id1 },
        ));

        System::set_block_number(2);

        assert_err!(
            MinInteriorPallet::request_kyc(account1.clone(), KycData { id: id1 },),
            <Error<Test>>::AlreadyAplliedKycRequest
        );

        assert_ok!(MinInteriorPallet::request_kyc(
            account2.clone(),
            KycData { id: id2 },
        ));

        assert_err!(
            MinInteriorPallet::request_kyc(account2.clone(), KycData { id: id2 },),
            <Error<Test>>::AlreadyAplliedKycRequest
        );

        // get KYC request data
        let requests = MinInteriorPallet::get_all_requests();

        assert_eq!(requests.len(), 2);
        let mut it = requests.iter();

        let request1 = it.next().unwrap().clone();
        assert_eq!(request1.account, ensure_signed(account1.clone()).unwrap());
        assert_eq!(request1.data.id, id1);

        let request2 = it.next().unwrap().clone();
        assert_eq!(request2.account, ensure_signed(account2.clone()).unwrap());
        assert_eq!(request2.data.id, id2);

        assert_err!(
            MinInteriorPallet::kyc_response(account1.clone(), request1.clone(), true),
            <Error<Test>>::AccountCannotProcessKyc
        );
        assert_err!(
            MinInteriorPallet::kyc_response(account2.clone(), request2.clone(), true),
            <Error<Test>>::AccountCannotProcessKyc
        );

        assert_ok!(MinInteriorPallet::kyc_response(
            reviewer_account.clone(),
            request1.clone(),
            true
        ));
        assert_ok!(MinInteriorPallet::kyc_response(
            reviewer_account.clone(),
            request2.clone(),
            false
        ));

        assert_eq!(
            IdentityPallet::check_id_identity(id1, IdentityType::EResident),
            true
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id2, IdentityType::EResident),
            false
        );

        assert_eq!(MinInteriorPallet::get_all_requests(), Default::default());

        assert_err!(
            MinInteriorPallet::kyc_response(reviewer_account.clone(), request1, true),
            <Error<Test>>::RequestDoesNotExist
        );
        assert_err!(
            MinInteriorPallet::kyc_response(reviewer_account, request2, false),
            <Error<Test>>::RequestDoesNotExist
        );

        assert_err!(
            MinInteriorPallet::request_kyc(account1, KycData { id: id1 }),
            <Error<Test>>::AccoundIdAlreadyUsed
        );

        assert_ok!(MinInteriorPallet::request_kyc(
            account2,
            KycData { id: id2 }
        ));
    });
}
#[test]
fn e_resident_aproving_test() {
    new_test_ext().execute_with(|| {
        let account1 = Origin::signed(1);
        let id1 = [1; 32];
        let account2 = Origin::signed(2);
        let id2 = [2; 32];

        let account3 = Origin::signed(4);
        let id3 = [4; 32];

        let reviewer_account = Origin::signed(3);
        let reviewer_id = [3; 32];

        IdentityPallet::match_account_to_id(
            ensure_signed(reviewer_account.clone()).unwrap(),
            reviewer_id,
        );
        IdentityPallet::push_identity(reviewer_id, IdentityType::Citizen).unwrap();
        IdentityPallet::push_identity(reviewer_id, IdentityType::MinisterOfInterior).unwrap();

        MinInteriorPallet::request_kyc(account1.clone(), KycData { id: id1 }).unwrap();
        let reqests = MinInteriorPallet::get_all_requests();
        let request_1 = reqests.iter().next().unwrap().clone();
        assert_err!(
            MinInteriorPallet::update_e_resident_to_citizen_reqest(account1.clone()),
            <Error<Test>>::EresidenceNotFound
        );

        MinInteriorPallet::kyc_response(reviewer_account.clone(), request_1.clone(), true).unwrap();
        assert_eq!(
            IdentityPallet::get_id_identities(id1),
            [IdentityType::EResident].iter().cloned().collect()
        );
        MinInteriorPallet::update_e_resident_to_citizen_reqest(account1.clone()).unwrap();

        assert_eq!(
            IdentityPallet::get_id_identities(id1),
            [IdentityType::EResident].iter().cloned().collect()
        );

        let duration = 7;
        MinInteriorPallet::on_finalize(duration);
        assert_eq!(
            IdentityPallet::get_id_identities(id1),
            [IdentityType::EResident].iter().cloned().collect()
        );

        let duration = 10;
        MinInteriorPallet::on_finalize(duration);
        assert_eq!(
            IdentityPallet::get_id_identities(id1),
            [IdentityType::Citizen].iter().cloned().collect()
        );

        MinInteriorPallet::request_kyc(account2.clone(), KycData { id: id2 }).unwrap();
        let reqests_2 = MinInteriorPallet::get_all_requests();
        let request_2 = reqests_2.iter().next().unwrap().clone();
        MinInteriorPallet::kyc_response(reviewer_account.clone(), request_2.clone(), true).unwrap();

        assert_eq!(
            IdentityPallet::get_id_identities(id2),
            [IdentityType::EResident].iter().cloned().collect()
        );

        assert_err!(
            MinInteriorPallet::aprove_to_citizen_or_not(account1, request_2.clone(), true),
            <Error<Test>>::OnlyMinistryOfInteriorCall
        );

        MinInteriorPallet::aprove_to_citizen_or_not(reviewer_account.clone(), request_2, true)
            .unwrap();

        assert_eq!(
            IdentityPallet::get_id_identities(id2),
            [IdentityType::Citizen].iter().cloned().collect()
        );

        MinInteriorPallet::request_kyc(account3.clone(), KycData { id: id3 }).unwrap();
        let reqests_3 = MinInteriorPallet::get_all_requests();
        let request_3 = reqests_3.iter().next().unwrap().clone();
        MinInteriorPallet::kyc_response(reviewer_account.clone(), request_3.clone(), true).unwrap();

        MinInteriorPallet::aprove_to_citizen_or_not(reviewer_account.clone(), request_3, false)
            .unwrap();

        assert_eq!(
            IdentityPallet::get_id_identities(id3),
            [IdentityType::EResident].iter().cloned().collect()
        );
    });
}

#[test]
fn update_assembly_to_minister_test() {
    new_test_ext().execute_with(|| {
        let id1 = [1; 32];
        let account2 = Origin::signed(2);
        let id2 = [2; 32];
        let id3 = [4; 32];

        let reviewer_account = Origin::signed(3);
        let reviewer_id = [3; 32];

        IdentityPallet::match_account_to_id(
            ensure_signed(reviewer_account.clone()).unwrap(),
            reviewer_id,
        );
        IdentityPallet::push_identity(reviewer_id, IdentityType::Citizen).unwrap();
        IdentityPallet::push_identity(reviewer_id, IdentityType::MinisterOfInterior).unwrap();

        IdentityPallet::push_identity(id1, IdentityType::Citizen).unwrap();
        IdentityPallet::push_identity(id1, IdentityType::Assembly).unwrap();

        //IdentityPallet
        MinInteriorPallet::update_assembly_to_minister(reviewer_account.clone(), id1.clone())
            .unwrap();

        assert_eq!(
            IdentityPallet::get_id_identities(id1),
            [IdentityType::Citizen, IdentityType::MinisterOfInterior,]
                .iter()
                .cloned()
                .collect()
        );

        IdentityPallet::match_account_to_id(ensure_signed(account2.clone()).unwrap(), id2);

        IdentityPallet::push_identity(id2, IdentityType::Citizen).unwrap();

        IdentityPallet::push_identity(id3, IdentityType::Citizen).unwrap();

        assert_err!(
            MinInteriorPallet::update_assembly_to_minister(account2.clone(), id3.clone()),
            <Error<Test>>::OnlyMinistryOfInteriorCall
        );

        assert_err!(
            MinInteriorPallet::update_assembly_to_minister(reviewer_account.clone(), id3.clone()),
            <Error<Test>>::AssemblyNotFound
        );
    });
}
