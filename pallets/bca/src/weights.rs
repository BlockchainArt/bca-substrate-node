
//! Autogenerated weights for `pallet_bca`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-08-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/bca-node
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_bca
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./pallets/bca/src


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn create_collection() -> Weight;
	fn create_print() -> Weight;
	fn transfer_print() -> Weight;
}

/// Weight functions for pallet_bca.
pub struct Weights<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for Weights<T> {
	// Storage: Bca Collections (r:1 w:1)
	// Storage: Bca NextCollectionId (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:4 w:4)
	fn create_collection(_s: u32, ) -> Weight {
		(371_111_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Attribute (r:2 w:2)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques ClassMetadataOf (r:1 w:0)
	// Storage: Uniques Account (r:0 w:1)
	fn create_print() -> Weight {
		(307_332_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Account (r:0 w:2)
	fn transfer_print() -> Weight {
		(121_842_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
