#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

// #[cfg(test)]
// mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::{*, DispatchResult};
	use frame_system::{pallet_prelude::*, ensure_signed};
	use frame_support::inherent::Vec;
	// use sp_std::prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct UserInfo {
		/// Username, stored as an array of bytes.
		pub username: Vec<u8>,
		/// Numberid of the user.
		pub id: i64,
		/// The "About Me" section of the user
		pub about_me: Vec<u8>,
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn info)]
	pub type AccountToUserInfo<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, UserInfo, OptionQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		UserCreated {user: T::AccountId},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_user(origin: OriginFor<T>, username: Vec<u8>, id: i64, about_me: Vec<u8>)-> DispatchResult {
			let sender = ensure_signed(origin)?;
			let new_user = UserInfo{username, id, about_me};
			<AccountToUserInfo<T>>::insert(&sender, new_user);
			Self::deposit_event(Event::<T>::UserCreated{user: sender});

			Ok(())
		}
	}
}
