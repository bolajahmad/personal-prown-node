#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

use frame_support::traits::Currency;

// Type which shortens the access to the Currency trait from the Balances pallet.
type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type ContentIdentifier = Vec<u8>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{LockableCurrency, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		/// Using the pallet_balances exposed 'Currency' trait to fetch user balance info
		type Currency: ReservableCurrency<Self::AccountId> + LockableCurrency<Self::AccountId>;
		/// The maximum length of the IPFS CID string
		type MaxCIDLength: Get<u32>;
		/// The minimum price to pay for creating an asset
		type MinAssetPrice: Get<BalanceOf<Self>>;
	}

	#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, MaxEncodedLen, Clone)]
	#[scale_info(skip_type_params(T))]
	pub struct Property<T: Config> {
		pub owner: T::AccountId,
		pub asset_cid: BoundedVec<u8, T::MaxCIDLength>,
		pub proposal_id: u128,
		pub property_id: u128,
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type PropertyCount<T> = StorageValue<_, u128>;

	#[pallet::storage]
	#[pallet::getter(fn properties_by_id)]
	pub type PropertiesByID<T: Config> =
		StorageMap<_, Blake2_128Concat, u128, Property<T>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		PropertyCreated { property_id: u128, owner: T::AccountId, proposal_cid: ContentIdentifier },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		InsufficientFunds,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		PropertyExists,
		CIDTooLong,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10)]
		pub fn create_new_property(
			origin: OriginFor<T>,
			property_id: u128,
			property_owner: T::AccountId,
			asset_cid: Vec<u8>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let balance = T::Currency::free_balance(&property_owner);
			ensure!(balance >= T::MinAssetPrice::get(), Error::<T>::InsufficientFunds);

			// verify that property does not exist
			ensure!(PropertiesByID::<T>::contains_key(property_id), Error::<T>::PropertyExists);

			let proposal_id = PropertyCount::<T>::get().unwrap_or(0).checked_add(1).unwrap();

			let bounded_asset_cid: BoundedVec<u8, T::MaxCIDLength> =
				BoundedVec::try_from(asset_cid.clone()).map_err(|_| Error::<T>::CIDTooLong)?;

			let new_property = Property {
				owner: property_owner.clone(),
				asset_cid: bounded_asset_cid,
				proposal_id,
				property_id,
			};
			<PropertiesByID<T>>::insert(&property_id, new_property);

			Self::deposit_event(Event::PropertyCreated {
				property_id,
				owner: property_owner,
				proposal_cid: asset_cid,
			});
			Ok(())
		}

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(10)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(11)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			Ok(())
		}
	}
}
