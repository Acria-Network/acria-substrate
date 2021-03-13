#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{debug,decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_system::ensure_signed;
use sp_std::prelude::*;
use core::str;
use core::str::FromStr;
use sp_runtime::{
	offchain as rt_offchain,
};



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
        OracleQuery get(fn get_oraclequery): double_map hasher(twox_64_concat) T::AccountId, hasher(twox_64_concat) u32 => Option<Vec<u8>>;
	}
}

// Events definition to inform users when important changes are made.
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation ends with an array that provides descriptive names for event
		/// parameters. [something, who]
		NewOracle(u32, AccountId),
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
        // - %varname% coming from query can be replaced in the API call to the data provider
        // Possible variables are:
        // %VARNAME% - Replace the var with the matching varname for example %username% will be replace with the json field "username" received in the extrinsic.
		// example: {"shortdescription":"xxxxxxxxxxxxxxxxxx","description":"xxxxxxxxxxxxxxxxxxxxxxxxx","apiurl":"https://api.supplier.com/price/?currency=BTC","fees":0.0000001}
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
            let feesf:f64 = match f64::from_str(fees_str){
                Ok(f) => f,
                Err(_) => 0.0,
            };
			ensure!(feesf > 0.0, Error::<T>::InvalidFees); //check fees must be > 0
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
        // function to query the Oracle
		#[weight = 50_000]
        pub fn query_oracle(origin, oracleaccount: T::AccountId, oracleid: u32) -> dispatch::DispatchResult {
            // check presence oracleaccount/oracleid pair
            let oracle = match <Oracle<T>>::get(&oracleaccount,&oracleid){
                Some(oracle) =>  oracle,
                // error if not found
                None => return Err(Error::<T>::OracleNotFound.into()), 
            };
            // get apiurl
            let apiurl=json_get_value(oracle,"apiurl".as_bytes().to_vec());
            if apiurl.len()==0 {
                return Err(Error::<T>::OracleWrongConfiguration.into());
            }
            debug::info!("Api Url{:?}", apiurl);
            //replace %variables% if any
            let apiurlclone=apiurl.clone();
            let buf=str::from_utf8(&apiurlclone).unwrap();
            let mut apiurlf = str::replace(buf, "?", "#");
            debug::info!("Api Url replaced {:?}", apiurlf);

            // call the off-chain worker
            let apiurlstr= match str::from_utf8(&apiurl){
                Ok(u) => u,
                Err(_) => return Err(Error::<T>::OracleWrongConfiguration.into()),
            };
            let request = rt_offchain::http::Request::get(apiurlstr);
            // setting timeout for https request to 4 seconds
            let timeout = sp_io::offchain::timestamp()
                .add(rt_offchain::Duration::from_millis(4000));
            // sending the https request
            let pending = request
                .add_header("User-Agent", "Acria-Network")  // same api servers require an user/agent
                .deadline(timeout) // Setting the timeout time
                .send() // Sending the request out by the host
                .map_err(|_| <Error<T>>::OracleFetchingError)?;
            // By default, the http request is async from the runtime perspective. So we are asking the runtime to wait the answer.
		    // The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
		    let response = pending
                .try_wait(timeout)
                .map_err(|_| <Error<T>>::OracleFetchingError)?
                .map_err(|_| <Error<T>>::OracleFetchingError)?;
            // checking for https error
            if response.code != 200 {
                debug::error!("Unexpected http request status code: {}", response.code);
                return Err(Error::<T>::OracleFetchingError.into());
            }
            let oracleaccountstorage=oracleaccount.clone();
            let oracleidstorage=oracleid.clone();
            let apianswer=response.body().collect::<Vec<u8>>();
            // we store the result in the blockchain
			<OracleQuery<T>>::insert(&oracleaccountstorage, oracleidstorage,apianswer);
			// Emit an event to report the "Oracle Query"
			Self::deposit_event(RawEvent::QueryOracle(oracleid, oracleaccount));
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
