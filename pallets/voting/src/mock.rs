use crate::{self as pallet_voting, Candidate};
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use sp_std::collections::btree_set::BTreeSet;
use sp_std::collections::vec_deque::VecDeque;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        VotingPallet: pallet_voting::{Pallet, Call, Storage},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
}

impl pallet_voting::Config for Test {
    type FinalizeVotingDispatch = ();
    type FinalizeAltVotingDispatch = ();
    type FinalizeAltVotingListDispatch = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

pub fn get_ballots_mock() -> Vec<pallet_voting::AltVote> {
    let v = vec![
        [1_u8; 32].to_vec(),
        [2_u8; 32].to_vec(),
        [3_u8; 32].to_vec(),
    ];
    let voutes = VecDeque::from(v);
    let ballot = pallet_voting::AltVote::new(voutes);
    let mut resoult_ballot_list = vec![ballot];

    let v = vec![
        [1_u8; 32].to_vec(),
        [2_u8; 32].to_vec(),
        [3_u8; 32].to_vec(),
    ];

    let voutes = VecDeque::from(v);
    let ballot = pallet_voting::AltVote::new(voutes);
    resoult_ballot_list.push(ballot);

    let v = vec![
        [1_u8; 32].to_vec(),
        [2_u8; 32].to_vec(),
        [3_u8; 32].to_vec(),
    ];

    let voutes = VecDeque::from(v);
    let ballot = pallet_voting::AltVote::new(voutes);
    resoult_ballot_list.push(ballot);

    let v = vec![
        [1_u8; 32].to_vec(),
        [2_u8; 32].to_vec(),
        [3_u8; 32].to_vec(),
    ];

    let voutes = VecDeque::from(v);
    let ballot = pallet_voting::AltVote::new(voutes);
    resoult_ballot_list.push(ballot);

    let v = vec![
        [2_u8; 32].to_vec(),
        [3_u8; 32].to_vec(),
        [1_u8; 32].to_vec(),
    ];

    let voutes = VecDeque::from(v);
    let ballot = pallet_voting::AltVote::new(voutes);
    resoult_ballot_list.push(ballot);

    let v = vec![
        [2_u8; 32].to_vec(),
        [3_u8; 32].to_vec(),
        [1_u8; 32].to_vec(),
    ];

    let voutes = VecDeque::from(v);
    let ballot = pallet_voting::AltVote::new(voutes);
    resoult_ballot_list.push(ballot);

    let v = vec![
        [2_u8; 32].to_vec(),
        [3_u8; 32].to_vec(),
        [1_u8; 32].to_vec(),
    ];

    let voutes = VecDeque::from(v);
    let ballot = pallet_voting::AltVote::new(voutes);
    resoult_ballot_list.push(ballot);

    let v = vec![
        [3_u8; 32].to_vec(),
        [2_u8; 32].to_vec(),
        [1_u8; 32].to_vec(),
    ];

    let voutes = VecDeque::from(v);
    let ballot = pallet_voting::AltVote::new(voutes);
    resoult_ballot_list.push(ballot);

    let v = vec![
        [3_u8; 32].to_vec(),
        [2_u8; 32].to_vec(),
        [1_u8; 32].to_vec(),
    ];

    let voutes = VecDeque::from(v);
    let ballot = pallet_voting::AltVote::new(voutes);
    resoult_ballot_list.push(ballot);

    let v = vec![
        [3_u8; 32].to_vec(),
        [2_u8; 32].to_vec(),
        [1_u8; 32].to_vec(),
    ];

    let voutes = VecDeque::from(v);
    let ballot = pallet_voting::AltVote::new(voutes);
    resoult_ballot_list.push(ballot);

    let v = vec![[4_u8; 32].to_vec(), [3_u8; 32].to_vec()];

    let voutes = VecDeque::from(v);
    let ballot = pallet_voting::AltVote::new(voutes);
    resoult_ballot_list.push(ballot);

    resoult_ballot_list
}

pub fn get_mock_subjects() -> BTreeSet<Candidate> {
    let id = vec![[1_u8; 32], [2_u8; 32], [3_u8; 32], [4_u8; 32]];
    let mut res = BTreeSet::new();
    id.iter().for_each(|e| {
        res.insert(e.to_vec());
    });
    res
}
