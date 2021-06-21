#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
pub use pallet::*;
use sp_std::cmp::{Ord, PartialOrd};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::collections::btree_set::BTreeSet;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        // emits when from provided VotingSubject has been applied
        VotingSubjectHasBeenApplied,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::storage]
    type SomePendingVotings<T: Config> =
        StorageMap<_, Blake2_128Concat, VotingSubject<T::Hash>, T::BlockNumber, OptionQuery>;

    #[pallet::storage]
    type SomeVotingResults<T: Config> =
        StorageMap<_, Blake2_128Concat, VotingSubject<T::Hash>, VotingResult, OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> Pallet<T> {
        pub fn get_current_votings() -> BTreeSet<VotingSubject<T::Hash>> {
            <SomeVotingResults<T>>::iter()
                .map(|(subject, _)| subject)
                .collect()
        }

        pub fn get_voting_results() -> BTreeMap<VotingSubject<T::Hash>, VotingResult> {
            <SomeVotingResults<T>>::iter().collect()
        }

        pub fn create_voting(subject: VotingSubject<T::Hash>) -> Result<(), Error<T>> {
            ensure!(
                <SomePendingVotings<T>>::get(subject.clone()) == None
                    && <SomeVotingResults<T>>::get(subject.clone()) == None,
                <Error<T>>::VotingSubjectHasBeenApplied
            );

            let block_number = <frame_system::Pallet<T>>::block_number();
            <SomePendingVotings<T>>::insert(subject, block_number);

            Ok(())
        }
    }
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct VotingResult {
    pub result: bool,
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct VotingSubject<Hash> {
    pub subject_hash: Hash,
}
