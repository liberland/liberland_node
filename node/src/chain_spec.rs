use liberland_node_runtime::{
    opaque::SessionKeys, AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
    IdentityPalletConfig, SessionConfig, Signature, StakingPalletConfig, SudoConfig, SystemConfig,
    WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};
use std::str::FromStr;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AuraId, GrandpaId) {
    (
        get_account_id_from_seed::<sr25519::Public>(s),
        get_from_seed::<AuraId>(s),
        get_from_seed::<GrandpaId>(s),
    )
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        pallet_session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        SessionKeys {
                            grandpa: x.2.clone(),
                            aura: x.1.clone(),
                        },
                    )
                })
                .collect::<Vec<_>>(),
        },
        pallet_staking: StakingPalletConfig {
            validator_count: initial_authorities.len() as u32,
            minimum_validator_count: initial_authorities.len() as u32,
            invulnerables: initial_authorities
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<_>>(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        },
        // Liberland config
        pallet_identity: IdentityPalletConfig {
            // TODO: add a constant variable
            citizens: vec![
                (
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    [
                        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                (
                    AccountId::from_str("5G3uZjEpvNAQ6U2eUjnMb66B8g6d8wyB68x6CfkRPNcno8eR")
                        .unwrap(),
                    [
                        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                (
                    AccountId::from_str("5Ct2J6wD3NHqicz9LdiyYwHWMHLwSa4YrndYJtr6fAKRJJvT")
                        .unwrap(),
                    [
                        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                (
                    AccountId::from_str("5FX5sQnbXXN9A1fM7PkSvwYU33NotLuNamM4Gjoht2vZZgBj")
                        .unwrap(),
                    [
                        4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                (
                    AccountId::from_str("5Ge179zgEVKbxYZ64L7Jkg7Li2Cw7bg5yAGWXqacoYS561xG")
                        .unwrap(),
                    [
                        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                (
                    AccountId::from_str("5EppCbmVvdjGoEU9nPzMEv5GmZGdfuBDHVZTg4nZvpe8S2SZ")
                        .unwrap(),
                    [
                        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                (
                    AccountId::from_str("5HVSKFnWm29YfFvPt3DVdHcknSzcwhFspFijHRYTFxodGs47")
                        .unwrap(),
                    [
                        7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                (
                    AccountId::from_str("5DXqJcioMQ8bVznqhAySoB5Yit1YcsFrrJmjKW9bRtXvpcEd")
                        .unwrap(),
                    [
                        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                (
                    AccountId::from_str("5FnhdLrXpChsDAN82NWNz1htFnnp84cbVBx5ZmfTLip5Fo2p")
                        .unwrap(),
                    [
                        9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                (
                    AccountId::from_str("5DLruQutYoajW9wXNipscCAGi1Gzx9ovrA7f6MayScKstobz")
                        .unwrap(),
                    [
                        10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                (
                    AccountId::from_str("5E4UzjgcNeCZCFTBbUhfTknM6qEGPKEmomCajQNLcYbzbq7x")
                        .unwrap(),
                    [
                        11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            ],
            reviewers: vec![[
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]],
        },

        frame_system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        },
        pallet_balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        pallet_aura: AuraConfig {
            authorities: Default::default(),
        },
        pallet_grandpa: GrandpaConfig {
            authorities: Default::default(),
        },
        pallet_sudo: SudoConfig {
            // Assign network admin rights.
            key: root_key,
        },
    }
}
