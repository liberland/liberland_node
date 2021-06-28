#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
pub use pallet::*;
use pallet_identity::{IdentityTrait, IdentityType, PassportId};
use pallet_voting::{VotingSettings, VotingTrait};
use sp_runtime::traits::Hash;
use sp_std::{
    cmp::{Ord, PartialOrd},
    collections::btree_map::BTreeMap,
    collections::btree_set::BTreeSet,
    vec::Vec,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_voting::Config + pallet_identity::Config
    {
        const PETITION_DURATION: Self::BlockNumber;

        const REFERENDUM_DURATION: Self::BlockNumber;

        // 50%
        const REFERENDUM_ACCEPTANCE_PERCENTAGE: f64 = 0.5;

        // 10%
        const PETITION_ACCEPTANCE_PERCENTAGE: f64 = 0.1;

        type VotingTrait: pallet_voting::VotingTrait<Self>;

        type IdentityTrait: pallet_identity::IdentityTrait<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        AccountCannotSuggestPetition,
        AccountCannotVote,
        SubjectDoesNotExist,
        AlreadyVoted,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::storage]
    type SomeActivePetitions<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Suggestion, OptionQuery>;

    #[pallet::storage]
    type SomeActiveReferendums<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Suggestion, OptionQuery>;

    #[pallet::storage]
    type SomeSuccessfulReferendums<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Suggestion, OptionQuery>;

    #[pallet::type_value]
    pub fn VotedCitizensDefault() -> BTreeSet<PassportId> {
        Default::default()
    }

    #[pallet::storage]
    type SomeVotedCitizens<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        BTreeSet<PassportId>,
        ValueQuery,
        VotedCitizensDefault,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // propose a petition
        #[pallet::weight(1)]
        pub(super) fn suggest_petition(
            origin: OriginFor<T>,
            petition: Suggestion,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(
                T::IdentityTrait::check_account_indetity(sender, IdentityType::Citizen),
                <Error<T>>::AccountCannotSuggestPetition,
            );

            let petition_hash = T::Hashing::hash(&petition.data[..]);

            <SomeActivePetitions<T>>::insert(petition_hash.clone(), petition);

            T::VotingTrait::create_voting(petition_hash, T::PETITION_DURATION)?;

            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn vote(
            origin: OriginFor<T>,
            subject_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(
                T::IdentityTrait::check_account_indetity(sender.clone(), IdentityType::Citizen),
                <Error<T>>::AccountCannotVote,
            );

            ensure!(
                <SomeActivePetitions<T>>::get(subject_hash) != None
                    || <SomeActiveReferendums<T>>::get(subject_hash) != None,
                <Error<T>>::SubjectDoesNotExist,
            );

            let mut voted = <SomeVotedCitizens<T>>::get(subject_hash);
            let passport_id = T::IdentityTrait::get_passport_id(sender).unwrap();

            ensure!(!voted.contains(&passport_id), <Error<T>>::AlreadyVoted);

            T::VotingTrait::vote(subject_hash, 1)?;
            voted.insert(passport_id);
            <SomeVotedCitizens<T>>::insert(subject_hash, voted);

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn get_suggestion_hash(suggestion: &Suggestion) -> T::Hash {
            T::Hashing::hash(&suggestion.data[..])
        }

        pub fn get_active_petitions() -> BTreeMap<T::Hash, Suggestion> {
            <SomeActivePetitions<T>>::iter().collect()
        }

        pub fn get_active_referendums() -> BTreeMap<T::Hash, Suggestion> {
            <SomeActiveReferendums<T>>::iter().collect()
        }

        pub fn get_successfull_referendums() -> BTreeMap<T::Hash, Suggestion> {
            <SomeSuccessfulReferendums<T>>::iter().collect()
        }
    }

    impl<T: Config> pallet_voting::FinalizeVotingDispatchTrait<T> for Pallet<T> {
        fn finalize_voting(subject: T::Hash, voting_setting: VotingSettings<T::BlockNumber>) {
            match <SomeActivePetitions<T>>::get(subject) {
                Some(petition) => {
                    // more than 10%
                    if voting_setting.result
                        >= ((T::IdentityTrait::get_citizens_amount() as f64)
                            * T::PETITION_ACCEPTANCE_PERCENTAGE) as u64
                    {
                        <SomeActiveReferendums<T>>::insert(subject, petition);
                        T::VotingTrait::create_voting(subject, T::REFERENDUM_DURATION).unwrap();
                    }
                    <SomeVotedCitizens<T>>::remove(subject);
                    <SomeActivePetitions<T>>::remove(subject);
                    return;
                }
                None => {}
            }
            match <SomeActiveReferendums<T>>::get(subject) {
                Some(referendum) => {
                    // more than 50%
                    if voting_setting.result
                        >= ((T::IdentityTrait::get_citizens_amount() as f64)
                            * T::REFERENDUM_ACCEPTANCE_PERCENTAGE) as u64
                    {
                        <SomeSuccessfulReferendums<T>>::insert(subject, referendum);
                    }
                    <SomeVotedCitizens<T>>::remove(subject);
                    <SomeActiveReferendums<T>>::remove(subject);
                }
                None => {}
            }
        }
    }
}

sp_api::decl_runtime_apis! {
    pub trait ReferendumPalletApi<T: Config> {
        fn get_suggestion_hash(suggestion: &Suggestion) -> T::Hash;

        fn get_active_petitions() -> BTreeMap<T::Hash, Suggestion>;

        fn get_active_referendums() -> BTreeMap<T::Hash, Suggestion>;

        fn get_successfull_referendums() -> BTreeMap<T::Hash, Suggestion>;
    }
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Suggestion {
    pub data: Vec<u8>,
}
