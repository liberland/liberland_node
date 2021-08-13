// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Tests for the module.

use crate::slashing::do_slash;

use super::*;
use frame_support::{
    assert_noop, assert_ok,
    traits::{Currency, OnInitialize},
};
use mock::*;
use pallet_balances::{BalanceLock, Error as BalancesError, Reasons};
use sp_runtime::traits::BadOrigin;
use substrate_test_utils::assert_eq_uvec;

#[test]
fn force_unstake_works() {
    ExtBuilder::default().build_and_execute(|| {
        // Account 11 is stashed and locked, and account 10 is the controller
        assert_eq!(Staking::bonded(&11), Some(10));
        // Adds 2 slashing spans
        add_slash(&11);
        // Cant transfer
        assert_noop!(
            Balances::transfer(Origin::signed(11), 1, 10),
            BalancesError::<Test, _>::LiquidityRestrictions
        );
        // Force unstake requires root.
        assert_noop!(Staking::force_unstake(Origin::signed(11), 11, 2), BadOrigin);
        // Force unstake needs correct number of slashing spans (for weight calculation)
        assert_noop!(
            Staking::force_unstake(Origin::root(), 11, 0),
            Error::<Test>::IncorrectSlashingSpans
        );
        // We now force them to unstake
        assert_ok!(Staking::force_unstake(Origin::root(), 11, 2));
        // No longer bonded.
        assert_eq!(Staking::bonded(&11), None);
        // Transfer works.
        assert_ok!(Balances::transfer(Origin::signed(11), 1, 10));
    });
}

#[test]
fn kill_stash_works() {
    ExtBuilder::default().build_and_execute(|| {
        // Account 11 is stashed and locked, and account 10 is the controller
        assert_eq!(Staking::bonded(&11), Some(10));
        // Adds 2 slashing spans
        add_slash(&11);
        // Only can kill a stash account
        assert_noop!(Staking::kill_stash(&12, 0), Error::<Test>::NotStash);
        // Respects slashing span count
        assert_noop!(
            Staking::kill_stash(&11, 0),
            Error::<Test>::IncorrectSlashingSpans
        );
        // Correct inputs, everything works
        assert_ok!(Staking::kill_stash(&11, 2));
        // No longer bonded.
        assert_eq!(Staking::bonded(&11), None);
    });
}

#[test]
fn basic_setup_works() {
    // Verifies initial conditions of mock
    ExtBuilder::default().build_and_execute(|| {
        // Account 11 is stashed and locked, and account 10 is the controller
        assert_eq!(Staking::bonded(&11), Some(10));
        // Account 21 is stashed and locked, and account 20 is the controller
        assert_eq!(Staking::bonded(&21), Some(20));
        // Account 1 is not a stashed
        assert_eq!(Staking::bonded(&1), None);

        // Account 10 controls the stash from account 11, which is 100 * balance_factor units
        assert_eq!(
            Staking::ledger(&10),
            Some(StakingLedger {
                stash: 11,
                total: 1000,
                active: 1000,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 1000,
                liber_amount: 0,
            })
        );
        // Account 20 controls the stash from account 21, which is 200 * balance_factor units
        assert_eq!(
            Staking::ledger(&20),
            Some(StakingLedger {
                stash: 21,
                total: 1000,
                active: 1000,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 1000,
                liber_amount: 0,
            })
        );
        // Account 1 does not control any stash
        assert_eq!(Staking::ledger(&1), None);

        // ValidatorPrefs are default
        assert_eq_uvec!(
            <Validators<Test>>::iter().collect::<Vec<_>>(),
            vec![
                (31, ValidatorPrefs::default()),
                (21, ValidatorPrefs::default()),
                (11, ValidatorPrefs::default())
            ]
        );

        assert_eq!(
            Staking::ledger(100),
            Some(StakingLedger {
                stash: 101,
                total: 500,
                active: 500,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 500,
                liber_amount: 0,
            })
        );
        assert_eq!(Staking::nominators(101).unwrap().targets, vec![11, 21]);

        assert_eq!(
            Staking::eras_stakers(Staking::active_era().unwrap().index, 11),
            Exposure {
                total: 1125,
                own: 1000,
                others: vec![IndividualExposure {
                    who: 101,
                    value: 125
                }]
            },
        );
        assert_eq!(
            Staking::eras_stakers(Staking::active_era().unwrap().index, 21),
            Exposure {
                total: 1375,
                own: 1000,
                others: vec![IndividualExposure {
                    who: 101,
                    value: 375
                }]
            },
        );

        // initial total stake = 1125 + 1375
        assert_eq!(
            Staking::eras_total_stake(Staking::active_era().unwrap().index),
            2500
        );

        // The number of validators required.
        assert_eq!(Staking::validator_count(), 2);

        // Initial Era and session
        assert_eq!(Staking::active_era().unwrap().index, 0);

        // Account 10 has `balance_factor` free balance
        assert_eq!(Balances::free_balance(10), 1);
        assert_eq!(Balances::free_balance(10), 1);

        // New era is not being forced
        assert_eq!(Staking::force_era(), Forcing::NotForcing);
    });
}

#[test]
fn liber_bond_extra_test() {
    ExtBuilder::default().build_and_execute(|| {
        Staking::bond(Origin::signed(1), 1, 5, RewardDestination::Controller).unwrap();
        assert_eq!(Staking::bonded(&1).unwrap(), 1);

        assert_eq!(
            Balances::locks(&1),
            [BalanceLock {
                id: POLKADOT_STAKING_ID,
                amount: 5,
                reasons: Reasons::All
            }]
        );

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 5,
                active: 5,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 5,
                liber_amount: 0,
            }
        );
        assert_ok!(Staking::liberland_bond_extra(Origin::signed(1), 5));

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 10,
                active: 10,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 5,
                liber_amount: 5,
            }
        );

        assert_eq!(
            Balances::locks(&1),
            [
                BalanceLock {
                    id: POLKADOT_STAKING_ID,
                    amount: 5,
                    reasons: Reasons::All
                },
                BalanceLock {
                    id: LIBERLAND_STAKING_ID,
                    amount: 5,
                    reasons: Reasons::All
                }
            ]
        );

        assert_ok!(Staking::unbond(Origin::signed(1), 5));

        assert_eq!(
            Balances::locks(&1),
            [
                BalanceLock {
                    id: POLKADOT_STAKING_ID,
                    amount: 5,
                    reasons: Reasons::All
                },
                BalanceLock {
                    id: LIBERLAND_STAKING_ID,
                    amount: 5,
                    reasons: Reasons::All
                }
            ]
        );
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 10,
                active: 5,
                unlocking: vec![UnlockChunk {
                    staking_id: POLKADOT_STAKING_ID,
                    value: 5,
                    era: 3
                },],
                claimed_rewards: vec![],
                polka_amount: 5,
                liber_amount: 5,
            }
        );
    });
}

#[test]
fn mirrored_liber_bond_extra_test() {
    ExtBuilder::default().build_and_execute(|| {
        Staking::liberland_bond(Origin::signed(1), 1, 5, RewardDestination::Controller).unwrap();

        assert_eq!(
            Balances::locks(&1),
            [BalanceLock {
                id: LIBERLAND_STAKING_ID,
                amount: 5,
                reasons: Reasons::All
            }]
        );

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 5,
                active: 5,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 5,
            }
        );
        assert_ok!(Staking::bond_extra(Origin::signed(1), 5));

        assert_eq!(
            Balances::locks(&1),
            [
                BalanceLock {
                    id: LIBERLAND_STAKING_ID,
                    amount: 5,
                    reasons: Reasons::All
                },
                BalanceLock {
                    id: POLKADOT_STAKING_ID,
                    amount: 5,
                    reasons: Reasons::All
                }
            ]
        );

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 10,
                active: 10,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 5,
                liber_amount: 5,
            }
        );

        assert_ok!(Staking::unbond(Origin::signed(1), 5));
        assert_eq!(
            Balances::locks(&1),
            [
                BalanceLock {
                    id: LIBERLAND_STAKING_ID,
                    amount: 5,
                    reasons: Reasons::All
                },
                BalanceLock {
                    id: POLKADOT_STAKING_ID,
                    amount: 5,
                    reasons: Reasons::All
                }
            ]
        );

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 10,
                active: 5,
                unlocking: vec![UnlockChunk {
                    staking_id: POLKADOT_STAKING_ID,
                    value: 5,
                    era: 3
                },],
                claimed_rewards: vec![],
                polka_amount: 5,
                liber_amount: 5,
            }
        );
    });
}

#[test]
fn doubule_bond_extra_test() {
    ExtBuilder::default().build_and_execute(|| {
        Staking::liberland_bond(Origin::signed(1), 1, 4, RewardDestination::Controller).unwrap();
        assert_eq!(Staking::bonded(&1).unwrap(), 1);

        assert_eq!(
            Balances::locks(&1),
            [BalanceLock {
                id: LIBERLAND_STAKING_ID,
                amount: 4,
                reasons: Reasons::All
            }]
        );

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 4,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );
        assert_ok!(Staking::bond_extra(Origin::signed(1), 3));

        assert_eq!(
            Balances::locks(&1),
            [
                BalanceLock {
                    id: LIBERLAND_STAKING_ID,
                    amount: 4,
                    reasons: Reasons::All
                },
                BalanceLock {
                    id: POLKADOT_STAKING_ID,
                    amount: 3,
                    reasons: Reasons::All
                }
            ]
        );

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 7,
                active: 7,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 3,
                liber_amount: 4,
            }
        );

        assert_ok!(Staking::liberland_bond_extra(Origin::signed(1), 3));

        assert_eq!(
            Balances::locks(&1),
            [
                BalanceLock {
                    id: LIBERLAND_STAKING_ID,
                    amount: 7,
                    reasons: Reasons::All
                },
                BalanceLock {
                    id: POLKADOT_STAKING_ID,
                    amount: 3,
                    reasons: Reasons::All
                }
            ]
        );

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 10,
                active: 10,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 3,
                liber_amount: 7,
            }
        );

        // Account `1` has staked all available amount, so bond_extra should not do anything
        assert_ok!(Staking::liberland_bond_extra(Origin::signed(1), 3));

        assert_eq!(
            Balances::locks(&1),
            [
                BalanceLock {
                    id: LIBERLAND_STAKING_ID,
                    amount: 7,
                    reasons: Reasons::All
                },
                BalanceLock {
                    id: POLKADOT_STAKING_ID,
                    amount: 3,
                    reasons: Reasons::All
                }
            ]
        );

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 10,
                active: 10,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 3,
                liber_amount: 7,
            }
        );

        assert_ok!(Staking::unbond(Origin::signed(1), 10));

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 10,
                active: 7,
                unlocking: vec![UnlockChunk {
                    staking_id: POLKADOT_STAKING_ID,
                    value: 3,
                    era: 3
                }],
                claimed_rewards: vec![],
                polka_amount: 3,
                liber_amount: 7,
            }
        );

        mock::start_active_era(3);

        assert_ok!(Staking::withdraw_unbonded(Origin::signed(1), 10));

        assert_eq!(
            Balances::locks(&1),
            [BalanceLock {
                id: LIBERLAND_STAKING_ID,
                amount: 7,
                reasons: Reasons::All
            }]
        );

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 7,
                active: 7,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 7,
            }
        );
    });
}

#[test]
fn default_bond_extra_test() {
    ExtBuilder::default().build_and_execute(|| {
        Staking::bond(Origin::signed(1), 1, 5, RewardDestination::Controller).unwrap();
        assert_eq!(Staking::bonded(&1).unwrap(), 1);

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 5,
                active: 5,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 5,
                liber_amount: 0,
            }
        );
        assert_eq!(
            Balances::locks(&1),
            [BalanceLock {
                id: POLKADOT_STAKING_ID,
                amount: 5,
                reasons: Reasons::All
            }]
        );
        assert_ok!(Staking::bond_extra(Origin::signed(1), 5));

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 10,
                active: 10,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 10,
                liber_amount: 0,
            }
        );

        assert_eq!(
            Balances::locks(&1),
            [BalanceLock {
                id: POLKADOT_STAKING_ID,
                amount: 10,
                reasons: Reasons::All
            }]
        );

        assert_ok!(Staking::unbond(Origin::signed(1), 5));

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 10,
                active: 5,
                unlocking: vec![UnlockChunk {
                    staking_id: POLKADOT_STAKING_ID,
                    value: 5,
                    era: 3
                }],
                claimed_rewards: vec![],
                polka_amount: 10,
                liber_amount: 0,
            }
        );

        assert_eq!(
            Balances::locks(&1),
            [BalanceLock {
                id: POLKADOT_STAKING_ID,
                amount: 10,
                reasons: Reasons::All
            }]
        );
        mock::start_active_era(3);

        assert_ok!(Staking::withdraw_unbonded(Origin::signed(1), 10));

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 5,
                active: 5,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 5,
                liber_amount: 0,
            }
        );

        assert_eq!(
            Balances::locks(&1),
            [BalanceLock {
                id: POLKADOT_STAKING_ID,
                amount: 5,
                reasons: Reasons::All
            }]
        );
    });
    // Particularly when she votes and the candidate is elected.
    ExtBuilder::default()
        .validator_count(3)
        .existential_deposit(5)
        .nominate(false)
        .minimum_validator_count(1)
        .build()
        .execute_with(|| {
            // Can't bond with 1
            assert_noop!(
                Staking::liberland_bond(Origin::signed(1), 2, 1, RewardDestination::Controller),
                Error::<Test>::InsufficientValue,
            );
            // bonded with absolute minimum value possible.
            assert_ok!(Staking::liberland_bond(
                Origin::signed(1),
                2,
                5,
                RewardDestination::Controller
            ));
            assert_eq!(Balances::locks(&1)[0].amount, 5);

            // unbonding even 1 will cause all to be unbonded.
            assert_ok!(Staking::unbond(Origin::signed(2), 1));
            assert_eq!(
                Staking::ledger(2),
                Some(StakingLedger {
                    stash: 1,
                    active: 5,
                    total: 5,
                    unlocking: vec![],
                    claimed_rewards: vec![],
                    polka_amount: 0,
                    liber_amount: 5
                })
            );

            mock::start_active_era(1);
            mock::start_active_era(2);

            // not yet removed.
            assert_ok!(Staking::withdraw_unbonded(Origin::signed(2), 0));
            assert!(Staking::ledger(2).is_some());
            assert_eq!(Balances::locks(&1)[0].amount, 5);

            mock::start_active_era(3);

            // poof. Account 1 is removed from the staking system.
            assert_ok!(Staking::withdraw_unbonded(Origin::signed(2), 0));
            assert!(Staking::ledger(2).is_some());
            assert_eq!(Balances::locks(&1).len(), 1);
        });
}

#[test]
fn liber_bond_extra_works() {
    // Tests that extra `free_balance` in the stash can be added to stake
    // NOTE: this tests only verifies `StakingLedger` for correct updates
    // See `bond_extra_and_withdraw_unbonded_works` for more details and updates on `Exposure`.
    ExtBuilder::default().build_and_execute(|| {
        // Check that account 10 is a validator
        assert!(<Validators<Test>>::contains_key(11));
        // Check that account 10 is bonded to account 11
        assert_eq!(Staking::bonded(&11), Some(10));
        // Check how much is at stake
        assert_eq!(
            Staking::ledger(&10),
            Some(StakingLedger {
                stash: 11,
                total: 1000,
                active: 1000,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 1000,
                liber_amount: 0,
            })
        );

        // Give account 11 some large free balance greater than total
        let _ = Balances::make_free_balance_be(&11, 1000000);

        // Call the bond_extra function from controller, add only 100
        assert_ok!(Staking::liberland_bond_extra(Origin::signed(11), 100));
        // There should be 100 more `total` and `active` in the ledger
        assert_eq!(
            Staking::ledger(&10),
            Some(StakingLedger {
                stash: 11,
                total: 1000 + 100,
                active: 1000 + 100,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 1000,
                liber_amount: 100,
            })
        );

        // Call the bond_extra function with a large number, should handle it
        assert_ok!(Staking::liberland_bond_extra(
            Origin::signed(11),
            Balance::max_value()
        ));
        // The full amount of the funds should now be in the total and active
        assert_eq!(
            Staking::ledger(&10),
            Some(StakingLedger {
                stash: 11,
                total: 1000000,
                active: 1000000,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 1000,
                liber_amount: 999000,
            })
        );
    });
}

#[test]
fn basic_liber_unstaking_test_v2() {
    ExtBuilder::default().build_and_execute(|| {
        Staking::liberland_bond(Origin::signed(1), 1, 4, RewardDestination::Controller).unwrap();
        assert_eq!(Staking::bonded(&1).unwrap(), 1);

        assert_eq!(
            Balances::locks(&1),
            [BalanceLock {
                id: LIBERLAND_STAKING_ID,
                amount: 4,
                reasons: Reasons::All
            }]
        );

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 4,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );
        assert_ok!(Staking::liberland_unbond_on(Origin::signed(1)));
        assert_eq!(Staking::liber_reqests(&1).unwrap(), 1);

        Staking::on_initialize(58);
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 3,
                unlocking: vec![UnlockChunk {
                    staking_id: LIBERLAND_STAKING_ID,
                    value: 1,
                    era: 3
                }],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );
        Staking::on_initialize(58 + 28);

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 2,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 3
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 3
                    }
                ],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );
        //dbg!(Balances::locks(&1));
        dbg!(Staking::active_era());
        dbg!(System::block_number());

        mock::start_active_era(3);
        dbg!(Staking::active_era());
        dbg!(System::block_number());
        assert_ok!(Staking::withdraw_unbonded(Origin::signed(1), 3));
        //dbg!(Staking::ledger(&1));
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 1, //FIXME it is not true should be 2
                active: 0,
                unlocking: vec![UnlockChunk {
                    staking_id: LIBERLAND_STAKING_ID,
                    value: 1,
                    era: 5
                }], //FIXME it is not true UnlockChunk should be removed
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 1, //FIXME it is not true should be 2
            }
        );
        //dbg!(Staking::ledger(&1));

        //dbg!(Balances::locks(&1));
        assert_eq!(
            Balances::locks(&1),
            [BalanceLock {
                id: LIBERLAND_STAKING_ID,
                amount: 1, //FIXME it is not true should be 2
                reasons: Reasons::All
            }]
        );
    });
}

#[test]
fn liberland_unbond_test_with_era() {
    ExtBuilder::default().build_and_execute(|| {
        Staking::liberland_bond(Origin::signed(1), 1, 4, RewardDestination::Controller).unwrap();
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 4,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );
        mock::start_active_era(1);
        assert_ok!(Staking::liberland_unbond_on(Origin::signed(1)));
        assert_eq!(Staking::liber_reqests(1).unwrap(), 15);

        mock::start_active_era(3 + 1);

        /*
        First chunk generated in the 16th block     (16 - 1 - 15)mod 28 = 0
        Second chunk generated in the 44th block    (44 - 1 - 15)mod 28 = 0
        */
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 2,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 6,
                    },
                ],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );

        mock::start_active_era(3 + 3 + 1);
        /*
        The third chunk generated in the 72th block     (72 - 1 - 15)mod 28 = 0
        Fourth chunk generated in the 100th block       (100 - 1 - 15)mod 28 = 0
        */
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 0,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 6,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 8,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 10,
                        value: 1
                    }
                ],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );
        mock::start_active_era(9 + 1);
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 0,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 6,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 8,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 10,
                        value: 1
                    }
                ],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );
        assert_ok!(Staking::withdraw_unbonded(Origin::signed(1), 3));
        assert_eq!(Staking::ledger(&1), None);
        assert_eq!(Staking::liber_reqests(1), None);
    });
}

#[test]
fn liberland_and_polka_unbond_test_wit_era() {
    ExtBuilder::default().build_and_execute(|| {
        Staking::liberland_bond(Origin::signed(1), 1, 4, RewardDestination::Controller).unwrap();
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 4,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );
        mock::start_active_era(1);
        assert_ok!(Staking::liberland_unbond_on(Origin::signed(1)));
        assert_eq!(Staking::liber_reqests(1).unwrap(), 15);

        assert_ok!(Staking::bond_extra(Origin::signed(1), 3));

        mock::start_active_era(3 + 1);

        /*
        First chunk generated in the 16th block     (16 - 1 - 15)mod 28 = 0
        Second chunk generated in the 44th block    (44 - 1 - 15)mod 28 = 0
        */
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 7,
                active: 5,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 6,
                    }
                ],
                claimed_rewards: vec![],
                polka_amount: 3,
                liber_amount: 4,
            }
        );

        mock::start_active_era(3 + 3 + 1);
        assert_ok!(Staking::unbond(Origin::signed(1), 3));
        /*
        The third chunk generated in the 72th block     (72 - 1 - 15)mod 28 = 0
        Fourth chunk generated in the 100th block       (100 - 1 - 15)mod 28 = 0
        */
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 7,
                active: 0,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 6,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 8,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 10,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: POLKADOT_STAKING_ID,
                        era: 10,
                        value: 3,
                    }
                ],
                claimed_rewards: vec![],
                polka_amount: 3,
                liber_amount: 4,
            }
        );
        mock::start_active_era(9 + 1);
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 7,
                active: 0,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 6,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 8,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 10,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: POLKADOT_STAKING_ID,
                        era: 10,
                        value: 3,
                    }
                ],
                claimed_rewards: vec![],
                polka_amount: 3,
                liber_amount: 4,
            }
        );

        assert_ok!(Staking::withdraw_unbonded(Origin::signed(1), 3));
        assert_eq!(Staking::ledger(&1), None);
    });
}

#[test]
fn liberland_and_polka_unbond_piecemeal_test_wit_era() {
    ExtBuilder::default().build_and_execute(|| {
        Staking::liberland_bond(Origin::signed(1), 1, 4, RewardDestination::Controller).unwrap();
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 4,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );
        assert_ok!(Staking::bond_extra(Origin::signed(1), 3));
        assert_ok!(Staking::unbond(Origin::signed(1), 3));
        mock::start_active_era(1);
        assert_ok!(Staking::liberland_unbond_on(Origin::signed(1)));
        assert_eq!(Staking::liber_reqests(1).unwrap(), 15);

        mock::start_active_era(3 + 1);

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 7,
                active: 2,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: POLKADOT_STAKING_ID,
                        era: 3,
                        value: 3
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 6,
                    }
                ],
                claimed_rewards: vec![],
                polka_amount: 3,
                liber_amount: 4,
            }
        );

        assert_ok!(Staking::withdraw_unbonded(Origin::signed(1), 3));

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 3,
                active: 2,
                unlocking: vec![UnlockChunk {
                    staking_id: LIBERLAND_STAKING_ID,
                    value: 1,
                    era: 6,
                }],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 3,
            }
        );

        mock::start_active_era(3 + 3 + 1);
        assert_ok!(Staking::unbond(Origin::signed(1), 3));

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 3,
                active: 0,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 6,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 8,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 10,
                        value: 1
                    },
                ],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 3,
            }
        );
        mock::start_active_era(9 + 1);
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 3,
                active: 0,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 6,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 8,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 10,
                        value: 1
                    },
                ],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 3,
            }
        );

        assert_ok!(Staking::withdraw_unbonded(Origin::signed(1), 3));
        assert_eq!(Staking::ledger(&1), None);
    });
}

#[test]
fn liberlnd_andbond_test_with_cansle_reqest() {
    ExtBuilder::default().build_and_execute(|| {
        Staking::liberland_bond(Origin::signed(1), 1, 10, RewardDestination::Controller).unwrap();
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 10,
                active: 10,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 10,
            }
        );

        assert_ok!(Staking::liberland_unbond_on(Origin::signed(1)));
        mock::start_active_era(3);
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 10,
                active: 8,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 3,
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 5,
                    },
                ],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 10,
            }
        );

        assert_ok!(Staking::liberland_unbond_off(Origin::signed(1)));
        mock::start_active_era(6);
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 10,
                active: 8,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 3,
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 5,
                    },
                ],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 10,
            }
        );
        assert_ok!(Staking::withdraw_unbonded(Origin::signed(1), 2));
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 8,
                active: 8,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 8,
            }
        );
    });
}

#[test]
fn make_payout_base_test() {
    ExtBuilder::default()
        .has_stakers(false)
        .build_and_execute(|| {
            Staking::liberland_bond(Origin::signed(1), 1, 5, RewardDestination::LiberStaked)
                .unwrap();

            let _t = Staking::make_payout(&1, 5).unwrap();
            assert_eq!(
                Staking::ledger(&1).unwrap(),
                StakingLedger {
                    stash: 1,
                    total: 10,
                    active: 10,
                    unlocking: vec![],
                    claimed_rewards: vec![],
                    polka_amount: 0,
                    liber_amount: 10,
                }
            );
            Staking::bond_extra(Origin::signed(1), 5).unwrap();

            assert_eq!(
                Staking::ledger(&1).unwrap(),
                StakingLedger {
                    stash: 1,
                    total: 15,
                    active: 15,
                    unlocking: vec![],
                    claimed_rewards: vec![],
                    polka_amount: 5,
                    liber_amount: 10,
                }
            );
            let _t = Staking::make_payout(&1, 5).unwrap();

            assert_eq!(
                Staking::ledger(&1).unwrap(),
                StakingLedger {
                    stash: 1,
                    total: 20,
                    active: 20,
                    unlocking: vec![],
                    claimed_rewards: vec![],
                    polka_amount: 5,
                    liber_amount: 15,
                }
            );
        });
}

#[test]
fn rebond_works_with_liber_staking() {
    ExtBuilder::default().build_and_execute(|| {
        Staking::liberland_bond(Origin::signed(1), 1, 4, RewardDestination::Controller).unwrap();
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 4,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );
        assert_ok!(Staking::bond_extra(Origin::signed(1), 3));
        assert_ok!(Staking::unbond(Origin::signed(1), 3));
        mock::start_active_era(1);
        assert_ok!(Staking::liberland_unbond_on(Origin::signed(1)));
        assert_eq!(Staking::liber_reqests(1).unwrap(), 15);

        mock::start_active_era(3 + 1);

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 7,
                active: 2,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: POLKADOT_STAKING_ID,
                        era: 3,
                        value: 3
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 6,
                    }
                ],
                claimed_rewards: vec![],
                polka_amount: 3,
                liber_amount: 4,
            }
        );

        assert_ok!(Staking::rebond(Origin::signed(1), 5));
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 7,
                active: 5,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 6,
                    }
                ],
                claimed_rewards: vec![],
                polka_amount: 3,
                liber_amount: 4,
            }
        );
    });
}

#[test]
fn polka_slash_tests() {
    ExtBuilder::default().build_and_execute(|| {
        let mut te_l = StakingLedger {
            stash: 1,
            total: 7,
            active: 2,
            unlocking: vec![
                UnlockChunk {
                    staking_id: POLKADOT_STAKING_ID,
                    era: 3,
                    value: 3,
                },
                UnlockChunk {
                    staking_id: LIBERLAND_STAKING_ID,
                    era: 4,
                    value: 1,
                },
                UnlockChunk {
                    staking_id: LIBERLAND_STAKING_ID,
                    value: 1,
                    era: 6,
                },
            ],
            claimed_rewards: vec![],
            polka_amount: 3,
            liber_amount: 4,
        };
        te_l.polka_slash(2_u32, 1_u32);
        assert_eq!(
            te_l,
            StakingLedger {
                stash: 1,
                total: 4,
                active: 2,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1,
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 6,
                    },
                ],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            },
        );

        let mut te_l_2 = StakingLedger {
            stash: 1,
            total: 7,
            active: 4,
            unlocking: vec![
                UnlockChunk {
                    staking_id: POLKADOT_STAKING_ID,
                    era: 3,
                    value: 3,
                },
                UnlockChunk {
                    staking_id: LIBERLAND_STAKING_ID,
                    era: 4,
                    value: 1,
                },
                UnlockChunk {
                    staking_id: LIBERLAND_STAKING_ID,
                    value: 1,
                    era: 6,
                },
            ],
            claimed_rewards: vec![],
            polka_amount: 5,
            liber_amount: 4,
        };

        te_l_2.polka_slash(10_u32, 1_u32);
        assert_eq!(
            te_l_2,
            StakingLedger {
                stash: 1,
                total: 2,
                active: 2,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1,
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 6,
                    },
                ],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );

        let mut te_l_3 = StakingLedger {
            stash: 1,
            total: 12,
            active: 4,
            unlocking: vec![
                UnlockChunk {
                    staking_id: POLKADOT_STAKING_ID,
                    era: 3,
                    value: 3,
                },
                UnlockChunk {
                    staking_id: POLKADOT_STAKING_ID,
                    era: 3,
                    value: 3,
                },
                UnlockChunk {
                    staking_id: LIBERLAND_STAKING_ID,
                    era: 4,
                    value: 1,
                },
                UnlockChunk {
                    staking_id: LIBERLAND_STAKING_ID,
                    value: 1,
                    era: 6,
                },
            ],
            claimed_rewards: vec![],
            polka_amount: 8,
            liber_amount: 4,
        };

        te_l_3.polka_slash(4_u32, 1_u32);
        assert_eq!(
            te_l_3,
            StakingLedger {
                stash: 1,
                total: 7,
                active: 2,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: POLKADOT_STAKING_ID,
                        era: 3,
                        value: 3,
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1,
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 6,
                    },
                ],
                claimed_rewards: vec![],
                polka_amount: 3,
                liber_amount: 4,
            }
        );
    });
}

#[test]
fn do_payout_test() {
    ExtBuilder::default().build_and_execute(|| {
        Staking::liberland_bond(Origin::signed(1), 1, 4, RewardDestination::Controller).unwrap();
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 4,
                unlocking: vec![],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );
        assert_ok!(Staking::bond_extra(Origin::signed(1), 3));
        assert_ok!(Staking::unbond(Origin::signed(1), 3));
        mock::start_active_era(1);
        assert_ok!(Staking::liberland_unbond_on(Origin::signed(1)));
        assert_eq!(Staking::liber_reqests(1).unwrap(), 15);

        mock::start_active_era(3 + 1);

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 7,
                active: 2,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: POLKADOT_STAKING_ID,
                        era: 3,
                        value: 3
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 6,
                    }
                ],
                claimed_rewards: vec![],
                polka_amount: 3,
                liber_amount: 4,
            }
        );
        let mut negative_imbalnce = NegativeImbalanceOf::<Test>::zero();
        // minimum balance in do_slash = 1
        do_slash::<Test>(&1, 2, &mut 0, &mut negative_imbalnce);
        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 2,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 6,
                    }
                ],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );

        do_slash::<Test>(&1, 100, &mut 0, &mut negative_imbalnce);

        assert_eq!(
            Staking::ledger(&1).unwrap(),
            StakingLedger {
                stash: 1,
                total: 4,
                active: 2,
                unlocking: vec![
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        era: 4,
                        value: 1
                    },
                    UnlockChunk {
                        staking_id: LIBERLAND_STAKING_ID,
                        value: 1,
                        era: 6,
                    }
                ],
                claimed_rewards: vec![],
                polka_amount: 0,
                liber_amount: 4,
            }
        );
    });
}
