#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
pub use pallet::*;
use sp_std::{
    cmp::{Ord, PartialOrd},
    collections::btree_map::BTreeMap,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod finalize_voiting_trait;
pub use finalize_voiting_trait::FinalizeVotingDispatchTrait;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type FinalizeVotingDispatch: FinalizeVotingDispatchTrait<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        // emits when from provided VotingSubject has been applied
        VotingSubjectHasBeenApplied,
        // emits when provided Voting subject does not exist
        VotingSubjectDoesNotExist,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // Block finalization
        fn on_finalize(block_number: BlockNumberFor<T>) {
            Self::finalize_votings(block_number);
        }
    }

    #[pallet::storage]
    type SomeActiveVotings<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, VotingSettings<T::BlockNumber>, OptionQuery>;

    #[pallet::storage]
    type SomeVotingResults<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, VotingSettings<T::BlockNumber>, OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> Pallet<T> {
        fn finalize_votings(block_number: BlockNumberFor<T>) {
            for (subject, voting_settings) in <SomeActiveVotings<T>>::iter() {
                // voting has been passed, so we will store the result and remove from the active votings list
                if (voting_settings.voting_duration + voting_settings.submitted_height)
                    <= block_number
                {
                    <SomeActiveVotings<T>>::remove(subject.clone());
                    <SomeVotingResults<T>>::insert(subject.clone(), voting_settings.clone());
                    <T::FinalizeVotingDispatch>::finalize_voting(subject, voting_settings);
                }
            }
        }
    }

    impl<T: Config> VotingTrait<T> for Pallet<T> {
        fn get_active_votings() -> BTreeMap<T::Hash, VotingSettings<T::BlockNumber>> {
            <SomeActiveVotings<T>>::iter().collect()
        }

        fn get_voting_results() -> BTreeMap<T::Hash, VotingSettings<T::BlockNumber>> {
            <SomeVotingResults<T>>::iter().collect()
        }

        fn create_voting(subject: T::Hash, duration: T::BlockNumber) -> Result<(), Error<T>> {
            ensure!(
                <SomeActiveVotings<T>>::get(subject.clone()) == None
                    && <SomeVotingResults<T>>::get(subject.clone()) == None,
                <Error<T>>::VotingSubjectHasBeenApplied
            );

            let block_number = <frame_system::Pallet<T>>::block_number();
            <SomeActiveVotings<T>>::insert(
                subject,
                VotingSettings {
                    result: 0,
                    voting_duration: duration,
                    submitted_height: block_number,
                },
            );

            Ok(())
        }

        fn vote(subject: T::Hash, power: u64) -> Result<(), Error<T>> {
            match <SomeActiveVotings<T>>::get(subject.clone()) {
                Some(mut settings) => {
                    settings.result += power;
                    <SomeActiveVotings<T>>::insert(subject, settings);
                    Ok(())
                }
                None => Err(<Error<T>>::VotingSubjectDoesNotExist),
            }
        }
    }
}

pub trait VotingTrait<T: Config> {
    fn get_active_votings() -> BTreeMap<T::Hash, VotingSettings<T::BlockNumber>>;

    fn get_voting_results() -> BTreeMap<T::Hash, VotingSettings<T::BlockNumber>>;

    fn create_voting(subject: T::Hash, duration: T::BlockNumber) -> Result<(), Error<T>>;

    fn vote(subject: T::Hash, power: u64) -> Result<(), Error<T>>;
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct VotingSettings<BlockNumber> {
    pub result: u64,
    pub voting_duration: BlockNumber,
    pub submitted_height: BlockNumber,
}
