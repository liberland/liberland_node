use crate::mock::*;
use crate::*;
use assert::assert_err;
use frame_support::{assert_ok, traits::OnFinalize};
use sp_runtime::traits::Hash;

#[test]
fn basic_voting_test() {
    new_test_ext().execute_with(|| {
        type Hashing = <Test as frame_system::Config>::Hashing;

        let subject = Hashing::hash(&[1; 32]);
        let duration = 100;

        assert!(VotingPallet::active_votings(subject.clone()).is_none());
        assert_ok!(VotingPallet::create_voting(subject.clone(), duration));
        assert!(VotingPallet::active_votings(subject.clone()).is_some());

        assert_err!(VotingPallet::create_voting(subject.clone(), duration));

        assert_err!(VotingPallet::vote(Hashing::hash(&[2; 32]), 1));

        assert_ok!(VotingPallet::vote(subject, 2));

        VotingPallet::on_finalize(duration);

        assert!(VotingPallet::active_votings(subject.clone()).is_none());
        assert_ok!(VotingPallet::create_voting(subject.clone(), duration));
        assert!(VotingPallet::active_votings(subject.clone()).is_some());
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

        assert!(VotingPallet::active_alt_votings(subject.clone()).is_none());
        assert_ok!(VotingPallet::create_alt_voting(
            subject.clone(),
            duration,
            subjects_list.clone()
        ));
        assert!(VotingPallet::active_alt_votings(subject.clone()).is_some());

        ballots_list.iter().for_each(|ballot| {
            let power = 1;
            assert_ok!(VotingPallet::alt_vote(
                subject.clone(),
                ballot.0,
                ballot.1.clone(),
                power
            ));
        });

        assert_eq!(
            VotingPallet::calculate_alt_vote_winner(subject).unwrap(),
            [3_u8; 32].to_vec()
        );

        VotingPallet::on_finalize(duration);

        assert!(VotingPallet::active_alt_votings(subject.clone()).is_none());
        assert_ok!(VotingPallet::create_alt_voting(
            subject.clone(),
            duration,
            subjects_list.clone()
        ));
        assert!(VotingPallet::active_alt_votings(subject.clone()).is_some());
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

        assert!(VotingPallet::active_alt_list_votings(subject.clone()).is_none());
        assert_ok!(VotingPallet::create_alt_voting_list(
            subject.clone(),
            duration,
            subjects_list.clone(),
            2,
        ));
        assert!(VotingPallet::active_alt_list_votings(subject.clone()).is_some());

        ballots_list.iter().for_each(|ballot| {
            let power = 1;
            assert_ok!(VotingPallet::alt_vote_list(
                subject.clone(),
                ballot.0,
                ballot.1.clone(),
                power
            ));
        });

        let mut winners = BTreeMap::new();
        winners.insert([1_u8; 32].to_vec(), 4);
        winners.insert([3_u8; 32].to_vec(), 7);
        assert_eq!(
            VotingPallet::calculate_alt_vote_winners_list(subject).unwrap(),
            winners
        );
        VotingPallet::on_finalize(duration);

        assert!(VotingPallet::active_alt_list_votings(subject.clone()).is_none());
        assert_ok!(VotingPallet::create_alt_voting_list(
            subject.clone(),
            duration,
            subjects_list.clone(),
            2,
        ));
        assert!(VotingPallet::active_alt_list_votings(subject.clone()).is_some());
    });
}

#[test]
fn alt_vote_list_teset_with_power() {
    new_test_ext().execute_with(|| {
        type Hashing = <Test as frame_system::Config>::Hashing;
        let ballots_list = get_ballots_mock();
        let subjects_list = get_mock_subjects();
        let subject = Hashing::hash(&[1; 32]);
        let duration = 100;

        assert!(VotingPallet::active_alt_list_votings(subject.clone()).is_none());
        assert_ok!(VotingPallet::create_alt_voting_list(
            subject.clone(),
            duration,
            subjects_list.clone(),
            2,
        ));
        assert!(VotingPallet::active_alt_list_votings(subject.clone()).is_some());

        assert_ok!(VotingPallet::alt_vote_list(
            subject.clone(),
            ballots_list[0].0.clone(),
            ballots_list[0].1.clone(),
            10
        ));

        assert_ok!(VotingPallet::alt_vote_list(
            subject.clone(),
            ballots_list[1].0.clone(),
            ballots_list[1].1.clone(),
            10
        ));

        assert_ok!(VotingPallet::alt_vote_list(
            subject.clone(),
            ballots_list[2].0.clone(),
            ballots_list[2].1.clone(),
            10
        ));

        assert_ok!(VotingPallet::alt_vote_list(
            subject.clone(),
            ballots_list[3].0.clone(),
            ballots_list[3].1.clone(),
            5
        ));

        assert_ok!(VotingPallet::alt_vote_list(
            subject.clone(),
            ballots_list[4].0.clone(),
            ballots_list[4].1.clone(),
            5
        ));

        assert_ok!(VotingPallet::alt_vote_list(
            subject.clone(),
            ballots_list[5].0.clone(),
            ballots_list[5].1.clone(),
            5
        ));

        assert_ok!(VotingPallet::alt_vote_list(
            subject.clone(),
            ballots_list[6].0.clone(),
            ballots_list[6].1.clone(),
            1
        ));

        assert_ok!(VotingPallet::alt_vote_list(
            subject.clone(),
            ballots_list[7].0.clone(),
            ballots_list[7].1.clone(),
            1
        ));

        assert_ok!(VotingPallet::alt_vote_list(
            subject.clone(),
            ballots_list[8].0.clone(),
            ballots_list[8].1.clone(),
            1
        ));

        assert_ok!(VotingPallet::alt_vote_list(
            subject.clone(),
            ballots_list[9].0.clone(),
            ballots_list[9].1.clone(),
            1
        ));
        assert_ok!(VotingPallet::alt_vote_list(
            subject.clone(),
            ballots_list[10].0.clone(),
            ballots_list[10].1.clone(),
            1
        ));

        let mut winners = BTreeMap::new();
        winners.insert([1_u8; 32].to_vec(), 35);
        winners.insert([2_u8; 32].to_vec(), 14);

        assert_eq!(
            VotingPallet::calculate_alt_vote_winners_list(subject).unwrap(),
            winners
        );
    });
}

#[test]
fn alt_vote_teset_with_power() {
    new_test_ext().execute_with(|| {
        type Hashing = <Test as frame_system::Config>::Hashing;
        let ballots_list = get_ballots_mock();
        let subjects_list = get_mock_subjects();
        let subject = Hashing::hash(&[1; 32]);
        let duration = 100;

        assert!(VotingPallet::active_alt_votings(subject.clone()).is_none());
        assert_ok!(VotingPallet::create_alt_voting(
            subject.clone(),
            duration,
            subjects_list.clone(),
        ));
        assert!(VotingPallet::active_alt_votings(subject.clone()).is_some());

        assert_ok!(VotingPallet::alt_vote(
            subject.clone(),
            ballots_list[0].0.clone(),
            ballots_list[0].1.clone(),
            10
        ));

        assert_ok!(VotingPallet::alt_vote(
            subject.clone(),
            ballots_list[1].0.clone(),
            ballots_list[1].1.clone(),
            10
        ));

        assert_ok!(VotingPallet::alt_vote(
            subject.clone(),
            ballots_list[2].0.clone(),
            ballots_list[2].1.clone(),
            10
        ));

        assert_ok!(VotingPallet::alt_vote(
            subject.clone(),
            ballots_list[3].0.clone(),
            ballots_list[3].1.clone(),
            5
        ));

        assert_ok!(VotingPallet::alt_vote(
            subject.clone(),
            ballots_list[4].0.clone(),
            ballots_list[4].1.clone(),
            5
        ));

        assert_ok!(VotingPallet::alt_vote(
            subject.clone(),
            ballots_list[5].0.clone(),
            ballots_list[5].1.clone(),
            5
        ));

        assert_ok!(VotingPallet::alt_vote(
            subject.clone(),
            ballots_list[6].0.clone(),
            ballots_list[6].1.clone(),
            1
        ));

        assert_ok!(VotingPallet::alt_vote(
            subject.clone(),
            ballots_list[7].0.clone(),
            ballots_list[7].1.clone(),
            1
        ));

        assert_ok!(VotingPallet::alt_vote(
            subject.clone(),
            ballots_list[8].0.clone(),
            ballots_list[8].1.clone(),
            1
        ));

        assert_ok!(VotingPallet::alt_vote(
            subject.clone(),
            ballots_list[9].0.clone(),
            ballots_list[9].1.clone(),
            1
        ));
        assert_ok!(VotingPallet::alt_vote(
            subject.clone(),
            ballots_list[10].0.clone(),
            ballots_list[10].1.clone(),
            1
        ));

        let winner = [1_u8; 32].to_vec();

        assert_eq!(
            VotingPallet::calculate_alt_vote_winner(subject).unwrap(),
            winner
        );
    });
}
