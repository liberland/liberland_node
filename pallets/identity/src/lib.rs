#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use frame_support::codec::{Decode, Encode};
pub use pallet::*;
use sp_std::cmp::{Ord, PartialOrd};
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

    #[pallet::type_value]
    pub fn IdentityTypesDefault() -> BTreeSet<IdentityType> {
        Default::default()
    }

    #[pallet::storage]
    type SomeAccountIdentities<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PassportId,
        BTreeSet<IdentityType>,
        ValueQuery,
        IdentityTypesDefault,
    >;

    #[pallet::storage]
    type SomeAccountToId<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, PassportId, OptionQuery>;

    #[pallet::storage]
    type CitizensAmount<T: Config> = StorageValue<_, u64, ValueQuery, DefaultCitizensAmountStorage>;

    #[pallet::type_value]
    pub fn DefaultCitizensAmountStorage() -> u64 {
        0_u64
    }

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub citizens: Vec<(T::AccountId, PassportId)>,
        pub reviewers: Vec<PassportId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                reviewers: Default::default(),
                citizens: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            <CitizensAmount<T>>::put(self.citizens.len() as u64);
            for (account, id) in self.citizens.iter() {
                <Pallet<T>>::match_account_to_id(account.clone(), *id);
                <Pallet<T>>::push_identity(*id, IdentityType::Citizen);
            }
            for id in self.reviewers.iter() {
                assert!(<Pallet<T>>::check_id_identity(*id, IdentityType::Citizen));
                <Pallet<T>>::push_identity(*id, IdentityType::MinisterOfInterior);
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> IdentityTrait<T> for Pallet<T> {
        fn match_account_to_id(account: T::AccountId, id: PassportId) {
            assert!(
                <SomeAccountToId<T>>::get(account.clone()) == None,
                "for this AccountId has been already matched PassportId"
            );
            <SomeAccountToId<T>>::insert(account, id);
        }

        fn push_identity(id: PassportId, id_type: IdentityType) {
            let mut types = <SomeAccountIdentities<T>>::get(id);
            types.insert(id_type);
            let id_2 = <SomeAccountIdentities<T>>::iter().find(|item| item.0 == id);

            <SomeAccountIdentities<T>>::insert(id, types);
            if id_2.is_none() {
                <CitizensAmount<T>>::mutate(|res| *res += 1);
            }
        }

        fn remove_identity(id: PassportId, id_type: IdentityType) {
            let mut types = <SomeAccountIdentities<T>>::get(id);
            // remove identity type
            types.remove(&id_type);
            if types.is_empty() {
                <SomeAccountIdentities<T>>::remove(id);
                <CitizensAmount<T>>::mutate(|res| {
                    //  Overflow protection
                    if *res != 0 {
                        *res -= 1;
                    }
                });
            } else {
                <SomeAccountIdentities<T>>::insert(id, types);
            }
        }

        fn get_passport_id(account: T::AccountId) -> Option<PassportId> {
            <SomeAccountToId<T>>::get(account)
        }

        fn get_id_identities(id: PassportId) -> BTreeSet<IdentityType> {
            <SomeAccountIdentities<T>>::get(id)
        }

        fn check_id_identity(id: PassportId, id_type: IdentityType) -> bool {
            let types = <SomeAccountIdentities<T>>::get(id);
            types.contains(&id_type)
        }

        fn get_account_identities(account: T::AccountId) -> BTreeSet<IdentityType> {
            match <SomeAccountToId<T>>::get(account) {
                Some(id) => Self::get_id_identities(id),
                None => BTreeSet::new(),
            }
        }

        fn check_account_indetity(account: T::AccountId, id_type: IdentityType) -> bool {
            match <SomeAccountToId<T>>::get(account) {
                Some(id) => Self::check_id_identity(id, id_type),
                None => false,
            }
        }

        fn get_citizens_amount() -> u64 {
            <CitizensAmount<T>>::get()
        }
    }
}

pub trait IdentityTrait<T: Config> {
    fn match_account_to_id(account: T::AccountId, id: PassportId);

    fn push_identity(id: PassportId, id_type: IdentityType);

    fn remove_identity(id: PassportId, id_type: IdentityType);

    fn get_passport_id(account: T::AccountId) -> Option<PassportId>;

    fn get_id_identities(id: PassportId) -> BTreeSet<IdentityType>;

    fn check_id_identity(id: PassportId, id_type: IdentityType) -> bool;

    fn get_account_identities(account: T::AccountId) -> BTreeSet<IdentityType>;

    fn check_account_indetity(account: T::AccountId, id_type: IdentityType) -> bool;

    fn get_citizens_amount() -> u64;
}

sp_api::decl_runtime_apis! {
    pub trait IdentityPalletApi<T: Config> {
        fn get_passport_id(account: T::AccountId) -> Option<PassportId>;

        fn get_id_identities(id: PassportId) -> BTreeSet<IdentityType>;

        fn check_id_identity(id: PassportId, id_type: IdentityType) -> bool;

        fn get_account_identities(account: T::AccountId) -> BTreeSet<IdentityType>;

        fn check_account_indetity(account: T::AccountId, id_type: IdentityType) -> bool;
    }
}

pub type PassportId = [u8; 32];

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdentityType {
    Citizen,
    MinisterOfInterior,
}
