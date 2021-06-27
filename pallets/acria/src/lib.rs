#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, dispatch, ensure,
	traits::{Currency, ReservableCurrency},
};
use frame_system::ensure_signed;
use sp_std::prelude::*;
use core::str;
use core::str::FromStr;
use crate::sp_api_hidden_includes_decl_storage::hidden_include::sp_runtime::SaturatedConversion;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait Config: frame_system::Config + Sized {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    /// The currency trait.
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}


// The RUNTIME storage
decl_storage! {
	trait Store for Module<T: Config> as AcriaModule {
        // Stores the Oracle data
		Oracle get(fn get_oracle): double_map hasher(twox_64_concat) T::AccountId, hasher(twox_64_concat) u32 => Option<Vec<u8>>;
        // Stores the query for an Oracle (not yet activated)
        //OracleQuery get(fn get_oraclequery): double_map hasher(twox_64_concat) T::AccountId, hasher(twox_64_concat) u32 => Option<Vec<u8>>;
        // Stores the answer of the Oracle
        OracleData get(fn get_oracledata): double_map hasher(twox_64_concat) T::AccountId, hasher(twox_64_concat) u32 => Option<Vec<u8>>;
        // Stores the stakes in Acria tokens for each Oracle (StakerAccountId )
        OracleStakes get (fn get_oracle_account_stakes): double_map hasher(twox_64_concat) T::AccountId, hasher(twox_64_concat) T::AccountId => BalanceOf<T>; 
	}
}

// Events definition to inform users when important changes are made.
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Event documentation ends with an array that provides descriptive names for event
        /// A new Oracle was added. \[OracleId, OracleAccountid\]
		NewOracle(u32, AccountId),
        /// An Oracle was deleted. \[OracleId, OracleAccountid\]
		RemovedOracle(u32, AccountId),
        /// An update request to an Oracle has been received. \[OracleId, OracleAccountid,RequestParameters \]
		RequestOracleUpdate(u32, AccountId,Vec<u8>),
        /// An update request to an Oracle has been received. \[OracleId, OracleAccountid,RequestParameters \]
		OracleUpdate(u32, AccountId),
        /// A settlment for Oracle fees has been completed \[OracleId, OracleAccountid\]
        OracleFeesSettlement(u32, AccountId),
        /// A settlment for Oracle fees has been completed \[OracleId, OracleAccountid\]
        StakerFeesSettlement(u32, AccountId),
        /// An account has staken some Acria tokens to an Oracle.  \[StakerAccountId, OracleAccountIdOracleI,Amount\]
        OracleLockedStakes(AccountId,AccountId),
        /// An account has un-staken Acria tokens from an Oracle.  \[StakerAccountId, OracleAccountIdOracleI\]
        OracleUnlockedStakes(AccountId,AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Missing value
		NoneValue,
		/// Value is too short to be valid
		TooShort,
		/// Value is too long to be valid
		TooLong,
		/// Value is not valid
		InvalidValue,
		/// Invalid Json Structure
		InvalidJson,
		/// Invalid Short Description of the Oracle
		InvalidShortDescription,
		/// Invalid Description of the Oracle
		InvalidDescription,
		/// Invalid Description of the Oracle
		InvalidUrl,
		/// Invalid Fees of the Oracle
		InvalidFees,
        /// Oracle not found
		OracleNotFound,
        /// Oracle duplicated
		OracleDuplicated,
        /// Oracle duplicated
		OracleWrongConfiguration,
        // Error returned during an Oracle Api Fetching
		OracleFetchingError,
        // Not enough funds available for the requested operation
        NotEnoughFunds,
        // Consisteny error
        ConsistencyError,
        // Oracle Fees Settlement Error
        OracleSettlementError,
        // Staker Fees Settlement Error
        StakerSettlementError,

	}
}

// Dispatchable functions to interact with this module
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors initizialization
		type Error = Error<T>;
		// Events inizialitation
		fn deposit_event() = default;
		// function to create a new ORACLE, the oracleid must be not already used and in the oracledata a json structure is expected with the following fields:
		// - shortdescription - a short description not longer than 64 bytes
		// - description  - a long description not longer than 6144 bytes
        // - apiurl  - an https address as reference for the API, explaining the possible parameters if any.
        // - fees - amount of fees applied to the requester.
        // Possible variables are:
		// example: {"shortdescription":"xxxxxxxxxxxxxxxxxx","description":"xxxxxxxxxxxxxxxxxxxxxxxxx","apiurl":"https://api.supplier.com/documentation","fees":100}
		#[weight = 10_000]
		pub fn new_oracle(origin, oracleid: u32, oracledata: Vec<u8>) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let sender = ensure_signed(origin)?;
			// check oracle data
			ensure!(oracledata.len() >= 8, Error::<T>::TooShort); //check minimum length
			ensure!(oracledata.len() <= 8192, Error::<T>::TooLong);  // check maximum length
			// check oracleid
			ensure!(oracleid > 0, Error::<T>::InvalidValue); //check for oracleid >0
            // check of the account id/oracle is free
            match <Oracle<T>>::get(&sender,&oracleid){
                // oracle is already existing
                Some(_) => {
                    return Err(Error::<T>::OracleDuplicated.into());
                }
                // oracle id is not yet used
                None => { //nothing to do
                }
            }
			// check json validity
			let js=oracledata.clone();
			ensure!(json_check_validity(js),Error::<T>::InvalidJson);
			// check short description
			let jsf=oracledata.clone();
			let shortdescription=json_get_value(jsf,"shortdescription".as_bytes().to_vec());
			ensure!(shortdescription.len() >= 4, Error::<T>::InvalidShortDescription); //check minimum length for short description
			// check (long) description
			let jsd=oracledata.clone();
			let description=json_get_value(jsd,"description".as_bytes().to_vec());
			ensure!(description.len() >= 4, Error::<T>::InvalidDescription); //check minimum length for description
			// check api url
			let jsu=oracledata.clone();
			let apiurl=json_get_value(jsu,"apiurl".as_bytes().to_vec());
			ensure!(apiurl.len() >= 8, Error::<T>::InvalidUrl); //check minimum length for api url
			// check fees
			let jst=oracledata.clone();
			let fees=json_get_value(jst,"fees".as_bytes().to_vec());
            let fees_slice=fees.as_slice();
            let fees_str=match str::from_utf8(&fees_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let feesf:u64 = match u64::from_str(fees_str){
                Ok(f) => f,
                Err(_) => 0,
            };
			ensure!(feesf > 0, Error::<T>::InvalidFees); //check fees must be > 0
			// Update storage.
			let oraclestorage=oracledata.clone();
			let oracleidstorage=oracleid.clone();
			<Oracle<T>>::insert(&sender, oracleidstorage, oraclestorage);
			// Emit an event
			Self::deposit_event(RawEvent::NewOracle(oracleid, sender));
			// Return a successful DispatchResult
			Ok(())
		}
        // function to remove an ORACLE, the oracleid must be created from the signer (only owner can remove the oracle)
		#[weight = 10_000]
		pub fn remove_oracle(origin, oracleid: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let sender = ensure_signed(origin)?;
            // chech that the oracle belongs to signer
            match <Oracle<T>>::get(&sender,&oracleid){
                // remove oracle
                Some(_) => {
                    <Oracle<T>>::take(&sender, &oracleid);
                    Self::deposit_event(RawEvent::RemovedOracle(oracleid, sender));
			        Ok(())
                }
                // error if not found
                None => Err(Error::<T>::OracleNotFound.into()), 
            }
		}
        // function to request a data update to the Oracle identified from accountid/oracleid
		#[weight = 50_000]
        pub fn request_oracle_update(origin, oracleaccount: T::AccountId, oracleid: u32, parameters: Vec<u8>) -> dispatch::DispatchResult {
            // verify it's a signed transaction
            let sender = ensure_signed(origin)?;
            // check presence oracleaccount/oracleid pair
            let oracle = match <Oracle<T>>::get(&oracleaccount,&oracleid){
                Some(oracle) =>  oracle,
                // error if not found
                None => return Err(Error::<T>::OracleNotFound.into()), 
            };
            // get fees in u32
            let fees=json_get_value(oracle,"fees".as_bytes().to_vec());
            let fees_slice=fees.as_slice();
            let fees_str=match str::from_utf8(&fees_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let feesu:u64 = match u64::from_str(fees_str){
                Ok(f) => f,
                Err(_) => 0,
            };
            //let feesf8: BalanceOf<T> = feesu.saturated_into();
            // compute 80% fees to dataprovider and 20% to stakers
            let feesudp: u64 = feesu * 80 / 100;
            let feesdp:BalanceOf<T> = feesudp.saturated_into();
            let tot_fees_stakers: BalanceOf<T> = (feesu-feesudp).saturated_into();
            // transfer the fees to data provider
            let _r = match T::Currency::transfer(&sender.clone(),&oracleaccount.clone(),feesdp, frame_support::traits::ExistenceRequirement::AllowDeath){
                Ok(r) => r,
                Err(_e)=> return Err(Error::<T>::OracleSettlementError.into()), 
            };

            // calculate total stakes stored
            let mut ik = <OracleStakes<T>>::iter_prefix(&oracleaccount);
            let mut tot_stakes: BalanceOf<T> = 0u64.saturated_into();
            loop {
               // get staker account id and stakes amount
               let (_staker_account,stakes_amount) = match ik.next(){
                    Some((staker,stakes)) => (staker,stakes),
                    None => break
                };
                tot_stakes=tot_stakes + stakes_amount;
            }
            //frame_support::debug::RuntimeLogger::init();
            //frame_support::debug::info!("****************************************** tot_stakes {:?}", tot_stakes);

            // loop the stakers to settle the fees
            let mut iks = <OracleStakes<T>>::iter_prefix(&oracleaccount);
            loop {
               // get staker account id and stakes amount
               //frame_support::debug::info!("************************************** oracleaccount {:?}", oracleaccount);
               let (staker_account,stakes_amount) = match iks.next(){
                    Some((staker,stakes)) => (staker,stakes),
                    None => break
                };
                //frame_support::debug::info!("************************************** Staker {:?}", staker_account);
                // compute the fees for the staker
                //frame_support::debug::info!("************************************** tot_fees_stakers {:?}", tot_fees_stakers);
                //frame_support::debug::info!("****************************************** tot_stakes {:?}", tot_stakes);
                //frame_support::debug::info!("************************************** stakes_amount {:?}", stakes_amount);
                let fees_stk = (tot_fees_stakers * stakes_amount) / tot_stakes;
                //frame_support::debug::info!("************************************** fees_stk {:?}", fees_stk);

                // transfer the fees to the staker
                let _r = match T::Currency::transfer(&sender,&staker_account.clone(),fees_stk, frame_support::traits::ExistenceRequirement::AllowDeath){
                    Ok(r) => r,
                    Err(_e)=>  return Err(Error::<T>::StakerSettlementError.into()), 
                };
            }
            

            /*
            // We store the query in the blockchain for further processing (option ready to be activated in case)
            let oracleaccountstorage=oracleaccount.clone();
            let oracleidstorage=oracleid.clone();
            let oracleparametersstorage=parameters.clone();
			<OracleQuery<T>>::insert(&oracleaccountstorage, oracleidstorage,oracleparametersstorage);
            */
            
			// Emit an event to report the "Oracle Query"
			Self::deposit_event(RawEvent::RequestOracleUpdate(oracleid, oracleaccount,parameters));
            // return back with positevely signal */
            Ok(())
        }
        // function to write back the signed answer from the Oracle identified by accountid/oracleid
        // the data provider is not charged for the data supplied
		#[weight = 0]
        pub fn oracle_update(origin, oracleid: u32,oracledata: Vec<u8>) -> dispatch::DispatchResult {
            // verify it's a signed transaction
            let sender = ensure_signed(origin)?;
            // check presence oracleaccount/oracleid pair
            let _oracle = match <Oracle<T>>::get(&sender,&oracleid){
                Some(oracle) =>  oracle,
                // error if not found
                None => return Err(Error::<T>::OracleNotFound.into()), 
            };
            // we store the data in the blockchain for further processing
            let oracleaccountstorage=sender.clone();
            let oracleidstorage=oracleid.clone();
			<OracleData<T>>::insert(&oracleaccountstorage, oracleidstorage,oracledata);
			// Emit an event to report the "Oracle Query"
			Self::deposit_event(RawEvent::OracleUpdate(oracleid, sender));
            // return back with positevely signal */
            Ok(())
        }
        // function to stake Acria Tokens to an Oracle
        #[weight = 50_000]
        pub fn lock_oracle_stakes(origin, oracleaccount: T::AccountId, amount: BalanceOf<T>) -> dispatch::DispatchResult {
            // verify it's a signed transaction
            let sender = ensure_signed(origin)?;
            // try to lock the amount requested
            let _r= match T::Currency::reserve(&sender, amount.clone()){
                Ok(r) => r,
                Err(_) => return Err(Error::<T>::NotEnoughFunds.into()), 
            };
            // removes previous stakes on same Oracle
            let oracle_stakes = <OracleStakes<T>>::take(&oracleaccount,&sender);
            // unlock the amount present for the previous Stakes if any
            T::Currency::unreserve(&sender,oracle_stakes.clone());
            // update the OracleStakes
            <OracleStakes<T>>::insert(&oracleaccount, &sender,amount.clone());
            // emits event for the successfully stakes reserved
            Self::deposit_event(RawEvent::OracleLockedStakes(sender.clone(),oracleaccount.clone()));
            // return back with positevely signal
            Ok(())
        }
        // function to unstake Acria Tokens to an Oracle
        #[weight = 50_000]
        pub fn unlock_oracle_stakes(origin, oracleaccount: T::AccountId) -> dispatch::DispatchResult {
            // verify it's a signed transaction
            let sender = ensure_signed(origin)?;
            // removes the stakes
            let oracle_stakes = <OracleStakes<T>>::take(&oracleaccount,&sender);
            // unlock the amount from the reserve
            T::Currency::unreserve(&sender,oracle_stakes);
            // emits event for the successfully stakes reserved
            Self::deposit_event(RawEvent::OracleUnlockedStakes(sender,oracleaccount));
            // return back with positevely signal
            Ok(())
        }
        
	}
}

// function to validate a json string
fn json_check_validity(j:Vec<u8>) -> bool{	
    // minimum lenght of 2
    if j.len()<2 {
        return false;
    }
    // checks star/end with {}
    if *j.get(0).unwrap()==b'{' && *j.get(j.len()-1).unwrap()!=b'}' {
        return false;
    }
    // checks start/end with []
    if *j.get(0).unwrap()==b'[' && *j.get(j.len()-1).unwrap()!=b']' {
        return false;
    }
    // check that the start is { or [
    if *j.get(0).unwrap()!=b'{' && *j.get(0).unwrap()!=b'[' {
            return false;
    }
    //checks that end is } or ]
    if *j.get(j.len()-1).unwrap()!=b'}' && *j.get(j.len()-1).unwrap()!=b']' {
        return false;
    }
    //checks " opening/closing and : as separator between name and values
    let mut s:bool=true;
    let mut d:bool=true;
    let mut pg:bool=true;
    let mut ps:bool=true;
    let mut bp = b' ';
    for b in j {
        if b==b'[' && s {
            ps=false;
        }
        if b==b']' && s && ps==false {
            ps=true;
        }
        else if b==b']' && s && ps==true {
            ps=false;
        }
        if b==b'{' && s {
            pg=false;
        }
        if b==b'}' && s && pg==false {
            pg=true;
        }
        else if b==b'}' && s && pg==true {
            pg=false;
        }
        if b == b'"' && s && bp != b'\\' {
            s=false;
            bp=b;
            d=false;
            continue;
        }
        if b == b':' && s {
            d=true;
            bp=b;
            continue;
        }
        if b == b'"' && !s && bp != b'\\' {
            s=true;
            bp=b;
            d=true;
            continue;
        }
        bp=b;
    }
    //fields are not closed properly
    if !s {
        return false;
    }
    //fields are not closed properly
    if !d {
        return false;
    }
    //fields are not closed properly
    if !ps {
        return false;
    }
    // every ok returns true
    return true;
}
// function to get value of a field for Substrate runtime (no std library and no variable allocation)
fn json_get_value(j:Vec<u8>,key:Vec<u8>) -> Vec<u8> {
    let mut result=Vec::new();
    let mut k=Vec::new();
    let keyl = key.len();
    let jl = j.len();
    k.push(b'"');
    for xk in 0..keyl{
        k.push(*key.get(xk).unwrap());
    }
    k.push(b'"');
    k.push(b':');
    let kl = k.len();
    for x in  0..jl {
        let mut m=0;
        let mut xx=0;
        if x+kl>jl {
            break;
        }
        for i in x..x+kl {
            if *j.get(i).unwrap()== *k.get(xx).unwrap() {
                m=m+1;
            }
            xx=xx+1;
        }
        if m==kl{
            let mut lb=b' ';
            let mut op=true;
            let mut os=true;
            for i in x+kl..jl-1 {
                if *j.get(i).unwrap()==b'[' && op==true && os==true{
                    os=false;
                }
                if *j.get(i).unwrap()==b'}' && op==true && os==false{
                    os=true;
                }
                if *j.get(i).unwrap()==b':' && op==true{
                    continue;
                }
                if *j.get(i).unwrap()==b'"' && op==true && lb!=b'\\' {
                    op=false;
                    continue
                }
                if *j.get(i).unwrap()==b'"' && op==false && lb!=b'\\' {
                    break;
                }
                if *j.get(i).unwrap()==b'}' && op==true{
                    break;
                }
                if *j.get(i).unwrap()==b',' && op==true && os==true{
                    break;
                }
                result.push(j.get(i).unwrap().clone());
                lb=j.get(i).unwrap().clone();
            }   
            break;
        }
    }
    return result;
}
/*
// function to get the position of an str insider another str, for no-std environment (equivalent to str.find() in std library)
fn strpos(search: &str, subject: &str)-> Option<usize>{
    let seax=search.len();
    let subx=subject.len();
    loop {
        for x in 0..subx {
            let mut cnt=0;
            for i in 0..seax {
                if search[i..i+1]==subject[x..x+1]{
                    cnt=cnt+1;
                }    
            } 
            if cnt==seax {
                return Some(x);
            }
        }  
        break;  
    }
    return None;
}
*/
