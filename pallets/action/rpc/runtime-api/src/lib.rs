//! Runtime API definition for valueflows action pallet.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use pallet_valueflows_action::Action;

sp_api::decl_runtime_apis! {
	pub trait ActionRuntimeApi {
		fn all_actions() -> Result<Vec<Action>>
	}
}
