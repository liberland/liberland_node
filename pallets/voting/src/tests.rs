use crate::mock::*;
use crate::*;
use assert::assert_err;
use frame_support::{assert_ok, traits::OnFinalize};
use sp_runtime::traits::Hash;

#[test]
fn basic_voting_test() {
    new_test_ext().execute_with(|| {
        type Hashing = <Test as frame_system::Config>::Hashing;

        assert_eq!(VotingPallet::get_active_votings(), Default::default());

        let subject = Hashing::hash(&[1; 32]);
        let duration = 100;

        assert_ok!(VotingPallet::create_voting(subject.clone(), duration));
        assert_eq!(VotingPallet::get_active_votings().len(), 1);

        assert_err!(VotingPallet::create_voting(subject.clone(), duration));

        assert_err!(VotingPallet::vote(Hashing::hash(&[2; 32]), 1));

        assert_ok!(VotingPallet::vote(subject, 2));

        VotingPallet::on_finalize(duration);

        assert_eq!(VotingPallet::get_active_votings(), Default::default());

        assert_ok!(VotingPallet::create_voting(subject.clone(), duration));
    });
}
