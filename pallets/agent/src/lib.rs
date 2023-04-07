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
	use sp_runtime::FixedI64;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The maximum length of string.
		#[pallet::constant]
		type MaxStringLength: Get<u32>;
		#[pallet::constant]
		type MaxArrayLength: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
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
	pub type UnitId<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub type Units<T: Config> = StorageMap<
		_,
		Twox64Concat,
		u32,
		Unit<T>,
	>;

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct SpatialThing<T: Config> {
		name: BoundedVec<u8, T::MaxStringLength>,
		note: Option<BoundedVec<u8, T::MaxStringLength>>,
		mappable_address: Option<BoundedVec<u8, T::MaxStringLength>>,
		lat: Option<FixedI64>,
		long: Option<FixedI64>,
		alt: Option<FixedI64>,
	}

	#[pallet::storage]
	pub type SpatialThingId<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub type SpatialThings<T: Config> = StorageMap<
		_,
		Twox64Concat,
		u32,
		SpatialThing<T>,
	>;

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct ProcessSpecification<T: Config> {
		name: BoundedVec<u8, T::MaxStringLength>,
		note: Option<BoundedVec<u8, T::MaxStringLength>>,
	}

	#[pallet::storage]
	pub type ProcessSpecificationId<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub type ProcessSpecifications<T: Config> = StorageMap<
		_,
		Twox64Concat,
		u32,
		ProcessSpecification<T>,
	>;

	// TODO use ipfs to store images
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct ResourceSpecification<T: Config> {
		name: BoundedVec<u8, T::MaxStringLength>,
		images: BoundedVec<T::Hash, T::MaxArrayLength>,
		note: Option<BoundedVec<u8, T::MaxStringLength>>,
		resource_classified_as: BoundedVec<BoundedVec<u8, T::MaxStringLength>, T::MaxArrayLength>,
		default_unit_of_resource_id: Option<u32>,
		default_unit_of_effort_id: Option<u32>,
	}

	#[pallet::storage]
	pub type ResourceSpecificationId<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub type ResourceSpecifications<T: Config> = StorageMap<
		_,
		Twox64Concat,
		u32,
		ResourceSpecification<T>,
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

		/// Create a spatial thing
		#[pallet::call_index(5)]
		#[pallet::weight(10_000)]
		pub fn create_spatial_thing(
			origin: OriginFor<T>,
			name: BoundedVec<u8, T::MaxStringLength>,
			note: Option<BoundedVec<u8, T::MaxStringLength>>,
			mappable_address: Option<BoundedVec<u8, T::MaxStringLength>>,
			lat: Option<FixedI64>,
			long: Option<FixedI64>,
			alt: Option<FixedI64>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Agents::<T>::contains_key(&who), Error::<T>::AgentIsNotRegistered);

			let spatial_thing_id = SpatialThingId::<T>::get();
			let spatial_thing = SpatialThing::<T> {
				name,
				note,
				mappable_address,
				lat,
				long,
				alt,
			};

			SpatialThings::<T>::insert(spatial_thing_id, spatial_thing);
			SpatialThingId::<T>::put(spatial_thing_id + 1);

			Ok(())
		}

		/// Update a spatial thing
		#[pallet::call_index(6)]
		#[pallet::weight(10_000)]
		pub fn update_spatial_thing(
			origin: OriginFor<T>,
			spatial_thing_id: u32,
			name: BoundedVec<u8, T::MaxStringLength>,
			note: Option<BoundedVec<u8, T::MaxStringLength>>,
			mappable_address: Option<BoundedVec<u8, T::MaxStringLength>>,
			lat: Option<FixedI64>,
			long: Option<FixedI64>,
			alt: Option<FixedI64>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Agents::<T>::contains_key(&who), Error::<T>::AgentIsNotRegistered);

			let spatial_thing = SpatialThing::<T> {
				name,
				note,
				mappable_address,
				lat,
				long,
				alt,
			};

			SpatialThings::<T>::insert(spatial_thing_id, spatial_thing);

			Ok(())
		}

		/// Delete a spatial thing
		#[pallet::call_index(7)]
		#[pallet::weight(10_000)]
		pub fn delete_spatial_thing(
			origin: OriginFor<T>,
			spatial_thing_id: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Agents::<T>::contains_key(&who), Error::<T>::AgentIsNotRegistered);

			SpatialThings::<T>::remove(spatial_thing_id);

			Ok(())
		}

		/// Create a process specification
		#[pallet::call_index(8)]
		#[pallet::weight(10_000)]
		pub fn create_process_specification(
			origin: OriginFor<T>,
			name: BoundedVec<u8, T::MaxStringLength>,
			note: Option<BoundedVec<u8, T::MaxStringLength>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Agents::<T>::contains_key(&who), Error::<T>::AgentIsNotRegistered);

			let process_spec_id = ProcessSpecificationId::<T>::get();
			let process_spec = ProcessSpecification::<T> {
				name,
				note,
			};

			ProcessSpecifications::<T>::insert(process_spec_id, process_spec);
			ProcessSpecificationId::<T>::put(process_spec_id + 1);

			Ok(())
		}

		/// Update a process specification
		#[pallet::call_index(9)]
		#[pallet::weight(10_000)]
		pub fn update_process_specification(
			origin: OriginFor<T>,
			process_spec_id: u32,
			name: BoundedVec<u8, T::MaxStringLength>,
			note: Option<BoundedVec<u8, T::MaxStringLength>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Agents::<T>::contains_key(&who), Error::<T>::AgentIsNotRegistered);

			let process_spec = ProcessSpecification::<T> {
				name,
				note,
			};

			ProcessSpecifications::<T>::insert(process_spec_id, process_spec);

			Ok(())
		}

		/// Delete a process specification
		#[pallet::call_index(10)]
		#[pallet::weight(10_000)]
		pub fn delete_process_specification(
			origin: OriginFor<T>,
			process_spec_id: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Agents::<T>::contains_key(&who), Error::<T>::AgentIsNotRegistered);

			ProcessSpecifications::<T>::remove(process_spec_id);

			Ok(())
		}

		/// Create a resource specification
		#[pallet::call_index(11)]
		#[pallet::weight(10_000)]
		pub fn create_resource_specification(
			origin: OriginFor<T>,
			name: BoundedVec<u8, T::MaxStringLength>,
			images: BoundedVec<T::Hash, T::MaxArrayLength>,
			note: Option<BoundedVec<u8, T::MaxStringLength>>,
			resource_classified_as: BoundedVec<BoundedVec<u8, T::MaxStringLength>, T::MaxArrayLength>,
			default_unit_of_resource_id: Option<u32>,
			default_unit_of_effort_id: Option<u32>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Agents::<T>::contains_key(&who), Error::<T>::AgentIsNotRegistered);

			let resource_spec_id = ResourceSpecificationId::<T>::get();
			let resource_spec = ResourceSpecification::<T> {
				name,
				images,
				note,
				resource_classified_as,
				default_unit_of_resource_id,
				default_unit_of_effort_id,
			};

			ResourceSpecifications::<T>::insert(resource_spec_id, resource_spec);
			ResourceSpecificationId::<T>::put(resource_spec_id + 1);

			Ok(())
		}

		/// Update a resource specification
		#[pallet::call_index(12)]
		#[pallet::weight(10_000)]
		pub fn update_resource_specification(
			origin: OriginFor<T>,
			resource_spec_id: u32,
			name: BoundedVec<u8, T::MaxStringLength>,
			images: BoundedVec<T::Hash, T::MaxArrayLength>,
			note: Option<BoundedVec<u8, T::MaxStringLength>>,
			resource_classified_as: BoundedVec<BoundedVec<u8, T::MaxStringLength>, T::MaxArrayLength>,
			default_unit_of_resource_id: Option<u32>,
			default_unit_of_effort_id: Option<u32>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Agents::<T>::contains_key(&who), Error::<T>::AgentIsNotRegistered);

			let resource_spec = ResourceSpecification::<T> {
				name,
				images,
				note,
				resource_classified_as,
				default_unit_of_resource_id,
				default_unit_of_effort_id,
			};

			ResourceSpecifications::<T>::insert(resource_spec_id, resource_spec);

			Ok(())
		}

		/// Delete a resource specification
		#[pallet::call_index(13)]
		#[pallet::weight(10_000)]
		pub fn delete_resource_specification(
			origin: OriginFor<T>,
			resource_spec_id: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Agents::<T>::contains_key(&who), Error::<T>::AgentIsNotRegistered);

			ResourceSpecifications::<T>::remove(resource_spec_id);

			Ok(())
		}

	}
}
