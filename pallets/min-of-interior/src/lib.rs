#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use frame_support::codec::{Decode, Encode};
pub use pallet::*;
use pallet_identity::{IdentityTrait, IdentityType, PassportId};
use sp_std::{
    cmp::{Ord, PartialOrd},
    collections::btree_set::BTreeSet,
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

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_identity::Config {
        #[pallet::constant]
        type RequestBlockNummber: Get<Self::BlockNumber>;
        type IdentityTrait: pallet_identity::IdentityTrait<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        // emits when from provided AccountId already has been used
        AccoundIdAlreadyUsed,
        // emits when from the same AccountId Kyc was already requested
        AlreadyAplliedKycRequest,
        // emits when provided account can not process Kyc
        AccountCannotProcessKyc,
        // emits when reviewer trying to response on the unexisting request
        RequestDoesNotExist,
        // emit when not found EResidence
        EresidenceNotFound,
        // emit when try to call a function that can only be called ministry of interior
        OnlyMinistryOfInteriorCall,
        // emit when not found Assembly
        AssemblyNotFound,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // Block finalization
        fn on_finalize(block_number: BlockNumberFor<T>) {
            Self::check_request_time(block_number);
        }
    }

    #[pallet::storage]
    type SomeKycRequests<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, KycData, OptionQuery>;

    #[pallet::storage]
    type EresidentRequests<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PassportId,
        CitizenRequest<T::BlockNumber, T::AccountId>,
        OptionQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // put the provided origin with provided Kyc data into the queue
        #[pallet::weight(1)]
        pub(super) fn request_kyc(
            origin: OriginFor<T>,
            kyc_data: KycData,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(
                pallet_identity::Pallet::<T>::passport_id(sender.clone()) == None,
                <Error<T>>::AccoundIdAlreadyUsed
            );

            ensure!(
                <SomeKycRequests<T>>::get(sender.clone()) == None,
                <Error<T>>::AlreadyAplliedKycRequest
            );

            <SomeKycRequests<T>>::insert(sender, kyc_data);

            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn kyc_response(
            origin: OriginFor<T>,
            info: KycRequest<T::AccountId>,
            approved: bool,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(
                T::IdentityTrait::check_account_indetity(sender, IdentityType::MinisterOfInterior),
                <Error<T>>::AccountCannotProcessKyc
            );

            ensure!(
                <SomeKycRequests<T>>::get(info.account.clone()) == Some(info.data.clone()),
                <Error<T>>::RequestDoesNotExist
            );

            // update Identity info
            if approved {
                T::IdentityTrait::match_account_to_id(info.account.clone(), info.data.id);
                T::IdentityTrait::push_identity(info.data.id, IdentityType::EResident).unwrap();
            }

            // remove request from the storage
            <SomeKycRequests<T>>::remove(info.account);

            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn update_e_resident_to_citizen_reqest(
            origin: OriginFor<T>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                T::IdentityTrait::check_account_indetity(sender.clone(), IdentityType::EResident),
                <Error<T>>::EresidenceNotFound
            );
            let pasport_id = pallet_identity::Pallet::<T>::passport_id(sender.clone()).unwrap();
            Self::create_citizen_request(pasport_id, sender);
            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn update_assembly_to_minister(
            origin: OriginFor<T>,
            account: PassportId,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                T::IdentityTrait::check_account_indetity(sender, IdentityType::MinisterOfInterior),
                <Error<T>>::OnlyMinistryOfInteriorCall
            );

            ensure!(
                T::IdentityTrait::check_id_identity(account, IdentityType::Assembly),
                <Error<T>>::AssemblyNotFound
            );
            T::IdentityTrait::push_identity(account, IdentityType::MinisterOfInterior).unwrap(); // This unwrap is correct
            T::IdentityTrait::remove_identity(account, IdentityType::Assembly);
            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn aprove_to_citizen_or_not(
            origin: OriginFor<T>,
            info: KycRequest<T::AccountId>,
            approved: bool,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(
                T::IdentityTrait::check_account_indetity(sender, IdentityType::MinisterOfInterior),
                <Error<T>>::OnlyMinistryOfInteriorCall
            );
            if approved {
                T::IdentityTrait::remove_identity(info.data.id, IdentityType::EResident);
                T::IdentityTrait::push_identity(info.data.id, IdentityType::Citizen).unwrap();
            }
            // remove request from the storage
            <SomeKycRequests<T>>::remove(info.account);
            <EresidentRequests<T>>::remove(info.data.id);
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn get_all_requests() -> BTreeSet<KycRequest<T::AccountId>> {
            <SomeKycRequests<T>>::iter()
                .map(|(account, data)| KycRequest { account, data })
                .collect()
        }
        pub fn check_request_time(block_nummber: T::BlockNumber) {
            <EresidentRequests<T>>::iter().for_each(|value| {
                if value.1.submitted_height + T::RequestBlockNummber::get() <= block_nummber {
                    T::IdentityTrait::remove_identity(value.0, IdentityType::EResident);
                    <EresidentRequests<T>>::remove(value.0);
                    <SomeKycRequests<T>>::remove(value.1.account);
                    T::IdentityTrait::push_identity(value.0, IdentityType::Citizen).unwrap();
                }
            });
        }
        pub fn create_citizen_request(id: PassportId, account_id: T::AccountId) {
            let block_nummber = <frame_system::Pallet<T>>::block_number();
            let request = CitizenRequest {
                submitted_height: block_nummber,
                data: KycData { id },
                account: account_id,
            };
            <EresidentRequests<T>>::insert(id, request);
        }
    }
}

sp_api::decl_runtime_apis! {
    pub trait MinInteriorPalletApi<T: Config> {
        fn get_all_requests() -> BTreeSet<KycRequest<T::AccountId>>;
    }
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct KycRequest<AccountId> {
    pub account: AccountId,
    pub data: KycData,
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct KycData {
    pub id: PassportId,
}
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct CitizenRequest<BlockNumber, AccountId> {
    pub submitted_height: BlockNumber,
    pub data: KycData,
    pub account: AccountId,
}
