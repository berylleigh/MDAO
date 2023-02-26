#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;


#[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::{pallet_prelude::{*, DispatchResult, OptionQuery}, dispatch::{Dispatchable, GetDispatchInfo}, PalletId, traits:: {Currency, ReservableCurrency, ExistenceRequirement::KeepAlive,}};
use frame_system::{pallet_prelude::*, ensure_signed};
use frame_support::inherent::Vec;
// use frame_support::dispatch::fmt::Debug;
use sp_runtime::{traits::{Zero, AccountIdConversion,}, Saturating, print, Perbill, };
// use LooseInterface::LooseInterface;

// use sp_runtime::traits::Printable;
use log::{info, debug};

use codec::{Compact, Encode};
// // use sp_core::{blake2_256, H256};

// use sp_runtime::{generic::Era, MultiAddress, MultiSignature};
use sp_runtime::{MultiAddress,};
use scale_info::prelude::format;
// use sp_std::vec;




// use subxt::{
//     tx::PairSigner,
//     OnlineClient,
//     PolkadotConfig,
// };



type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// #[derive( Encode, Decode, Clone, PartialEq, Default, TypeInfo, Debug,)]
#[derive( Encode, Decode, Clone, PartialEq, Default, TypeInfo,)]
#[scale_info(skip_type_params(T))]
pub struct BetInfo<T: Config, Balance> {
	pub owner: T::AccountId,
	pub better: T::AccountId,
	pub betval: Balance,
	
	}


#[frame_support::pallet] 
pub mod pallet {
	use super::*;
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_utility::Config {
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
		/// A dispatchable call.
		type RuntimeCall: Parameter
			+ Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo
			+ From<frame_system::Call<Self>>;
	}

	// type ClassIdOf<T> = <<T as Config>::Interface as LooseInterface>::ClassId;

	
		// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	
	#[pallet::storage]
	#[pallet::getter(fn clips)]
	pub type AllClips<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn bets)]
	pub type BetsDoubleMap <T: Config> = StorageDoubleMap <
	_, Blake2_128Concat, u64, Blake2_128Concat, Vec<u8>, Vec <BetInfo<T, BalanceOf<T>>>, OptionQuery,>;
	
	// All this adds a zero in the type BalanceOf<T https://substrate.stackexchange.com/questions/13/how-do-you-convert-between-substrate-specific-types-and-rust-primitive-types 	
	#[pallet::type_value]	
	pub fn ZeroDefault<T: Config>() -> BalanceOf<T> { 
		u32_to_balance::<T>(0)
	}
	pub fn u32_to_balance<T: Config>(input: u32) -> BalanceOf<T> {
		input.into()
	}
		
	#[pallet::storage]
	#[pallet::getter(fn roundtally)]
	pub type RoundTally<T: Config> = StorageMap<_, Blake2_128Concat, u64, BalanceOf<T>, ValueQuery, ZeroDefault<T>>;	
	
	#[pallet::storage]
	#[pallet::getter(fn roundcliptally)]
	pub type RoundClipTally<T: Config> = StorageDoubleMap<_, Blake2_128Concat, u64, Blake2_128Concat, Vec<u8>,  BalanceOf<T>, ValueQuery, ZeroDefault<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]

	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ClipStored (T::AccountId, Vec<u8>),

		ClipRemoved(Vec<u8>),

		BetPlaced(Vec<u8>, BalanceOf<T>),

		SongStarted (T::AccountId, Vec<u8>, T::AccountId, BalanceOf<T>),

		RewardsSent(Vec<BalanceOf<T>>),

		// Thisbetvec(Vec<BetInfo<T, <<T as pallet::Config>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance>>)
					
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
		/// There are no bets for this clip.
		NoBetsOnClip,
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
		#[pallet::weight(10_000)]
		pub fn place_bet(
			origin: OriginFor<T>, 
			cliphash: Vec<u8>,
			roundid: u64,
			value: BalanceOf<T>,
			)-> DispatchResult {	
			let bettr = ensure_signed(origin)?;
			// import the clip owner from the allClips map then add all details to BetsDoubleMap 
			
			match Self::clips(&cliphash) {
				// If the getter function returns None - there is no clip with this hash
				None => return Err(Error::<T>::NoClip.into()),
				// If the getter function option returns some, 
				Some(acid) => {
					// Add the bet value to roundtally StoragMap
					let new_roundtally = Self::roundtally(&roundid) + value;
					<RoundTally<T>>::insert(&roundid, &new_roundtally);

					// add the bet value to the roundclip DoubleMap
					let new_roundcliptally = Self::roundcliptally(&roundid, &cliphash) + value;
					<RoundClipTally<T>>::insert(&roundid, &cliphash, &new_roundcliptally);

					// Check user has enough funds and send it to the Lottery account.
					T::Currency::transfer(&bettr, &Self::account_id(), value, KeepAlive)?;

					let new_bet:BetInfo<T, BalanceOf<T>> = BetInfo {owner: acid, better: bettr, betval: value, };
					
					<BetsDoubleMap<T>>::append(&roundid, &cliphash, new_bet,);

					Self::deposit_event(Event::<T>::BetPlaced(cliphash, value));
					Ok(()) // used with -> DispatchResult
							
				}
			}
						
		}

		// calculate the proportional ratio each better contributed to roundcliptally. Going to have to itterate over all betters fot the winning cliphash to get each better and value? do calculation and transfer in one step?  An alternative would be to have users do a manual extrinsic to retrieve their winnings.

			// rather than storageNMap, use a double map and append each bet in a Vec of structs containing the bet value and better under roundid k1 and cliphash k2. 
			// I can then use get() to get the whole Vec to delegate rewards.   
			// Then I can convert each transfer to a RuntimeCall and send a batch of transfers with the batch_all function of the Utility pallet formatted:  Vec<<T as Config>::RuntimeCall>

			// pub struct BetInfo<T: Config, Balance> {
			// 	pub owner: T::AccountId,
			// 	pub better: T::AccountId,
			// 	pub betval: Balance,}

		#[pallet::call_index(6)]
		#[pallet::weight(10_000)]
		pub fn choose_winner_clip(
			origin: OriginFor<T>, 
			cliphash: Vec<u8>,
			roundid: u64,
			)-> DispatchResult {	
			let picker = ensure_signed(origin)?;
			// ensure this cliphash has bets-this works as the default value of the doublemap is zero.
			ensure!(!Self::roundcliptally(&roundid, &cliphash).is_zero(), Error::<T>::NoBetsOnClip);
			//check to see if there are already bets in this round, on this clip, by this better.   If so, combine them. - do this after bets locked as part of the wins distribution. 
		let mut rewardsvec:Vec<BalanceOf<T>> = Vec::new();
		let rctally = Self::roundcliptally(&roundid, &cliphash);
		let rtally = Self::roundtally(&roundid);
		
			match Self::bets(&roundid, &cliphash){
				Some(betsvec) => { 
					// log::info!("betsvec = {:?}", betsvec);
					// get the ratio of each bet value to the roundcliptally and then * it by the roundtally
					// Make a Vec of tuple structs made of betters and their reward values
										
				// let better_exposure_part = Perbill::from_rational(x.betval, rctally);
				// https://substrate.stackexchange.com/questions/6147/balance-division-in-substrate-runtime	
				rewardsvec = betsvec.iter().map(|x| (Perbill::from_rational(x.betval, rctally))*rtally).collect();					
					}
				None => {}	
			}
			// pub struct BetInfo<T: Config, Balance> {
			// 	pub owner: T::AccountId,
			// 	pub better: T::AccountId,
			// 	pub betval: Balance,}
			log::info!("roundcliptally = {:?}", Self::roundcliptally(&roundid, &cliphash));
			log::info!("roundtally = {:?}", Self::roundtally(&roundid));
			debug!("roundtally = {:?}", Self::roundtally(&roundid));
			log::info!("rewardsvec = {:?}", rewardsvec);

			//Construct a vec of RuntimeCalls - calls: Vec<<T as Config>::RuntimeCall>	to send to the utility pallet batch() function.	
			let pallet_index: u8 = 6;
			let call_index: u8 = 0;
		   	// The "transfer" call takes 2 arguments, which are as follows (if we wanted, we could
			// avoid using `MultiAddress` and encode a 0 u8 and then the account ID, but for simplicity..)
			// let address = MultiAddress::Id::<_, u32>(AccountKeyring::Bob.to_account_id());
			let address = MultiAddress::Id::<_, u32>(picker);

			let balance = Compact::from(2000000000000u128);
		
			// We put the above data together and now we have something that will encode to the
			// Same shape as the generated enum would have led to (variant indexes, then args):
			let call = (pallet_index, call_index, address, balance);
			// println!("Call: {:?}", call);
		
			let mycall = &call.encode();
			let mycall_hex = format!("0x{}", hex::encode(&mycall));
			// let mycall_hex = hex_literal::hex![String::from(mycall)].into();
			// let mycall_hex = hex::encode(&mycall);
			info!("mycallhex: {:?}", mycall_hex);
		

			let _calls = sp_std::vec![mycall_hex];


			// calls: Vec<<T as Config>::RuntimeCall>
	
			// let ding:DispatchResultWithPostInfo = pallet_utility::Pallet::<T>::batch(origin, calls);
			
			


			// Check pot account has enough funds and send it to the winning addresses.
			
			// for (roundid, cliphash) in <BetsDoubleMap<T>>::drain_prefix(&collection) {}

			// let betval = Self::bets ((&roundid, &cliphash, &bettr)).unwrap();
			// let clipval = Self::roundcliptally(&roundid, &cliphash);
			// T::Currency::transfer(&Self::account_id(), &bettr, betval / clipval , KeepAlive)?;
					
			// clear the roundtally StoragMap
					// let new_roundtally = Self::roundtally(&roundid) ;
					// <RoundTally<T>>::clear();

					// clear the roundcliptally DoubleMap
					// let new_roundcliptally = Self::roundcliptally(&roundid, &cliphash) ;
					// <RoundClipTally<T>>::insert(&roundid, &cliphash, &new_roundcliptally);

					//check to see if there are already bets in this round, on this clip, by this better.   If so,add the value to the existing betval. 
				
					// let new_bet:BetInfo<T, BalanceOf<T>> = BetInfo {owner: acid, betval: final_val, };
					// <BetsDoubleMap<T>>::insert((&roundid, &cliphash, &picker), new_bet,);

					// Print a message
					print("Hello World");
					
					
					
					// Self::deposit_event(Event::<T>::RewardsSent(rewardsvec));
					
					Ok(()) // used with -> DispatchResult
							
			}

		
					
	}
	
}




// type BalanceOf<T> =
	// <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

impl<T: Config> Pallet<T> {
	
	/// The account ID of the clipbet pot.
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