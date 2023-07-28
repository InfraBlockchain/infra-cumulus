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
pub mod matching;
pub mod runtime_api;
use crate::matching::{Equals, LocalMultiLocationPattern, ParentLocation, StartsWith};

use frame_support::traits::EverythingBut;
use parachains_common::AssetIdForTrustBackedAssets;
use xcm::prelude::MultiLocation;
use xcm_builder::{AsPrefixedGeneralIndex, MatchedConvertedConcreteId};
use xcm_executor::traits::JustTry;

pub type AssetId = u32;

/// `MultiLocation` vs `AssetIdForTrustBackedAssets` converter for `TrustBackedAssets`
pub type AssetIdForTrustBackedAssetsConvert<TrustBackedAssetsPalletLocation> =
	AsPrefixedGeneralIndex<TrustBackedAssetsPalletLocation, AssetIdForTrustBackedAssets, JustTry>;

/// [`MatchedConvertedConcreteId`] converter dedicated for `TrustBackedAssets`
pub type TrustBackedAssetsConvertedConcreteId<TrustBackedAssetsPalletLocation, Balance> =
	MatchedConvertedConcreteId<
		AssetIdForTrustBackedAssets,
		Balance,
		StartsWith<TrustBackedAssetsPalletLocation>,
		AssetIdForTrustBackedAssetsConvert<TrustBackedAssetsPalletLocation>,
		JustTry,
	>;

/// AssetId used for identifying assets by MultiLocation.
pub type MultiLocationForAssetId = MultiLocation;

/// [`MatchedConvertedConcreteId`] converter dedicated for storing `AssetId` as `MultiLocation`.
pub type MultiLocationConvertedConcreteId<MultiLocationFilter, AssetConverter, Balance> =
	MatchedConvertedConcreteId<
		AssetIdForTrustBackedAssets,
		Balance,
		MultiLocationFilter,
		xcm_primitives::AsAssetMultiLocation<AssetIdForTrustBackedAssets, AssetConverter>,
		JustTry,
	>;

/// [`MatchedConvertedConcreteId`] converter dedicated for storing `ForeignAssets` with `AssetId` as `MultiLocation`.
///
/// Excludes by default:
/// - parent as relay chain
/// - all local MultiLocations
///
/// `AdditionalMultiLocationExclusionFilter` can customize additional excluded MultiLocations
pub type ForeignAssetsConvertedConcreteId<
	AdditionalMultiLocationExclusionFilter,
	AssetConverter,
	Balance,
> = MultiLocationConvertedConcreteId<
	EverythingBut<(
		// Excludes relay/parent chain currency
		Equals<ParentLocation>,
		// Here we rely on fact that something like this works:
		// assert!(MultiLocation::new(1, X1(Parachain(100))).starts_with(&MultiLocation::parent()));
		// assert!(X1(Parachain(100)).starts_with(&Here));
		StartsWith<LocalMultiLocationPattern>,
		// Here we can exclude more stuff or leave it as `()`
		AdditionalMultiLocationExclusionFilter,
	)>,
	AssetConverter,
	Balance,
>;

#[cfg(test)]
mod tests {

	use super::*;
	use xcm::latest::prelude::*;
	use xcm_executor::traits::Convert;

	frame_support::parameter_types! {
		pub TrustBackedAssetsPalletLocation: MultiLocation = MultiLocation::new(5, X1(PalletInstance(13)));
	}

	#[test]
	fn asset_id_for_trust_backed_assets_convert_works() {
		let local_asset_id = 123456789 as AssetIdForTrustBackedAssets;
		let expected_reverse_ref =
			MultiLocation::new(5, X2(PalletInstance(13), GeneralIndex(local_asset_id.into())));

		assert_eq!(
			AssetIdForTrustBackedAssetsConvert::<TrustBackedAssetsPalletLocation>::reverse_ref(
				local_asset_id
			)
			.unwrap(),
			expected_reverse_ref
		);
		assert_eq!(
			AssetIdForTrustBackedAssetsConvert::<TrustBackedAssetsPalletLocation>::convert_ref(
				expected_reverse_ref
			)
			.unwrap(),
			local_asset_id
		);
	}
}
