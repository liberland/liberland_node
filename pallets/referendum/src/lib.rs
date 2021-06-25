#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
pub use pallet::*;
use pallet_identity::{IdentityTrait, IdentityType};
use pallet_voting::{VotingSettings, VotingTrait};
use sp_runtime::traits::Hash;
use sp_std::{
    cmp::{Ord, PartialOrd},
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
        type VotingTrait: pallet_voting::VotingTrait<Self>;
        type IdentityTrait: pallet_identity::IdentityTrait<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        AccountCannotSuggestPetition,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::storage]
    type SomeActivePetitions<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Suggestion, OptionQuery>;

    #[pallet::storage]
    type SomeActiveReferendums<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Suggestion, OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // propose a
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
    }

    impl<T: Config> Pallet<T> {}

    impl<T: Config> pallet_voting::FinalizeVotingDispatchTrait<T> for Pallet<T> {
        fn finalize_voting(subject: T::Hash, voting_setting: VotingSettings<T::BlockNumber>) {
            match <SomeActivePetitions<T>>::get(subject) {
                Some(petition) => {
                    return;
                }
                None => {}
            }
            match <SomeActiveReferendums<T>>::get(subject) {
                Some(referendum) => {}
                None => {}
            }
        }
    }
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Suggestion {
    pub data: Vec<u8>,
}
