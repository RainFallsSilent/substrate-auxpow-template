use primitives::{Pair, Public};
use node_template_runtime::{
	AccountId, BalancesConfig, GenesisConfig,
	SudoConfig, IndicesConfig, SystemConfig, WASM_BINARY,
};
use substrate_service;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Whatever the current runtime is, with simple Alice/Bob auths.
	LocalTestnet,
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

impl Alternative {
	/// Get an actual chain config from one of the alternatives.
	pub(crate) fn load(self) -> Result<ChainSpec, String> {
		Ok(match self {
			Alternative::Development => ChainSpec::from_genesis(
				"Development",
				"dev",
				|| testnet_genesis(
					get_from_seed::<AccountId>("Alice"),
					vec![
						get_from_seed::<AccountId>("Alice"),
						get_from_seed::<AccountId>("Bob"),
						get_from_seed::<AccountId>("Alice//stash"),
						get_from_seed::<AccountId>("Bob//stash"),
					],
					true
				),
				vec![],
				None,
				None,
				None,
				None
			),
			Alternative::LocalTestnet => ChainSpec::from_genesis(
				"Local Testnet",
				"local_testnet",
				|| testnet_genesis(
					get_from_seed::<AccountId>("Alice"),
					vec![
						get_from_seed::<AccountId>("Alice"),
						get_from_seed::<AccountId>("Bob"),
						get_from_seed::<AccountId>("Charlie"),
						get_from_seed::<AccountId>("Dave"),
						get_from_seed::<AccountId>("Eve"),
						get_from_seed::<AccountId>("Ferdie"),
						get_from_seed::<AccountId>("Alice//stash"),
						get_from_seed::<AccountId>("Bob//stash"),
						get_from_seed::<AccountId>("Charlie//stash"),
						get_from_seed::<AccountId>("Dave//stash"),
						get_from_seed::<AccountId>("Eve//stash"),
						get_from_seed::<AccountId>("Ferdie//stash"),
					],
					true
				),
				vec![],
				None,
				None,
				None,
				None
			),
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(Alternative::Development),
			"" | "local" => Some(Alternative::LocalTestnet),
			_ => None,
		}
	}
}

fn testnet_genesis(
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool
) -> GenesisConfig {
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
			vesting: vec![],
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
	}
}
