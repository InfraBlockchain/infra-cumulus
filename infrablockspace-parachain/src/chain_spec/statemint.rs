// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use crate::chain_spec::{
	get_account_id_from_seed, get_collator_keys_from_seed, Extensions, SAFE_XCM_VERSION,
};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use parachains_common::{AccountId, AuraId, Balance as StatemintBalance, StatemintAuraId};
use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type StatemintChainSpec =
	sc_service::GenericChainSpec<statemint_runtime::GenesisConfig, Extensions>;
pub type StatemineChainSpec =
	sc_service::GenericChainSpec<statemine_runtime::GenesisConfig, Extensions>;

const STATEMINT_ED: StatemintBalance = statemint_runtime::constants::currency::EXISTENTIAL_DEPOSIT;
const STATEMINE_ED: StatemintBalance = statemine_runtime::constants::currency::EXISTENTIAL_DEPOSIT;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn statemint_session_keys(keys: StatemintAuraId) -> statemint_runtime::SessionKeys {
	statemint_runtime::SessionKeys { aura: keys }
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn statemine_session_keys(keys: AuraId) -> statemine_runtime::SessionKeys {
	statemine_runtime::SessionKeys { aura: keys }
}

pub fn statemint_development_config() -> StatemintChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 0.into());
	properties.insert("tokenSymbol".into(), "DOT".into());
	properties.insert("tokenDecimals".into(), 10.into());

	StatemintChainSpec::from_genesis(
		// Name
		"Statemint Development",
		// ID
		"statemint_dev",
		ChainType::Local,
		move || {
			statemint_genesis(
				// initial collators.
				vec![(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_collator_keys_from_seed::<StatemintAuraId>("Alice"),
				)],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "polkadot-dev".into(), para_id: 1000 },
	)
}

pub fn statemint_local_config() -> StatemintChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 0.into());
	properties.insert("tokenSymbol".into(), "DOT".into());
	properties.insert("tokenDecimals".into(), 10.into());

	StatemintChainSpec::from_genesis(
		// Name
		"Statemint Local",
		// ID
		"statemint_local",
		ChainType::Local,
		move || {
			statemint_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed::<StatemintAuraId>("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed::<StatemintAuraId>("Bob"),
					),
				],
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
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "polkadot-local".into(), para_id: 1000 },
	)
}

// Not used for syncing, but just to determine the genesis values set for the upgrade from shell.
pub fn statemint_config() -> StatemintChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 0.into());
	properties.insert("tokenSymbol".into(), "DOT".into());
	properties.insert("tokenDecimals".into(), 10.into());

	StatemintChainSpec::from_genesis(
		// Name
		"Statemint",
		// ID
		"statemint",
		ChainType::Live,
		move || {
			statemint_genesis(
				// initial collators.
				vec![
					(
						hex!("4c3d674d2a01060f0ded218e5dcc6f90c1726f43df79885eb3e22d97a20d5421")
							.into(),
						hex!("4c3d674d2a01060f0ded218e5dcc6f90c1726f43df79885eb3e22d97a20d5421")
							.unchecked_into(),
					),
					(
						hex!("c7d7d38d16bc23c6321152c50306212dc22c0efc04a2e52b5cccfc31ab3d7811")
							.into(),
						hex!("c7d7d38d16bc23c6321152c50306212dc22c0efc04a2e52b5cccfc31ab3d7811")
							.unchecked_into(),
					),
					(
						hex!("c5c07ba203d7375675f5c1ebe70f0a5bb729ae57b48bcc877fcc2ab21309b762")
							.into(),
						hex!("c5c07ba203d7375675f5c1ebe70f0a5bb729ae57b48bcc877fcc2ab21309b762")
							.unchecked_into(),
					),
					(
						hex!("0b2d0013fb974794bd7aa452465b567d48ef70373fe231a637c1fb7c547e85b3")
							.into(),
						hex!("0b2d0013fb974794bd7aa452465b567d48ef70373fe231a637c1fb7c547e85b3")
							.unchecked_into(),
					),
				],
				vec![],
				1000u32.into(),
			)
		},
		vec![
			"/ip4/34.65.251.121/tcp/30334/p2p/12D3KooWG3GrM6XKMM4gp3cvemdwUvu96ziYoJmqmetLZBXE8bSa".parse().unwrap(),
			"/ip4/34.65.35.228/tcp/30334/p2p/12D3KooWMRyTLrCEPcAQD6c4EnudL3vVzg9zji3whvsMYPUYevpq".parse().unwrap(),
			"/ip4/34.83.247.146/tcp/30334/p2p/12D3KooWE4jFh5FpJDkWVZhnWtFnbSqRhdjvC7Dp9b8b3FTuubQC".parse().unwrap(),
			"/ip4/104.199.117.230/tcp/30334/p2p/12D3KooWG9R8pVXKumVo2rdkeVD4j5PVhRTqmYgLHY3a4yPYgLqM".parse().unwrap(),
		],
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "polkadot".into(), para_id: 1000 },
	)
}

fn statemint_genesis(
	invulnerables: Vec<(AccountId, StatemintAuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> statemint_runtime::GenesisConfig {
	let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");

	statemint_runtime::GenesisConfig {
		system: statemint_runtime::SystemConfig {
			code: statemint_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: statemint_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, STATEMINT_ED * 4096)).collect(),
		},
		parachain_info: statemint_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: statemint_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: STATEMINT_ED * 16,
			..Default::default()
		},
		session: statemint_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                  // account id
						acc,                          // validator id
						statemint_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		sudo: statemint_runtime::SudoConfig { key: Some(root_key) },
		parachain_system: Default::default(),
		polkadot_xcm: statemint_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
	}
}

pub fn statemine_development_config() -> StatemineChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 2.into());
	properties.insert("tokenSymbol".into(), "KSM".into());
	properties.insert("tokenDecimals".into(), 12.into());

	StatemineChainSpec::from_genesis(
		// Name
		"Statemine Development",
		// ID
		"statemine_dev",
		ChainType::Local,
		move || {
			statemine_genesis(
				// root_address.
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// initial collators.
				vec![(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_collator_keys_from_seed::<AuraId>("Alice"),
				)],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "kusama-dev".into(), para_id: 1000 },
	)
}

pub fn statemine_local_config() -> StatemineChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 2.into());
	properties.insert("tokenSymbol".into(), "KSM".into());
	properties.insert("tokenDecimals".into(), 12.into());

	StatemineChainSpec::from_genesis(
		// Name
		"Statemine Local",
		// ID
		"statemine_local",
		ChainType::Local,
		move || {
			statemine_genesis(
				// root_address.
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed::<AuraId>("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed::<AuraId>("Bob"),
					),
				],
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
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "kusama-local".into(), para_id: 1000 },
	)
}

pub fn statemine_config() -> StatemineChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 2.into());
	properties.insert("tokenSymbol".into(), "KSM".into());
	properties.insert("tokenDecimals".into(), 12.into());

	StatemineChainSpec::from_genesis(
		// Name
		"Statemine",
		// ID
		"statemine",
		ChainType::Live,
		move || {
			statemine_genesis(
				// root_address.
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					(
						hex!("50673d59020488a4ffc9d8c6de3062a65977046e6990915617f85fef6d349730")
							.into(),
						hex!("50673d59020488a4ffc9d8c6de3062a65977046e6990915617f85fef6d349730")
							.unchecked_into(),
					),
					(
						hex!("fe8102dbc244e7ea2babd9f53236d67403b046154370da5c3ea99def0bd0747a")
							.into(),
						hex!("fe8102dbc244e7ea2babd9f53236d67403b046154370da5c3ea99def0bd0747a")
							.unchecked_into(),
					),
					(
						hex!("38144b5398e5d0da5ec936a3af23f5a96e782f676ab19d45f29075ee92eca76a")
							.into(),
						hex!("38144b5398e5d0da5ec936a3af23f5a96e782f676ab19d45f29075ee92eca76a")
							.unchecked_into(),
					),
					(
						hex!("3253947640e309120ae70fa458dcacb915e2ddd78f930f52bd3679ec63fc4415")
							.into(),
						hex!("3253947640e309120ae70fa458dcacb915e2ddd78f930f52bd3679ec63fc4415")
							.unchecked_into(),
					),
				],
				Vec::new(),
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "kusama".into(), para_id: 1000 },
	)
}

fn statemine_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> statemine_runtime::GenesisConfig {
	statemine_runtime::GenesisConfig {
		system: statemine_runtime::SystemConfig {
			code: statemine_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: statemine_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, STATEMINE_ED * 524_288))
				.collect(),
		},
		parachain_info: statemine_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: statemine_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: STATEMINE_ED * 16,
			..Default::default()
		},
		session: statemine_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                  // account id
						acc,                          // validator id
						statemine_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		sudo: statemine_runtime::SudoConfig { key: Some(root_key) },
		aura: Default::default(),
		aura_ext: Default::default(),
		assets: pallet_assets::GenesisConfig {
			assets: vec![
				(1, get_account_id_from_seed::<sr25519::Public>("Alice"), true, 1300),
				(2, get_account_id_from_seed::<sr25519::Public>("Alice"), true, 1),
			],
			metadata: vec![
				(1, "iKRW".into(), "iKRW".into(), 12),
				(2, "iUSD".into(), "iUSD".into(), 12),
			],
			accounts: vec![
				(
					1,
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					1_000_000_000_000_000_000_000,
				),
				(
					2,
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					1_000_000_000_000_000_000,
				),
			],
			..Default::default()
		},
		parachain_system: Default::default(),
		polkadot_xcm: statemine_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
	}
}
