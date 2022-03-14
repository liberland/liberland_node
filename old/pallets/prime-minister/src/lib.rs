#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
use pallet_voting::{AltVoutingSettings, Candidate};
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
    pub trait Config:
        frame_system::Config + pallet_identity::Config + pallet_voting::Config
    {
        type IdentityTrait: pallet_identity::IdentityTrait<Self>;
        type VotingTrait: pallet_voting::VotingTrait<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> pallet_voting::finalize_voiting_trait::FinilizeAltVotingDispatchTrait<T>
        for Pallet<T>
    {
        fn finalize_voting(
            _subject: T::Hash,
            _voting_setting: AltVoutingSettings<T::BlockNumber>,
            _winner: Candidate,
        ) {
        }
    }
}
