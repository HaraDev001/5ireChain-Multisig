#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

use frame_support::{dispatch::DispatchResult, ensure, log, pallet_prelude::*};
use frame_system::{
	offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer},
	pallet_prelude::*,
};
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use scale_info::TypeInfo;
	use sp_core::crypto::KeyTypeId;
	use sp_io;
	use sp_runtime::offchain::{http, Duration};
	use sp_std::{prelude::*, str};

	use serde::Deserialize;

	pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"esg!");

	use codec::alloc::string::String;
	use scale_info::prelude::format;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	// implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericPublic = sp_core::sr25519::Public;
		type GenericSignature = sp_core::sr25519::Signature;
	}

	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericPublic = sp_core::sr25519::Public;
		type GenericSignature = sp_core::sr25519::Signature;
	}

	// We are fetching information from the esg oracle endpoint, currently ran on localhost:8080/score
	const HTTP_REMOTE_REQUEST: &str = "https://testnet.5ire.network/oracle";
	const FETCH_TIMEOUT_PERIOD: u64 = 3000; // in milli-seconds

	/// EsgInfo is the payload that we expect to receive from the oracle.
	#[derive(Deserialize, Encode, Decode, Default, TypeInfo)]
	struct EsgInfo {
		score: u8,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Because we sign transactions we need the authority ID
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		/// The overarching dispatch call type.
		type Call: From<Call<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	// TODO once the esg oracle contains actual data or deterministic test data, we can add some form
	// of a ttl to keep to a score struct (defined by the oracle) and cache the info for that time.

	// #[pallet::storage]
	// #[pallet::getter(fn something)]
	// // Learn more about declaring storage items:
	// // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	// pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The score gotten from the ESG Oracle on a validator.
		/// [score, score_owner]
		Score(u8, T::AccountId),
	}

	#[pallet::type_value]
	pub(super) fn ScoreDefault() -> u8 {
		0
	}

	#[pallet::storage]
	#[pallet::getter(fn score)]
	pub(super) type Scores<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u8, ValueQuery, ScoreDefault>;

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// The offchain worker could not fetch the ESG score from the ESG oracle
		OffchainFetchESGScoreError,
		/// The offchain worker could not sign the transaction with the esg score
		OffchainSignedTxError,
		/// The offchain worker could not successfully execute the http request
		HttpFetchingError,
		/// The offchain worker culd not find a local account for signing
		NoLocalAcctForSigning,
		/// Could not find a score for the given account id in the Scores map
		NoAccountScore,
	}

	// the offchain worker entrypoint
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_block_number: T::BlockNumber) {
			let score = Self::fetch_esg_score();
			if let Err(e) = score {
				log::error!("offchain_worker error: {:?}", e);
			}
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// TODO decide if we want to make this an unsigned tx with signed payload
		#[pallet::weight(10_000)]
		pub fn emit_score(origin: OriginFor<T>, score: u8) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// This is the on-chain function
			Self::deposit_event(Event::Score(score, who));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// get the esg_score from the endpoint, emit the esg score in a signed transaction
		pub fn fetch_esg_score() -> Result<(), Error<T>> {
			let resp_bytes = Self::fetch_from_endpoint().map_err(|e| {
				log::error!("fetch_from_remote error: {:?}", e);
				<Error<T>>::HttpFetchingError
			})?;

			let resp_str =
				str::from_utf8(&resp_bytes).map_err(|_| <Error<T>>::HttpFetchingError)?;

			let score: EsgInfo =
				serde_json::from_str(resp_str).map_err(|_| <Error<T>>::HttpFetchingError)?;

			Self::offchain_signed_score(score.score)?;
			Ok(())
		}

		pub fn offchain_signed_score(score: u8) -> Result<(), Error<T>> {
			let signer = Signer::<T, T::AuthorityId>::any_account();
			// `result` is in the type of `Option<(Account<T>, Result<(), ()>)>`. It is:
			//   - `None`: no account is available for sending transaction
			//   - `Some((account, Ok(())))`: transaction is successfully sent
			//   - `Some((account, Err(())))`: error occured when sending the transaction
			let result = signer.send_signed_transaction(|_acct|
				// This is the on-chain function
				Call::emit_score {score });

			// Display error if the signed tx fails.
			if let Some((acc, res)) = result {
				if res.is_err() {
					log::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
					return Err(<Error<T>>::OffchainSignedTxError)
				}
				// Transaction is sent successfully
				// Now we can add it to the scores map since it's successfully submitted
				// If the key exists, update it, else create a new entry
				Self::store_esg_score(&acc.id, score);
				Ok(())
			} else {
				// The case result == `None`: no account is available for sending
				log::error!("No local account available");
				Err(<Error<T>>::NoLocalAcctForSigning)
			}
		}

		fn fetch_from_endpoint() -> Result<Vec<u8>, Error<T>> {
			// Initiate an external HTTP GET request.
			let request = http::Request::get(HTTP_REMOTE_REQUEST);

			// Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
			let timeout =
				sp_io::offchain::timestamp().add(Duration::from_millis(FETCH_TIMEOUT_PERIOD));

			let pending = request
				.deadline(timeout) // Setting the timeout time
				.send() // Sending the request out by the host
				.map_err(|e| {
					log::error!("{:?}", e);
					<Error<T>>::HttpFetchingError
				})?;

			// By default, the http request is async from the runtime perspective. So we are asking the
			// runtime to wait here
			// The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
			//   ref: https://docs.substrate.io/rustdocs/latest/sp_runtime/offchain/http/struct.PendingRequest.html#method.try_wait
			let response = pending
				.try_wait(timeout)
				.map_err(|e| {
					log::error!("{:?}", e);
					<Error<T>>::HttpFetchingError
				})?
				.map_err(|e| {
					log::error!("{:?}", e);
					<Error<T>>::HttpFetchingError
				})?;

			if response.code != 200 {
				log::error!("Unexpected http request status code: {}", response.code);
				return Err(<Error<T>>::HttpFetchingError)
			}

			// Next we fully read the response body and collect it to a vector of bytes.
			Ok(response.body().collect::<Vec<u8>>())
		}

		// add a new score to the storage under the account id
		pub fn store_esg_score(acc_id: &T::AccountId, score: u8) {
			if Scores::<T>::contains_key(acc_id) {
				Scores::<T>::mutate(acc_id, |_val| score);
			} else {
				Scores::<T>::insert(acc_id, score);
			};
		}
	}
}

pub trait Score<T: Config> {
	fn get_esg_score(id: &T::AccountId) -> Result<u8, Error<T>>;
}

pub struct ScoreProvider<T: Config>(PhantomData<T>);

impl<T: Config> Score<T> for ScoreProvider<T> {
	fn get_esg_score(id: &T::AccountId) -> Result<u8, Error<T>> {
		<Pallet<T>>::get_esg_score(id)
	}
}

impl<T: Config> Score<T> for Pallet<T> {
	fn get_esg_score(id: &T::AccountId) -> Result<u8, Error<T>> {
		ensure!(Scores::<T>::contains_key(&id), <Error<T>>::NoAccountScore);
		let val = Scores::<T>::get(id);
		Ok(val)
	}
}
