#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::codec::{Decode, Encode};
pub use pallet::*;
use pallet_identity::{IdentityTrait, IdentityType, PassportId};
use pallet_voting::{AltVote, Candidate, VotingTrait};
use sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};
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
        #[pallet::constant]
        type AssemblyElectionPeriod: Get<Self::BlockNumber>;

        #[pallet::constant]
        type AssemblyVotingDuration: Get<Self::BlockNumber>;

        #[pallet::constant]
        type LawVotingDuration: Get<Self::BlockNumber>;

        #[pallet::constant]
        type AssemblyVotingHash: Get<Self::Hash>;

        #[pallet::constant]
        type WinnersAmount: Get<u32>;

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
            if (block_number
                % (T::AssemblyElectionPeriod::get() + T::AssemblyVotingDuration::get()))
            .is_zero()
            {
                Self::initialize();
                <VotingState<T>>::mutate(|state| *state = true);
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
        StorageValue<_, BTreeMap<Candidate, u64>, ValueQuery, DefaultMinisters>;

    #[pallet::storage]
    type SomeVotedCitizens<T: Config> =
        StorageValue<_, BTreeSet<PassportId>, ValueQuery, DefaultVotedCitizens>;

    #[pallet::storage]
    type VotedAssemblies<T: Config> =
        StorageValue<_, BTreeSet<PassportId>, ValueQuery, DefaultVotedCitizens>;

    #[pallet::storage]
    #[pallet::getter(fn voting_state)]
    type VotingState<T: Config> = StorageValue<_, bool, ValueQuery, DefaultState>;

    #[pallet::storage]
    #[pallet::getter(fn laws)]
    type Laws<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, LawState, OptionQuery>;

    #[pallet::type_value]
    pub fn DefaultCandidates() -> BTreeSet<Candidate> {
        Default::default()
    }

    #[pallet::type_value]
    pub fn DefaultVotedCitizens() -> BTreeSet<PassportId> {
        Default::default()
    }

    #[pallet::type_value]
    pub fn DefaultState() -> bool {
        false
    }

    #[pallet::type_value]
    pub fn DefaultMinisters() -> BTreeMap<Candidate, u64> {
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
            let citizen = pallet_identity::Pallet::<T>::passport_id(sender).unwrap();
            ensure!(
                !<SomeVotedCitizens<T>>::get().contains(&citizen),
                <Error<T>>::AlreadyVoted
            );
            let mut power: pallet_staking::BalanceOf<T> = Zero::zero();
            pallet_identity::Pallet::<T>::account_ids(citizen)
                .iter()
                .for_each(|account_id| {
                    power += T::StakingTrait::get_liber_amount(account_id.clone());
                });

            let power = TryInto::<u64>::try_into(power).ok().unwrap();
            Self::alt_vote(T::AssemblyVotingHash::get(), ballot, power)?;
            <SomeVotedCitizens<T>>::mutate(|voted_citizens| {
                voted_citizens.insert(citizen);
            });

            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn add_candidate(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            // Add check that candidate is citizen
            let citizen = pallet_identity::Pallet::<T>::passport_id(sender)
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
            T::VotingTrait::create_voting(law_hash, T::LawVotingDuration::get())?;
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
            let assembly = pallet_identity::Pallet::<T>::passport_id(sender).unwrap();
            ensure!(
                !<VotedAssemblies<T>>::get().contains(&assembly),
                <Error<T>>::AlreadyVoted
            );

            //this unwrap() is correct
            let power = *<CurrentMinistersList<T>>::get()
                .get(&assembly.to_vec())
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
                T::AssemblyVotingHash::get(),
                T::AssemblyVotingDuration::get(),
                candidates,
                T::WinnersAmount::get(),
            )
            .unwrap();
            <CandidatesList<T>>::kill();
        }

        pub fn add_candidate_internal(id: PassportId) -> Result<(), Error<T>> {
            let candidate = pallet_identity::Pallet::<T>::identities(id);
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

        fn vec_u8_to_pasport_id(id: &[u8]) -> PassportId {
            let mut id_slice: [u8; 32] = [Default::default(); 32];
            id_slice[..id.len()].copy_from_slice(id);
            id_slice
        }
    }

    impl<T: Config> pallet_voting::finalize_voiting_trait::FinalizeAltVotingListDispatchTrait<T>
        for Pallet<T>
    {
        fn finalize_voting(
            _subject: T::Hash,
            _voting_settings: pallet_voting::AltVotingListSettings<T::BlockNumber>,
            winners: BTreeMap<Candidate, u64>,
        ) {
            <CurrentMinistersList<T>>::get()
                .iter()
                .for_each(|assembly| {
                    T::IdentTrait::remove_identity(
                        Self::vec_u8_to_pasport_id(assembly.0),
                        IdentityType::Assembly,
                    );
                });
            <CurrentMinistersList<T>>::kill();
            <CurrentMinistersList<T>>::mutate(|e| {
                for (id, power) in winners.iter() {
                    T::IdentTrait::push_identity(
                        Self::vec_u8_to_pasport_id(id),
                        IdentityType::Assembly,
                    )
                    .unwrap();
                    e.insert(id.clone(), *power);
                }
            });
            <VotingState<T>>::mutate(|state| *state = false);
        }
    }

    impl<T: Config> pallet_voting::FinalizeVotingDispatchTrait<T> for Pallet<T> {
        fn finalize_voting(
            subject: T::Hash,
            voting_setting: pallet_voting::VotingSettings<T::BlockNumber>,
        ) {
            let total_power: u64 = <CurrentMinistersList<T>>::get().iter().map(|e| e.1).sum();
            if ((voting_setting.result as f64 / total_power as f64) * 100.0) > 50.0 {
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
