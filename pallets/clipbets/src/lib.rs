#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;

#[cfg(test)]
mod mock;

// #[cfg(test)]
// mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// use frame_support::storage::StorageAppend;
// pub type JustWork = Vec<u8> as StorageAppend<T>;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{*, DispatchResult, DispatchResultWithPostInfo}, storage::PrefixIterator};
	use frame_system::{pallet_prelude::*, ensure_signed};
	use frame_support::inherent::Vec;
	// use crate::pallet::storage::StorageAppend;
	use codec::{Encode, Decode, EncodeLike, WrapperTypeEncode, EncodeAppend};

	
	

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


	// pub fn encode_like<T: Encode, R: EncodeLike<T>>(data: &R) {
	// 	data.encode(); // Valid `T` encoded value.
	// }
	// #[derive( Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	// pub struct ClipInfo {
	// 	/// Username, stored as an array of bytes.
	// 	pub cliphash: Vec<u8>,
	// 	/// Numberid of the user.
	// 	pub id: i64,
	// 	pub description: Vec<u8>, 
	// 	// The "About Me" section of the user
	// 	// pub about_me: Vec<u8>,
	// }

	// impl core::ops::Deref for ClipInfo{ 
	// 	type Target = (u8,i64,u8);
	// 	fn deref(&self) -> &Self::Target { &self.0 }
	// }

	// impl EncodeLike<(u8,i64,u8)> for ClipInfo {}
	// impl WrapperTypeEncode for ClipInfo {}

	
   
	
		

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn info)]
	
	// pub type AccountToClipInfo<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ClipInfo, OptionQuery>;
	// pub type AccountToClipInfo<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, Vec<u8>, Vec<u8>, OptionQuery>;
	
	// pub type AccountToClipInfo<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;

	// pub type AccountToClipInfo<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>, OptionQuery>;
	pub type AccountToClipInfo<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>, OptionQuery>;

	
	



	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]

	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ClipStored {user: T::AccountId},
		
		// ClipStored (T::AccountId, Vec<u8>, Vec<u8>),
		// ClipStored (T::AccountId, Vec<u8>),

		GotStuff (T::AccountId),
		// GotStuff (core::option::Option<(Vec<u8>, Vec<u8>)>),
			
	}
	
	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}


	
	#[pallet::call]

	impl<T: Config> Pallet<T> {
		
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		//  pub fn register_user(origin: OriginFor<T>, cliphash: Vec<u8>, id: i64, description: Vec<u8>)->
		
		
		
		pub fn post_clip(
			origin: OriginFor<T>, 
				
			// cliphash: BoundedVec<u8,u32>,
			cliphash: Vec<u8>,
			// )-> DispatchResultWithPostInfo {
			)-> DispatchResult {	
			let sender = ensure_signed(origin)?;

			// let new_clip = ClipInfo{cliphash, id, description};
			// encode_like::<(u8,i64,u8), _>(&ClipInfo{cliphash, id, description});
					
			// <AccountToClipInfo<T>>::insert(&sender, new_clip);
			// Value is required to implement codec::EncodeAppend.
			<AccountToClipInfo<T>>::insert(&sender, &cliphash);
		
			Self::deposit_event(Event::<T>::ClipStored{user: sender});
			// Self::deposit_event(Event::<T>::ClipStored(sender, description, cliphash));
			// Self::deposit_event(Event::<T>::ClipStored(sender, description));


			// Ok(().into()) // used with -> DispatchResultWithPostInfo
			Ok(()) // used with -> DispatchResult
		}

		// #[pallet::call_index(1)]
		// #[pallet::weight(10_000)]
		// //  pub fn register_user(origin: OriginFor<T>, cliphash: Vec<u8>, id: i64, about_me: Vec<u8>)->
		
		// pub fn get_clip_stuff(
		// 	origin: OriginFor<T>,
		// 	// cliphash: Vec<u8>, 
		// 	// description: Vec<u8>
		// // )-> DispatchResult{
		// )-> DispatchResultWithPostInfo {
		// 	let sender = ensure_signed(origin)?;
			
		// 	let mut stuff: PrefixIterator<(Vec<u8>, Vec<u8>)> = <AccountToClipInfo<T>>::iter_prefix(&sender);
			
		// 	Self::deposit_event(Event::<T>::GotStuff(stuff.next()));
		// 	// Self::deposit_event(Event::<T>::GotStuff(stuff.count()));
			


		// 	Ok(().into())
		// }
		

	}
	
}
