// Copyright 2022 Parity Technologies (UK) Ltd.
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

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{
	pallet_prelude::*,
	traits::{fungibles::Inspect, tokens::fungibles::Balanced, OriginTrait, PalletInfoAccess},
	PalletId,
};
use frame_system::pallet_prelude::*;
use pallet_collator_selection::SystemTokenAggregator;
pub use sp_runtime::types::VoteWeight;
use sp_runtime::{
	traits::AccountIdConversion,
	types::{SystemTokenId, SystemTokenLocalAssetProvider, VoteAssetId},
};
use sp_std::{boxed::Box, vec};
use xcm::{
	latest::Junction,
	opaque::lts::{AssetId::Concrete, Fungibility::Fungible, Junctions::*, MultiLocation},
};
use xcm_primitives::AssetMultiLocationGetter;

pub type ParaAssetId = VoteAssetId;
pub type PalletIndex = u32;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type AssetIdOf<T> =
	<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;

#[frame_support::pallet]
pub mod pallet {
	use xcm::v3::MultiAsset;

	use super::*;

	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ pallet_xcm::Config
		+ pallet_assets::Config
		+ parachain_info::Config
		+ pallet_asset_registry::Config
	{
		type RuntimeEvent: From<Event> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The fungibles instance used to pay for transactions in assets.
		type Assets: Balanced<Self::AccountId>
			+ Inspect<Self::AccountId>
			+ SystemTokenLocalAssetProvider;
	}

	#[pallet::storage]
	#[pallet::getter(fn allowed_system_token)]
	/// Wrapped System token list used in parachains.
	/// Key: (PalletIndex, ParaAssetId) of Wrapped System token. ParaId is omitted.
	/// Value: (SystemTokenId)
	pub(super) type AllowedSystemToken<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		PalletIndex,
		Twox64Concat,
		ParaAssetId,
		SystemTokenId,
		OptionQuery,
	>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event {
		SystemTokenAggregated { system_token_id: SystemTokenId },
	}

	#[pallet::error]
	pub enum Error<T> {
		Unknown,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<AccountIdOf<T>>,
		<<T as frame_system::Config>::RuntimeOrigin as OriginTrait>::AccountId:
			From<AccountIdOf<T>>,
	{
		/// Set the list of invulnerable (fixed) collators.
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn test(
			origin: OriginFor<T>,
			system_token_id: SystemTokenId,
			amount: u128,
		) -> DispatchResult {
			Self::aggregate_system_token(system_token_id, amount);
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn account_id() -> T::AccountId {
			let pallet_id = PalletId(*b"infrapid");
			pallet_id.into_account_truncating()
		}
	}

	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<AccountIdOf<T>>,
		<<T as frame_system::Config>::RuntimeOrigin as OriginTrait>::AccountId:
			From<AccountIdOf<T>>,
	{
		fn aggregate_system_token(system_token_id: SystemTokenId, amount: VoteWeight) {
			let sovereign = Self::account_id();

			let result = pallet_xcm::Pallet::<T>::limited_teleport_assets(
				<T as frame_system::Config>::RuntimeOrigin::signed(sovereign.clone().into()),
				Box::new(xcm::VersionedMultiLocation::V3(MultiLocation {
					parents: 1,
					interior: X1(Junction::Parachain(system_token_id.clone().para_id)),
				})),
				Box::new(
					Junction::AccountId32 { network: None, id: sovereign.clone().into() }
						.into_location()
						.into(),
				),
				Box::new(
					MultiAsset {
						id: Concrete(MultiLocation {
							parents: 1,
							interior: X3(
								Junction::Parachain(system_token_id.clone().para_id),
								Junction::PalletInstance(system_token_id.clone().pallet_id as u8),
								Junction::GeneralIndex(system_token_id.clone().asset_id as u128),
							),
						}),
						fun: Fungible(amount),
					}
					.into(),
				),
				0,
				xcm::v3::WeightLimit::Unlimited,
			);

			match result {
				Err(err) => log::warn!("Failed to teleport slashed assets: {:?}", err),
				_ => Self::deposit_event(Event::SystemTokenAggregated {
					system_token_id: system_token_id.clone(),
				}),
			};
		}
	}
}

impl<T: Config> SystemTokenAggregator for Pallet<T> {
	fn aggregate_system_token(system_token_id: SystemTokenId, amount: VoteWeight) {
		let sovereign = Self::account_id();
		let para_id = <parachain_info::Pallet<T>>::get();
		let system_token_asset_list = match <T as Config>::Assets::token_list() {
			Some(system_token_asset_list) => system_token_asset_list,
			_ => vec![],
		};
		let asset_id: T::AssetId = pallet_assets::Pallet::<T>::get_most_account_balance(
			system_token_asset_list,
			sovereign.clone(),
		)
		.into();

		let multi_location_asset =
			match pallet_asset_registry::Pallet::<T>::get_asset_multi_location(asset_id) {
				Some(multi_location_asset) => multi_location_asset,
				_ => return,
			};
		Self::aggregate_system_token(system_token_id, amount);
	}
}
