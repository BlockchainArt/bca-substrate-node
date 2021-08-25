use bca_runtime::{
    AccountId, AuraConfig, Balance, BalancesConfig, BcaConfig, GenesisConfig, GrandpaConfig,
    Signature, SudoConfig, SystemConfig, DOLLARS, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

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
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
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
                BcaTokenDistribution {
                    early_investors: get_account_id_from_seed::<sr25519::Public>("Alice"),
                    team: get_account_id_from_seed::<sr25519::Public>("Bob"),
                    token_sale: frame_benchmarking::whitelisted_caller(),
                    foundation: get_account_id_from_seed::<sr25519::Public>("Dave"),
                },
                // BCA prints
                vec![],
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
                BcaTokenDistribution {
                    early_investors: get_account_id_from_seed::<sr25519::Public>("Alice"),
                    team: get_account_id_from_seed::<sr25519::Public>("Bob"),
                    token_sale: get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    foundation: get_account_id_from_seed::<sr25519::Public>("Dave"),
                },
                // BCA prints
                vec![
                    (
                        // artist
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        // collections
                        vec![(
                            // Currents
                            // https://bca.mypinata.cloud/ipfs/QmWRhmiesgsHbExoBJ38yyQ3Ti4W57ADLvRrm5bJvMMXVJ
                            // https://polkascan.io/kusama/transaction/0x5f63a0b4b9cb19bc6e88dddfff139c75540e1959b6d2b0a33ae06077a3fec246
                            b"QmWRhmiesgsHbExoBJ38yyQ3Ti4W57ADLvRrm5bJvMMXVJ".to_vec(),
                            pallet_bca::Edition {
                                proofs: 2,
                                prints: 5,
                            },
                            // prints
                            vec![
                                // // print
                                // (false, get_account_id_from_seed::<sr25519::Public>("Bob")),
                                // // proof
                                // (true, get_account_id_from_seed::<sr25519::Public>("Alice")),
                            ],
                        )],
                    ),
                    (
                        // artist
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        // collections
                        vec![(
                            // PRIMITIVE
                            // https://bca.mypinata.cloud/ipfs/QmRoZqVTDyfJbSLLYupmmUgUdiooBk9Twk69BCMgcYwQCw
                            // https://polkascan.io/kusama/transaction/0x949b0eb56438b56dbaf964050db0a35efc74ad47767ac11fece8e968f2c124f8
                            b"QmRoZqVTDyfJbSLLYupmmUgUdiooBk9Twk69BCMgcYwQCw".to_vec(),
                            pallet_bca::Edition {
                                proofs: 2,
                                prints: 1,
                            },
                            // prints
                            vec![],
                        )],
                    ),
                    (
                        // artist
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        // collections
                        vec![(
                            // KEY
                            // https://bca.mypinata.cloud/ipfs/QmSwMS2JGCrrjhxmbmwJZn6BD7LtHFXc66Px2BEeRUYfma
                            // https://polkascan.io/kusama/transaction/0xdd41592759c17421386b6e5d57861fc2610b33b971f7d90858e3e17ce6e7abe8
                            b"QmSwMS2JGCrrjhxmbmwJZn6BD7LtHFXc66Px2BEeRUYfma".to_vec(),
                            pallet_bca::Edition {
                                proofs: 1,
                                prints: 1,
                            },
                            // prints
                            vec![],
                        )],
                    ),
                    (
                        // artist
                        get_account_id_from_seed::<sr25519::Public>("Dave"),
                        // collections
                        vec![
                            (
                                // Vorspann
                                // https://bca.mypinata.cloud/ipfs/QmVnierx98VFVPyJtEFvjxdiu4mXWtiG5VczTYisYmEZfm
                                // https://polkascan.io/kusama/transaction/0xa2a690646e247fccd2929744eb0bc2c5b3f33344abcd0564f7db90f629c8fd3c
                                b"QmVnierx98VFVPyJtEFvjxdiu4mXWtiG5VczTYisYmEZfm".to_vec(),
                                pallet_bca::Edition {
                                    proofs: 1,
                                    prints: 5,
                                },
                                // prints
                                vec![],
                            ),
                            (
                                // Hidden in plain sight
                                // https://bca.mypinata.cloud/ipfs/QmWyedWuC735rFoRBwebR4UcC4XmDcH2u7ZaGvRGcmuP2d
                                // https://polkascan.io/kusama/transaction/0x735655545f7d103f5ae6de38469aba3f43fcaabacdb88cd66ab3d9e601c196ef
                                b"QmWyedWuC735rFoRBwebR4UcC4XmDcH2u7ZaGvRGcmuP2d".to_vec(),
                                pallet_bca::Edition {
                                    proofs: 1,
                                    prints: 3,
                                },
                                // prints
                                vec![],
                            ),
                            (
                                // Smoke rings
                                // https://bca.mypinata.cloud/ipfs/QmQhXTu3UaUX8XwRGJQQ95LEQoMQakBfFqjmPXCALoRWxo
                                // https://polkascan.io/kusama/transaction/0x86a5487acc5be4762fb358f11f1708c0ec0050b81095f936957af47144fbf717
                                b"QmQhXTu3UaUX8XwRGJQQ95LEQoMQakBfFqjmPXCALoRWxo".to_vec(),
                                pallet_bca::Edition {
                                    proofs: 1,
                                    prints: 10,
                                },
                                // prints
                                vec![],
                            ),
                            (
                                // Death To False Monuments
                                // https://bca.mypinata.cloud/ipfs/QmQQBJDpLpapmiHD2ASpSSQfoRs76QC2SwmrfZoGLpYny3
                                // https://polkascan.io/kusama/transaction/0x5176cd3dc7b84dd3108725c21577d1664484dbd0d17ba856ab29c743d344af53
                                b"QmQQBJDpLpapmiHD2ASpSSQfoRs76QC2SwmrfZoGLpYny3".to_vec(),
                                pallet_bca::Edition {
                                    proofs: 1,
                                    prints: 5,
                                },
                                // prints
                                vec![],
                            ),
                        ],
                    ),
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

struct BcaTokenDistribution {
    early_investors: AccountId,
    team: AccountId,
    token_sale: AccountId,
    foundation: AccountId,
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: BcaTokenDistribution,
    prints: Vec<(
        AccountId,
        Vec<(Vec<u8>, pallet_bca::Edition, Vec<(bool, AccountId)>)>,
    )>,
    _enable_println: bool,
) -> GenesisConfig {
    const TOKEN_SUPPLY: Balance = 21_000_000 * DOLLARS;
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
            balances: vec![
                (endowed_accounts.early_investors, TOKEN_SUPPLY / 10),
                (endowed_accounts.team, TOKEN_SUPPLY / 5),
                (endowed_accounts.token_sale, TOKEN_SUPPLY / 2),
                (endowed_accounts.foundation, TOKEN_SUPPLY / 5),
            ],
        },
        bca: BcaConfig { prints },
        aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        grandpa: GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: root_key,
        },
    }
}
