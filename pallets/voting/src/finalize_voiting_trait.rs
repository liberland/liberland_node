use crate::*;

// a dispatchable trait for the other pallets who wants to do some actions after voting will finish
pub trait FinalizeVotingDispatchTrait<T: Config> {
    fn finalize_voting(subject: T::Hash, voting_setting: VotingSettings<T::BlockNumber>);
}

pub trait FinilizeAltVotingDispatchTrait<T: Config> {
    fn finalize_voting(
        subject: T::Hash,
        voting_setting: AltVoutingSettings<T::BlockNumber>,
        winner: Candidate,
    );
}
//pub type AccountIdAndBallot <T: Config> = BTreeMap<T::AccountId, (AltVote, u64)>;
#[allow(clippy::type_complexity)]
pub trait FinalizeAltVotingListDispatchTrait<T: Config> {
    fn finalize_voting(
        subject: T::Hash,
        voting_settings: AltVotingListSettings<T::BlockNumber>,
        winners: BTreeMap<Candidate, u64>,
        ballots_storage: BTreeMap<T::Hash, BTreeMap<T::AccountId, (AltVote, u64)>>,
    );
}

// basic implementations

macro_rules! finalize_voting_dispatch_trait_impls {
    ($($name:ident)*) => {
        impl<T: Config, $($name: FinalizeVotingDispatchTrait<T>,)*> FinalizeVotingDispatchTrait<T> for ($($name,)*) {
            fn finalize_voting(_subject: T::Hash, _voting_setting: VotingSettings<T::BlockNumber>) {
                $($name::finalize_voting(_subject.clone(), _voting_setting.clone());)*
            }
        }
    };
}

finalize_voting_dispatch_trait_impls! {}
finalize_voting_dispatch_trait_impls! {_1}
finalize_voting_dispatch_trait_impls! {_1 _2}
finalize_voting_dispatch_trait_impls! {_1 _2 _3}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6 _7}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6 _7 _8}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6 _7 _8 _9}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6 _7 _8 _9 _10}

macro_rules! finalize_voting_dispatch_trait_impls {
    ($($name:ident)*) => {
        impl<T: Config, $($name: FinilizeAltVotingDispatchTrait<T>,)*> FinilizeAltVotingDispatchTrait<T> for ($($name,)*) {
            fn finalize_voting(_subject: T::Hash, _voting_setting: AltVoutingSettings<T::BlockNumber>,_winner: Candidate) {
                $($name::finalize_voting(_subject.clone(), _voting_setting.clone(),_winner.clone());)*
            }
        }
    };
}

finalize_voting_dispatch_trait_impls! {}
finalize_voting_dispatch_trait_impls! {_1 }
finalize_voting_dispatch_trait_impls! {_1 _2}
finalize_voting_dispatch_trait_impls! {_1 _2 _3}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6 _7}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6 _7 _8}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6 _7 _8 _9}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6 _7 _8 _9 _10}

macro_rules! finalize_voting_dispatch_trait_impls {
    ($($name:ident)*) => {
        impl<T: Config, $($name: FinalizeAltVotingListDispatchTrait<T>,)*> FinalizeAltVotingListDispatchTrait<T> for ($($name,)*) {
            fn finalize_voting(_subject: T::Hash, _voting_setting: AltVotingListSettings<T::BlockNumber>,_winners: BTreeMap<Candidate, u64>,_ballots_storage: BTreeMap<T::Hash, BTreeMap<T::AccountId, (AltVote, u64)>>) {
                $($name::finalize_voting(_subject.clone(), _voting_setting.clone(),_winners.clone(),_ballots_storage.clone());)*
            }
        }
    };
}

finalize_voting_dispatch_trait_impls! {}
finalize_voting_dispatch_trait_impls! {_1 }
finalize_voting_dispatch_trait_impls! {_1 _2}
finalize_voting_dispatch_trait_impls! {_1 _2 _3}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6 _7}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6 _7 _8}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6 _7 _8 _9}
finalize_voting_dispatch_trait_impls! {_1 _2 _3 _4 _5 _6 _7 _8 _9 _10}
