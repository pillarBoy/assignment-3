# Kitty pallet design

- Calls
    - fn deposit_event
    - pub fn create_kitty(origin)
    - create_kitty_baby(origin, kitty_a_id: u32, kitty_b_id: u32)
    - make_kitty(owner: T::AccountId, kitty_dna: [u8; 16])


- Storages
    - pub Kitties get(fn get_kitty_by_id): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) kitty_id => Option<Kitty>
    - NextKittyId: u32
    - KittyOwner: map hasher(blake2_128_concat) u32 => Option<T::AccountId>

- Types
    - enum Gender: { Female, Male }
    - struct Kitty { dna: [u8; 16], gender: Option<Gender> }


- Events
    - KittyCreated
        - owner: AccountId
        - kitty_id: u32
        - kitty: Kitty

