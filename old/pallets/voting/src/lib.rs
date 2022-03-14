#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_system::pallet_prelude::BlockNumberFor;
use if_chain::if_chain;
pub use pallet::*;
use sp_std::{
    cmp::{Ord, PartialOrd},
    collections::btree_map::BTreeMap,
    collections::btree_set::BTreeSet,
    collections::vec_deque::VecDeque,
    vec::Vec,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod finalize_voiting_trait;
use crate::finalize_voiting_trait::FinalizeAltVotingListDispatchTrait; // FinalizeAltVotingListDispatchTrait
pub use finalize_voiting_trait::FinalizeVotingDispatchTrait;
pub use finalize_voiting_trait::FinilizeAltVotingDispatchTrait;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type FinalizeVotingDispatch: FinalizeVotingDispatchTrait<Self>;
        type FinalizeAltVotingDispatch: FinilizeAltVotingDispatchTrait<Self>;
        type FinalizeAltVotingListDispatch: FinalizeAltVotingListDispatchTrait<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        // emits when from provided VotingSubject has been created
        VotingHasBeenCreated,
        // emits when provided Voting subject does not exist
        VotingSubjectDoesNotExist,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // Block finalization
        fn on_finalize(block_number: BlockNumberFor<T>) {
            Self::finalize_votings(block_number);
            Self::finalize_alt_votings(block_number);
            Self::finalize_list_alt_votings(block_number);
        }
    }

    #[pallet::storage]
    #[pallet::getter(fn active_votings)]
    type ActiveVotings<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, VotingSettings<T::BlockNumber>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn active_alt_votings)]
    type ActiveAltVoitings<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, AltVoutingSettings<T::BlockNumber>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn active_alt_list_votings)]
    type ActiveAltListVoitings<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        AltVotingListSettings<T::BlockNumber>,
        OptionQuery,
    >;
    #[pallet::storage]
    type BallotsStorage<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        BTreeMap<T::AccountId, (AltVote, u64)>,
        ValueQuery,
        DefaultBallot<T>,
    >;

    #[pallet::type_value]
    pub fn DefaultBallot<T: Config>() -> BTreeMap<T::AccountId, (AltVote, u64)> {
        Default::default()
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> Pallet<T> {
        fn finalize_votings(block_number: BlockNumberFor<T>) {
            for (subject, voting_settings) in <ActiveVotings<T>>::iter() {
                if voting_settings.voters_number == voting_settings.voted {
                    <T::FinalizeVotingDispatch>::finalize_voting(subject, voting_settings.clone());
                    <ActiveVotings<T>>::remove(subject);
                }
                // voting has been passed, so we will store the result and remove from the active votings list
                if (voting_settings.voting_duration + voting_settings.submitted_height)
                    <= block_number
                {
                    <T::FinalizeVotingDispatch>::finalize_voting(subject, voting_settings);
                    <ActiveVotings<T>>::remove(subject);
                }
            }
        }
        fn finalize_alt_votings(block_number: BlockNumberFor<T>) {
            <ActiveAltVoitings<T>>::iter().for_each(|(subject, alt_voting_settings)| {
                if alt_voting_settings.voters_number == alt_voting_settings.voted {
                    if let Ok(winner) = Self::calculate_alt_vote_winner(subject) {
                        <T::FinalizeAltVotingDispatch>::finalize_voting(
                            subject,
                            alt_voting_settings.clone(),
                            winner,
                        );
                    }
                    <BallotsStorage<T>>::remove(subject);
                    <ActiveAltVoitings<T>>::remove(subject);
                }
                if (alt_voting_settings.voting_duration + alt_voting_settings.submitted_height)
                    <= block_number
                {
                    if let Ok(winner) = Self::calculate_alt_vote_winner(subject) {
                        <T::FinalizeAltVotingDispatch>::finalize_voting(
                            subject,
                            alt_voting_settings,
                            winner,
                        );
                    }

                    <BallotsStorage<T>>::remove(subject);
                    <ActiveAltVoitings<T>>::remove(subject);
                }
            });
        }
        #[allow(clippy::type_complexity)]
        fn finalize_list_alt_votings(block_number: BlockNumberFor<T>) {
            <ActiveAltListVoitings<T>>::iter().for_each(|(subject, alt_voting_settings)| {
                if (alt_voting_settings.voting_duration + alt_voting_settings.submitted_height)
                    <= block_number
                {
                    let winners = Self::calculate_alt_vote_winners_list(subject).unwrap();
                    let ballots_storage: BTreeMap<T::Hash, BTreeMap<T::AccountId, (AltVote, u64)>> =
                        <BallotsStorage<T>>::iter().collect();
                    <T::FinalizeAltVotingListDispatch>::finalize_voting(
                        subject,
                        alt_voting_settings,
                        winners,
                        ballots_storage,
                    );
                    <BallotsStorage<T>>::remove(subject);
                    <ActiveAltListVoitings<T>>::remove(subject);
                }
            });
        }
        pub fn calculate_alt_vote_winner(subject: T::Hash) -> Result<Candidate, Error<T>> {
            if let Some(settings) = <ActiveAltVoitings<T>>::get(subject) {
                let ballots_list = <BallotsStorage<T>>::get(subject);
                let mut candidate_list: BTreeMap<Candidate, u64> = settings
                    .candidates
                    .iter()
                    .map(|candidate| (candidate.clone(), 0))
                    .collect();

                ballots_list.iter().for_each(|(_, ballot)| {
                    if_chain! {
                        if let Some(vout) = ballot.0.content.front();
                        if let Some(result) = candidate_list.get_mut(vout);
                        then {
                            *result += ballot.1;
                        }
                    }
                });

                let all_voutes: u64 = candidate_list.iter().map(|(_, result)| *result).sum();

                if let Some((max_vouts_candidate, result)) =
                    candidate_list.iter().max_by_key(|(_, result)| *result)
                {
                    if ((*result as f64 / all_voutes as f64) * 100.0) > 50.0 {
                        return Ok(max_vouts_candidate.clone());
                    }
                }

                while candidate_list.len() >= 2 {
                    let mut removeble_key = Vec::new();
                    if let Some((min_voted_candidate, _)) =
                        candidate_list.iter_mut().min_by_key(|(_, result)| **result)
                    {
                        removeble_key = min_voted_candidate.clone();
                        let buffer: Vec<_> = ballots_list
                            .iter()
                            .filter(|(_, ballot)| {
                                ballot.0.content.front() == Some(min_voted_candidate)
                            })
                            .collect();
                        buffer.iter().for_each(|(_, ballot)| {
                            let mut ballot_tmp = ballot.0.content.clone();
                            ballot_tmp.pop_front();
                            if_chain! {
                                if let Some(vout) = ballot_tmp.front();
                                if *vout != removeble_key;
                                if let Some(result) = candidate_list.get_mut(vout);
                                then {
                                    *result += ballot.1;
                                }
                            }
                        });
                    }
                    candidate_list.remove(&removeble_key);
                    let all_voutes: u64 = candidate_list.iter().map(|(_, result)| *result).sum();
                    if let Some((max_vouts_candidate, result)) =
                        candidate_list.iter().max_by_key(|(_, result)| *result)
                    {
                        if ((*result as f64 / all_voutes as f64) * 100.0) > 50.0 {
                            return Ok(max_vouts_candidate.clone());
                        }
                    }
                }
            }
            Err(<Error<T>>::VotingSubjectDoesNotExist)
        }

        pub fn calculate_alt_vote_winners_list(
            subject: T::Hash,
        ) -> Result<BTreeMap<Candidate, u64>, Error<T>> {
            if let Some(settings) = <ActiveAltListVoitings<T>>::get(subject) {
                let ballots_list = <BallotsStorage<T>>::get(subject);
                let mut candidate_list: BTreeMap<Candidate, u64> = settings
                    .candidates
                    .iter()
                    .map(|candidate| (candidate.clone(), 0))
                    .collect();

                ballots_list.iter().for_each(|(_, ballot)| {
                    if_chain! {
                        if let Some(vout) = ballot.0.content.front();
                        if let Some(result) = candidate_list.get_mut(vout);
                        then {
                            *result += ballot.1;
                        }
                    }
                });

                while candidate_list.len() > settings.winners_amount as usize {
                    let mut removeble_key = Vec::new();
                    if let Some((min_voted_candidate, _)) =
                        candidate_list.iter_mut().min_by_key(|(_, result)| **result)
                    {
                        removeble_key = min_voted_candidate.clone();
                        let buffer: Vec<_> = ballots_list
                            .iter()
                            .filter(|(_, ballot)| {
                                ballot.0.content.front() == Some(min_voted_candidate)
                            })
                            .collect();
                        buffer.iter().for_each(|(_, ballot)| {
                            let mut ballot_tmp = ballot.0.content.clone();
                            ballot_tmp.pop_front();
                            if_chain! {
                                if let Some(vout) = ballot_tmp.front();
                                if *vout != removeble_key;
                                if let Some(result) = candidate_list.get_mut(vout);
                                then {
                                    *result += ballot.1;
                                }
                            }
                        });
                    }
                    candidate_list.remove(&removeble_key);
                }
                return Ok(candidate_list);
            }

            Err(<Error<T>>::VotingSubjectDoesNotExist)
        }
    }

    impl<T: Config> VotingTrait<T> for Pallet<T> {
        fn create_voting(
            subject: T::Hash,
            duration: T::BlockNumber,
            all_voters: Option<u32>,
        ) -> Result<(), Error<T>> {
            ensure!(
                <ActiveVotings<T>>::get(subject) == None,
                <Error<T>>::VotingHasBeenCreated
            );

            let block_number = <frame_system::Pallet<T>>::block_number();
            if let Some(all) = all_voters {
                <ActiveVotings<T>>::insert(
                    subject,
                    VotingSettings {
                        result: 0,
                        voting_duration: duration,
                        submitted_height: block_number,
                        voted: 0,
                        voters_number: all,
                    },
                );
            } else {
                <ActiveVotings<T>>::insert(
                    subject,
                    VotingSettings {
                        result: 0,
                        voting_duration: duration,
                        submitted_height: block_number,
                        voted: 0,
                        voters_number: 0,
                    },
                );
            }

            Ok(())
        }

        fn create_alt_voting(
            subject: T::Hash,
            duration: T::BlockNumber,
            candidates: BTreeSet<Candidate>,
            all_voters: Option<u32>,
        ) -> Result<(), Error<T>> {
            ensure!(
                <ActiveAltVoitings<T>>::get(subject) == None,
                <Error<T>>::VotingHasBeenCreated
            );

            let block_number = <frame_system::Pallet<T>>::block_number();
            if let Some(all) = all_voters {
                <ActiveAltVoitings<T>>::insert(
                    subject,
                    AltVoutingSettings {
                        voting_duration: duration,
                        submitted_height: block_number,
                        candidates,
                        voted: 0,
                        voters_number: all,
                    },
                );
            } else {
                <ActiveAltVoitings<T>>::insert(
                    subject,
                    AltVoutingSettings {
                        voting_duration: duration,
                        submitted_height: block_number,
                        candidates,
                        voted: 0,
                        voters_number: 0,
                    },
                );
            }

            Ok(())
        }

        fn create_alt_voting_list(
            subject: T::Hash,
            duration: T::BlockNumber,
            candidates: BTreeSet<Candidate>,
            winners_amount: u32,
        ) -> Result<(), Error<T>> {
            ensure!(
                <ActiveAltListVoitings<T>>::get(subject) == None,
                <Error<T>>::VotingHasBeenCreated
            );

            let block_nummber = <frame_system::Pallet<T>>::block_number();
            <ActiveAltListVoitings<T>>::insert(
                subject,
                AltVotingListSettings {
                    voting_duration: duration,
                    submitted_height: block_nummber,
                    candidates,
                    winners_amount,
                },
            );
            Ok(())
        }

        fn vote(subject: T::Hash, power: u64) -> Result<(), Error<T>> {
            match <ActiveVotings<T>>::get(subject) {
                Some(mut settings) => {
                    settings.result += power;
                    if settings.voted <= settings.voters_number {
                        settings.voted += 1;
                    }
                    <ActiveVotings<T>>::insert(subject, settings);
                    Ok(())
                }
                None => Err(<Error<T>>::VotingSubjectDoesNotExist),
            }
        }

        fn alt_vote(
            subject: T::Hash,
            account_id: T::AccountId,
            ballot: AltVote,
            power: u64,
        ) -> Result<(), Error<T>> {
            match <ActiveAltVoitings<T>>::get(subject) {
                Some(mut settings) => {
                    let mut ballots_list = <BallotsStorage<T>>::get(subject);
                    ballots_list.insert(account_id, (ballot, power));
                    <BallotsStorage<T>>::insert(subject, ballots_list);
                    if settings.voted <= settings.voters_number {
                        settings.voted += 1;
                        <ActiveAltVoitings<T>>::insert(subject, settings);
                    }
                    Ok(())
                }
                None => Err(<Error<T>>::VotingSubjectDoesNotExist),
            }
        }

        fn alt_vote_list(
            subject: T::Hash,
            account_id: T::AccountId,
            ballot: AltVote,
            power: u64,
        ) -> Result<(), Error<T>> {
            match <ActiveAltListVoitings<T>>::get(subject) {
                Some(_) => {
                    let mut ballots_list = <BallotsStorage<T>>::get(subject);
                    ballots_list.insert(account_id, (ballot, power));
                    <BallotsStorage<T>>::insert(subject, ballots_list);
                    Ok(())
                }
                None => Err(<Error<T>>::VotingSubjectDoesNotExist),
            }
        }
    }
}

pub trait VotingTrait<T: Config> {
    fn create_voting(
        subject: T::Hash,
        duration: T::BlockNumber,
        all_voters: Option<u32>,
    ) -> Result<(), Error<T>>;

    fn create_alt_voting(
        subject: T::Hash,
        duration: T::BlockNumber,
        candidates: BTreeSet<Candidate>,
        all_voters: Option<u32>,
    ) -> Result<(), Error<T>>;

    fn create_alt_voting_list(
        subject: T::Hash,
        duration: T::BlockNumber,
        candidates: BTreeSet<Candidate>,
        winners_amount: u32,
    ) -> Result<(), Error<T>>;

    fn vote(subject: T::Hash, power: u64) -> Result<(), Error<T>>;

    fn alt_vote(
        subject: T::Hash,
        account_id: T::AccountId,
        ballot: AltVote,
        power: u64,
    ) -> Result<(), Error<T>>;

    fn alt_vote_list(
        subject: T::Hash,
        account_id: T::AccountId,
        ballot: AltVote,
        power: u64,
    ) -> Result<(), Error<T>>;
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct VotingSettings<BlockNumber> {
    pub result: u64,
    pub voting_duration: BlockNumber,
    pub submitted_height: BlockNumber,
    pub voted: u32,
    pub voters_number: u32,
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct AltVoutingSettings<BlockNumber> {
    pub voting_duration: BlockNumber,
    pub submitted_height: BlockNumber,
    pub candidates: BTreeSet<Candidate>,
    pub voted: u32,
    pub voters_number: u32,
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct AltVotingListSettings<BlockNumber> {
    pub voting_duration: BlockNumber,
    pub submitted_height: BlockNumber,
    pub candidates: BTreeSet<Candidate>,
    pub winners_amount: u32,
}

pub type Candidate = Vec<u8>;

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct AltVote {
    pub content: VecDeque<Candidate>,
}

impl AltVote {
    pub fn new(content: VecDeque<Candidate>) -> Self {
        Self { content }
    }
}
