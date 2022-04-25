//! Runtime API definition for valueflows action pallet.

#![cfg_attr(not(feature = "std"), no_std)]

use pallet_valueflows_action::Action;
use sp_std::prelude::Vec;

sp_api::decl_runtime_apis! {
	pub trait ActionRuntimeApi {
		fn all_actions() -> Vec<Action>;
	}
}
