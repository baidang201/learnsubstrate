#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::{Randomness, Currency, ReservableCurrency, ExistenceRequirement},};
	use frame_system::pallet_prelude::*;
  use codec::{Encode, Decode};
  use sp_io::hashing::blake2_128;
  use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, One, Saturating, Zero};

  #[derive(Encode, Decode)]
  pub struct Kitty(pub [u8; 16]);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

    type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

    type KittyIndex: Parameter + Member + AtLeast32BitUnsigned + Bounded + Default + Copy;

    type Currency: ReservableCurrency<Self::AccountId>;
	}

  type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

  #[pallet::storage]
  #[pallet::getter(fn kitties_count)]
  pub(super) type KittiesCount<T: Config> = StorageValue<_, T::KittyIndex>;

  #[pallet::storage]
  #[pallet::getter(fn kitties)]
  pub(super) type Kitties<T: Config> = StorageMap<
    _, 
    Blake2_128Concat, 
    T::KittyIndex, 
    Option<Kitty>, 
    ValueQuery
  >;

  #[pallet::storage]
  #[pallet::getter(fn owner)]
  pub(super) type Owner<T: Config> = StorageMap<
    _, 
    Blake2_128Concat, 
    T::KittyIndex, 
    Option<T::AccountId>, 
    ValueQuery
  >;


  #[pallet::storage]
  #[pallet::getter(fn kitty_price)]
  pub(super) type KittyPrices<T: Config> = StorageMap<
    _, 
    Blake2_128Concat, 
    T::KittyIndex, 
    BalanceOf<T>, 
    ValueQuery
  >;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId", T::KittyIndex = "KittyIndex", BalanceOf<T> = "Balance")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
    /// A kitty is created. (owner, kitty_id)
		Created(T::AccountId, T::KittyIndex),
		/// A kitty is transferred. (from, to, kitty_id)
		Transferred(T::AccountId, T::AccountId, T::KittyIndex),
		/// A kitty is available for sale. (owner, kitty_id, price)
		Ask(T::AccountId, T::KittyIndex, Option<BalanceOf<T>>),
		/// A kitty is sold. (from, to, kitty_id, price)
		Sold(T::AccountId, T::AccountId, T::KittyIndex, BalanceOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
    KittiesCountOverflow,
		InvalidKittyId,
		RequireDifferentParent,
		RequireOwner,
		NotForSale,
		PriceTooLow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

    #[pallet::weight(0)]
		pub fn create(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let kitty_id = Self::next_kitty_id()?;

			// Generate a random 128bit value
			let dna = Self::random_value(&sender);

			// Create and store kitty
			let kitty = Kitty(dna);
			Self::insert_kitty(&sender, kitty_id, kitty);

      T::Currency::reserve(&sender, One::one())?;

			Self::deposit_event(Event::Created(sender, kitty_id));

      Ok(())
		}

    /// Breed kitties
		#[pallet::weight(0)]
		pub fn breed(origin: OriginFor<T>, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let new_kitty_id = Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;

      Self::deposit_event(Event::Created(sender, new_kitty_id));

      Ok(())
		}

    /// Transfer a kitty to new owner
    #[pallet::weight(0)]
    pub fn transfer(origin: OriginFor<T>, to: T::AccountId, kitty_id: T::KittyIndex) -> DispatchResult {
      let sender = ensure_signed(origin)?;

      ensure!(Some(sender.clone()) == Owner::<T>::get(kitty_id), Error::<T>::RequireOwner);

      Self::do_transfer(&sender, &to, kitty_id);

      Self::deposit_event(Event::Transferred(sender, to, kitty_id));

      Ok(())
    }

    #[pallet::weight(0)]
    pub fn ask(origin: OriginFor<T>, kitty_id: T::KittyIndex, new_price: Option<BalanceOf<T>>) -> DispatchResult {
     let sender = ensure_signed(origin)?;

     ensure!(Some(sender.clone()) == Owner::<T>::get(kitty_id), Error::<T>::RequireOwner);

     <KittyPrices<T>>::mutate_exists(kitty_id, |price| *price = new_price);

     Self::deposit_event(Event::Ask(sender, kitty_id, new_price));

     Ok(())
   }

   /// Buy a kitty
   #[pallet::weight(0)]
   pub fn buy(origin: OriginFor<T>, kitty_id: T::KittyIndex, price: BalanceOf<T>) -> DispatchResult {
     let sender = ensure_signed(origin)?;

     let owner = Self::owner(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;

     ensure!(<KittyPrices<T>>::contains_key(kitty_id), Error::<T>::NotForSale);
     let kitty_price = Self::kitty_price(kitty_id);

     ensure!(price >= kitty_price, Error::<T>::PriceTooLow);

     T::Currency::transfer(&sender, &owner, kitty_price, ExistenceRequirement::KeepAlive)?;

     <KittyPrices<T>>::remove(kitty_id);

     Self::do_transfer(&owner, &sender, kitty_id);

     Self::deposit_event(Event::Sold(owner, sender, kitty_id, kitty_price));

     Ok(())
   }
	}

  fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
    (selector & dna1) | (!selector & dna2)
  }

  impl<T: Config> Pallet<T> {
    fn random_value(sender: &T::AccountId) -> [u8; 16] {
      let payload = (
        T::Randomness::random_seed(),
        &sender,
        <frame_system::Module<T>>::extrinsic_index(),
      );
      payload.using_encoded(blake2_128)
    }
  
    fn next_kitty_id() -> Result<T::KittyIndex, DispatchError> {
      let kitty_id = match Self::kitties_count() {
        Some(id) => {
          ensure!(id != T::KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
          id
        },
        None => {
          One::one()
        }
      };

      Ok(kitty_id)
    }

    fn insert_owned_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex) {
      <Owner<T>>::insert(kitty_id, Some(owner));
    }
  
    fn insert_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
      // Create and store kitty
      Kitties::<T>::insert(kitty_id, Some(kitty));
      KittiesCount::<T>::put(kitty_id.saturating_add(One::one()));
  
      Self::insert_owned_kitty(owner, kitty_id);
    }

    fn do_breed(sender: &T::AccountId, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> Result<T::KittyIndex, DispatchError> {
      let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
      let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;
  
      // ensure!(<Owner<T>>::contains_key((&sender, Some(kitty_id_1))), Error::<T>::RequireOwner);
      // ensure!(<Owner<T>>::contains_key((&sender, Some(kitty_id_2))), Error::<T>::RequireOwner);
      // ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);
  
      let kitty_id = Self::next_kitty_id()?;
  
      let kitty1_dna = kitty1.0;
      let kitty2_dna = kitty2.0;
  
      // Generate a random 128bit value
      let selector = Self::random_value(&sender);
      let mut new_dna = [0u8; 16];
  
      // Combine parents and selector to create new kitty
      for i in 0..kitty1_dna.len() {
        new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
      }
  
      Self::insert_kitty(sender, kitty_id, Kitty(new_dna));
  
      Ok(kitty_id)
    }

    fn do_transfer(from: &T::AccountId, to: &T::AccountId, kitty_id: T::KittyIndex)  {
      Self::insert_owned_kitty(&to, kitty_id);
    }
  }
}
