#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::codec::{Decode, Encode};
pub use pallet::*;
use pallet_identity::{IdentityTrait, IdentityType, PassportId};
use pallet_voting::{AltVote, AltVoutingSettings, Candidate, VotingTrait};
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
        type PrimeMinVotingHash: Get<Self::Hash>;

        #[pallet::constant]
        type WinnersAmount: Get<u32>;

        #[pallet::constant]
        type PrimeMinVotingDuration: Get<Self::BlockNumber>;

        #[pallet::constant]
        type PrimeMinVotingDelay: Get<Self::BlockNumber>;

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
        AccountCannotProposeLaw,
        VotingNotFound,
        AccountCannotVote,
        IsNotActiveVoting,
        AlreadyVoted,
        VotingIsAlreadyInProgress,
        LifeTimeIsLessThanLaws,
        AssemblyNotFound,
        NoSuchBallot,
        ChangePowerTooBig,
        AccountCannotSupport,
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

        fn on_finalize(block_number: BlockNumberFor<T>) {
            let current_block = TryInto::<u64>::try_into(block_number).ok().unwrap();
            let assembly_voting_duration =
                TryInto::<u64>::try_into(T::AssemblyVotingDuration::get())
                    .ok()
                    .unwrap();
            let assembly_election_period =
                TryInto::<u64>::try_into(T::AssemblyElectionPeriod::get())
                    .ok()
                    .unwrap();
            let prime_min_voting_delay = TryInto::<u64>::try_into(T::PrimeMinVotingDelay::get())
                .ok()
                .unwrap();
            let x = (current_block / (assembly_voting_duration + assembly_election_period)) as u64;
            let sub_block = x * (assembly_voting_duration + assembly_election_period);
            if (current_block % (sub_block + assembly_voting_duration + prime_min_voting_delay))
                .is_zero()
            {
                Self::start_prime_min_voting();
            }
        }
    }

    #[pallet::storage]
    #[pallet::getter(fn candidates_list)]
    type CandidatesList<T: Config> =
        StorageValue<_, BTreeSet<Candidate>, ValueQuery, DefaultCandidates>;

    #[pallet::storage]
    #[pallet::getter(fn ministers_list)]
    type CurrentAssembliesList<T: Config> =
        StorageValue<_, BTreeMap<Candidate, u64>, ValueQuery, DefaultMinisters>;

    #[pallet::storage]
    type VotedCitizens<T: Config> =
        StorageValue<_, BTreeSet<PassportId>, ValueQuery, DefaultVotedCitizens>;

    #[pallet::storage]
    type VotedAssemblies<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        BTreeSet<VotedAssembly>,
        ValueQuery,
        DefaultVotedAssemblies,
    >;

    #[pallet::storage]
    #[pallet::getter(fn voting_state)]
    type VotingState<T: Config> = StorageValue<_, bool, ValueQuery, DefaultState>;

    #[pallet::storage]
    #[pallet::getter(fn laws)]
    type Laws<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Law, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn assemblys_stake_amount)]
    type AssemblyStakeAmount<T: Config> = StorageValue<_, u64, ValueQuery, DefaultLiberAmount>;

    #[pallet::storage]
    #[pallet::getter(fn ballots)]
    type Ballots<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery, DefaultBallot<T>>;

    #[pallet::storage]
    #[pallet::getter(fn current_prime_min)]
    type CurrentPrimeMinister<T: Config> = StorageValue<_, Candidate, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn prime_min_candidates_list)]
    type PrimeMinCandidatesList<T: Config> =
        StorageValue<_, BTreeSet<Candidate>, ValueQuery, DefaultCandidates>;

    #[pallet::storage]
    type VotedForPrimeMinAssemblies<T: Config> =
        StorageValue<_, BTreeSet<PassportId>, ValueQuery, DefaultVotedForPrimeMinAssemblies>;

    #[pallet::type_value]
    pub fn DefaultBallot<T: Config>() -> u64 {
        Default::default()
    }

    #[pallet::type_value]
    pub fn DefaultCandidates() -> BTreeSet<Candidate> {
        Default::default()
    }

    #[pallet::type_value]
    pub fn DefaultVotedCitizens() -> BTreeSet<PassportId> {
        Default::default()
    }

    #[pallet::type_value]
    pub fn DefaultVotedAssemblies() -> BTreeSet<VotedAssembly> {
        BTreeSet::new()
    }
    #[pallet::type_value]
    pub fn DefaultVotedForPrimeMinAssemblies() -> BTreeSet<PassportId> {
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
    #[pallet::type_value]
    pub fn DefaultLiberAmount() -> u64 {
        Default::default()
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1)]
        pub fn vote(origin: OriginFor<T>, ballot: AltVote) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                T::IdentTrait::check_account_indetity(sender.clone(), IdentityType::Citizen),
                <Error<T>>::AccountCannotVote
            );
            //this unwrap() is correct
            let citizen = pallet_identity::Pallet::<T>::passport_id(sender.clone()).unwrap();
            ensure!(
                !<VotedCitizens<T>>::get().contains(&citizen),
                <Error<T>>::AlreadyVoted
            );
            let mut power: pallet_staking::BalanceOf<T> = Zero::zero();
            pallet_identity::Pallet::<T>::account_ids(citizen)
                .iter()
                .for_each(|account_id| {
                    power += T::StakingTrait::get_liber_amount(account_id.clone());
                });

            let power = TryInto::<u64>::try_into(power).ok().unwrap();
            Self::alt_vote(sender, ballot, power)?;
            <VotedCitizens<T>>::mutate(|voted_citizens| {
                voted_citizens.insert(citizen);
            });

            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn vote_to_prime_min(
            origin: OriginFor<T>,
            ballot: AltVote,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                T::IdentTrait::check_account_indetity(sender.clone(), IdentityType::Assembly),
                <Error<T>>::AccountCannotVote
            );
            //this unwrap() is correct
            let assembly_id = pallet_identity::Pallet::<T>::passport_id(sender.clone()).unwrap();

            ensure!(
                !<VotedForPrimeMinAssemblies<T>>::get().contains(&assembly_id),
                <Error<T>>::AlreadyVoted
            );
            let assemblies_list = Self::ministers_list();
            //this unwrap() is correct
            let assembly_power = assemblies_list.get(&assembly_id.to_vec()).unwrap();
            Self::prime_min_alt_vote(sender, ballot, *assembly_power)?;
            <VotedForPrimeMinAssemblies<T>>::mutate(|storage| {
                storage.insert(assembly_id);
            });
            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn add_candidate(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                !<VotingState<T>>::get(),
                <Error<T>>::VotingIsAlreadyInProgress
            );
            let citizen = pallet_identity::Pallet::<T>::passport_id(sender)
                .ok_or(<Error<T>>::AccountCannotBeAddedAsCandiate)?;
            Self::add_candidate_internal(citizen)?;
            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn add_prime_min_condidate(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let assembly = pallet_identity::Pallet::<T>::passport_id(sender)
                .ok_or(<Error<T>>::AccountCannotBeAddedAsCandiate)?;
            Self::add_prime_min_candidate_internal(assembly)?;
            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn propose_law(
            origin: OriginFor<T>,
            law_hash: T::Hash,
            law_type: LawType,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let current_block = TryInto::<u64>::try_into(<frame_system::Pallet<T>>::block_number())
                .ok()
                .unwrap();
            let assembly_voting_duration =
                TryInto::<u64>::try_into(T::AssemblyVotingDuration::get())
                    .ok()
                    .unwrap();
            let assembly_election_period =
                TryInto::<u64>::try_into(T::AssemblyElectionPeriod::get())
                    .ok()
                    .unwrap();
            let laws_voting_duration = TryInto::<u64>::try_into(T::LawVotingDuration::get())
                .ok()
                .unwrap();
            let x: f64 =
                (current_block / (assembly_voting_duration + assembly_election_period)) as f64;
            ensure!(
                (x + 1.0) * (assembly_election_period + assembly_election_period) as f64
                    >= (current_block + laws_voting_duration) as f64,
                <Error<T>>::LifeTimeIsLessThanLaws
            );
            ensure!(
                T::IdentTrait::check_account_indetity(sender, IdentityType::Assembly),
                <Error<T>>::AccountCannotProposeLaw
            );
            T::VotingTrait::create_voting(
                law_hash,
                T::LawVotingDuration::get(),
                Some(<CurrentAssembliesList<T>>::get().len() as u32),
            )?;
            <Laws<T>>::insert(
                law_hash,
                Law {
                    state: LawState::InProgress,
                    law_type,
                },
            );
            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn vote_to_law(
            origin: OriginFor<T>,
            law_hash: T::Hash,
            estimate: Decision,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                T::IdentTrait::check_account_indetity(sender.clone(), IdentityType::Assembly),
                <Error<T>>::AccountCannotVote
            );

            //this unwrap() is correct
            let pasport_id = pallet_identity::Pallet::<T>::passport_id(sender).unwrap();
            let assembly = VotedAssembly {
                id: pasport_id,
                estimate,
            };
            ensure!(
                !<VotedAssemblies<T>>::get(law_hash).contains(&assembly),
                <Error<T>>::AlreadyVoted
            );

            //this unwrap() is correct
            let power = *<CurrentAssembliesList<T>>::get()
                .get(&assembly.id.to_vec())
                .unwrap();

            T::VotingTrait::vote(law_hash, power)?;
            <VotedAssemblies<T>>::mutate(law_hash, |voted_assemblyes| {
                voted_assemblyes.insert(assembly);
            });
            Ok(().into())
        }

        #[pallet::weight(1)]
        pub(super) fn change_support(
            origin: OriginFor<T>,
            assembly_id: Candidate,
            change_power: i64,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                T::IdentTrait::check_account_indetity(sender.clone(), IdentityType::Citizen),
                <Error<T>>::AccountCannotSupport
            );
            let assemblies = <CurrentAssembliesList<T>>::get();
            ensure!(
                assemblies.contains_key(&assembly_id),
                <Error<T>>::AssemblyNotFound
            );
            let liber_stake = T::StakingTrait::get_liber_amount(sender.clone());
            let liber_stake = TryInto::<u64>::try_into(liber_stake).ok().unwrap();
            if change_power.is_negative() {
                ensure!(
                    (-change_power) as u64 <= liber_stake,
                    <Error<T>>::ChangePowerTooBig
                );
            } else {
                ensure!(
                    change_power as u64 <= liber_stake,
                    <Error<T>>::ChangePowerTooBig
                );
            }
            let current_citizen_power = Self::ballots(sender.clone());
            let current_assembly_power = assemblies.get(&assembly_id).unwrap();
            let assemblies_res_power;
            let citizens_res_power;
            if change_power > 0 {
                if current_citizen_power + change_power as u64 > liber_stake {
                    citizens_res_power = liber_stake;
                    assemblies_res_power = *current_assembly_power;
                } else {
                    citizens_res_power = current_citizen_power + change_power.unsigned_abs();
                    assemblies_res_power = current_assembly_power + change_power.unsigned_abs();
                }
            } else if change_power.unsigned_abs() > current_citizen_power {
                citizens_res_power = 0;
                assemblies_res_power = *current_assembly_power;
            } else {
                citizens_res_power = current_citizen_power - change_power.unsigned_abs();
                assemblies_res_power = current_assembly_power - change_power.unsigned_abs();
            }

            <Ballots<T>>::insert(sender, citizens_res_power);
            <CurrentAssembliesList<T>>::mutate(|asymblies_storage| {
                asymblies_storage.insert(assembly_id, assemblies_res_power);
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

        pub fn add_prime_min_candidate_internal(id: PassportId) -> Result<(), Error<T>> {
            let candidate = pallet_identity::Pallet::<T>::identities(id);
            if !candidate.contains(&IdentityType::Assembly) {
                return Err(<Error<T>>::AccountCannotBeAddedAsCandiate);
            }
            <PrimeMinCandidatesList<T>>::mutate(|storage| {
                storage.insert(id.to_vec());
            });
            Ok(())
        }

        fn start_prime_min_voting() {
            //This unwrap() is correct
            T::VotingTrait::create_alt_voting(
                T::PrimeMinVotingHash::get(),
                T::PrimeMinVotingDuration::get(),
                <PrimeMinCandidatesList<T>>::get(),
                Some(<CurrentAssembliesList<T>>::get().len() as u32),
            )
            .unwrap();
        }

        pub fn alt_vote(
            account_id: T::AccountId,
            ballot: AltVote,
            power: u64,
        ) -> Result<(), Error<T>> {
            match T::VotingTrait::alt_vote_list(
                T::AssemblyVotingHash::get(),
                account_id,
                ballot,
                power,
            ) {
                Ok(_) => Ok(()),
                Err(_) => Err(<Error<T>>::VotingNotFound),
            }
        }

        pub fn prime_min_alt_vote(
            account_id: T::AccountId,
            ballot: AltVote,
            power: u64,
        ) -> Result<(), Error<T>> {
            match T::VotingTrait::alt_vote(T::PrimeMinVotingHash::get(), account_id, ballot, power)
            {
                Ok(_) => Ok(()),
                Err(_) => Err(<Error<T>>::VotingNotFound),
            }
        }

        pub fn vec_u8_to_pasport_id(id: &[u8]) -> PassportId {
            let mut id_slice: [u8; 32] = [Default::default(); 32];
            id_slice[..id.len()].copy_from_slice(id);
            id_slice
        }
    }

    #[allow(clippy::type_complexity)]
    impl<T: Config> pallet_voting::finalize_voiting_trait::FinalizeAltVotingListDispatchTrait<T>
        for Pallet<T>
    {
        fn finalize_voting(
            _subject: T::Hash,
            _voting_settings: pallet_voting::AltVotingListSettings<T::BlockNumber>,
            winners: BTreeMap<Candidate, u64>,
            ballots_storage: BTreeMap<T::Hash, BTreeMap<T::AccountId, (AltVote, u64)>>,
        ) {
            <CurrentAssembliesList<T>>::get()
                .iter()
                .for_each(|assembly| {
                    T::IdentTrait::remove_identity(
                        Self::vec_u8_to_pasport_id(assembly.0),
                        IdentityType::Assembly,
                    );
                });

            <Ballots<T>>::remove_all();
            <AssemblyStakeAmount<T>>::kill();
            <CurrentAssembliesList<T>>::kill();
            <CandidatesList<T>>::kill();
            <VotedCitizens<T>>::kill();

            <CurrentAssembliesList<T>>::mutate(|e| {
                for (id, power) in winners.iter() {
                    T::IdentTrait::push_identity(
                        Self::vec_u8_to_pasport_id(id),
                        IdentityType::Assembly,
                    )
                    .unwrap();
                    e.insert(id.clone(), *power);
                }
            });

            ballots_storage.iter().for_each(|storage| {
                storage.1.iter().for_each(|map| {
                    let alt = map.1;
                    <Ballots<T>>::insert(map.0, alt.1);
                });
            });
            <VotingState<T>>::mutate(|state| *state = false);
        }
    }

    impl<T: Config> pallet_voting::FinilizeAltVotingDispatchTrait<T> for Pallet<T> {
        fn finalize_voting(
            _subject: T::Hash,
            _voting_setting: AltVoutingSettings<T::BlockNumber>,
            winner: Candidate,
        ) {
            if let Some(prime) = <CurrentPrimeMinister<T>>::get() {
                T::IdentTrait::remove_identity(
                    Self::vec_u8_to_pasport_id(&prime),
                    IdentityType::PrimeMinister,
                );
                <CurrentPrimeMinister<T>>::kill();
            }
            <CurrentPrimeMinister<T>>::put(winner.clone());
            T::IdentTrait::push_identity(
                Self::vec_u8_to_pasport_id(&winner),
                IdentityType::PrimeMinister,
            )
            .unwrap();
            <PrimeMinCandidatesList<T>>::kill();
            <VotedForPrimeMinAssemblies<T>>::kill();
        }
    }

    impl<T: Config> pallet_voting::FinalizeVotingDispatchTrait<T> for Pallet<T> {
        fn finalize_voting(
            subject: T::Hash,
            voting_setting: pallet_voting::VotingSettings<T::BlockNumber>,
        ) {
            let total_power: u64 = <CurrentAssembliesList<T>>::get().iter().map(|e| e.1).sum();
            <AssemblyStakeAmount<T>>::mutate(|value| *value = total_power);
            if let Some(law) = <Laws<T>>::get(subject) {
                match law.law_type {
                    LawType::ConstitutionalChange | LawType::Legislation => {
                        if ((voting_setting.result as f64 / total_power as f64) * 100.0) > 66.6 {
                            <Laws<T>>::insert(
                                subject,
                                Law {
                                    state: LawState::Approved,
                                    law_type: law.law_type,
                                },
                            );
                        } else {
                            <Laws<T>>::insert(
                                subject,
                                Law {
                                    state: LawState::Declined,
                                    law_type: law.law_type,
                                },
                            );
                        }
                    }
                    _ => {
                        if ((voting_setting.result as f64 / total_power as f64) * 100.0) > 50.0 {
                            <Laws<T>>::insert(
                                subject,
                                Law {
                                    state: LawState::Approved,
                                    law_type: law.law_type,
                                },
                            );
                        } else {
                            <Laws<T>>::insert(
                                subject,
                                Law {
                                    state: LawState::Declined,
                                    law_type: law.law_type,
                                },
                            );
                        }
                    }
                }
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

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Encode, Decode, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LawType {
    ConstitutionalChange,
    Legislation,
    Decision,
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Encode, Decode, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Law {
    pub state: LawState,
    pub law_type: LawType,
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Encode, Decode, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Decision {
    Accept,
    Decline,
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Encode, Decode, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct VotedAssembly {
    pub id: PassportId,
    pub estimate: Decision,
}
