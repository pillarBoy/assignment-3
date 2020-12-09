#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, StorageDoubleMap,
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
pub struct Kitty {
	dna: [u8; 16],
	gender: Option<Gender>,
}

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id
		pub Kitties get(fn kitties): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<Kitty>;
		/// Stores the next kitty ID
		NextKittyId get(fn next_kitty_id): map hasher(blake2_128_concat) T::AccountId => u32;
		// kitty id and accountId releationship mapping
		KittyOwner get(fn owner_of): map hasher(blake2_128_concat) u32 => Option<T::AccountId>;
	}
}

decl_event! {
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
	{
		/// A kitty is created. \[owner, kitty_id, kitty\]
		KittyCreated(AccountId, u32, Kitty),
		// CreateKittyBaby(AccountId, u32, Kitty),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesIdOverflow,
		KittiesNonExistent,
		KittiesParentSexChooseError,
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
		
			ensure!(kitty_id + 1 < u32::MAX, Error::<T>::KittiesIdOverflow);

			ensure!(<KittyOwner<T>>::contains_key(kitty_a_id), Error::<T>::KittiesNonExistent);
			ensure!(<KittyOwner<T>>::contains_key(kitty_b_id), Error::<T>::KittiesNonExistent);
 
			ensure!(<Kitties<T>>::get(&sender, kitty_a_id) != None, Error::<T>::KittyParentError);
			ensure!(<Kitties<T>>::get(&sender, kitty_b_id) != None, Error::<T>::KittyParentError);

			let kitty_a = <Kitties<T>>::get(&sender, kitty_a_id).unwrap();
			let kitty_b = <Kitties<T>>::get(&sender, kitty_b_id).unwrap();

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

		let kitty_gender = if kitty_dna[15]%2 == 0 {
			Some(Gender::Female)
		} else {
			Some(Gender::Male)
		};

		let new_kitty = Kitty {
			dna: kitty_dna,
			gender: kitty_gender
		};


		<Kitties<T>>::insert(&owner, kitty_id, new_kitty.clone());

		<KittyOwner<T>>::insert(kitty_id, &owner);

		<NextKittyId<T>>::insert(&owner, kitty_id + 1);


		// Emit event
		Self::deposit_event(RawEvent::KittyCreated(owner, kitty_id, new_kitty));
	}
}