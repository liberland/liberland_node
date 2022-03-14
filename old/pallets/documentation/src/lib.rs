#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::codec::{Decode, Encode};
pub use pallet::*;
use sp_std::cmp::{Ord, PartialOrd};

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

    #[pallet::storage]
    pub type SomeDocuments<T: Config> =
        StorageMap<_, Blake2_128Concat, DocumentType, Document, OptionQuery>;

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> DocumentationTrait<T> for Pallet<T> {
        fn push_document(key: DocumentType, document: Document) {
            if <SomeDocuments<T>>::get(key.clone()).is_none() {
                <SomeDocuments<T>>::insert(key, document);
            }
        }
        fn update_document(key: DocumentType, document: Document) {
            if <SomeDocuments<T>>::get(key.clone()).is_some() {
                <SomeDocuments<T>>::insert(key, document);
            }
        }
        fn get_document(key: DocumentType) -> Option<Document> {
            let res = <SomeDocuments<T>>::get(key)?;
            Some(res)
        }
    }
}

pub trait DocumentationTrait<T: Config> {
    fn push_document(key: DocumentType, document: Document);
    fn update_document(key: DocumentType, document: Document);
    fn get_document(key: DocumentType) -> Option<Document>;
}

pub type DocumentType = sp_std::vec::Vec<u8>;

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Document {
    pub content: sp_std::vec::Vec<u8>,
    pub submited: bool,
}
