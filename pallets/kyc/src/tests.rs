use crate::mock::*;
use crate::*;
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
        IdentityPallet::push_identity(reviewer_id, IdentityType::MinisterOfInterior);

        assert_eq!(
            IdentityPallet::check_id_identity(id1, IdentityType::Citizen),
            false
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id2, IdentityType::Citizen),
            false
        );

        assert_eq!(KycPallet::get_earliest_request(), None);

        // request kyc
        assert_ok!(KycPallet::request_kyc(
            account1.clone(),
            KycData { id: id1 },
        ));

        System::set_block_number(2);

        assert_err!(
            KycPallet::request_kyc(account1.clone(), KycData { id: id1 },),
            <Error<Test>>::AlreadyAplliedKycRequest
        );

        assert_ok!(KycPallet::request_kyc(
            account2.clone(),
            KycData { id: id2 },
        ));

        assert_err!(
            KycPallet::request_kyc(account2.clone(), KycData { id: id2 },),
            <Error<Test>>::AlreadyAplliedKycRequest
        );

        // get KYC request data
        let request1 = KycPallet::get_earliest_request().unwrap();
        assert_eq!(request1.block_number, 0);
        assert_eq!(request1.account, ensure_signed(account1.clone()).unwrap());
        assert_eq!(request1.data.id, id1);

        let request2 = KycPallet::get_earliest_request().unwrap();
        assert_eq!(request2.block_number, 2);
        assert_eq!(request2.account, ensure_signed(account2.clone()).unwrap());
        assert_eq!(request2.data.id, id2);

        assert_err!(
            KycPallet::kyc_response(account1.clone(), request1.clone(), true),
            <Error<Test>>::AccountCannotProcessKyc
        );
        assert_err!(
            KycPallet::kyc_response(account2.clone(), request2.clone(), true),
            <Error<Test>>::AccountCannotProcessKyc
        );

        assert_ok!(KycPallet::kyc_response(
            reviewer_account.clone(),
            request1.clone(),
            true
        ));
        assert_ok!(KycPallet::kyc_response(
            reviewer_account.clone(),
            request2.clone(),
            false
        ));

        assert_eq!(
            IdentityPallet::check_id_identity(id1, IdentityType::Citizen),
            true
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id2, IdentityType::Citizen),
            false
        );

        assert_eq!(KycPallet::get_earliest_request(), None);

        assert_err!(
            KycPallet::kyc_response(reviewer_account.clone(), request1, true),
            <Error<Test>>::RequestDoesNotExist
        );
        assert_err!(
            KycPallet::kyc_response(reviewer_account, request2, false),
            <Error<Test>>::RequestDoesNotExist
        );

        assert_err!(
            KycPallet::request_kyc(account1, KycData { id: id1 }),
            <Error<Test>>::AccoundIdAlreadyUsed
        );

        assert_ok!(KycPallet::request_kyc(account2, KycData { id: id2 }));
    });
}
