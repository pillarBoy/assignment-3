# Kitty pallet design

- Calls
    - fn deposit_event
    - pub fn create_kitty(origin)
    - pub fn create_kitty_baby(origin, kitty_a_id: u32, kitty_b_id: u32)
    - fn make_kitty(owner: T::AccountId, kitty_dna: [u8; 16])

- Types
    - enum Gender: { Female, Male }
    - struct Kitty<T> { owner: T, dna: [u8; 16], gender: Option<Gender> }

- Storages
    - Kitties get(fn get_kitty_by_id): map hasher(blake2_128_concat) u32 => Option<Kitty<T::AccountId>>;
    - NextKittyId: u32


- Error
    - KittiesIdOverflow
    - KittiesNonExistent
    - KittyParentError
    - KittyGenderCanNotBeSame


- Events
    - KittyCreated
        - owner: AccountId
        - kitty_id: u32
        - kitty: Kitty

