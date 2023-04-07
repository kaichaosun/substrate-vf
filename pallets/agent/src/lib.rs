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
	use scale_info::TypeInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The maximum length of string.
		#[pallet::constant]
		type MaxStringLength: Get<u32>;
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

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Unit<T: Config> {
		label: BoundedVec<u8, T::MaxStringLength>,
		symbol: BoundedVec<u8, T::MaxStringLength>,
	}

	#[pallet::storage]
	#[pallet::getter(fn unit_id)]
	pub type UnitId<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn units)]
	pub type Units<T: Config> = StorageMap<
		_,
		Twox64Concat,
		u32,
		Unit<T>,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Register an agent with success
		AgentRegistered(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		AgentAlreadyRegistered,
		AgentIsNotRegistered,
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

		/// Create an unit
		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn create_unit(
			origin: OriginFor<T>,
			label: BoundedVec<u8, T::MaxStringLength>,
			symbol: BoundedVec<u8, T::MaxStringLength>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Agents::<T>::contains_key(&who), Error::<T>::AgentIsNotRegistered);

			let unit_id = UnitId::<T>::get();
			let unit = Unit::<T> {
				label,
				symbol,
			};

			Units::<T>::insert(unit_id, unit);
			UnitId::<T>::put(unit_id + 1);

			Ok(())
		}

		/// Update an unit
		#[pallet::call_index(3)]
		#[pallet::weight(10_000)]
		pub fn update_unit(
			origin: OriginFor<T>,
			unit_id: u32,
			label: BoundedVec<u8, T::MaxStringLength>,
			symbol: BoundedVec<u8, T::MaxStringLength>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Agents::<T>::contains_key(&who), Error::<T>::AgentIsNotRegistered);

			let unit = Unit::<T> {
				label,
				symbol,
			};

			Units::<T>::insert(unit_id, unit);

			Ok(())
		}

		/// Delete an unit
		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		pub fn delete_unit(origin: OriginFor<T>, unit_id: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Agents::<T>::contains_key(&who), Error::<T>::AgentIsNotRegistered);

			Units::<T>::remove(unit_id);

			Ok(())
		}
	}
}
