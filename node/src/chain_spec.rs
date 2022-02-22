use node_5ire_runtime::{
	opaque::SessionKeys, wasm_binary_unwrap, AccountId, AuraConfig, AuthorityDiscoveryConfig,
	Balance, BalancesConfig, Block, CouncilConfig, EVMConfig, ElectionsConfig, EthereumConfig,
	GenesisConfig, GrandpaConfig, ImOnlineConfig, NftConfig, SchedulerConfig, SessionConfig,
	StakerStatus, StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
};
pub use node_primitives::Signature;
use sp_core::{H160, U256};
// use pallet_im_online::ed25519::AuthorityId as ImOnlineId;
use crate::testnet_fixtures;
use node_5ire_runtime::constants::currency::*;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::{ChainSpecExtension, Properties};
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use std::{collections::BTreeMap, str::FromStr};
// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.testnet.5ire.network/submit/";
const DEFAULT_PROTOCOL_ID: &str = "5ire";
/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

fn session_keys(
	grandpa: GrandpaId,
	aura: AuraId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, aura, im_online, authority_discovery }
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
pub fn get_authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, Balance, GrandpaId, AuraId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		testnet_fixtures::STAKED_ENDOWMENT,
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<AuraId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn testnet_accounts() -> Vec<AccountId> {
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
	]
}

fn dev_evm_accounts() -> BTreeMap<H160, pallet_evm::GenesisAccount> {
	let endowed_account = vec![
		// H160 address of Alice dev account
		// Derived from SS58 (42 prefix) address
		// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
		// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
		// Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
		H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558").unwrap(),
		// Bob evm account
		H160::from_str("8eaf04151687736326c9fea17e25fc5287613693").unwrap(),
		// Alice Stash evm account
		H160::from_str("be5ddb1579b72e84524fc29e78609e3caf42e85a").unwrap(),
		// Charlie evm account
		H160::from_str("90b5ab205c6974c9ea841be688864633dc9ca8a3").unwrap(),
		// Dave evm account
		H160::from_str("306721211d5404bd9da88e0204360a1a9ab8b87c").unwrap(),
		// Bob Stash account
		H160::from_str("fe65717dad0447d715f660a0a58411de509b42e6").unwrap(),
		// Eve evm Account
		H160::from_str("e659a7a1628cdd93febc04a4e0646ea20e9f5f0c").unwrap(),
		// Ferdie evm account
		H160::from_str("1cbd2d43530a44705ad088af313e18f80b53ef16").unwrap(),
		// H160 address of CI test runner account
		// Secret key:5087cdc9b364f842b856aea161e078ee7cc1f56465f0476e9e26a8ab7f85483d
		H160::from_str("544373db034533D23F6D8E998E3e86079050084C").unwrap(),
	];

	get_endowed_evm_accounts(endowed_account)
}

fn get_endowed_evm_accounts(
	endowed_account: Vec<H160>,
) -> BTreeMap<H160, pallet_evm::GenesisAccount> {
	let mut evm_accounts = BTreeMap::new();
	for account in endowed_account {
		evm_accounts.insert(
			account,
			pallet_evm::GenesisAccount {
				balance: U256::from(10_000_000 * FIREB), // 10_000_000 token
				code: Default::default(),
				nonce: Default::default(),
				storage: Default::default(),
			},
		);
	}
	evm_accounts
}

pub fn fire_testnet_config() -> ChainSpec {
	let data = r#"
		{
			"ss58Format": 42,
			"tokenDecimals": 18,
			"tokenSymbol": "5IRE"
		}"#;
	let properties = serde_json::from_str(data).unwrap();
	let boot_nodes = testnet_fixtures::get_fire_bootnodes();

	ChainSpec::from_genesis(
		"5ire Test Net",
		"5ire_fire_testnet",
		ChainType::Live,
		fire_testnet_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		properties,
		Default::default(),
	)
}

fn fire_testnet_config_genesis() -> GenesisConfig {
	let initial_authorities = testnet_fixtures::get_fire_initial_authorities();
	// let endowed_accounts: Vec<AccountId> = vec![
	// 	hex!("80cf86904f8f57c8de318eedd70617a2e5b0d2e4cba3276c3e923899bc7ea610").into(), //stash account validator 1
	// 	hex!("84a19d62208287b6f061b303c0760dfce3ba6d6e0c7e3bcf8c44e94e86917614").into(), //stash account validator 2
	// 	hex!("865b4d7a9697c3494f782327c6cd99be199f5b4d25aa4878e166a327b693f20e").into(), //stash account validator 3
	// 	hex!("58e8c326d42ffcd22600e5d6e5792ba74d28d7445557de63db132dfeaa85f145").into(), // Controller account Validator1
	// ]; // stash account validator 3
	let founder_allocation = testnet_fixtures::get_commonwealth_allocation();

	fire_testnet_genesis(
		initial_authorities,
		testnet_fixtures::get_testnet_root_key(),
		founder_allocation,
		false,
	)
}

/// Configure initial storage state for FRAME modules.
fn fire_testnet_genesis(
	// wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		Balance,
		GrandpaId,
		AuraId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	founder_allocation: Vec<(AccountId, Balance)>,
	_enable_println: bool,
) -> GenesisConfig {
	let endowed_balances: Vec<(AccountId, Balance)> =
		founder_allocation.iter().map(|x| (x.0.clone(), x.1.clone())).collect();

	let mut total_supply: u128 = 0;
	for (_, balance) in &endowed_balances {
		// println!("Balance for {} is {}", balance, balance);
		total_supply += *balance;
	}

	assert_eq!(
		total_supply + (2 * testnet_fixtures::EVM_ENDOWMENT),
		1_500_000_000 * FIREB,
		"Total Supply exceed to 1.5B"
	);

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_balances,
		},
		aura: AuraConfig { authorities: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.3.clone(), x.4.clone(), x.5.clone(), x.6.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), x.2.clone(), StakerStatus::Validator))
				.collect(),
			..Default::default()
		},
		grandpa: GrandpaConfig { authorities: vec![] },
		elections: ElectionsConfig { members: vec![] },
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		evm: EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map.insert(
					// 5ireChain faucet
					// Derived from SS58 (42 prefix) address
					// SS58: 5CoLpJnPM8SrBcokvzaEUcnBb4SRJMkBT9XJHWHSexzPx5EX
					// hex: 0x2084b918cbbd841b3f67ebcae9f9394b3d9c1e296571aed23d8775c0735d077d
					// Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
					H160::from_str("2084b918cbbd841b3f67ebcae9f9394b3d9c1e29")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount {
						balance: U256::from(testnet_fixtures::EVM_ENDOWMENT), // 500_000_000 token
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// 5ireChain faucet
					// H160 address of CI test runner account
					// hex: 0xde58f89f0ba1a67d5fe9bd7d95b4db7bffdace05c0b93e1eb595599ba1d42b0b
					H160::from_str("de58f89f0ba1a67d5fe9bd7d95b4db7bffdace05")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount {
						balance: U256::from(testnet_fixtures::EVM_ENDOWMENT), // 500_000_000 token
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map
			},
		},
		ethereum: EthereumConfig {},
		nft: NftConfig { tokens: vec![] },
		scheduler: SchedulerConfig {},
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![get_authority_keys_from_seed("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		true,
		dev_evm_accounts(),
	)
}

pub fn development_config() -> ChainSpec {
	let data = r#"
	{
		"ss58Format": 42,
		"tokenDecimals": 18,
		"tokenSymbol": "5IRE"
	}"#;
	let properties = serde_json::from_str(data).unwrap();

	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		None,
		Some(properties),
		Default::default(),
	)
}

fn testnet_genesis(
	// wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		Balance,
		GrandpaId,
		AuraId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	_enable_println: bool,
	endowed_evm_accounts: BTreeMap<H160, pallet_evm::GenesisAccount>,
) -> GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);
	const ENDOWMENT: u128 = 1_000_000 * FIREB;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		aura: AuraConfig { authorities: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.3.clone(), x.4.clone(), x.5.clone(), x.6.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), x.2.clone(), StakerStatus::Validator))
				.collect(),
			..Default::default()
		},
		grandpa: GrandpaConfig { authorities: vec![] },
		elections: ElectionsConfig { members: vec![] },
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		evm: EVMConfig { accounts: endowed_evm_accounts },
		ethereum: EthereumConfig {},
		nft: NftConfig { tokens: vec![] },
		scheduler: SchedulerConfig {},
	}
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![get_authority_keys_from_seed("Alice"), get_authority_keys_from_seed("Bob")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		false,
		dev_evm_accounts(),
	)
}

pub fn local_testnet_config() -> ChainSpec {
	let mut props: Properties = Properties::new();
	let value = json!("5IRE");
	props.insert("tokenSymbol".to_string(), value);

	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		local_testnet_genesis,
		vec![],
		None,
		None,
		Some(props),
		Default::default(),
	)
}
