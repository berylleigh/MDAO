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
	#[scale_info(skip_type_params(T))]
	pub struct BetInfo<T: Config> {
		pub owner: T::AccountId,
		pub better: T::AccountId,
		pub roundid: u64,
		pub description: Vec<u8>, 
		pub betterval: u64,
	}

	pub type GroupIndex = u32;	

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	
	#[pallet::storage]
	#[pallet::getter(fn clips)]
	pub type AllClips<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn bets)]
	pub type BetRound<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<BetInfo<T>>, OptionQuery>;
	// pub type BetRound<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<T::AccountId>,  OptionQuery>;


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
		// ClipStored (T::AccountId, Vec<u8>),
		ClipStored (T::AccountId, Vec<u8>),
		ClipRemoved(Vec<u8>),
		BetPlaced(Vec<u8>),
		/// Put member score (id, index, score)
		MemberJoinsGroup(T::AccountId, GroupIndex, u32),
			
	}
	
	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Someone has already entered this clip.
		DuplicateClip,
		/// There is no clip with this hash ID
		NoClip,
		/// You didn't post this clip
		NotYours,
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
			//check to see if this cliphash key has been added already
			ensure!(!<AllClips<T>>::contains_key(&cliphash), Error::<T>::DuplicateClip);
			<AllClips<T>>::insert(&cliphash, &sender,);

			Self::deposit_event(Event::<T>::ClipStored(sender, cliphash));
			Ok(()) // used with -> DispatchResult
						
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn rm_clip(
			origin: OriginFor<T>, 
			cliphash: Vec<u8>,
			)-> DispatchResult {	
			let sender = ensure_signed(origin)?;
			
			match Self::clips(&cliphash) {
				// If the getter function returns None - there is no clip with this hash
				None => return Err(Error::<T>::NoClip.into()),
				// If the getter function option returns some, 
				Some(acid) => {
					// ensure the accountID of sender matches the owner
					ensure!(acid == sender, Error::<T>::NotYours);
					// If all ok - remove the value from the storageMap
					<AllClips<T>>::remove(&cliphash);
					Self::deposit_event(Event::<T>::ClipRemoved(cliphash));
					Ok(())
							
				}
			}
		}
		// BetRound

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn place_bet(
			origin: OriginFor<T>, 
			cliphash: Vec<u8>,
			roundid: u64,
			description: Vec<u8>,
			value: u64,
			)-> DispatchResult {	
			let better = ensure_signed(origin)?;
			// import the clip owner from the allClips map then add all details to BetRound map 
			// let owner = <AllClips<T>>::get(&cliphash).unwrap();
			match Self::clips(&cliphash) {
				// If the getter function returns None - there is no clip with this hash
				None => return Err(Error::<T>::NoClip.into()),
				// If the getter function option returns some, 
				Some(acid) => {
					let new_bet = BetInfo {owner :acid, better , roundid, description, betterval: value, };
					//check to see if this cliphash key has been bet on by the better already if so - just add the value to betterval.
					<BetRound<T>>::append(&cliphash, new_bet,);
					Self::deposit_event(Event::<T>::BetPlaced(cliphash));
					Ok(()) // used with -> DispatchResult
							
				}
			}

			
						
		}

		// pub owner: Vec<u8>,
		// // pub better: T::AccountId,
		// pub better: Vec<u8>,
		// pub roundid: u64,
		// pub description: Vec<u8>, 
		// pub betterval: u64,


		#[pallet::call_index(4)]
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


