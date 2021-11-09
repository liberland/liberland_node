#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::codec::{Decode, Encode};
pub use pallet::*;
use pallet_identity::{IdentityTrait, IdentityType, PassportId};
use pallet_voting::{AltVoutingSettings, Candidate};
use sp_std::vec::Vec;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::{ensure_signed, pallet_prelude::*};
    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_identity::Config + pallet_voting::Config
    {
        type IdentityTrait: pallet_identity::IdentityTrait<Self>;
        type VotingTrait: pallet_voting::VotingTrait<Self>;

        #[pallet::constant]
        type InvitationsDuration: Get<Self::BlockNumber>;
    }

    #[pallet::storage]
    #[pallet::getter(fn current_prime_min)]
    type CurrentPrimeMinister<T: Config> = StorageValue<_, PassportId, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn invitations)]
    type Invitations<T: Config> =
        StorageMap<_, Blake2_128Concat, PassportId, MinistersSettings<T::BlockNumber>, OptionQuery>;

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
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(block_number: BlockNumberFor<T>) {
            Self::check_invitation(block_number);
        }
    }

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

        #[pallet::weight(1)]
        pub(super) fn send_invitation(
            origin: OriginFor<T>,
            id_s: Vec<PassportId>,
            min_type: MinistersType,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                T::IdentityTrait::check_account_indetity(sender, IdentityType::PrimeMinister),
                <Error<T>>::OnlyPrimeMinCall
            );
            let submited_block = <frame_system::Pallet<T>>::block_number();
            id_s.iter().for_each(|v| {
                if T::IdentityTrait::check_id_identity(*v, IdentityType::Assembly) {
                    <Invitations<T>>::insert(
                        v,
                        MinistersSettings {
                            min_type,
                            submited_block,
                            status: Status::Pending,
                        },
                    );
                }
            });
            Ok(().into())
        }
    }
    impl<T: Config> Pallet<T> {
        fn check_invitation(block_nummber: T::BlockNumber) {
            <Invitations<T>>::iter().for_each(|v| {
                //v.0.submited_block;
                if v.1.status == Status::Pending
                    && block_nummber >= v.1.submited_block + T::InvitationsDuration::get()
                {
                    <Invitations<T>>::remove(v.0);
                } else if v.1.status == Status::Declined {
                    <Invitations<T>>::remove(v.0);
                }
            });
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

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Encode, Decode, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MinistersType {
    MinOfInterior,
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Encode, Decode, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Status {
    Pending,
    Accepted,
    Declined,
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Encode, Decode, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MinistersSettings<BlockNumber> {
    pub min_type: MinistersType,
    pub submited_block: BlockNumber,
    pub status: Status,
}
