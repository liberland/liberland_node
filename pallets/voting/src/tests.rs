use crate::mock::*;
use crate::*;
use assert::assert_err;
use frame_support::assert_ok;
use sp_runtime::traits::Hash;

#[test]
fn basic_voting_test() {
    new_test_ext().execute_with(|| {
        type Hashing = <Test as frame_system::Config>::Hashing;

        assert_eq!(VotingPallet::get_current_votings(), Default::default());

        let subject = VotingSubject {
            subject_hash: Hashing::hash(&[1; 32]),
        };

        assert_ok!(VotingPallet::create_voting(subject.clone()));
        assert_eq!(
            VotingPallet::get_current_votings(),
            [subject.clone()].iter().cloned().collect()
        );

        assert_err!(VotingPallet::create_voting(subject.clone()));
    });
}
