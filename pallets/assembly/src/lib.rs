#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::dispatch::DispatchResultWithPostInfo;
pub use pallet::*;
use pallet_documentation::{DocumentType, DocumentationTrait};

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
    pub trait Config: frame_system::Config + pallet_documentation::Config {
        type DocumentsTrait: DocumentationTrait<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        DocumentIsNotFound,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> AssemblyTrait<T> for Pallet<T> {
        fn submit_document(document_key: DocumentType) -> DispatchResultWithPostInfo {
            let mut doc = T::DocumentsTrait::get_document(document_key.clone())
                .ok_or(<Error<T>>::DocumentIsNotFound)?;
            doc.submited = true;
            T::DocumentsTrait::update_document(document_key, doc);
            Ok(().into())
        }
    }
}

pub trait AssemblyTrait<T: Config> {
    fn submit_document(document_key: DocumentType) -> DispatchResultWithPostInfo;
}
