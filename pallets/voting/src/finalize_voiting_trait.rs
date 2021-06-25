use crate::*;

// a dispatchable trait for the other pallets who wants to do some actions after voting will finish
pub trait FinalizeVotingDispatchTrait<T: Config> {
    fn finalize_voting(subject: T::Hash, voting_setting: VotingSettings<T::BlockNumber>);
}

// basic implementations

impl<T: Config> FinalizeVotingDispatchTrait<T> for () {
    fn finalize_voting(_subject: T::Hash, _voting_setting: VotingSettings<T::BlockNumber>) {}
}

macro_rules! tuple_impls {
    ($($name:ident)+) => {
        impl<T: Config, $($name: FinalizeVotingDispatchTrait<T>),+> FinalizeVotingDispatchTrait<T> for ($($name,)+) {
            fn finalize_voting(subject: T::Hash, voting_setting: VotingSettings<T::BlockNumber>) {
                $($name::finalize_voting(subject.clone(), voting_setting.clone());)+
            }
        }
    };
}

tuple_impls! { A }
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }
tuple_impls! { A B C D E F G H I J K L M }
tuple_impls! { A B C D E F G H I J K L M N }
tuple_impls! { A B C D E F G H I J K L M N O }
tuple_impls! { A B C D E F G H I J K L M N O P }
