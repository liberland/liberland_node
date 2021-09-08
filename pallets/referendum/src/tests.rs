use crate::mock::*;
use crate::*;
use frame_support::{assert_err, assert_ok, traits::OnFinalize};
use frame_system::ensure_signed;

type Hashing = <Test as frame_system::Config>::Hashing;

#[test]
fn basic_referendum_test() {
    new_test_ext().execute_with(|| {
        // create 10 citizens
        let account1 = Origin::signed(1);
        let id1 = [1; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account1.clone()).unwrap(), id1);
        IdentityPallet::push_identity(id1.clone(), IdentityType::Citizen).unwrap();

        let account2 = Origin::signed(2);
        let id2 = [2; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account2.clone()).unwrap(), id2);
        IdentityPallet::push_identity(id2.clone(), IdentityType::Citizen).unwrap();

        let account3 = Origin::signed(3);
        let id3 = [3; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account3.clone()).unwrap(), id3);
        IdentityPallet::push_identity(id3.clone(), IdentityType::Citizen).unwrap();

        let account4 = Origin::signed(4);
        let id4 = [4; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account4.clone()).unwrap(), id4);
        IdentityPallet::push_identity(id4.clone(), IdentityType::Citizen).unwrap();

        let account5 = Origin::signed(5);
        let id5 = [5; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account5.clone()).unwrap(), id5);
        IdentityPallet::push_identity(id5.clone(), IdentityType::Citizen).unwrap();

        let account6 = Origin::signed(6);
        let id6 = [6; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account6.clone()).unwrap(), id6);
        IdentityPallet::push_identity(id6.clone(), IdentityType::Citizen).unwrap();

        let account7 = Origin::signed(7);
        let id7 = [7; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account7.clone()).unwrap(), id7);
        IdentityPallet::push_identity(id7.clone(), IdentityType::Citizen).unwrap();

        let account8 = Origin::signed(8);
        let id8 = [8; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account8.clone()).unwrap(), id8);
        IdentityPallet::push_identity(id8.clone(), IdentityType::Citizen).unwrap();

        let account9 = Origin::signed(9);
        let id9 = [9; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account9.clone()).unwrap(), id9);
        IdentityPallet::push_identity(id9.clone(), IdentityType::Citizen).unwrap();

        let account10 = Origin::signed(10);
        let id10 = [10; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account10.clone()).unwrap(), id10);
        IdentityPallet::push_identity(id10.clone(), IdentityType::Citizen).unwrap();

        assert_eq!(ReferendumPallet::get_active_petitions().len(), 0);
        assert_eq!(ReferendumPallet::get_active_referendums().len(), 0);
        assert_eq!(ReferendumPallet::get_successfull_referendums().len(), 0);

        // suggest petition
        let sug = Suggestion { data: vec![] };
        let sug_hash = Hashing::hash(&sug.data[..]);

        assert_ok!(ReferendumPallet::suggest_petition(
            account1.clone(),
            sug.clone()
        ));

        assert_eq!(ReferendumPallet::get_active_petitions().len(), 1);
        assert_eq!(ReferendumPallet::get_active_referendums().len(), 0);
        assert_eq!(ReferendumPallet::get_successfull_referendums().len(), 0);

        assert_ok!(ReferendumPallet::vote(account1.clone(), sug_hash));
        assert_ok!(ReferendumPallet::vote(account2.clone(), sug_hash));
        VotingPallet::on_finalize(PetitionDuration::get());

        assert_eq!(ReferendumPallet::get_active_petitions().len(), 0);
        assert_eq!(ReferendumPallet::get_active_referendums().len(), 1);
        assert_eq!(ReferendumPallet::get_successfull_referendums().len(), 0);

        assert_ok!(ReferendumPallet::vote(account1.clone(), sug_hash));
        assert_ok!(ReferendumPallet::vote(account2.clone(), sug_hash));
        assert_ok!(ReferendumPallet::vote(account3.clone(), sug_hash));
        assert_ok!(ReferendumPallet::vote(account4.clone(), sug_hash));
        assert_ok!(ReferendumPallet::vote(account5.clone(), sug_hash));
        assert_ok!(ReferendumPallet::vote(account6.clone(), sug_hash));

        VotingPallet::on_finalize(ReferendumDuration::get());

        assert_eq!(ReferendumPallet::get_active_petitions().len(), 0);
        assert_eq!(ReferendumPallet::get_active_referendums().len(), 0);
        assert_eq!(ReferendumPallet::get_successfull_referendums().len(), 1);
    });
}

#[test]
fn referendum_error_test() {
    new_test_ext().execute_with(|| {
        let account1 = Origin::signed(1);
        let id1 = [1; 32];
        let sug = Suggestion { data: vec![] };
        let sug_hash = ReferendumPallet::get_suggestion_hash(&sug);

        assert_err!(
            ReferendumPallet::suggest_petition(account1.clone(), sug.clone()),
            <Error<Test>>::AccountCannotSuggestPetition
        );

        assert_err!(
            ReferendumPallet::vote(account1.clone(), sug_hash),
            <Error<Test>>::AccountCannotVote
        );

        IdentityPallet::match_account_to_id(ensure_signed(account1.clone()).unwrap(), id1);
        IdentityPallet::push_identity(id1.clone(), IdentityType::Citizen).unwrap();

        assert_err!(
            ReferendumPallet::vote(account1.clone(), sug_hash),
            <Error<Test>>::SubjectDoesNotExist
        );

        assert_ok!(ReferendumPallet::suggest_petition(
            account1.clone(),
            sug.clone()
        ));

        assert_ok!(ReferendumPallet::vote(account1.clone(), sug_hash));

        assert_err!(
            ReferendumPallet::vote(account1.clone(), sug_hash),
            <Error<Test>>::AlreadyVoted
        );
    });
}
