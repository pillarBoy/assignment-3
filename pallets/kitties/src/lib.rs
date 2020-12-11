#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{
	decl_module, decl_storage, decl_event, decl_error,
	traits::Randomness, RuntimeDebug,
	ensure,
};
use sp_io::hashing::blake2_128;
use frame_system::{ ensure_signed };

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub enum Gender {
	Female,
	Male,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Kitty<T> {
	owner: T,
	dna: [u8; 16],
	gender: Option<Gender>,
}

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id
		pub Kitties get(fn get_kitty_by_id): map hasher(blake2_128_concat) u32 => Option<Kitty<T::AccountId>>;
		/// Stores the next kitty ID
		NextKittyId get(fn next_kitty_id): map hasher(blake2_128_concat) T::AccountId => u32;
		// kitty id and accountId releationship mapping
		// KittyOwner get(fn get_owner_by_kitty_id): map hasher(blake2_128_concat) u32 => Option<T::AccountId>;
	}
}

decl_event! {
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
	{
		/// A kitty is created. \[owner, kitty_id, kitty\]
		KittyCreated(AccountId, u32, Kitty<AccountId>),
		// CreateKittyBaby(AccountId, u32, Kitty),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesIdOverflow,
		KittiesNonExistent,
		// KittiesParentSexChooseError,
		KittyParentError,
		KittyGenderCanNotBeSame,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		/// Create a new kitty
		#[weight = 0]
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;

			let kitty_id = Self::next_kitty_id(&sender);
		
			ensure!(kitty_id + 1 < u32::MAX, Error::<T>::KittiesIdOverflow);

			// TODO: ensure kitty id does not overflow
			// return Err(Error::<T>::KittiesIdOverflow.into());

			// Generate a random 128bit value
			let payload = (
				<pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
				&sender,
				<frame_system::Module<T>>::extrinsic_index(),
			);
			let dna = payload.using_encoded(blake2_128);

			Self::make_kitty(sender, dna);
		}

		#[weight = 100]
		pub fn create_kitty_baby(origin, kitty_a_id: u32, kitty_b_id: u32) {
			let sender = ensure_signed(origin)?;

			let kitty_id = Self::next_kitty_id(&sender);
			
			// ensure kitty overflow
			ensure!(kitty_id + 1 < u32::MAX, Error::<T>::KittiesIdOverflow);

			// kitty option
			let kitty_a_option = <Kitties<T>>::get(kitty_a_id);
			let kitty_b_option = <Kitties<T>>::get(kitty_b_id);

			// ensure kitty existent
			ensure!(kitty_a_option != None, Error::<T>::KittiesNonExistent);
			ensure!(kitty_b_option != None, Error::<T>::KittiesNonExistent);

			let kitty_a = kitty_a_option.unwrap();
			let kitty_b = kitty_b_option.unwrap();
 
			ensure!(kitty_a.owner == sender, Error::<T>::KittyParentError);
			ensure!(kitty_b.owner == sender, Error::<T>::KittyParentError);

			let kitty_baby_id = Self::next_kitty_id(&sender);

			ensure!(kitty_a.gender != kitty_b.gender, Error::<T>::KittyGenderCanNotBeSame);

			let baby_attr = (kitty_a.dna, kitty_b.dna, kitty_baby_id);

			let kitty_baby_dna = baby_attr.using_encoded(blake2_128);

			Self::make_kitty(sender, kitty_baby_dna);
		}
	}
}


impl<T: Trait> Module<T> {
	fn make_kitty(owner: T::AccountId, kitty_dna: [u8; 16]) {
		let kitty_id = Self::next_kitty_id(&owner);

		let mut dna_feature:u8 = 0;

		for dna_item in kitty_dna.iter() {
			dna_feature += dna_item;
		}
		
		let kitty_gender = if dna_feature%2_u8 == 0 {
			Some(Gender::Female)
		} else {
			Some(Gender::Male)
		};

		let new_kitty = Kitty {
			owner: owner.clone(),
			dna: kitty_dna,
			gender: kitty_gender
		};

		<Kitties<T>>::insert(kitty_id, new_kitty.clone());

		<NextKittyId<T>>::insert(&owner, kitty_id + 1_u32);

		// Emit event
		Self::deposit_event(RawEvent::KittyCreated(owner, kitty_id, new_kitty));
	}
}