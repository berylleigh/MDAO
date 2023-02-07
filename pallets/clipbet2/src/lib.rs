#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{*, DispatchResult, DispatchResultWithPostInfo},};
	use frame_system::{pallet_prelude::*, ensure_signed};
	use frame_support::inherent::Vec;
	// use codec::{Encode, Decode, EncodeLike, WrapperTypeEncode, EncodeAppend};

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

	
	#[derive( Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct ClipInfo {
		/// Username, stored as an array of bytes.
		pub cliphash: Vec<u8>,
		/// Numberid of the user.
		pub id: i64,
		pub description: Vec<u8>, 
		// The "About Me" section of the user
		// pub about_me: Vec<u8>,
	}

	pub type GroupIndex = u32;	

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	
	#[pallet::storage]
	#[pallet::getter(fn info)]
	// pub type AccountToClipInfo<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>, OptionQuery>;
	pub type AccountToClipInfo<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<T::AccountId>,  OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn member_score)]
	pub(super) type MemberScoreDmap<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		GroupIndex,
		Vec<u32>,
		ValueQuery,
	>;
	

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]

	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ClipStored (T::AccountId, Vec<u8>),
		/// Put member score (id, index, score)
		MemberJoinsGroup(T::AccountId, GroupIndex, u32),
			
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
		
		pub fn post_clip(
			origin: OriginFor<T>, 
			cliphash: Vec<u8>,
			)-> DispatchResult {	
			let sender = ensure_signed(origin)?;
			<AccountToClipInfo<T>>::append(&cliphash, &sender,);
			Self::deposit_event(Event::<T>::ClipStored(sender, cliphash));
			Ok(()) // used with -> DispatchResult
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn join_a_doublemap(
			origin: OriginFor<T>,
			index: GroupIndex,
			score: u32,
		) -> DispatchResultWithPostInfo {
			let member = ensure_signed(origin)?;
			<MemberScoreDmap<T>>::append(&member, &index, score);
			Self::deposit_event(Event::MemberJoinsGroup(member, index, score));
			Ok(().into())
		}

				
	}
	
}


