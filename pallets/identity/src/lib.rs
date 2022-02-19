// Module do identify the citizens of Liberland, registering the association of a passportid with a blockchain account.
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
    // definition of the storage for identities
    #[pallet::storage]
    #[pallet::getter(fn identities)]           
    pub type Identities<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PassportId,
        BTreeSet<IdentityType>,
        ValueQuery,
        IdentityTypesDefault,
    >;

    // definition of the storage for passport id
    #[pallet::storage]
    #[pallet::getter(fn passport_id)]
    type PassportIds<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, PassportId, OptionQuery>;

    // definition of the storage for account id
    #[pallet::storage]
    #[pallet::getter(fn account_ids)]
    type AccountIds<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PassportId,
        BTreeSet<T::AccountId>,
        ValueQuery,
        DefaultAccountIdsSet<T>,
    >;
    // definition of the storage for the total number of registered citizens
    #[pallet::storage]
    #[pallet::getter(fn citizens_amount)]
    type CitizensAmount<T: Config> = StorageValue<_, u64, ValueQuery, DefaultCitizensAmountStorage>;

    // definition of default values 
    #[pallet::type_value]
    pub fn DefaultCitizensAmountStorage() -> u64 {
        0_u64
    }
    #[pallet::type_value]
    pub fn DefaultAccountIdsSet<T: Config>() -> BTreeSet<T::AccountId> {
        Default::default()
    }

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // definition of genesis configuration
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub citizens: Vec<(T::AccountId, PassportId)>,
        pub reviewers: Vec<PassportId>,
	pub assembly_members: Vec<(T::AccountId, PassportId)>,
    }
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                reviewers: Default::default(),
                citizens: Default::default(),
		assembly_members: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            <CitizensAmount<T>>::put(self.citizens.len() as u64);
            for (account, id) in self.citizens.iter() {
                <Pallet<T>>::match_account_to_id(account.clone(), *id);
                <Pallet<T>>::push_identity(*id, IdentityType::Citizen).unwrap();
            }

	    for (accountid, id) in self.assembly_members.iter() {

                <Pallet<T>>::match_account_to_id(accountid.clone(), *id); //insert passport
                <Pallet<T>>::push_identity(*id, IdentityType::Assembly).unwrap(); // create user 
            }

            for id in self.reviewers.iter() {
                <Pallet<T>>::push_identity(*id, IdentityType::MinisterOfInterior).unwrap();
            }
        }
    }
    // TODO VERIFY AUTHORIZATION TO CHANGE STATE - only a limited set of accounts should be authorized
    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> IdentityTrait<T> for Pallet<T> {
        // TODO return value of the function should be Result<(), Error>
        // function to store the association of account with passport id
        fn match_account_to_id(account: T::AccountId, id: PassportId) {
            assert!(
                <PassportIds<T>>::get(account.clone()) == None,
                "for this AccountId has been already matched PassportId"
            );
            <PassportIds<T>>::insert(account.clone(), id);
            <AccountIds<T>>::mutate(id, |accounts| {
                accounts.insert(account);
            });
        }

        fn push_identity(id: PassportId, id_type: IdentityType) -> Result<(), &'static str> {
            match id_type {
                IdentityType::EResident => {
                    let mut types = <Identities<T>>::get(id);
                    if !types.contains(&IdentityType::Citizen) {
                        types.insert(id_type);
                        <Identities<T>>::insert(id, types);
                        Ok(())
                    } else {
                        Err("Citizen cannot become the E-resident at the same time")
                    }
                }


                IdentityType::Assembly => {
                    let mut types = <Identities<T>>::get(id);
                    if !types.contains(&IdentityType::Assembly) {
                        types.insert(id_type);
                        let id_2 = <Identities<T>>::iter().find(|item| item.0 == id);
                        <Identities<T>>::insert(id, types);
                        if id_2.is_none() {
                            <CitizensAmount<T>>::mutate(|res| *res += 1);
                        }
                        Ok(())
                    } else {
                        Err("Assembly members cannot become the Citizen at the same time")
                    }
                }
                IdentityType::Citizen => {
                    let mut types = <Identities<T>>::get(id);
                    if !types.contains(&IdentityType::EResident) {
                        types.insert(id_type);
                        let id_2 = <Identities<T>>::iter().find(|item| item.0 == id);
                        <Identities<T>>::insert(id, types);
                        if id_2.is_none() {
                            <CitizensAmount<T>>::mutate(|res| *res += 1);
                        }
                        Ok(())
                    } else {
                        Err("Eresidence cannot become the Citizen at the same time")
                    }
                }
                _ => {
                    let mut types = <Identities<T>>::get(id);
                    if types.contains(&IdentityType::Citizen) {
                        types.insert(id_type);
                        <Identities<T>>::insert(id, types);
                        Ok(())
                    } else {
                        Err("We can not add any other IdentityTypes before we have add IdentityType::Citizen")
                    }
                }
            }
        }
        // function to remove a citizend from the state
        fn remove_identity(id: PassportId, id_type: IdentityType) {
            let mut types = <Identities<T>>::get(id);
            if id_type == IdentityType::Citizen {
                types.clear();
                <Identities<T>>::remove(id);
                <CitizensAmount<T>>::mutate(|res| {
                    *res -= 1;
                });
            } else {
                // remove identity type
                types.remove(&id_type);
                if types.is_empty() {
                    <Identities<T>>::remove(id);
                } else {
                    <Identities<T>>::insert(id, types);
                }
            }
        }
        // funtion to check the identity by passport id
        fn check_id_identity(id: PassportId, id_type: IdentityType) -> bool {
            let types = <Identities<T>>::get(id);
            types.contains(&id_type)
        }
        // function to check the identity by account 
        fn check_account_identity(account: T::AccountId, id_type: IdentityType) -> bool {
            match <PassportIds<T>>::get(account) {
                Some(id) => Self::check_id_identity(id, id_type),
                None => false,
            }
        }
    }
}

pub trait IdentityTrait<T: Config> {
    fn match_account_to_id(account: T::AccountId, id: PassportId);

    fn push_identity(id: PassportId, id_type: IdentityType) -> Result<(), &'static str>;

    fn remove_identity(id: PassportId, id_type: IdentityType);

    fn check_id_identity(id: PassportId, id_type: IdentityType) -> bool;

    fn check_account_identity(account: T::AccountId, id_type: IdentityType) -> bool;
}

sp_api::decl_runtime_apis! {
    pub trait IdentityPalletApi<T: Config> {
        fn check_id_identity(id: PassportId, id_type: IdentityType) -> bool;

        fn check_account_identity(account: T::AccountId, id_type: IdentityType) -> bool;
    }
}

pub type PassportId = [u8; 32];

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Encode, Decode, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdentityType {
    Citizen,
    MinisterOfInterior,
    EResident,
    Assembly,
    PrimeMinister,
}
