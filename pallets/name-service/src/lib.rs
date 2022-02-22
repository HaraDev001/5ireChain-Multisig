#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::DispatchResult,
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement, WithdrawReasons},
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use scale_info::{prelude::vec, TypeInfo};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

const YEAR: u32 = 5259492;
pub type IPV4 = [u16; 4];
pub type IPV6 = [u16; 6];
pub type BYTES = Vec<u8>;
// pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo)]
pub struct Domain<AccountId, Balance, BlockNumber> {
	/// domain name in bytestring e.g. b'hyungsukkang.eth'
	name: BYTES,
	/// source of this domain a.k.a. the address of the blockchain
	source: AccountId,
	/// the current domain price
	price: Balance,
	/// Time to claim the ownership
	ttl: BlockNumber,
	/// Registered date in block height
	registered_date: BlockNumber,
	/// whether it is available for purchase or sale
	available: bool,
	/// highest bid in the auction stage
	highest_bid: Balance,
	/// bidder who bidded highest
	bidder: AccountId,
	/// Auction closing date
	auction_closed: BlockNumber,

	/// TODO: Try to make browser engine which asks for this with Servo fork
	/// IPV4 in case where the owner wants to put IP address
	ipv4: IPV4,
	/// IPV6 in case where the owner wants to put IP address for his or her IoT device
	ipv6: IPV6,
}

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Define  currency for fungible assets
		type Currency: Currency<Self::AccountId>;
	}

	// Alias type Balance
	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn domains)]
	pub(super) type Domains<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn resolver)]
	pub(super) type Resolver<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash,
		Domain<T::AccountId, BalanceOf<T>, T::BlockNumber>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn reverse)]
	pub(super) type Reverse<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::Hash>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		DomainRegistered(T::AccountId, BalanceOf<T>, T::BlockNumber, T::BlockNumber),
		SetIPV4(T::Hash, Vec<u16>, Vec<u16>),
		NewAuction(T::AccountId, T::Hash, T::BlockNumber, T::BlockNumber),
		NewBid(T::AccountId, T::Hash, BalanceOf<T>),
		AuctionFinalized(T::AccountId, T::Hash, BalanceOf<T>),
		DomainResolved(
			T::Hash,
			T::AccountId,
			BalanceOf<T>,
			bool,
			BalanceOf<T>,
			T::AccountId,
			T::BlockNumber,
		),
		ReverseResolved(T::AccountId, Vec<T::Hash>),
		DomainRenewed(T::Hash, T::AccountId, T::BlockNumber),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Domain Already Exists
		DomainAlreadyExists,
		/// The domain does not exist
		DomainNotExists,
		/// 	The auction for the domain is currently not available
		DomainNotExistsForAuction,
		/// The bid for the auction is already finalized
		BidForAuctionAlreadyFinalized,
		/// 	Please Bid higher
		BidHigher,
		/// 	The domain is not registered yet
		DomainNotRegistered,
		/// 	"The auction has not been finalized yet"
		AuctionNotFinalized,
		/// 	The account have not registered or owned any domain
		AccountNotRegisteredOrOwnedAnyDomain,
		/// 	You are either not the source of the domain or the domain is expired
		YourNotDomainSourceOrDomainExpired,
		// 	You are not the source of the domain
		YourNotSourceOfDomain,
		// 	You are neither the source of the domain or the claimer after the domain's TTL
		YourNeitherSourceDomainOrClaimerAfterTTL,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000)]
		pub fn register_domain(
			origin: OriginFor<T>,
			domain_hash: T::Hash,
			domain_name: BYTES,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(!<Resolver<T>>::contains_key(domain_hash), Error::<T>::DomainAlreadyExists);
			// Make new Domain struct
			let new_domain = Self::new_domain(domain_name, sender.clone());

			// Try to withdraw registration fee from the user without killing the account
			let _ = T::Currency::withdraw(
				&sender.clone(),
				new_domain.price,
				WithdrawReasons::all(),
				ExistenceRequirement::KeepAlive,
			)?;

			<Reverse<T>>::insert(sender.clone(), vec![domain_hash]);
			// Insert new domain to the Resolver state
			<Resolver<T>>::insert(domain_hash, new_domain.clone());

			// Increment domain number
			let mut domains = Self::domains();
			domains = domains.wrapping_add(1);

			// Store domain number to Domains state
			Domains::<T>::put(domains);

			// Deposit event
			Self::deposit_event(Event::DomainRegistered(
				sender.clone(),
				new_domain.price,
				new_domain.ttl,
				new_domain.registered_date,
			));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		/// Set IPV4 for existing domain
		pub fn set_ipv4(origin: OriginFor<T>, domain_hash: T::Hash, ipv4: IPV4) -> DispatchResult {
			// Ensure that
			// domain exists
			ensure!(<Resolver<T>>::contains_key(domain_hash), Error::<T>::DomainNotExists);
			// the sender is the source of the domain
			let sender = ensure_signed(origin)?;
			let mut new_domain = Resolver::<T>::get(&domain_hash);
			// let mut new_domain = Self::domain(domain_hash);
			// Self::Domain{ name: domain_hash,source: origin};
			ensure!(sender == new_domain.source, Error::<T>::YourNotSourceOfDomain);
			// Set ipv4 for new domain
			let old_ipv4 = new_domain.ipv4;
			new_domain.ipv4 = ipv4;

			// Change domain data with the new one and emit event
			<Resolver<T>>::mutate(domain_hash.clone(), |d| *d = new_domain.clone());
			Self::deposit_event(Event::SetIPV4(
				domain_hash,
				old_ipv4.to_vec(),
				new_domain.ipv4.to_vec(),
			));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn resolve(_origin: OriginFor<T>, domain_hash: T::Hash) -> DispatchResult {
			ensure!(<Resolver<T>>::contains_key(domain_hash), Error::<T>::DomainNotExists);
			let domain = Resolver::<T>::get(&domain_hash);
			Self::deposit_event(Event::DomainResolved(
				domain_hash,
				domain.source,
				domain.price,
				domain.available,
				domain.highest_bid,
				domain.bidder,
				domain.auction_closed,
			));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn renew(origin: OriginFor<T>, domain_hash: T::Hash) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let mut new_domain = Resolver::<T>::get(&domain_hash);
			let now = <frame_system::Pallet<T>>::block_number();
			// Ensure the sender is the source of the domain and its ttl is not expired
			ensure!(
				new_domain.source == sender && now < new_domain.registered_date + new_domain.ttl,
				Error::<T>::YourNotDomainSourceOrDomainExpired
			);
			// Extend domain TTL by a year
			let ttl = T::BlockNumber::from(YEAR);
			new_domain.ttl += ttl;

			// Try to withdraw price from the user account to renew the domain
			let _ = T::Currency::withdraw(
				&sender,
				new_domain.price,
				WithdrawReasons::all(),
				ExistenceRequirement::KeepAlive,
			)?;

			// mutate domain with new_domain struct in the Domain state
			<Resolver<T>>::mutate(domain_hash.clone(), |domain| *domain = new_domain.clone());
			Self::deposit_event(Event::DomainRenewed(
				domain_hash,
				sender,
				new_domain.registered_date + new_domain.ttl,
			));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn claim_auction(origin: OriginFor<T>, domain_hash: T::Hash) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// Ensure that
			// Domain does already exist
			ensure!(<Resolver<T>>::contains_key(domain_hash), Error::<T>::DomainNotExists);
			// But wait, get domain data and time
			let mut new_domain = Resolver::<T>::get(&domain_hash);
			let now = <frame_system::Pallet<T>>::block_number();
			// Ensure the sender is the source of the domain or its ttl is expired
			ensure!(
				sender == new_domain.source || new_domain.registered_date + new_domain.ttl < now,
				Error::<T>::YourNeitherSourceDomainOrClaimerAfterTTL
			);

			// Set domain available for selling
			new_domain.available = true;

			// Set auction to be closed after 1 hour(60* 60 seconds) * 1000(milliseconds conversion) using timestamp
			let converted = T::BlockNumber::from(600u32);
			new_domain.auction_closed = now + converted;

			// mutate domain with new_domain struct in the Domain state
			<Resolver<T>>::mutate(domain_hash.clone(), |domain| *domain = new_domain.clone());
			Self::deposit_event(Event::NewAuction(
				sender,
				domain_hash,
				now,
				new_domain.auction_closed,
			));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn new_bid(
			origin: OriginFor<T>,
			domain_hash: T::Hash,
			bid: BalanceOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// Ensure that
			// Domain does already exist
			ensure!(<Resolver<T>>::contains_key(domain_hash), Error::<T>::DomainNotExists);
			// But wait, get domain data
			let mut new_domain = Resolver::<T>::get(&domain_hash);
			// The auction is available
			ensure!(new_domain.available, Error::<T>::DomainNotExistsForAuction);
			// The auction is not finalized
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(new_domain.auction_closed > now, Error::<T>::BidForAuctionAlreadyFinalized);
			// The bid price is higher than the current highest bid
			ensure!(new_domain.highest_bid < bid.clone(), Error::<T>::BidHigher);

			// Set new domain data
			new_domain.bidder = sender.clone();
			new_domain.highest_bid = bid.clone();
			// mutate domain with new_domain struct in the Domain state
			<Resolver<T>>::mutate(domain_hash.clone(), |domain| *domain = new_domain.clone());
			Self::deposit_event(Event::NewBid(sender, domain_hash, bid));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn finalize_auction(origin: OriginFor<T>, domain_hash: T::Hash) -> DispatchResult {
			let _sender = ensure_signed(origin)?;
			// Ensure that
			// Domain does already exist
			ensure!(<Resolver<T>>::contains_key(domain_hash), Error::<T>::DomainNotRegistered);
			// But wait, get domain data and time
			let mut new_domain = Resolver::<T>::get(&domain_hash);
			let now = <frame_system::Pallet<T>>::block_number();
			// The auction is available
			ensure!(new_domain.available, Error::<T>::DomainNotExistsForAuction);
			// The auction is finalized or the source wants to finalize the auction(test)
			// TEST: If you want to test auction finalization without waiting for 1 hour, just add '|| sender == new_domain.source in ensure! macro
			ensure!(now > new_domain.auction_closed, Error::<T>::AuctionNotFinalized);

			let _ = T::Currency::transfer(
				&new_domain.bidder,
				&new_domain.source,
				new_domain.highest_bid,
				ExistenceRequirement::KeepAlive,
			);

			let ttl = T::BlockNumber::from(YEAR);

			// Remove domain hash from the prior owner's reverse registrar
			let old_reverse = Reverse::<T>::get(new_domain.source.clone());

			let new_reverse = Self::remove_domain(domain_hash.clone(), old_reverse);

			// Mutate reverse with new_reverse arrray in the Reverse state
			<Reverse<T>>::mutate(new_domain.source.clone(), |account| {
				*account = new_reverse.clone()
			});
			// Set reverse for the new owner
			// if the account is in reverse registrar
			if <Reverse<T>>::contains_key(new_domain.bidder.clone()) {
				let mut new_reverse: Vec<T::Hash> = Reverse::<T>::get(new_domain.bidder.clone());
				new_reverse.push(domain_hash.clone());
				// Mutate reverse with new_reverse arrray in the Reverse state
				<Reverse<T>>::mutate(new_domain.bidder.clone(), |reverses: &mut Vec<T::Hash>| {
					*reverses = new_reverse.clone()
				});
			} else {
				let new_reverse = vec![domain_hash];
				<Reverse<T>>::insert(new_domain.bidder.clone(), new_reverse.clone());
			}

			// Set new domain data to bidder as source, highest_bid as price, and reinitialize rest of them
			new_domain.source = new_domain.bidder.clone();
			new_domain.price = new_domain.highest_bid;
			new_domain.available = false;
			new_domain.ttl = ttl;
			new_domain.registered_date = now;
			new_domain.available = false;
			new_domain.highest_bid = 0u32.into();

			new_domain.auction_closed = T::BlockNumber::from(0u32);

			// Mutate domain with new_domain struct in the Domain state
			<Resolver<T>>::mutate(domain_hash.clone(), |domain| *domain = new_domain.clone());
			Self::deposit_event(Event::AuctionFinalized(
				new_domain.bidder,
				domain_hash,
				new_domain.highest_bid,
			));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn reverse_resolve(_origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
			ensure!(
				<Reverse<T>>::contains_key(account_id.clone()),
				Error::<T>::AccountNotRegisteredOrOwnedAnyDomain
			);
			let domains = Reverse::<T>::get(account_id.clone());
			Self::deposit_event(Event::ReverseResolved(account_id, domains));

			Ok(())
		}
	}
}

// Module's function and Methods of custom struct to be placed here
impl<T: Config> Pallet<T> {
	pub fn new_domain(
		domain_name: BYTES,
		source: T::AccountId,
	) -> Domain<T::AccountId, BalanceOf<T>, T::BlockNumber> {
		// Convert numbers into generic types which is mapped to native type in lib.rs
		// Generic types can process arithmetics and comparisons just as other rust variables
		let ttl = T::BlockNumber::from(YEAR);
		let init_price = Self::to_balance(1, "milli");
		let reg_date: T::BlockNumber = <frame_system::Pallet<T>>::block_number();
		Domain {
			name: domain_name,
			source: source.clone(),
			price: init_price,
			ttl,
			registered_date: reg_date,
			available: false,
			highest_bid: 0u32.into(),
			// highest_bid: 0,
			bidder: source,
			auction_closed: T::BlockNumber::from(0u32),
			ipv4: [0, 0, 0, 0],
			ipv6: [0, 0, 0, 0, 0, 0],
		}
	}

	// TODO: Add this to <balances::Module<T>> and test with u128
	/// Convert u32 to u128 generic type Balance type
	pub fn to_balance(u: u32, digit: &str) -> BalanceOf<T> {
		// Power exponent function
		let pow = |u: u32, p: u32| -> BalanceOf<T> {
			let mut base = u.into();
			for _i in 0..p {
				base *= 10u32.into();
			}
			return base
		};
		let result = match digit {
			"femto" => u.into(),
			"nano" => pow(u, 3),
			"micro" => pow(u, 6),
			"milli" => pow(u, 9),
			"one" => pow(u, 12),
			"kilo" => pow(u, 15),
			"mega" => pow(u, 18),
			"giga" => pow(u, 21),
			"tera" => pow(u, 24),
			"peta" => pow(u, 27),
			"exa" => pow(u, 30),
			"zetta" => pow(u, 33),
			"yotta" => pow(u, 36),
			_ => u.into(),
		};
		result
	}

	pub fn remove_domain(domain_hash: T::Hash, domains: Vec<T::Hash>) -> Vec<T::Hash> {
		let mut new_reverse_list: Vec<T::Hash> = vec![];

		for i in domains {
			if i != domain_hash {
				new_reverse_list.push(i);
			}
		}

		return new_reverse_list
	}
}
