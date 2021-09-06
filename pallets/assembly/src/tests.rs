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

        AssemblyPallet::on_initialize(10 + 100);
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

        AssemblyPallet::on_initialize(10 + 100);

        assert_ok!(AssemblyPallet::vote(account2.clone(), ballot_1.clone()));

        assert_err!(
            AssemblyPallet::vote(account2.clone(), ballot_1.clone()),
            <Error<Test>>::AlreadyVoted
        );
    });
}

#[test]
fn basic_low_voting_test() {
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

        AssemblyPallet::on_initialize(10 + 100);
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
        AssemblyPallet::propose_law(account1.clone(), law_hash_1).unwrap();
        AssemblyPallet::propose_law(account2.clone(), law_hash_2).unwrap();

        AssemblyPallet::vote_to_law(account1.clone(), law_hash_1).unwrap();
        AssemblyPallet::vote_to_law(account2.clone(), law_hash_1).unwrap();
        AssemblyPallet::vote_to_law(account3.clone(), law_hash_1).unwrap();
        assert_eq!(
            AssemblyPallet::laws(law_hash_1).unwrap(),
            LawState::InProgress,
        );
        assert_eq!(
            AssemblyPallet::laws(law_hash_2).unwrap(),
            LawState::InProgress,
        );
        VotingPallet::on_finalize(10 + 100 + 1);

        assert_eq!(
            AssemblyPallet::laws(law_hash_1).unwrap(),
            LawState::Approved,
        );
        assert_eq!(
            AssemblyPallet::laws(law_hash_2).unwrap(),
            LawState::Declined,
        );
    });
}
