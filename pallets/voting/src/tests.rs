use crate::mock::*;
use crate::*;
use assert::assert_err;
use frame_support::{assert_ok, traits::OnFinalize};
use sp_runtime::traits::Hash;

#[test]
fn basic_voting_test() {
    new_test_ext().execute_with(|| {
        type Hashing = <Test as frame_system::Config>::Hashing;

        assert_eq!(VotingPallet::get_active_votings().len(), 0);

        let subject = Hashing::hash(&[1; 32]);
        let duration = 100;

        assert_ok!(VotingPallet::create_voting(subject.clone(), duration));
        assert_eq!(VotingPallet::get_active_votings().len(), 1);

        assert_err!(VotingPallet::create_voting(subject.clone(), duration));

        assert_err!(VotingPallet::vote(Hashing::hash(&[2; 32]), 1));

        assert_ok!(VotingPallet::vote(subject, 2));

        VotingPallet::on_finalize(duration);

        assert_eq!(VotingPallet::get_active_votings().len(), 0);

        assert_ok!(VotingPallet::create_voting(subject.clone(), duration));
        assert_eq!(VotingPallet::get_active_votings().len(), 1);
    });
}

#[test]
fn basic_alt_voting_test() {
    new_test_ext().execute_with(|| {
        type Hashing = <Test as frame_system::Config>::Hashing;

        let ballots_list = get_ballots_mock();
        let subjects_list = get_mock_subjects();
        let subject = Hashing::hash(&[1; 32]);
        let duration = 100;

        assert_eq!(VotingPallet::get_active_alt_votings().len(), 0);

        assert_ok!(VotingPallet::create_alt_voting(
            subject.clone(),
            duration,
            subjects_list.clone()
        ));
        assert_eq!(VotingPallet::get_active_alt_votings().len(), 1);

        ballots_list.iter().for_each(|e| {
            assert_ok!(VotingPallet::alt_vote(subject.clone(), e.clone()));
        });

        assert_eq!(
            VotingPallet::calculate_alt_vote_winner(subject).unwrap(),
            [3_u8; 32].to_vec()
        );

        VotingPallet::on_finalize(duration);

        assert_eq!(VotingPallet::get_active_alt_votings().len(), 0);

        assert_ok!(VotingPallet::create_alt_voting(
            subject.clone(),
            duration,
            subjects_list.clone()
        ));
        assert_eq!(VotingPallet::get_active_alt_votings().len(), 1);
    });
}

#[test]
fn basic_alt_voting_list_test() {
    new_test_ext().execute_with(|| {
        type Hashing = <Test as frame_system::Config>::Hashing;

        let ballots_list = get_ballots_mock();
        let subjects_list = get_mock_subjects();
        let subject = Hashing::hash(&[1; 32]);
        let duration = 100;

        assert_eq!(VotingPallet::get_active_alt_list_votings().len(), 0);

        assert_ok!(VotingPallet::create_alt_voting_list(
            subject.clone(),
            duration,
            subjects_list.clone(),
            2
        ));
        assert_eq!(VotingPallet::get_active_alt_list_votings().len(), 1);

        ballots_list.iter().for_each(|e| {
            assert_ok!(VotingPallet::alt_vote_list(subject.clone(), e.clone()));
        });

        let mut winners = BTreeSet::new();
        winners.insert([1_u8; 32].to_vec());
        winners.insert([3_u8; 32].to_vec());
        assert_eq!(
            VotingPallet::calculate_alt_vote_winners_list(subject).unwrap(),
            winners
        );
        VotingPallet::on_finalize(duration);

        assert_eq!(VotingPallet::get_active_alt_list_votings().len(), 0);

        assert_ok!(VotingPallet::create_alt_voting_list(
            subject.clone(),
            duration,
            subjects_list.clone(),
            2
        ));
        assert_eq!(VotingPallet::get_active_alt_list_votings().len(), 1);
    });
}
