#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
pub use pallet::*;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(Encode, Decode, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Action {
	pub id: Vec<u8>, // todo use enum, and convert to string in client
	pub label: Vec<u8>,
	pub resource_effect: ActionEffect,
	pub input_output: ProcessType,
	pub pairs_with: Vec<u8>,
}

#[derive(Encode, Decode, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ActionEffect {
	// for 'process' events
	NoEffect,
	Increment,
	Decrement,
	// for 'transfer' events
	DecrementIncrement,
}

#[derive(Encode, Decode, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
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
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {}

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

impl<T: Config> Pallet<T> {
	pub fn all_actions() -> Vec<Action> {
		vec![
			Action {
				id: "dropoff".into(),
				label: "dropoff".into(),
				resource_effect: ActionEffect::Increment,
				input_output: ProcessType::Output,
				pairs_with: "pickup".into(),
			},
			Action {
				id: "pickup".into(),
				label: "pickup".into(),
				resource_effect: ActionEffect::Decrement,
				input_output: ProcessType::Input,
				pairs_with: "dropoff".into(),
			},
			Action {
				id: "consume".into(),
				label: "consume".into(),
				resource_effect: ActionEffect::Decrement,
				input_output: ProcessType::Input,
				pairs_with: "notApplicable".into(),
			},
			Action {
				id: "use".into(),
				label: "use".into(),
				resource_effect: ActionEffect::NoEffect,
				input_output: ProcessType::Input,
				pairs_with: "notApplicable".into(),
			},
			Action {
				id: "work".into(),
				label: "work".into(),
				resource_effect: ActionEffect::NoEffect,
				input_output: ProcessType::Input,
				pairs_with: "notApplicable".into(),
			},
			Action {
				id: "cite".into(),
				label: "cite".into(),
				resource_effect: ActionEffect::NoEffect,
				input_output: ProcessType::Input,
				pairs_with: "notApplicable".into(),
			},
			Action {
				id: "produce".into(),
				label: "produce".into(),
				resource_effect: ActionEffect::Increment,
				input_output: ProcessType::Output,
				pairs_with: "notApplicable".into(),
			},
			Action {
				id: "accept".into(),
				label: "accept".into(),
				resource_effect: ActionEffect::NoEffect,
				input_output: ProcessType::Input,
				pairs_with: "modify".into(),
			},
			Action {
				id: "modify".into(),
				label: "modify".into(),
				resource_effect: ActionEffect::NoEffect,
				input_output: ProcessType::Output,
				pairs_with: "accept".into(),
			},
			Action {
				id: "pass".into(),
				label: "pass".into(),
				resource_effect: ActionEffect::NoEffect,
				input_output: ProcessType::Output,
				pairs_with: "accept".into(),
			},
			Action {
				id: "fail".into(),
				label: "fail".into(),
				resource_effect: ActionEffect::NoEffect,
				input_output: ProcessType::Output,
				pairs_with: "accept".into(),
			},
			Action {
				id: "deliver_service".into(),
				label: "deliver_service".into(),
				resource_effect: ActionEffect::NoEffect,
				input_output: ProcessType::Output,
				pairs_with: "notApplicable".into(),
			},
			Action {
				id: "transfer_all_rights".into(),
				label: "transfer_all_rights".into(),
				resource_effect: ActionEffect::DecrementIncrement,
				input_output: ProcessType::NotApplicable,
				pairs_with: "notApplicable".into(),
			},
			Action {
				id: "transfer_custody".into(),
				label: "transfer_custody".into(),
				resource_effect: ActionEffect::DecrementIncrement,
				input_output: ProcessType::NotApplicable,
				pairs_with: "notApplicable".into(),
			},
			Action {
				id: "transfer".into(),
				label: "transfer".into(),
				resource_effect: ActionEffect::DecrementIncrement,
				input_output: ProcessType::NotApplicable,
				pairs_with: "notApplicable".into(),
			},
			Action {
				id: "move".into(),
				label: "move".into(),
				resource_effect: ActionEffect::DecrementIncrement,
				input_output: ProcessType::NotApplicable,
				pairs_with: "notApplicable".into(),
			},
			Action {
				id: "raise".into(),
				label: "raise".into(),
				resource_effect: ActionEffect::Increment,
				input_output: ProcessType::NotApplicable,
				pairs_with: "notApplicable".into(),
			},
			Action {
				id: "lower".into(),
				label: "lower".into(),
				resource_effect: ActionEffect::Decrement,
				input_output: ProcessType::NotApplicable,
				pairs_with: "notApplicable".into(),
			},
		]
	}
}
