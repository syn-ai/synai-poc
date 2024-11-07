//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
use weights::WeightInfo;
use frame_support::sp_runtime::traits::Hash;
use frame_support::BoundedVec;
use scale_info::prelude::vec;
use crate::vec::Vec;
// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The pallet's configuration trait.
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
		/// The maximum number of vectors that can be stored in the pallet.
		#[pallet::constant]
		type MaxVectors: Get<u32>;
		/// Maximum length for vector data
		#[pallet::constant]
		type MaxVectorLength: Get<u32>;
		/// Maximum length for tag data
		#[pallet::constant]
		type MaxTagLength: Get<u32>;
		/// Maximum number of tags per vector 
		#[pallet::constant]
		type MaxTagsPerVector: Get<u32>;

	}

	/// Events that functions in this pallet can emit.
	///
	/// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
	/// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
	/// documentation for each event field and its parameters is added to a node's metadata so it
	/// can be used by external interfaces or tools.
	///
	///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
	/// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
	/// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
	#[pallet::event] 
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	
	pub enum Event<T: Config> {
		/// Vector stored successfully
		VectorStored {
			vector_id: T::Hash,
			author: <T as frame_system::Config>::AccountId,
		},
		/// Vectors pruned
		VectorsPruned {
			count: u32,
		},
	}

	/// Errors that can be returned by this pallet.
	///
	/// Errors tell users that something went wrong so it's important that their naming is
	/// informative. Similar to events, error documentation is added to a node's metadata so it's
	/// equally important that they have helpful documentation associated with them.
	///
	/// This type of runtime error can be up to 4 bytes in size should you want to return additional
	/// information.
	#[pallet::error]
	pub enum Error<T> {
		/// Vector not found
		VectorNotFound,
		/// Tag not found
		TagNotFound,
		/// Nothing to prune
		NothingToPrune,
		/// Tag exceeds maximum length
		TagTooLong,
		/// Maximum number of vectors reached for author
		MaxVectorsReached,
		/// Invalid vector data
		InvalidVectorData,
	}

	/// The pallet's dispatchable functions ([`Call`]s).
	///
	/// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	/// These functions materialize as "extrinsics", which are often compared to transactions.
	/// They must always return a `DispatchResult` and be annotated with a weight and call index.
	///
	/// The [`call_index`] macro is used to explicitly
	/// define an index for calls in the [`Call`] enum. This is useful for pallets that may
	/// introduce new dispatchables over time. If the order of a dispatchable changes, its index
	/// will also change which will break backwards compatibility.
	///
	/// The [`weight`] macro is used to assign a weight to each call.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::store_weight_data(
			weight_data.0.len() as u32,
			tags.len() as u32
		))]
		pub fn store_weight_data(
			origin: OriginFor<T>,
			weight_data: WeightData<T>,
			tags: Vec<Vec<u8>>,
		) -> DispatchResult {
			let author = ensure_signed(origin)?;
			
			// WASM-safe error handling using ensure!
			ensure!(
				weight_data.0.len() <= T::MaxVectorLength::get() as usize,
				Error::<T>::InvalidVectorData
			);

			// Generate vector ID
			let vector_id = T::Hashing::hash_of(&weight_data);
			
			// Process tags
			let mut tag_refs: BoundedVec<T::Hash, T::MaxTagsPerVector> = 
				BoundedVec::try_from(Vec::new())
				.expect("Empty vec should always fit bounds");

			for tag_data in tags {
				let tag_id = T::Hashing::hash_of(&tag_data);
				let bounded_tag = BoundedVec::<u8, T::MaxTagLength>::try_from(tag_data)
					.map_err(|_| Error::<T>::TagTooLong)?;
				Tags::<T>::insert(tag_id, bounded_tag);
				tag_refs.try_push(tag_id)
					.map_err(|_| Error::<T>::TagTooLong)?;
			}
			
			// Store vector
			Vectors::<T>::insert(vector_id, weight_data);
			
			// Update author's vector list
			AuthorVectors::<T>::try_mutate(author.clone(), |vectors| -> Result<(), DispatchError> {
				match vectors {
					Some(v) => {
						v.try_push(vector_id)
							.map_err(|_| Error::<T>::VectorNotFound)?;
					},
					None => {
						*vectors = Some(BoundedVec::try_from(vec![vector_id])
							.map_err(|_| Error::<T>::VectorNotFound)?);
					},
				}
				Ok(())
			})?;

			Self::deposit_event(Event::VectorStored { vector_id, author });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::get_weights_by_tag(1))]
		pub fn get_weights_by_tag(
			origin: OriginFor<T>,
			tag_data: Vec<u8>,
		) -> DispatchResult {
			let _caller = ensure_signed(origin)?;
			
			let tag_id = T::Hashing::hash_of(&tag_data);
			ensure!(Tags::<T>::contains_key(tag_id), Error::<T>::TagNotFound);
			
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::get_weights_by_author(1))]
		pub fn get_weights_by_author(
			origin: OriginFor<T>,
			author: <T as frame_system::Config>::AccountId,
		) -> DispatchResult {
			let _caller = ensure_signed(origin)?;
			
			ensure!(AuthorVectors::<T>::contains_key(&author), Error::<T>::VectorNotFound);
			
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::prune_weight_data(1))]
		pub fn prune_weight_data(
			origin: OriginFor<T>,
			vector_ids: Vec<T::Hash>,
		) -> DispatchResult {
			let _caller = ensure_signed(origin)?;
			
			ensure!(!vector_ids.is_empty(), Error::<T>::NothingToPrune);
			
			let mut pruned = 0;
			for id in vector_ids {
				if Vectors::<T>::take(id).is_some() {
					pruned += 1;
				}
			}
			
			Self::deposit_event(Event::VectorsPruned { count: pruned });
			Ok(())
		}
	}

	/// Storage type definitions for the pallet
	/// WeightData is a tuple containing:
	/// - Vector data (bounded by MaxVectorLength)
	/// - Author's account ID
	/// - List of tag references (bounded by MaxTagsPerVector)
	type WeightData<T> = (
		BoundedVec<u8, <T as Config>::MaxVectorLength>,
		<T as frame_system::Config>::AccountId,
		BoundedVec<<T as frame_system::Config>::Hash, <T as Config>::MaxTagsPerVector>
	);

	/// Main storage for vector data, mapping vector IDs to WeightData
	#[pallet::storage]
	pub type Vectors<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, WeightData<T>>;

	/// Storage for tags, mapping tag hashes to tag content
	#[pallet::storage]
	pub type Tags<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, BoundedVec<u8, T::MaxTagLength>>;

	/// Storage mapping authors to their vectors
	#[pallet::storage]
	pub type AuthorVectors<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		<T as frame_system::Config>::AccountId,
		BoundedVec<T::Hash, T::MaxVectors>,
	>;
}
