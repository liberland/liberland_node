#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
pub use pallet::*;
use pallet_identity::IdentityTrait;
use pallet_identity::IdentityType;
use pallet_identity::PassportId;
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
    pub trait Config: frame_system::Config {
        type IdentityTrait: pallet_identity::IdentityTrait<Self::AccountId>;
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

    #[pallet::type_value]
    pub fn StoredKycRequestsDefault<T: Config>() -> BTreeSet<T::AccountId> {
        Default::default()
    }

    #[pallet::storage]
    type SomeKycRequests<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::BlockNumber,
        BTreeSet<T::AccountId>,
        ValueQuery,
        StoredKycRequestsDefault<T>,
    >;

    #[pallet::storage]
    type SomeKycData<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, KycData, OptionQuery>;

    // TODO: return back all these requests into the SomeKycRequests storage after they will wait long time
    #[pallet::storage]
    type SomePendingReviewKycRequests<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::BlockNumber,
        BTreeSet<T::AccountId>,
        ValueQuery,
        StoredKycRequestsDefault<T>,
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
                T::IdentityTrait::get_passport_id(sender.clone()) == None,
                <Error<T>>::AccoundIdAlreadyUsed
            );

            ensure!(
                <SomeKycData<T>>::get(sender.clone()) == None,
                <Error<T>>::AlreadyAplliedKycRequest
            );

            let block_number = <frame_system::Pallet<T>>::block_number();

            let mut requests = <SomeKycRequests<T>>::get(block_number);
            requests.insert(sender.clone());
            <SomeKycRequests<T>>::insert(block_number, requests);

            <SomeKycData<T>>::insert(sender, kyc_data);

            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn kyc_response(
            origin: OriginFor<T>,
            info: KycRequest<T::AccountId, T::BlockNumber>,
            approved: bool,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(
                T::IdentityTrait::check_account_indetity(sender, IdentityType::MinisterOfInterior),
                <Error<T>>::AccountCannotProcessKyc
            );

            let mut requests = <SomePendingReviewKycRequests<T>>::get(info.block_number);

            ensure!(
                <SomeKycData<T>>::get(info.account.clone()) == Some(info.data.clone())
                    && requests.contains(&info.account),
                <Error<T>>::RequestDoesNotExist
            );

            // update Identity info
            if approved {
                T::IdentityTrait::match_account_to_id(info.account.clone(), info.data.id);
                T::IdentityTrait::push_identity(info.data.id, IdentityType::Citizen);
            }

            // remove request from the storage
            requests.remove(&info.account);
            <SomeKycData<T>>::remove(info.account);

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn get_earliest_request() -> Option<KycRequest<T::AccountId, T::BlockNumber>> {
            if let Some((mut erlieast_height, mut erlieast_requests)) =
                <SomeKycRequests<T>>::iter().next()
            {
                // find the erliest request
                <SomeKycRequests<T>>::iter().for_each(|(height, requests)| {
                    if height < erlieast_height {
                        erlieast_height = height;
                        erlieast_requests = requests;
                    }
                });

                // get the first value, it should exists
                let request = erlieast_requests.iter().next().unwrap().clone();
                // remove this entry from the set
                erlieast_requests.remove(&request);

                // remove this entry is vec is empty
                if erlieast_requests.is_empty() {
                    <SomeKycRequests<T>>::remove(erlieast_height);
                } else {
                    <SomeKycRequests<T>>::insert(erlieast_height, erlieast_requests.clone());
                }

                // add this request as pending review
                let mut pending = <SomePendingReviewKycRequests<T>>::get(erlieast_height);
                pending.insert(request.clone());
                <SomePendingReviewKycRequests<T>>::insert(erlieast_height, pending);

                return Some(KycRequest {
                    account: request.clone(),
                    block_number: erlieast_height,
                    data: <SomeKycData<T>>::get(request).unwrap(),
                });
            }

            None
        }
    }
}

sp_api::decl_runtime_apis! {
    pub trait KycPalletApi<T: Config> {
        fn get_earliest_request() -> Option<KycRequest<T::AccountId, T::BlockNumber>>;
    }
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug)]
pub struct KycRequest<AccountId, BlockNumber> {
    pub account: AccountId,
    pub block_number: BlockNumber,
    pub data: KycData,
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug)]
pub struct KycData {
    pub id: PassportId,
}
