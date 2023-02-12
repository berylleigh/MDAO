#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;


#[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::{pallet_prelude::{*, DispatchResult, OptionQuery}, PalletId, traits:: {Currency, ReservableCurrency}};
use frame_system::{pallet_prelude::*, ensure_signed};
use frame_support::inherent::Vec;
use sp_runtime::{traits::Zero, Saturating};
// use codec::{Encode, Decode, EncodeLike, WrapperTypeEncode, EncodeAppend};
type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive( Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct BetInfo<T: Config> {
	pub owner: T::AccountId,
	pub betterval: u64,
	}


#[frame_support::pallet] 
pub mod pallet {
	use super::*;
	// use frame_support::{pallet_prelude::{*, DispatchResult, DispatchResultWithPostInfo, OptionQuery},};
	// use frame_support::{pallet_prelude::{*, DispatchResult, OptionQuery}, PalletId, traits:: {Currency, ReservableCurrency}};
	// use frame_system::{pallet_prelude::*, ensure_signed};
	// use frame_support::inherent::Vec;
	// use sp_runtime::traits::Zero;
	// use codec::{Encode, Decode, EncodeLike, WrapperTypeEncode, EncodeAppend};
	// type BalanceOf<T> =
	// <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Get the PalletId
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;
		/// The max number of calls available in a single bet.
		#[pallet::constant]
		type MaxCalls: Get<u32>;
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
	pub type BetsNMap<T: Config> = StorageNMap<
		Key = (NMapKey<Blake2_128Concat, u64>, 
				NMapKey<Blake2_128Concat, Vec<u8>>,
				NMapKey<Blake2_128Concat, T::AccountId>,),
		Value = BetInfo<T>,
		QueryKind = OptionQuery,
		// OnEmpty = ZeroDefault,
		// MaxValues = ConstU32<11>,
		>;

	#[pallet::type_value]	
	pub fn ZeroDefault() -> u64 {0}
		
	#[pallet::storage]
	#[pallet::getter(fn roundtally)]
	pub type RoundTally<T: Config> = StorageMap<_, Blake2_128Concat, u64, u64, ValueQuery, ZeroDefault>;	
	
	#[pallet::storage]
	#[pallet::getter(fn roundcliptally)]
	pub type RoundClipTally<T: Config> = StorageDoubleMap<_, Blake2_128Concat, u64, Blake2_128Concat, Vec<u8>,  u64, ValueQuery, ZeroDefault>;

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

		SongStarted (T::AccountId, Vec<u8>, T::AccountId, BalanceOf<T>),
					
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

		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn start_song (
			origin: OriginFor<T>, 
			cliphash: Vec<u8>,
			)-> DispatchResult {	
			let sender = ensure_signed(origin)?;
			//check to see if this cliphash key has been added already to the song
			// ensure!(!<AllClips<T>>::contains_key(&cliphash), Error::<T>::DuplicateClip);
			// <AllClips<T>>::insert(&cliphash, &sender,);

			// Make sure pot exists. assign variable to accound id
			let clipbet_account = Self::account_id();
			if T::Currency::total_balance(&clipbet_account).is_zero() {
			T::Currency::deposit_creating(&clipbet_account, T::Currency::minimum_balance());
			}	

			let potshow = Self::pot();
			Self::deposit_event(Event::<T>::SongStarted(sender, cliphash, potshow.0, potshow.1 ));
			Ok(()) // used with -> DispatchResult
						
		}
		

		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn place_bet(
			origin: OriginFor<T>, 
			cliphash: Vec<u8>,
			roundid: u64,
			value: u64,
			)-> DispatchResult {	
			let bettr = ensure_signed(origin)?;
			// import the clip owner from the allClips map then add all details to BetsNMap 
			// let owner = <AllClips<T>>::get(&cliphash).unwrap(); -cheats by unwraping the option.  Use match
			match Self::clips(&cliphash) {
				// If the getter function returns None - there is no clip with this hash
				None => return Err(Error::<T>::NoClip.into()),
				// If the getter function option returns some, 
				Some(acid) => {
					// Add the bet value to roundtally StoragMap
					let new_roundtally = Self::roundtally(&roundid) + value;
					<RoundTally<T>>::insert(&roundid, new_roundtally);

					// add the bet value to the roundclip DoubleMap
					let new_roundcliptally = Self::roundcliptally(&roundid, &cliphash) + value;
					<RoundClipTally<T>>::insert(&roundid, &cliphash, new_roundcliptally);

					//check to see if there are already bets in this round, on this clip, by this better.   If so,add the value to the existing betterval. 
					let mut final_val:u64 = value;
					match Self::bets ((&roundid, &cliphash, &bettr)){
						Some(add_bet) => { 
							//Add the bet additional bet value to the final better value.
							final_val += add_bet.betterval;
							}
						None => {}	
					}

					let new_bet:BetInfo<T> = BetInfo {owner: acid, betterval: final_val, };
					<BetsNMap<T>>::insert((&roundid, &cliphash, &bettr), new_bet,);

					

					Self::deposit_event(Event::<T>::BetPlaced(cliphash));
					Ok(()) // used with -> DispatchResult
							
				}
			}
						
		}
				
	}
	
}

use frame_support::traits::Get;
use sp_runtime::traits::AccountIdConversion;
// type BalanceOf<T> =
	// <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

impl<T: Config> Pallet<T> {
	
	/// The account ID of the lottery pot.
	///
	/// This actually does computation. If you need to keep using it, then make sure you cache the
	/// value and only call this once.
	pub fn account_id() -> T::AccountId {
		T::PalletId::get().into_account_truncating()
	}

	// / Return the pot account and amount of money in the pot.
	// / The existential deposit is not part of the pot so lottery account never gets deleted.
	pub fn pot() -> (T::AccountId, BalanceOf<T>) {
		let account_id = Self::account_id();
		let balance =
			T::Currency::free_balance(&account_id).saturating_sub(T::Currency::minimum_balance());

		(account_id, balance)
	}


	
}