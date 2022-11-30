
#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
 
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	enum Gender {
		Male,
		Female,
	}

	// #[pallet::pallet]
	// #[pallet::generate_store(pub(super) trait Store)]
	// pub struct Pallet<T>(_);

	
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Kitty {
		dna: Vec<u8>,
		owner: AccountId,
		price: u32,
		gender: Gender,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		// type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type dna
		
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type NumberKitty<T> = StorageValue<_, u32>;


	#[pallet::storage]
	pub type InforKitty<T:Config> = StorageMap<_,Blake2_128Concat,
					T::dna,
					Kitty,
					ValueQuery,>;

	#[pallet::storage]
	pub type NumberKittyOfAccountId<T:Config> = StorageMap<_,Blake2_128Concat,
					T::AccountId,
					Vec<T::dna>,
					ValueQuery,>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		Kitty(T::AccountId, Kitty),
		
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Kitty<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn createKitty(origin: OriginFor<T>, price: u32, dna: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			let mut gender : Gender;

			if dna.len() % 2 == 0 {
				gender = Gender::Male
			}
			else {
				gender = Gender::Female
			}

			let kitty = self {
				dna : dna,
				owner : who,
				price: price,
				gender : gender
			}

			// Update storage.
			let numberkitty = <NumberKitty<T>>:get();

			<NumberKitty<T>>::put(numberkitty + 1);
			<InforKitty<T>>::insert(dna, kitty);
			
			// <Something<T>>::put(something);

			// // Emit an event.
			 Self::deposit_event(Event::kitty(who, kitty));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn changeOwner(origin: OriginFor<T>, newOwner: AccountId, dna: ) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			//<Number<T>>::insert(who.clone(), number);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(number,who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
Đang hiển thị 9180587826524498910.