#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The maximum length of string.
		// #[pallet::constant]
		// type MaxStringLength: Get<u32>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn agents)]
	pub type Agents<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		bool,
	>;

	// #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, scale_info::TypeInfo)]
	// pub struct Unit<T: Config> {
	// 	label: BoundedVec<u8, T::MaxStringLength>,
	// 	symbol: BoundedVec<u8, T::MaxStringLength>,
	// }

	// #[pallet::storage]
	// pub type Units<T: Config> = StorageMap<
	// 	_,
	// 	Twox64Concat,
	// 	T::AccountId,
	// 	Unit<T>,
	// >;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Register an agent with success
		AgentRegistered(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		AgentAlreadyRegistered,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// TODO Get agent public key, should be RPC instead or from client side
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn get_my_agent_pubkey(origin: OriginFor<T>) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			Ok(())
		}

		/// Register agent
		/// TODO instead of put it in a vector, should better use a map
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn register_agent(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!Agents::<T>::contains_key(&who), Error::<T>::AgentAlreadyRegistered);

			Agents::<T>::insert(&who, true);

			Self::deposit_event(Event::AgentRegistered(who));

			Ok(())
		}
	}
}
