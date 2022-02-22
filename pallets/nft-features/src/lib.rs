#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{
	dispatch::DispatchResult,
	pallet_prelude::*,
	traits::{Currency, NamedReservableCurrency},
	transactional, PalletId,
};
use frame_system::pallet_prelude::*;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
// #[cfg(feature = "std")]
// use serde::{Deserialize, Serialize};
use sp_runtime::traits::Saturating;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

pub type Data = Vec<u8>;

#[derive(Deserialize, Serialize, Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ClassData<Balance> {
	pub deposit: Balance,
	/// Class data
	//#[serde(with = "serde_bytes")]
	pub classdata: Data,
}

#[derive(Deserialize, Serialize, Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TokenData<Balance> {
	pub deposit: Balance,
	/// Token data
	pub tokendata: Data,
}

pub type TokenIdOf<T> = <T as pallet_nft::Config>::TokenId;
pub type ClassIdOf<T> = <T as pallet_nft::Config>::ClassId;
pub const RESERVE_NFT_ID: [u8; 8] = *b"nft/5ire";

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ pallet_nft::Config<
			TokenData = TokenData<BalanceOf<Self>>,
			ClassData = ClassData<BalanceOf<Self>>,
		>
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The minimum balance to create class
		#[pallet::constant]
		type ClassDepositBase: Get<BalanceOf<Self>>;

		/// The minimum balance to create token
		#[pallet::constant]
		type TokenDepositBase: Get<BalanceOf<Self>>;

		// type Currency: NamedReservableCurrency<Self::AccountId>;

		type Currency: NamedReservableCurrency<Self::AccountId, ReserveIdentifier = [u8; 8]>;
		/// Maximum number of bytes in data
		#[pallet::constant]
		type MaxDataBytes: Get<u32>;

		#[pallet::constant]
		type DataNftDepositPerByte: Get<BalanceOf<Self>>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CreatedClass {
			owner_id: T::AccountId,
			class_id: ClassIdOf<T>,
		},
		MintSuccessToken {
			class_id: ClassIdOf<T>,
		},

		TransferSuccessToken {
			from: T::AccountId,
			to: T::AccountId,
			class_id: ClassIdOf<T>,
			token_id: TokenIdOf<T>,
		},
		BurnSuccessToken {
			owner_id: T::AccountId,
			class_id: ClassIdOf<T>,
			token_id: TokenIdOf<T>,
		},
		DestroySuccessToken {
			owner_id: T::AccountId,
			class_id: ClassIdOf<T>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ClassIdNotExist,
		TokenIdNotExist,
		NotOwner,
		NotTransfer,
		NotMint,
		NotBurn,
		DataTooLarge,
		NftBankNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn create_class(
			origin: OriginFor<T>,
			metadata: Vec<u8>,
			classdata: Data,
		) -> DispatchResult {
			let creator = ensure_signed(origin)?;
			let next_id = pallet_nft::Pallet::<T>::next_class_id();
			let class_deposit_base = T::ClassDepositBase::get();

			// get amount deposit = base_deposit_per_byte * (length(metadata) + length(data))
			let class_deposit = Self::data_deposit(&metadata, &classdata)?;

			// deposit = base_class_deposit + data_deposit
			let deposit = class_deposit_base.saturating_add(class_deposit);

			// ensure enough token for class deposit + data deposit
			// T::Currency::transfer(&creator, &sub_creator, deposit, KeepAlive)?;
			<T as Config>::Currency::reserve_named(&RESERVE_NFT_ID, &creator, deposit)?;
			let class_data = ClassData { deposit, classdata };

			pallet_nft::Pallet::<T>::create_class(&creator, metadata, class_data)?;

			Self::deposit_event(Event::CreatedClass { owner_id: creator, class_id: next_id });
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn mint(
			origin: OriginFor<T>,
			class_id: ClassIdOf<T>,
			metadata: Vec<u8>,
			tokendata: Data,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// Get class info
			let class_info =
				pallet_nft::Pallet::<T>::classes(&class_id).ok_or(Error::<T>::ClassIdNotExist)?;
			// User must be class owner, then mint token nft
			ensure!(sender == class_info.owner, Error::<T>::NotOwner);
			// get amount deposit = base_deposit_per_byte * (length(metadata) + length(data))
			let token_deposit = Self::data_deposit(&metadata, &tokendata)?;

			let base_token_deposit = T::TokenDepositBase::get();

			// deposit = base_token_deposit + data_deposit
			let deposit = base_token_deposit.saturating_add(token_deposit);

			// Token data Onchain storage
			let token_data = TokenData { deposit, tokendata };

			// Mint token
			let _ = pallet_nft::Pallet::<T>::mint(&sender, class_id, metadata, token_data);
			<T as Config>::Currency::reserve_named(&RESERVE_NFT_ID, &sender, deposit)?;
			Self::deposit_event(Event::MintSuccessToken { class_id });

			Ok(())
		}

		/// Tranfer NFT token
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			token: (ClassIdOf<T>, TokenIdOf<T>),
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = pallet_nft::Pallet::<T>::classes(token.0).ok_or(Error::<T>::ClassIdNotExist)?;
			let token_info = pallet_nft::Pallet::<T>::tokens(token.0, token.1)
				.ok_or(Error::<T>::TokenIdNotExist)?;
			let _ = pallet_nft::Pallet::<T>::transfer(&who, &to, token)?;

			<T as Config>::Currency::unreserve_named(
				&RESERVE_NFT_ID,
				&who,
				token_info.data.deposit,
			);
			//<T as Config>::Currency::transfer(&who, &to, token_info.data.deposit, KeepAlive)?;
			<T as Config>::Currency::reserve_named(&RESERVE_NFT_ID, &to, token_info.data.deposit)?;
			Self::deposit_event(Event::TransferSuccessToken {
				from: who,
				to,
				class_id: token.0,
				token_id: token.1,
			});
			Ok(())
		}

		/// Burn NFT token
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn burn_token(
			origin: OriginFor<T>,
			token: (ClassIdOf<T>, TokenIdOf<T>),
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = pallet_nft::Pallet::<T>::classes(token.0).ok_or(Error::<T>::ClassIdNotExist)?;
			let token_info = pallet_nft::Pallet::<T>::tokens(token.0, token.1)
				.ok_or(Error::<T>::TokenIdNotExist)?;
			ensure!(who == token_info.owner, Error::<T>::NotOwner);
			let _ = pallet_nft::Pallet::<T>::burn(&who, token)?;

			<T as Config>::Currency::unreserve_named(
				&RESERVE_NFT_ID,
				&who,
				token_info.data.deposit,
			);

			Self::deposit_event(Event::BurnSuccessToken {
				owner_id: who,
				class_id: token.0,
				token_id: token.1,
			});
			Ok(())
		}
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn destroy_class(origin: OriginFor<T>, class_id: ClassIdOf<T>) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let class_info =
				pallet_nft::Pallet::<T>::classes(class_id).ok_or(Error::<T>::ClassIdNotExist)?;

			ensure!(owner == class_info.owner, Error::<T>::NotOwner);
			let _ = pallet_nft::Pallet::<T>::destroy_class(&owner, class_id)?;
			<T as Config>::Currency::unreserve_named(
				&RESERVE_NFT_ID,
				&owner,
				class_info.data.deposit,
			);

			Self::deposit_event(Event::DestroySuccessToken { owner_id: owner, class_id });
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn data_deposit(metadata: &[u8], data: &Data) -> Result<BalanceOf<T>, DispatchError> {
		ensure!((data.len() as u32) < T::MaxDataBytes::get(), Error::<T>::DataTooLarge);

		let total_data = metadata.len().saturating_add(data.len()) as u32;

		Ok(T::DataNftDepositPerByte::get().saturating_mul(total_data.into()))
	}
}
