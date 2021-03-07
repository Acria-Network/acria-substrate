#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_system::ensure_signed;
use sp_std::prelude::*;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The RUNTIME storage
decl_storage! {
	trait Store for Module<T: Trait> as AcriaModule {
		Oracle get(fn get_oracle): double_map hasher(twox_64_concat) T::AccountId, hasher(twox_64_concat) u32 => Option<Vec<u8>>;
	}
}

// Events definition to inform users when important changes are made.
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation ends with an array that provides descriptive names for event
		/// parameters. [something, who]
		NewOracle(u32, AccountId),
		UpdatedOracle(u32, AccountId),
		RemovedOracle(u32, AccountId),
		QueryOracle(u32, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Missing value
		NoneValue,
		/// Value is too short to be valid
		TooShort,
		/// Value is too long to be valid
		TooLong,
		/// Value is not valid
		InvalidValue,
	}
}

// Dispatchable functions to interact with this module
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors initizialization
		type Error = Error<T>;
		// Events inizialitation
		fn deposit_event() = default;
		// function to create a new ORACLE, the oracleid must be not already used and in the oracledata a json structure is expected with the following fields:
		// - shortdescription - a short description not longer than 64 bytes
		// - Long description  - a long description not longer than 6144 bytes
		// - API url with %VAR% replacements if necessary - The endpoint of the API supplier
		// example: {"shortdescription","xxxxxxxxxxxxxxxxxx","description","xxxxxxxxxxxxxxxxxxxxxxxxx","apiurl","https://api.supplier.com/price/?currency=BTC"}
		#[weight = 500_000]
		pub fn new_oracle(origin, oracleid: u32, oracledata: Vec<u8>) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let sender = ensure_signed(origin)?;
			// check oracle data
			ensure!(oracledata.is_empty(), Error::<T>::NoneValue); //check not empty
			ensure!(oracledata.len() >= 8, Error::<T>::TooShort); //check minimum length
			ensure!(oracledata.len() <= 8192, Error::<T>::TooLong);  // check maximum length
			// check oracleid
			ensure!(oracleid > 0, Error::<T>::InvalidValue); //check for oracleid >0

			// Update storage.
			let oraclestorage=oracledata.clone();
			let oracleidstorage=oracleid.clone();
			<Oracle<T>>::insert(&sender, oracleidstorage, oraclestorage);
			// Emit an event
			Self::deposit_event(RawEvent::NewOracle(oracleid, sender));
			// Return a successful DispatchResult
			Ok(())
		}
		
	}
}
