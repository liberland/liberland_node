use crate::mock::*;
use crate::*;
use frame_system::ensure_signed;

#[test]
fn basic_identity_test() {
    new_test_ext().execute_with(|| {
        let account = ensure_signed(Origin::signed(1)).unwrap();
        let id = [1; 32];
        // check empty
        assert_eq!(IdentityPallet::get_id_identities(id), BTreeSet::new());
        assert_eq!(
            IdentityPallet::get_account_identities(account),
            BTreeSet::new()
        );

        // Push Citizen
        IdentityPallet::match_account_to_id(account, id);
        IdentityPallet::push_identity(id, IdentityType::Citizen).unwrap();
        assert_eq!(
            IdentityPallet::get_id_identities(id),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::get_account_identities(account),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::Citizen),
            true
        );
        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::Citizen),
            true
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::MinisterOfInterior),
            false
        );
        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::MinisterOfInterior),
            false
        );

        // Push MinisterOfInterior
        IdentityPallet::push_identity(id, IdentityType::MinisterOfInterior).unwrap();
        assert_eq!(
            IdentityPallet::get_id_identities(id),
            [IdentityType::Citizen, IdentityType::MinisterOfInterior]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::get_account_identities(account),
            [IdentityType::Citizen, IdentityType::MinisterOfInterior]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::Citizen),
            true
        );
        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::Citizen),
            true
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::MinisterOfInterior),
            true
        );
        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::MinisterOfInterior),
            true
        );
        IdentityPallet::remove_identity(id, IdentityType::Citizen);
        // Push Eresidence

        IdentityPallet::push_identity(id, IdentityType::EResident).unwrap();
        assert_eq!(
            IdentityPallet::get_id_identities(id),
            [IdentityType::EResident].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::get_account_identities(account),
            [IdentityType::EResident].iter().cloned().collect()
        );

        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::EResident),
            true
        );
        assert_eq!(
            IdentityPallet::get_account_identities(account),
            [IdentityType::EResident].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::EResident),
            true
        );

        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::EResident),
            true
        );

        IdentityPallet::remove_identity(id, IdentityType::EResident);

        //remove
        IdentityPallet::push_identity(id, IdentityType::Citizen).unwrap();
        IdentityPallet::remove_identity(id, IdentityType::Citizen);

        assert_eq!(IdentityPallet::get_id_identities(id), BTreeSet::new());
        assert_eq!(
            IdentityPallet::get_account_identities(account),
            BTreeSet::new()
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::Citizen),
            false
        );
        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::Citizen),
            false
        );
        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::MinisterOfInterior),
            false
        );
        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::MinisterOfInterior),
            false
        );

        assert_eq!(
            IdentityPallet::check_id_identity(id, IdentityType::EResident),
            false
        );
        assert_eq!(
            IdentityPallet::check_account_indetity(account, IdentityType::EResident),
            false
        );
    });
}

#[test]
fn citizen_amount_test() {
    new_test_ext().execute_with(|| {
        let id = [1; 32];
        let id_2 = [2; 32];
        IdentityPallet::push_identity(id, IdentityType::Citizen).unwrap();
        IdentityPallet::push_identity(id, IdentityType::MinisterOfInterior).unwrap();

        assert_eq!(
            IdentityPallet::get_id_identities(id),
            [IdentityType::Citizen, IdentityType::MinisterOfInterior]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(IdentityPallet::get_citizens_amount(), 1);

        IdentityPallet::remove_identity(id, IdentityType::Citizen);
        assert_eq!(IdentityPallet::get_citizens_amount(), 0);

        IdentityPallet::push_identity(id, IdentityType::Citizen).unwrap();
        assert_eq!(
            IdentityPallet::push_identity(id, IdentityType::EResident),
            Err("Citizen cannot become the Eresidence at the same time")
        );
        assert_eq!(IdentityPallet::get_citizens_amount(), 1);

        IdentityPallet::push_identity(id_2, IdentityType::EResident).unwrap();
        assert_eq!(IdentityPallet::get_citizens_amount(), 1);

        assert_eq!(
            IdentityPallet::push_identity(id_2, IdentityType::MinisterOfInterior),
            Err("We can not add any other IdentityTypes before we have add IdentityType::Citizen")
        );

        assert_eq!(
            IdentityPallet::push_identity(id_2, IdentityType::Citizen),
            Err("Eresidence cannot become the Citizen at the same time")
        );

        IdentityPallet::remove_identity(id_2, IdentityType::EResident);
        assert_eq!(IdentityPallet::get_citizens_amount(), 1);

        IdentityPallet::push_identity(id_2, IdentityType::Citizen).unwrap();

        assert_eq!(IdentityPallet::get_citizens_amount(), 2);
    });
}
