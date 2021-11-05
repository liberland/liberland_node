#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
use pallet_identity::{IdentityTrait, IdentityType, PassportId};
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

    #[pallet::storage]
    #[pallet::getter(fn current_prime_min)]
    type CurrentPrimeMinister<T: Config> = StorageValue<_, PassportId, OptionQuery>;

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        /// Yore not prime minister
        OnlyPrimeMinCall,
        /// Assembly not fond
        AssemblyNotFound,
        /// This account is not ministr of interior
        NotMinOfInterior,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1)]
        pub(super) fn update_assembly_to_minister(
            origin: OriginFor<T>,
            account: PassportId,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                T::IdentityTrait::check_account_indetity(sender, IdentityType::PrimeMinister),
                <Error<T>>::OnlyPrimeMinCall
            );

            ensure!(
                T::IdentityTrait::check_id_identity(account, IdentityType::Assembly),
                <Error<T>>::AssemblyNotFound
            );
            T::IdentityTrait::push_identity(account, IdentityType::MinisterOfInterior).unwrap(); // This unwrap is correct
            T::IdentityTrait::remove_identity(account, IdentityType::Assembly);
            <CurrentPrimeMinister<T>>::put(account);
            Ok(().into())
        }
        #[pallet::weight(1)]
        pub(super) fn remove_min_of_interior(
            origin: OriginFor<T>,
            account: PassportId,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                T::IdentityTrait::check_account_indetity(sender, IdentityType::MinisterOfInterior),
                <Error<T>>::OnlyPrimeMinCall
            );
            ensure!(
                T::IdentityTrait::check_id_identity(account, IdentityType::MinisterOfInterior),
                <Error<T>>::NotMinOfInterior
            );
            T::IdentityTrait::remove_identity(account, IdentityType::MinisterOfInterior);
            <CurrentPrimeMinister<T>>::kill();
            Ok(().into())
        }
    }

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
