#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::codec::{Decode, Encode};
pub use pallet::*;
use pallet_identity::{IdentityTrait, IdentityType, PassportId};
use pallet_voting::{AltVote, Candidate, VotingTrait};
use sp_std::collections::btree_set::BTreeSet;
use sp_std::convert::TryInto;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::sp_runtime::traits::Zero;
    use frame_system::pallet_prelude::*;
    use pallet_staking::StakingTrait;
    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_identity::Config
        + pallet_voting::Config
        + pallet_staking::Config
    {
        const ASSEMBLY_ELECTION_PERIOD: Self::BlockNumber;

        const ASSEMBLY_VOTING_DURATION: Self::BlockNumber;

        const ASSEMBLY_VOTING_HASH: Self::Hash;

        const LAW_VOTING_DURATION: Self::BlockNumber;

        const WINNERS_AMOUNT: u32;

        type IdentTrait: IdentityTrait<Self>;

        type VotingTrait: pallet_voting::VotingTrait<Self>;

        type StakingTrait: pallet_staking::StakingTrait<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        AccountCannotBeAddedAsCandiate,
        VotingNotFound,
        AccountCannotVote,
        IsNotActiveVoting,
        AlreadyVoted,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(block_number: BlockNumberFor<T>) -> frame_support::weights::Weight {
            if (block_number % T::ASSEMBLY_ELECTION_PERIOD).is_zero() {
                Self::initialize();
            }
            0
        }
    }

    #[pallet::storage]
    #[pallet::getter(fn candidates_list)]
    type CandidatesList<T: Config> =
        StorageValue<_, BTreeSet<Candidate>, ValueQuery, DefaultCandidates>;

    #[pallet::storage]
    #[pallet::getter(fn ministers_list)]
    type CurrentMinistersList<T: Config> =
        StorageValue<_, BTreeSet<Candidate>, ValueQuery, DefaultCandidates>;

    #[pallet::storage]
    type SomeVotedCitizens<T: Config> =
        StorageValue<_, BTreeSet<PassportId>, ValueQuery, DefaultVotedCitizens>;

    #[pallet::storage]
    type VotedAssemblies<T: Config> =
        StorageValue<_, BTreeSet<PassportId>, ValueQuery, DefaultVotedCitizens>;

    #[pallet::storage]
    #[pallet::getter(fn laws)]
    type Laws<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, LawState, OptionQuery>;

    #[pallet::type_value]
    pub fn DefaultCandidates() -> BTreeSet<Candidate> {
        BTreeSet::default()
    }

    #[pallet::type_value]
    pub fn DefaultVotedCitizens() -> BTreeSet<PassportId> {
        Default::default()
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1)]
        pub(super) fn vote(origin: OriginFor<T>, ballot: AltVote) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                T::IdentTrait::check_account_indetity(sender.clone(), IdentityType::Citizen),
                <Error<T>>::AccountCannotVote
            );
            //this unwrap() is correct
            let citizen = T::IdentTrait::get_passport_id(sender.clone()).unwrap();
            ensure!(
                !<SomeVotedCitizens<T>>::get().contains(&citizen),
                <Error<T>>::AlreadyVoted
            );
            let power = T::StakingTrait::get_liber_amount(sender);
            let b = TryInto::<u64>::try_into(power).ok().unwrap();
            Self::alt_vote(T::ASSEMBLY_VOTING_HASH, ballot, b)?;
            <SomeVotedCitizens<T>>::mutate(|voted_citizens| {
                voted_citizens.insert(citizen);
            });

            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn add_candidate(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let citizen = T::IdentTrait::get_passport_id(sender)
                .ok_or(<Error<T>>::AccountCannotBeAddedAsCandiate)?;
            Self::add_candidate_internal(citizen)?;
            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn propose_law(
            origin: OriginFor<T>,
            law_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                T::IdentTrait::check_account_indetity(sender, IdentityType::Assembly),
                <Error<T>>::AccountCannotBeAddedAsCandiate
            );
            T::VotingTrait::create_voting(law_hash, T::LAW_VOTING_DURATION)?;
            <Laws<T>>::insert(law_hash, LawState::InProgress);
            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn vote_to_law(
            origin: OriginFor<T>,
            law_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                T::IdentTrait::check_account_indetity(sender.clone(), IdentityType::Assembly),
                <Error<T>>::AccountCannotBeAddedAsCandiate
            );

            //this unwrap() is correct
            let assembly = T::IdentTrait::get_passport_id(sender.clone()).unwrap();
            ensure!(
                !<VotedAssemblies<T>>::get().contains(&assembly),
                <Error<T>>::AlreadyVoted
            );
            let power = TryInto::<u64>::try_into(T::StakingTrait::get_liber_amount(sender))
                .ok()
                .unwrap();
            T::VotingTrait::vote(law_hash, power)?;
            <VotedAssemblies<T>>::mutate(|voted_assemblyes| {
                voted_assemblyes.insert(assembly);
            });
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        fn initialize() {
            let candidates = <CandidatesList<T>>::get();
            T::VotingTrait::create_alt_voting_list(
                T::ASSEMBLY_VOTING_HASH,
                T::ASSEMBLY_VOTING_DURATION,
                candidates,
                T::WINNERS_AMOUNT,
            )
            .unwrap();
            <CandidatesList<T>>::kill();
        }

        pub fn add_candidate_internal(id: PassportId) -> Result<(), Error<T>> {
            let candidate = T::IdentTrait::get_id_identities(id);
            if !candidate.contains(&IdentityType::Citizen) {
                return Err(<Error<T>>::AccountCannotBeAddedAsCandiate);
            }
            <CandidatesList<T>>::mutate(|elem| {
                elem.insert(id.to_vec());
            });
            Ok(())
        }

        pub fn alt_vote(subject: T::Hash, ballot: AltVote, power: u64) -> Result<(), Error<T>> {
            match T::VotingTrait::alt_vote_list(subject, ballot, power) {
                Ok(_) => Ok(()),
                Err(_) => Err(<Error<T>>::VotingNotFound),
            }
        }
    }

    impl<T: Config> pallet_voting::finalize_voiting_trait::FinalizeAltVotingListDispatchTrait<T>
        for Pallet<T>
    {
        fn finalize_voting(
            _subject: T::Hash,
            _voting_settings: pallet_voting::AltVotingListSettings<T::BlockNumber>,
            winners: BTreeSet<Candidate>,
        ) {
            <CurrentMinistersList<T>>::mutate(|e| {
                for i in winners.iter() {
                    let mut id_slice: [u8; 32] = [Default::default(); 32];
                    id_slice[..i.len()].copy_from_slice(i);
                    T::IdentTrait::push_identity(id_slice, IdentityType::Assembly).unwrap();
                    e.insert(i.clone());
                }
            });
        }
    }

    impl<T: Config> pallet_voting::FinalizeVotingDispatchTrait<T> for Pallet<T> {
        fn finalize_voting(
            subject: T::Hash,
            voting_setting: pallet_voting::VotingSettings<T::BlockNumber>,
        ) {
            let ministers = <CurrentMinistersList<T>>::get();
            if ((voting_setting.result as f64 / ministers.len() as f64) * 100.0) > 50.0 {
                <Laws<T>>::insert(subject, LawState::Approved);
            } else {
                <Laws<T>>::insert(subject, LawState::Declined);
            }
        }
    }
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Encode, Decode, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LawState {
    Approved,
    InProgress,
    Declined,
}
