use crate::*;

// a dispatchable trait for the other pallets who wants to do some actions after voting will finish
pub trait FinalizeVotingDispatchTrait<T: Config> {
    fn finalize_voting(subject: T::Hash, voting_setting: VotingSettings<T::BlockNumber>);
}

// basic implementations

macro_rules! tuple_impls {
    ($trait_name:ident $($name:ident)*) => {
        impl<T: Config, $($name: $trait_name<T>,)*> $trait_name<T> for ($($name,)*) {
            fn finalize_voting(_subject: T::Hash, _voting_setting: VotingSettings<T::BlockNumber>) {
                $($name::finalize_voting(_subject.clone(), _voting_setting.clone());)*
            }
        }
    };
}

tuple_impls! { FinalizeVotingDispatchTrait }
tuple_impls! { FinalizeVotingDispatchTrait _1 }
tuple_impls! { FinalizeVotingDispatchTrait _1 _2 }
tuple_impls! { FinalizeVotingDispatchTrait _1 _2 _3 }
tuple_impls! { FinalizeVotingDispatchTrait _1 _2 _3 _4 }
tuple_impls! { FinalizeVotingDispatchTrait _1 _2 _3 _4 _5 }
tuple_impls! { FinalizeVotingDispatchTrait _1 _2 _3 _4 _5 _6 }
tuple_impls! { FinalizeVotingDispatchTrait _1 _2 _3 _4 _5 _6 _7 }
tuple_impls! { FinalizeVotingDispatchTrait _1 _2 _3 _4 _5 _6 _7 _8 }
tuple_impls! { FinalizeVotingDispatchTrait _1 _2 _3 _4 _5 _6 _7 _8 _9 }
tuple_impls! { FinalizeVotingDispatchTrait _1 _2 _3 _4 _5 _6 _7 _8 _9 _10}
