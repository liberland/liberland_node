#![cfg_attr(not(feature = "std"), no_std)]

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
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::storage]
    type SomeKycRequests<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, KycData, OptionQuery>;

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
                T::IdentityTrait::get_passport_id(sender.clone()) == None,
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
                T::IdentityTrait::push_identity(info.data.id, IdentityType::Citizen);
            }

            // remove request from the storage
            <SomeKycRequests<T>>::remove(info.account);

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn get_all_requests() -> BTreeSet<KycRequest<T::AccountId>> {
            <SomeKycRequests<T>>::iter()
                .map(|(account, data)| KycRequest { account, data })
                .collect()
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
