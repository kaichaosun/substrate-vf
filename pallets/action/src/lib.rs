#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use sp_std::prelude::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub struct Action {
	pub id: Vec<u8>, // todo use enum, and convert to string in client
	pub label: Vec<u8>,
	pub resource_effect: ActionEffect,
	pub input_output: ProcessType,
	pub pairs_with: Vec<u8>,
}

pub enum ActionEffect {
	// for 'process' events
	NoEffect,
	Increment,
	Decrement,
	// for 'transfer' events
	DecrementIncrement,
}

pub enum ProcessType {
	NotApplicable,
	Input,
	Output,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	}

	#[pallet::error]
	pub enum Error<T> {
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// TODO Get an action by id, should be RPC
		#[pallet::weight(10_000)]
		pub fn get_action(origin: OriginFor<T>, _id: Vec<u8>) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			Ok(())
		}

		/// Get all actions, should use RPC instead
		#[pallet::weight(10_000)]
		pub fn get_all_actions(origin: OriginFor<T>) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			Ok(())
		}
	}
}
