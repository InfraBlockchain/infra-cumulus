// Copyright 2021 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_xcm`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_xcm
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/pallet_xcm.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_xcm`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm::WeightInfo for WeightInfo<T> {
	/// Storage: InfrablockspaceXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: InfrablockspaceXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: InfrablockspaceXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: InfrablockspaceXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn send() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `38`
		//  Estimated: `4645`
		// Minimum execution time: 23_618 nanoseconds.
		Weight::from_parts(24_118_000, 0)
			.saturating_add(Weight::from_parts(0, 4645))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn teleport_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `499`
		// Minimum execution time: 23_764 nanoseconds.
		Weight::from_parts(24_114_000, 0)
			.saturating_add(Weight::from_parts(0, 499))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn reserve_transfer_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551 nanoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn execute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551 nanoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: InfrablockspaceXcm SupportedVersion (r:0 w:1)
	/// Proof Skipped: InfrablockspaceXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	fn force_xcm_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_266 nanoseconds.
		Weight::from_parts(8_578_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: InfrablockspaceXcm SafeXcmVersion (r:0 w:1)
	/// Proof Skipped: InfrablockspaceXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	fn force_default_xcm_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_289 nanoseconds.
		Weight::from_parts(2_382_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: InfrablockspaceXcm VersionNotifiers (r:1 w:1)
	/// Proof Skipped: InfrablockspaceXcm VersionNotifiers (max_values: None, max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm QueryCounter (r:1 w:1)
	/// Proof Skipped: InfrablockspaceXcm QueryCounter (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: InfrablockspaceXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: InfrablockspaceXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: InfrablockspaceXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm Queries (r:0 w:1)
	/// Proof Skipped: InfrablockspaceXcm Queries (max_values: None, max_size: None, mode: Measured)
	fn force_subscribe_version_notify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `38`
		//  Estimated: `7729`
		// Minimum execution time: 28_592 nanoseconds.
		Weight::from_parts(29_116_000, 0)
			.saturating_add(Weight::from_parts(0, 7729))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: InfrablockspaceXcm VersionNotifiers (r:1 w:1)
	/// Proof Skipped: InfrablockspaceXcm VersionNotifiers (max_values: None, max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: InfrablockspaceXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: InfrablockspaceXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: InfrablockspaceXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm Queries (r:0 w:1)
	/// Proof Skipped: InfrablockspaceXcm Queries (max_values: None, max_size: None, mode: Measured)
	fn force_unsubscribe_version_notify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `220`
		//  Estimated: `8470`
		// Minimum execution time: 30_604 nanoseconds.
		Weight::from_parts(31_361_000, 0)
			.saturating_add(Weight::from_parts(0, 8470))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: InfrablockspaceXcm SupportedVersion (r:4 w:2)
	/// Proof Skipped: InfrablockspaceXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	fn migrate_supported_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `95`
		//  Estimated: `9995`
		// Minimum execution time: 13_750 nanoseconds.
		Weight::from_parts(14_051_000, 0)
			.saturating_add(Weight::from_parts(0, 9995))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: InfrablockspaceXcm VersionNotifiers (r:4 w:2)
	/// Proof Skipped: InfrablockspaceXcm VersionNotifiers (max_values: None, max_size: None, mode: Measured)
	fn migrate_version_notifiers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `99`
		//  Estimated: `9999`
		// Minimum execution time: 13_897 nanoseconds.
		Weight::from_parts(14_080_000, 0)
			.saturating_add(Weight::from_parts(0, 9999))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: InfrablockspaceXcm VersionNotifyTargets (r:5 w:0)
	/// Proof Skipped: InfrablockspaceXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	fn already_notified_target() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `12481`
		// Minimum execution time: 16_364 nanoseconds.
		Weight::from_parts(16_867_000, 0)
			.saturating_add(Weight::from_parts(0, 12481))
			.saturating_add(T::DbWeight::get().reads(5))
	}
	/// Storage: InfrablockspaceXcm VersionNotifyTargets (r:2 w:1)
	/// Proof Skipped: InfrablockspaceXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: InfrablockspaceXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: InfrablockspaceXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: InfrablockspaceXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn notify_current_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `10041`
		// Minimum execution time: 27_372 nanoseconds.
		Weight::from_parts(28_128_000, 0)
			.saturating_add(Weight::from_parts(0, 10041))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: InfrablockspaceXcm VersionNotifyTargets (r:3 w:0)
	/// Proof Skipped: InfrablockspaceXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	fn notify_target_migration_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `7561`
		// Minimum execution time: 7_804 nanoseconds.
		Weight::from_parts(8_096_000, 0)
			.saturating_add(Weight::from_parts(0, 7561))
			.saturating_add(T::DbWeight::get().reads(3))
	}
	/// Storage: InfrablockspaceXcm VersionNotifyTargets (r:4 w:2)
	/// Proof Skipped: InfrablockspaceXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	fn migrate_version_notify_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `10006`
		// Minimum execution time: 14_948 nanoseconds.
		Weight::from_parts(15_422_000, 0)
			.saturating_add(Weight::from_parts(0, 10006))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: InfrablockspaceXcm VersionNotifyTargets (r:4 w:2)
	/// Proof Skipped: InfrablockspaceXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: InfrablockspaceXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: InfrablockspaceXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: InfrablockspaceXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: InfrablockspaceXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn migrate_and_notify_old_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `112`
		//  Estimated: `15027`
		// Minimum execution time: 33_791 nanoseconds.
		Weight::from_parts(34_282_000, 0)
			.saturating_add(Weight::from_parts(0, 15027))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
