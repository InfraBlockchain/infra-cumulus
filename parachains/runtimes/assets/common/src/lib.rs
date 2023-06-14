// Copyright (C) 2023 Parity Technologies (UK) Ltd.
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

#![cfg_attr(not(feature = "std"), no_std)]

pub mod fungible_conversion;
pub mod runtime_api;

use xcm_builder::{AsPrefixedGeneralIndex, ConvertedConcreteId};

use parachains_common::AssetIdForTrustBackedAssets;
use sp_std::{borrow::Borrow, marker::PhantomData};
use xcm::latest::MultiLocation;
use xcm_executor::traits::JustTry;

/// `MultiLocation` vs `AssetIdForTrustBackedAssets` converter for `TrustBackedAssets`
pub type AssetIdForTrustBackedAssetsConvert<TrustBackedAssetsPalletLocation> =
	AsPrefixedGeneralIndex<TrustBackedAssetsPalletLocation, AssetIdForTrustBackedAssets, JustTry>;

/// [`ConvertedConcreteId`] converter dedicated for `TrustBackedAssets`
pub type TrustBackedAssetsConvertedConcreteId<TrustBackedAssetsPalletLocation, Balance> =
	ConvertedConcreteId<
		AssetIdForTrustBackedAssets,
		Balance,
		AssetIdForTrustBackedAssetsConvert<TrustBackedAssetsPalletLocation>,
		JustTry,
	>;

pub struct AsAssetMultiLocation<AssetId, AssetIdInfoGetter>(
	PhantomData<(AssetId, AssetIdInfoGetter)>,
);
impl<AssetId, AssetIdInfoGetter> xcm_executor::traits::Convert<MultiLocation, AssetId>
	for AsAssetMultiLocation<AssetId, AssetIdInfoGetter>
where
	AssetId: Clone,
	AssetIdInfoGetter: AssetMultiLocationGetter<AssetId>,
{
	fn convert_ref(asset_multi_location: impl Borrow<MultiLocation>) -> Result<AssetId, ()> {
		AssetIdInfoGetter::get_asset_id(asset_multi_location.borrow().clone()).ok_or(())
	}

	fn reverse_ref(asset_id: impl Borrow<AssetId>) -> Result<MultiLocation, ()> {
		AssetIdInfoGetter::get_asset_multi_location(asset_id.borrow().clone()).ok_or(())
	}
}
pub trait AssetMultiLocationGetter<AssetId> {
	fn get_asset_multi_location(asset_id: AssetId) -> Option<MultiLocation>;
	fn get_asset_id(asset_multi_location: MultiLocation) -> Option<AssetId>;
}

// pub type InfraConvertedConcreteId<AssetRegistry> =
// 	ConvertedConcreteId<u32, u128, AsAssetMultiLocation<u32, AssetRegistry>, JustTry>;

// #[cfg(test)]
// mod tests {

// 	use super::*;
// 	use xcm::latest::prelude::*;
// 	use xcm_executor::traits::Convert;

// 	frame_support::parameter_types! {
// 		pub TrustBackedAssetsPalletLocation: MultiLocation = MultiLocation::new(5, X1(PalletInstance(13)));
// 	}

// 	#[test]
// 	fn asset_id_for_trust_backed_assets_convert_works() {
// 		let local_asset_id = 123456789 as AssetIdForTrustBackedAssets;
// 		let expected_reverse_ref =
// 			MultiLocation::new(5, X2(PalletInstance(13), GeneralIndex(local_asset_id.into())));

// 		assert_eq!(
// 			AssetIdForTrustBackedAssetsConvert::<TrustBackedAssetsPalletLocation>::reverse_ref(
// 				local_asset_id
// 			)
// 			.unwrap(),
// 			expected_reverse_ref
// 		);
// 		assert_eq!(
// 			AssetIdForTrustBackedAssetsConvert::<TrustBackedAssetsPalletLocation>::convert_ref(
// 				expected_reverse_ref
// 			)
// 			.unwrap(),
// 			local_asset_id
// 		);
// 	}
// }
