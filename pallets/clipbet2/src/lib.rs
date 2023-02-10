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
	use frame_support::{pallet_prelude::{*, DispatchResult, DispatchResultWithPostInfo, OptionQuery},};
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
		pub betterval: u64,
	}

	

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	
	#[pallet::storage]
	#[pallet::getter(fn clips)]
	pub type AllClips<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn bets)]
	pub type BetRound<T: Config> = StorageNMap<
		Key = (NMapKey<Blake2_128Concat, u64>, 
				NMapKey<Blake2_128Concat, Vec<u8>>,
				NMapKey<Blake2_128Concat, T::AccountId>,),
		Value = BetInfo<T>,
		QueryKind = OptionQuery,
		// MaxValues = ConstU32<11>,
	// _,
	// Blake2_128Concat, 
	// u64,  
	// Blake2_128Concat, 
	// Vec<u8>, 
	// Vec<BetInfo<T>>, 
	// OptionQuery,
	>;
	
	

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]

	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ClipStored (T::AccountId, Vec<u8>),
		ClipRemoved(Vec<u8>),
		BetPlaced(Vec<u8>),
		
		
			
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
			value: u64,
			)-> DispatchResult {	
			let bettr = ensure_signed(origin)?;
			// import the clip owner from the allClips map then add all details to BetRound map 
			// let owner = <AllClips<T>>::get(&cliphash).unwrap(); -cheats by unwraping the option.  Use match
			match Self::clips(&cliphash) {
				// If the getter function returns None - there is no clip with this hash
				None => return Err(Error::<T>::NoClip.into()),
				// If the getter function option returns some, 
				Some(acid) => {
					//check to see if this clip has already been bet on by this better in this round. If so,add the value to the existing betterval. Self::all_members().contains(who)
					let mut final_val:u64 = value;
					match Self::bets ((&roundid, &cliphash, &bettr)){
						
						Some(mut add_bet) => { 
							add_bet.betterval += value;
							final_val = add_bet.betterval;
						}

						None => { 
							
						}
						
					}

					let new_bet:BetInfo<T> = BetInfo {owner: acid, betterval: final_val, };
					<BetRound<T>>::insert((&roundid, &cliphash, &bettr), new_bet,);
					
					Self::deposit_event(Event::<T>::BetPlaced(cliphash));
					Ok(()) // used with -> DispatchResult
							
				}
			}
						
		}
				
	}
	
}


