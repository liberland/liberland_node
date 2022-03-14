use std::collections::VecDeque;

use crate::mock::*;
use crate::*;
use frame_support::{
    assert_err, assert_ok,
    traits::{OnFinalize, OnInitialize},
};
use frame_system::ensure_signed;
use pallet_staking::RewardDestination;
use sp_runtime::traits::Hash;

#[test]
fn basic_assembly_test() {
    ExtBuilder::default().build_and_execute(|| {
        let id1 = [1; 32];
        let account1 = Origin::signed(1);

        IdentityPallet::match_account_to_id(ensure_signed(account1.clone()).unwrap(), id1);
        IdentityPallet::push_identity(id1.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id1).unwrap();

        let account2 = Origin::signed(2);
        let id2 = [2; 32];

        Staking::liberland_bond(Origin::signed(2), 2, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account2.clone()).unwrap(), id2);
        IdentityPallet::push_identity(id2.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id2).unwrap();

        let account3 = Origin::signed(3);
        let id3 = [3; 32];

        IdentityPallet::match_account_to_id(ensure_signed(account3.clone()).unwrap(), id3);
        IdentityPallet::push_identity(id3.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id3).unwrap();

        let account4 = Origin::signed(4);
        let id4 = [4; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account4.clone()).unwrap(), id4);
        IdentityPallet::push_identity(id4.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id4).unwrap();

        let account5 = Origin::signed(5);
        let id5 = [5; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account5.clone()).unwrap(), id5);
        IdentityPallet::push_identity(id5.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id5).unwrap();

        // Add vouters
        let account6 = Origin::signed(6);
        let id6 = [6; 32];

        Staking::liberland_bond(Origin::signed(6), 6, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account6.clone()).unwrap(), id6);
        IdentityPallet::push_identity(id6.clone(), IdentityType::Citizen).unwrap();

        let account7 = Origin::signed(7);
        let id7 = [7; 32];
        Staking::liberland_bond(Origin::signed(7), 7, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account7.clone()).unwrap(), id7);
        IdentityPallet::push_identity(id7.clone(), IdentityType::Citizen).unwrap();

        let account8 = Origin::signed(8);
        let id8 = [8; 32];
        Staking::liberland_bond(Origin::signed(8), 8, 3, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account8.clone()).unwrap(), id8);
        IdentityPallet::push_identity(id8.clone(), IdentityType::Citizen).unwrap();

        let account9 = Origin::signed(9);
        let id9 = [9; 32];
        Staking::liberland_bond(Origin::signed(9), 9, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account9.clone()).unwrap(), id9);
        IdentityPallet::push_identity(id9.clone(), IdentityType::Citizen).unwrap();

        let account10 = Origin::signed(17);
        let id10 = [17; 32];
        Staking::liberland_bond(Origin::signed(17), 17, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account10.clone()).unwrap(), id10);
        IdentityPallet::push_identity(id10.clone(), IdentityType::Citizen).unwrap();
        // voting state test
        assert_eq!(AssemblyPallet::voting_state(), false);
        AssemblyPallet::on_initialize(50);
        assert_eq!(AssemblyPallet::voting_state(), true);
        let v = vec![
            [2_u8; 32].to_vec(),
            [1_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_1 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [1_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_2 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [1_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_3 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [3_u8; 32].to_vec(),
            [1_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_4 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [1_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_5 = pallet_voting::AltVote::new(voutes);

        AssemblyPallet::vote(account6.clone(), ballot_1.clone()).unwrap();
        AssemblyPallet::vote(account7.clone(), ballot_2.clone()).unwrap();
        AssemblyPallet::vote(account8.clone(), ballot_3.clone()).unwrap();
        AssemblyPallet::vote(account9.clone(), ballot_4.clone()).unwrap();
        AssemblyPallet::vote(account10.clone(), ballot_5.clone()).unwrap();
        assert_eq!(
            IdentityPallet::identities([1_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::identities([2_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::identities([3_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        VotingPallet::on_finalize(50 + 20);
        assert_eq!(AssemblyPallet::voting_state(), false);
        let mut winners = BTreeMap::new();
        winners.insert([1_u8; 32].to_vec(), 5);
        winners.insert([2_u8; 32].to_vec(), 1);
        winners.insert([3_u8; 32].to_vec(), 1);
        assert_eq!(AssemblyPallet::ministers_list(), winners);
        assert_eq!(
            IdentityPallet::identities([1_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::identities([2_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::identities([3_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );

        AssemblyPallet::on_initialize(70 + 30);
        // Change power test

        //AssemblyPallet::change_support_v2(account8.clone(), [1_u8; 32].to_vec(), -1).unwrap();
        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), 1).unwrap();
        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), 1).unwrap();
        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), 1).unwrap();
        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), 1).unwrap();
        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), 1).unwrap();

        let mut winners = BTreeMap::new();
        winners.insert([1_u8; 32].to_vec(), 5);
        winners.insert([2_u8; 32].to_vec(), 1);
        winners.insert([3_u8; 32].to_vec(), 1);
        assert_eq!(AssemblyPallet::ministers_list(), winners);

        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), -1).unwrap();
        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), -1).unwrap();
        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), -1).unwrap();
        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), -1).unwrap();
        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), -1).unwrap();

        let mut winners = BTreeMap::new();
        winners.insert([1_u8; 32].to_vec(), 2);
        winners.insert([2_u8; 32].to_vec(), 1);
        winners.insert([3_u8; 32].to_vec(), 1);
        assert_eq!(AssemblyPallet::ministers_list(), winners);

        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), 1).unwrap();
        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), 1).unwrap();
        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), 1).unwrap();
        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), 1).unwrap();
        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), 1).unwrap();

        let mut winners = BTreeMap::new();
        winners.insert([1_u8; 32].to_vec(), 5);
        winners.insert([2_u8; 32].to_vec(), 1);
        winners.insert([3_u8; 32].to_vec(), 1);
        assert_eq!(AssemblyPallet::ministers_list(), winners);

        AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), -3).unwrap();
        AssemblyPallet::change_support(account8.clone(), [2_u8; 32].to_vec(), 3).unwrap();

        let mut winners = BTreeMap::new();
        winners.insert([1_u8; 32].to_vec(), 2);
        winners.insert([2_u8; 32].to_vec(), 4);
        winners.insert([3_u8; 32].to_vec(), 1);
        assert_eq!(AssemblyPallet::ministers_list(), winners);

        assert_err!(
            AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), 10),
            <Error<Test>>::ChangePowerTooBig
        );
        assert_err!(
            AssemblyPallet::change_support(account8.clone(), [1_u8; 32].to_vec(), -10),
            <Error<Test>>::ChangePowerTooBig
        );

        AssemblyPallet::on_initialize(10 + 210);

        assert_eq!(
            IdentityPallet::identities([1_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::identities([2_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::identities([3_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );
        VotingPallet::on_finalize(100 + 20);

        assert_eq!(
            IdentityPallet::identities([1_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::identities([2_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::identities([3_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );

        // Testing multiple votes
        AssemblyPallet::add_candidate_internal(id1).unwrap();
        AssemblyPallet::add_candidate_internal(id2).unwrap();
        AssemblyPallet::add_candidate_internal(id3).unwrap();
        AssemblyPallet::add_candidate_internal(id4).unwrap();
        AssemblyPallet::add_candidate_internal(id5).unwrap();

        AssemblyPallet::on_initialize(120 + 30);

        AssemblyPallet::vote(account6.clone(), ballot_1.clone()).unwrap();
        AssemblyPallet::vote(account7.clone(), ballot_2.clone()).unwrap();
        AssemblyPallet::vote(account8.clone(), ballot_3.clone()).unwrap();
        AssemblyPallet::vote(account9.clone(), ballot_4.clone()).unwrap();
        AssemblyPallet::vote(account10.clone(), ballot_5.clone()).unwrap();

        assert_eq!(
            IdentityPallet::identities([1_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::identities([2_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::identities([3_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        VotingPallet::on_finalize(150 + 20);
        assert_eq!(AssemblyPallet::voting_state(), false);
        let mut winners = BTreeMap::new();
        winners.insert([1_u8; 32].to_vec(), 5);
        winners.insert([2_u8; 32].to_vec(), 1);
        winners.insert([3_u8; 32].to_vec(), 1);
        assert_eq!(AssemblyPallet::ministers_list(), winners);
        assert_eq!(
            IdentityPallet::identities([1_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::identities([2_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::identities([3_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );

        AssemblyPallet::add_candidate_internal(id1).unwrap();
        AssemblyPallet::add_candidate_internal(id2).unwrap();
        AssemblyPallet::add_candidate_internal(id3).unwrap();
        AssemblyPallet::add_candidate_internal(id4).unwrap();
        AssemblyPallet::add_candidate_internal(id5).unwrap();

        AssemblyPallet::on_initialize(170 + 30);

        AssemblyPallet::vote(account6.clone(), ballot_1.clone()).unwrap();
        AssemblyPallet::vote(account7.clone(), ballot_2.clone()).unwrap();
        AssemblyPallet::vote(account8.clone(), ballot_3.clone()).unwrap();
        AssemblyPallet::vote(account9.clone(), ballot_4.clone()).unwrap();
        AssemblyPallet::vote(account10.clone(), ballot_5.clone()).unwrap();

        VotingPallet::on_finalize(200 + 20);
        assert_eq!(AssemblyPallet::voting_state(), false);
        let mut winners = BTreeMap::new();
        winners.insert([1_u8; 32].to_vec(), 5);
        winners.insert([2_u8; 32].to_vec(), 1);
        winners.insert([3_u8; 32].to_vec(), 1);
        assert_eq!(AssemblyPallet::ministers_list(), winners);
        assert_eq!(
            IdentityPallet::identities([1_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::identities([2_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::identities([3_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );
    });
}

#[test]
fn assembly_errorss_test() {
    ExtBuilder::default().build_and_execute(|| {
        let id1 = [1; 32];
        let account1 = Origin::signed(1);

        IdentityPallet::match_account_to_id(ensure_signed(account1.clone()).unwrap(), id1);
        IdentityPallet::push_identity(id1.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id1).unwrap();

        let account2 = Origin::signed(2);
        let id2 = [2; 32];

        IdentityPallet::match_account_to_id(ensure_signed(account2.clone()).unwrap(), id2);
        IdentityPallet::push_identity(id2.clone(), IdentityType::Citizen).unwrap();

        let v = vec![
            [2_u8; 32].to_vec(),
            [1_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_1 = pallet_voting::AltVote::new(voutes.clone());
        assert_err!(
            AssemblyPallet::vote(account2.clone(), ballot_1.clone()),
            <Error<Test>>::VotingNotFound
        );

        AssemblyPallet::on_initialize(50);

        assert_ok!(AssemblyPallet::vote(account2.clone(), ballot_1.clone()));
        assert_err!(
            AssemblyPallet::add_candidate(account2.clone()),
            <Error<Test>>::VotingIsAlreadyInProgress
        );
        assert_err!(
            AssemblyPallet::vote(account2.clone(), ballot_1.clone()),
            <Error<Test>>::AlreadyVoted
        );
    });
}

#[test]
fn basic_law_voting_test() {
    ExtBuilder::default().build_and_execute(|| {
        let id1 = [1; 32];
        let account1 = Origin::signed(1);

        IdentityPallet::match_account_to_id(ensure_signed(account1.clone()).unwrap(), id1);
        IdentityPallet::push_identity(id1.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id1).unwrap();

        let account2 = Origin::signed(2);
        let id2 = [2; 32];

        Staking::liberland_bond(Origin::signed(2), 2, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account2.clone()).unwrap(), id2);
        IdentityPallet::push_identity(id2.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id2).unwrap();

        let account3 = Origin::signed(3);
        let id3 = [3; 32];

        IdentityPallet::match_account_to_id(ensure_signed(account3.clone()).unwrap(), id3);
        IdentityPallet::push_identity(id3.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id3).unwrap();

        let account4 = Origin::signed(4);
        let id4 = [4; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account4.clone()).unwrap(), id4);
        IdentityPallet::push_identity(id4.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id4).unwrap();

        let account5 = Origin::signed(5);
        let id5 = [5; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account5.clone()).unwrap(), id5);
        IdentityPallet::push_identity(id5.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id5).unwrap();

        // Add vouters
        let account6 = Origin::signed(6);
        let id6 = [6; 32];

        Staking::liberland_bond(Origin::signed(6), 6, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account6.clone()).unwrap(), id6);
        IdentityPallet::push_identity(id6.clone(), IdentityType::Citizen).unwrap();

        let account7 = Origin::signed(7);
        let id7 = [7; 32];
        Staking::liberland_bond(Origin::signed(7), 7, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account7.clone()).unwrap(), id7);
        IdentityPallet::push_identity(id7.clone(), IdentityType::Citizen).unwrap();

        let account8 = Origin::signed(8);
        let id8 = [8; 32];
        Staking::liberland_bond(Origin::signed(8), 8, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account8.clone()).unwrap(), id8);
        IdentityPallet::push_identity(id8.clone(), IdentityType::Citizen).unwrap();

        let account9 = Origin::signed(9);
        let id9 = [9; 32];
        Staking::liberland_bond(Origin::signed(9), 9, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account9.clone()).unwrap(), id9);
        IdentityPallet::push_identity(id9.clone(), IdentityType::Citizen).unwrap();

        let account10 = Origin::signed(17);
        let id10 = [17; 32];
        Staking::liberland_bond(Origin::signed(17), 17, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account10.clone()).unwrap(), id10);
        IdentityPallet::push_identity(id10.clone(), IdentityType::Citizen).unwrap();

        AssemblyPallet::on_initialize(50);
        let v = vec![
            [2_u8; 32].to_vec(),
            [1_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_1 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [1_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_2 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [1_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_3 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [3_u8; 32].to_vec(),
            [1_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_4 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [1_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_5 = pallet_voting::AltVote::new(voutes);
        AssemblyPallet::vote(account6, ballot_1).unwrap();
        AssemblyPallet::vote(account7, ballot_2).unwrap();
        AssemblyPallet::vote(account8, ballot_3).unwrap();
        AssemblyPallet::vote(account9, ballot_4).unwrap();
        AssemblyPallet::vote(account10, ballot_5).unwrap();
        assert_eq!(
            IdentityPallet::identities([1_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::identities([2_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::identities([3_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        VotingPallet::on_finalize(10 + 100 + 1);

        let mut winners = BTreeMap::new();
        winners.insert([1_u8; 32].to_vec(), 3);
        winners.insert([2_u8; 32].to_vec(), 1);
        winners.insert([3_u8; 32].to_vec(), 1);
        assert_eq!(AssemblyPallet::ministers_list(), winners);
        assert_eq!(
            IdentityPallet::identities([1_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::identities([2_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            IdentityPallet::identities([3_u8; 32]),
            [IdentityType::Citizen, IdentityType::Assembly]
                .iter()
                .cloned()
                .collect()
        );

        type Hashing = <Test as frame_system::Config>::Hashing;
        let law_hash_1 = Hashing::hash(&[1; 32]);
        let law_hash_2 = Hashing::hash(&[2; 32]);
        let law_hash_3 = Hashing::hash(&[3; 32]);
        AssemblyPallet::propose_law(account1.clone(), law_hash_1, LawType::ConstitutionalChange)
            .unwrap();
        AssemblyPallet::propose_law(account2.clone(), law_hash_2, LawType::Legislation).unwrap();
        AssemblyPallet::propose_law(account2.clone(), law_hash_3, LawType::Legislation).unwrap();

        AssemblyPallet::vote_to_law(account1.clone(), law_hash_1, Decision::Accept).unwrap();
        AssemblyPallet::vote_to_law(account2.clone(), law_hash_1, Decision::Accept).unwrap();
        AssemblyPallet::vote_to_law(account3.clone(), law_hash_1, Decision::Accept).unwrap();
        assert_eq!(
            AssemblyPallet::laws(law_hash_1).unwrap(),
            Law {
                state: LawState::InProgress,
                law_type: LawType::ConstitutionalChange
            }
        );
        assert_err!(
            AssemblyPallet::vote_to_law(account1.clone(), law_hash_1, Decision::Accept),
            <Error<Test>>::AlreadyVoted
        );
        AssemblyPallet::vote_to_law(account1.clone(), law_hash_3, Decision::Accept).unwrap();

        assert_eq!(
            AssemblyPallet::laws(law_hash_2).unwrap(),
            Law {
                state: LawState::InProgress,
                law_type: LawType::Legislation
            }
        );
        assert_eq!(
            AssemblyPallet::laws(law_hash_3).unwrap(),
            Law {
                state: LawState::InProgress,
                law_type: LawType::Legislation
            }
        );
        VotingPallet::on_finalize(10 + 100 + 1);

        assert_eq!(
            AssemblyPallet::laws(law_hash_1).unwrap(),
            Law {
                state: LawState::Approved,
                law_type: LawType::ConstitutionalChange
            }
        );
        assert_eq!(
            AssemblyPallet::laws(law_hash_3).unwrap(),
            Law {
                state: LawState::Declined,
                law_type: LawType::Legislation
            }
        );
        assert_eq!(
            AssemblyPallet::laws(law_hash_2).unwrap(),
            Law {
                state: LawState::Declined,
                law_type: LawType::Legislation
            }
        );

        let law_hash_4 = Hashing::hash(&[4; 32]);
        AssemblyPallet::propose_law(account1.clone(), law_hash_4, LawType::ConstitutionalChange)
            .unwrap();
        AssemblyPallet::vote_to_law(account1.clone(), law_hash_4, Decision::Accept).unwrap();

        assert_eq!(
            AssemblyPallet::laws(law_hash_4).unwrap(),
            Law {
                state: LawState::InProgress,
                law_type: LawType::ConstitutionalChange
            }
        );
        VotingPallet::on_finalize(1);

        assert_eq!(
            AssemblyPallet::laws(law_hash_4).unwrap(),
            Law {
                state: LawState::InProgress,
                law_type: LawType::ConstitutionalChange
            }
        );
        AssemblyPallet::vote_to_law(account2.clone(), law_hash_4, Decision::Accept).unwrap();
        AssemblyPallet::vote_to_law(account3.clone(), law_hash_4, Decision::Accept).unwrap();

        VotingPallet::on_finalize(1);

        assert_eq!(
            AssemblyPallet::laws(law_hash_4).unwrap(),
            Law {
                state: LawState::Approved,
                law_type: LawType::ConstitutionalChange
            }
        );
    });
}

#[test]
fn basic_prime_min_voting_test() {
    ExtBuilder::default().build_and_execute(|| {
        let id1 = [1; 32];
        let account1 = Origin::signed(1);

        IdentityPallet::match_account_to_id(ensure_signed(account1.clone()).unwrap(), id1);
        IdentityPallet::push_identity(id1.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id1).unwrap();

        let account2 = Origin::signed(2);
        let id2 = [2; 32];

        Staking::liberland_bond(Origin::signed(2), 2, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account2.clone()).unwrap(), id2);
        IdentityPallet::push_identity(id2.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id2).unwrap();

        let account3 = Origin::signed(3);
        let id3 = [3; 32];

        IdentityPallet::match_account_to_id(ensure_signed(account3.clone()).unwrap(), id3);
        IdentityPallet::push_identity(id3.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id3).unwrap();

        let account4 = Origin::signed(4);
        let id4 = [4; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account4.clone()).unwrap(), id4);
        IdentityPallet::push_identity(id4.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id4).unwrap();

        let account5 = Origin::signed(5);
        let id5 = [5; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account5.clone()).unwrap(), id5);
        IdentityPallet::push_identity(id5.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id5).unwrap();

        // Add vouters
        let account6 = Origin::signed(6);
        let id6 = [6; 32];

        Staking::liberland_bond(Origin::signed(6), 6, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account6.clone()).unwrap(), id6);
        IdentityPallet::push_identity(id6.clone(), IdentityType::Citizen).unwrap();

        let account7 = Origin::signed(7);
        let id7 = [7; 32];
        Staking::liberland_bond(Origin::signed(7), 7, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account7.clone()).unwrap(), id7);
        IdentityPallet::push_identity(id7.clone(), IdentityType::Citizen).unwrap();

        let account8 = Origin::signed(8);
        let id8 = [8; 32];
        Staking::liberland_bond(Origin::signed(8), 8, 3, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account8.clone()).unwrap(), id8);
        IdentityPallet::push_identity(id8.clone(), IdentityType::Citizen).unwrap();

        let account9 = Origin::signed(9);
        let id9 = [9; 32];
        Staking::liberland_bond(Origin::signed(9), 9, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account9.clone()).unwrap(), id9);
        IdentityPallet::push_identity(id9.clone(), IdentityType::Citizen).unwrap();

        let account10 = Origin::signed(17);
        let id10 = [17; 32];
        Staking::liberland_bond(Origin::signed(17), 17, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account10.clone()).unwrap(), id10);
        IdentityPallet::push_identity(id10.clone(), IdentityType::Citizen).unwrap();
        // voting state test
        assert_eq!(AssemblyPallet::voting_state(), false);
        AssemblyPallet::on_initialize(50);
        assert_eq!(AssemblyPallet::voting_state(), true);
        let v = vec![
            [2_u8; 32].to_vec(),
            [1_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_1 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [1_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_2 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [1_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_3 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [3_u8; 32].to_vec(),
            [1_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_4 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [1_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_5 = pallet_voting::AltVote::new(voutes);

        AssemblyPallet::vote(account6.clone(), ballot_1.clone()).unwrap();
        AssemblyPallet::vote(account7.clone(), ballot_2.clone()).unwrap();
        AssemblyPallet::vote(account8.clone(), ballot_3.clone()).unwrap();
        AssemblyPallet::vote(account9.clone(), ballot_4.clone()).unwrap();
        AssemblyPallet::vote(account10.clone(), ballot_5.clone()).unwrap();
        assert_eq!(
            IdentityPallet::identities([1_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::identities([2_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::identities([3_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        VotingPallet::on_finalize(50 + 20);

        // Add prime minister candidates
        AssemblyPallet::add_prime_min_condidate(account1.clone()).unwrap();
        AssemblyPallet::add_prime_min_condidate(account2.clone()).unwrap();

        AssemblyPallet::on_finalize(50 + 20 + 10);

        let v = vec![[1_u8; 32].to_vec(), [2_u8; 32].to_vec()];
        let voutes = VecDeque::from(v);
        let ballot_1 = pallet_voting::AltVote::new(voutes);

        AssemblyPallet::vote_to_prime_min(account1.clone(), ballot_1.clone()).unwrap();
        AssemblyPallet::vote_to_prime_min(account2.clone(), ballot_1.clone()).unwrap();
        AssemblyPallet::vote_to_prime_min(account3.clone(), ballot_1.clone()).unwrap();

        VotingPallet::on_finalize(70 + 30);
        assert_eq!(
            AssemblyPallet::current_prime_min().unwrap(),
            [1_u8; 32].to_vec()
        );
    });
}

#[test]
fn test_accelerated_alt_voting() {
    ExtBuilder::default().build_and_execute(|| {
        let id1 = [1; 32];
        let account1 = Origin::signed(1);

        IdentityPallet::match_account_to_id(ensure_signed(account1.clone()).unwrap(), id1);
        IdentityPallet::push_identity(id1.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id1).unwrap();

        let account2 = Origin::signed(2);
        let id2 = [2; 32];

        Staking::liberland_bond(Origin::signed(2), 2, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account2.clone()).unwrap(), id2);
        IdentityPallet::push_identity(id2.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id2).unwrap();

        let account3 = Origin::signed(3);
        let id3 = [3; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account3.clone()).unwrap(), id3);
        IdentityPallet::push_identity(id3.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id3).unwrap();

        let account4 = Origin::signed(4);
        let id4 = [4; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account4.clone()).unwrap(), id4);
        IdentityPallet::push_identity(id4.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id4).unwrap();

        let account5 = Origin::signed(5);
        let id5 = [5; 32];
        IdentityPallet::match_account_to_id(ensure_signed(account5.clone()).unwrap(), id5);
        IdentityPallet::push_identity(id5.clone(), IdentityType::Citizen).unwrap();
        AssemblyPallet::add_candidate_internal(id5).unwrap();

        // Add vouters
        let account6 = Origin::signed(6);
        let id6 = [6; 32];

        Staking::liberland_bond(Origin::signed(6), 6, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account6.clone()).unwrap(), id6);
        IdentityPallet::push_identity(id6.clone(), IdentityType::Citizen).unwrap();

        let account7 = Origin::signed(7);
        let id7 = [7; 32];
        Staking::liberland_bond(Origin::signed(7), 7, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account7.clone()).unwrap(), id7);
        IdentityPallet::push_identity(id7.clone(), IdentityType::Citizen).unwrap();

        let account8 = Origin::signed(8);
        let id8 = [8; 32];
        Staking::liberland_bond(Origin::signed(8), 8, 3, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account8.clone()).unwrap(), id8);
        IdentityPallet::push_identity(id8.clone(), IdentityType::Citizen).unwrap();

        let account9 = Origin::signed(9);
        let id9 = [9; 32];
        Staking::liberland_bond(Origin::signed(9), 9, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account9.clone()).unwrap(), id9);
        IdentityPallet::push_identity(id9.clone(), IdentityType::Citizen).unwrap();

        let account10 = Origin::signed(17);
        let id10 = [17; 32];
        Staking::liberland_bond(Origin::signed(17), 17, 1, RewardDestination::Controller).unwrap();
        IdentityPallet::match_account_to_id(ensure_signed(account10.clone()).unwrap(), id10);
        IdentityPallet::push_identity(id10.clone(), IdentityType::Citizen).unwrap();
        // voting state test
        assert_eq!(AssemblyPallet::voting_state(), false);
        AssemblyPallet::on_initialize(50);
        assert_eq!(AssemblyPallet::voting_state(), true);
        let v = vec![
            [2_u8; 32].to_vec(),
            [1_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_1 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [1_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_2 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [1_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_3 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [3_u8; 32].to_vec(),
            [1_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_4 = pallet_voting::AltVote::new(voutes);

        let v = vec![
            [1_u8; 32].to_vec(),
            [2_u8; 32].to_vec(),
            [3_u8; 32].to_vec(),
            [4_u8; 32].to_vec(),
            [5_u8; 32].to_vec(),
        ];
        let voutes = VecDeque::from(v);
        let ballot_5 = pallet_voting::AltVote::new(voutes);

        AssemblyPallet::vote(account6.clone(), ballot_1.clone()).unwrap();
        AssemblyPallet::vote(account7.clone(), ballot_2.clone()).unwrap();
        AssemblyPallet::vote(account8.clone(), ballot_3.clone()).unwrap();
        AssemblyPallet::vote(account9.clone(), ballot_4.clone()).unwrap();
        AssemblyPallet::vote(account10.clone(), ballot_5.clone()).unwrap();
        assert_eq!(
            IdentityPallet::identities([1_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::identities([2_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        assert_eq!(
            IdentityPallet::identities([3_u8; 32]),
            [IdentityType::Citizen].iter().cloned().collect()
        );
        VotingPallet::on_finalize(50 + 20);

        // Add prime minister candidates
        AssemblyPallet::add_prime_min_condidate(account1.clone()).unwrap();
        AssemblyPallet::add_prime_min_condidate(account2.clone()).unwrap();

        AssemblyPallet::on_finalize(50 + 20 + 10);

        let v = vec![[1_u8; 32].to_vec(), [2_u8; 32].to_vec()];
        let voutes = VecDeque::from(v);
        let ballot_1 = pallet_voting::AltVote::new(voutes);

        AssemblyPallet::vote_to_prime_min(account1.clone(), ballot_1.clone()).unwrap();

        AssemblyPallet::vote_to_prime_min(account2.clone(), ballot_1.clone()).unwrap();

        VotingPallet::on_finalize(20);

        //dbg!(AssemblyPallet::current_prime_min());
        assert_eq!(AssemblyPallet::current_prime_min(), None);
        AssemblyPallet::vote_to_prime_min(account3.clone(), ballot_1.clone()).unwrap();
        VotingPallet::on_finalize(20);
        assert_eq!(
            AssemblyPallet::current_prime_min().unwrap(),
            [1_u8; 32].to_vec()
        );
    });
}
