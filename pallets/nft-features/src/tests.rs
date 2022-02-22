#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok, traits::Currency};
use mock::{Event, *};
use node_primitives::Balance;
use sp_runtime::traits::AccountIdConversion;

fn class_id_account() -> AccountId {
	<Runtime as Config>::PalletId::get().into_sub_account(CLASS_ID)
}

fn reserved_balance(who: &AccountId) -> Balance {
	<Runtime as Config>::Currency::reserved_balance(who)
}
#[test]
fn create_class_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = vec![1];
		let classdata = vec![1, 2, 3];
		// class_id =0
		assert_eq!(pallet_nft::Pallet::<Runtime>::next_class_id(), 0);

		assert_ok!(NftFeatures::create_class(
			Origin::signed(ALICE),
			metadata.clone(),
			classdata.clone(),
		));

		// next class id =1
		assert_eq!(pallet_nft::Pallet::<Runtime>::next_class_id(), 1);
		System::assert_last_event(Event::NftFeatures(crate::Event::CreatedClass {
			owner_id: class_id_account(),
			class_id: CLASS_ID,
		}));

		let class_deposit = ClassDepositBase::get() +
			DataNftDepositPerByte::get() * (metadata.len() as u128 + classdata.len() as u128);

		assert_eq!(reserved_balance(&class_id_account()), class_deposit);

		let class_info = pallet_nft::Pallet::<Runtime>::classes(CLASS_ID).unwrap();
		assert_eq!(class_info.data.deposit, class_deposit);
	})
}

#[test]
fn create_class_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = vec![1];
		let classdata = vec![1, 2, 3];

		let classdata_large = (0..10).collect::<Vec<u8>>();

		// check origin have no balance to execute extrinsic call
		assert_noop!(
			NftFeatures::create_class(Origin::signed(BOB), metadata.clone(), classdata.clone(),),
			pallet_balances::Error::<Runtime>::InsufficientBalance
		);
		assert_eq!(reserved_balance(&class_id_account()), 0);

		// check data too large
		assert_noop!(
			NftFeatures::create_class(
				Origin::signed(ALICE),
				metadata.clone(),
				classdata_large.clone()
			),
			Error::<Runtime>::DataTooLarge
		);
	})
}

#[test]
fn mint_token_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = vec![1];
		let classdata = vec![1, 2, 3];
		assert_ok!(NftFeatures::create_class(
			Origin::signed(ALICE),
			metadata.clone(),
			classdata.clone(),
		));
		assert_eq!(pallet_nft::Pallet::<Runtime>::next_class_id(), 1);

		let class_deposit = ClassDepositBase::get() +
			DataNftDepositPerByte::get() * (metadata.len() as u128 + classdata.len() as u128);

		assert_eq!(reserved_balance(&class_id_account()), class_deposit);

		let metadata_2 = vec![2];
		let tokendata = vec![1, 2];

		let token_deposit = TokenDepositBase::get() +
			DataNftDepositPerByte::get() * (metadata_2.len() as u128 + tokendata.len() as u128);
		// mint token for sub account
		assert_ok!(Balances::deposit_into_existing(&class_id_account(), token_deposit));
		assert_ok!(NftFeatures::mint(
			Origin::signed(class_id_account()),
			CLASS_ID,
			metadata_2.clone(),
			tokendata.clone()
		));

		let token_info = pallet_nft::Pallet::<Runtime>::tokens(CLASS_ID, TOKEN_ID).unwrap();
		assert_eq!(reserved_balance(&class_id_account()), class_deposit + token_deposit);
		assert_eq!(token_info.data.deposit, token_deposit);
		assert_eq!(token_info.metadata, metadata_2);
		assert_eq!(token_info.data.tokendata, tokendata);
	})
}

#[test]
fn mint_token_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = vec![1];
		let classdata = vec![1, 2, 3];
		assert_ok!(NftFeatures::create_class(
			Origin::signed(ALICE),
			metadata.clone(),
			classdata.clone(),
		));

		let metadata_2 = vec![2];
		let tokendata = vec![1, 2];
		let tokendata_large = (0..10).collect::<Vec<u8>>();
		let token_deposit = TokenDepositBase::get() +
			DataNftDepositPerByte::get() * (metadata_2.len() as u128 + tokendata.len() as u128);
		assert_ok!(Balances::deposit_into_existing(&class_id_account(), token_deposit));

		assert_noop!(
			NftFeatures::mint(
				Origin::signed(class_id_account()),
				CLASS_ID_NOT_EXIST,
				metadata_2.clone(),
				tokendata.clone()
			),
			Error::<Runtime>::ClassIdNotExist
		);

		assert_noop!(
			NftFeatures::mint(
				Origin::signed(class_id_account()),
				CLASS_ID,
				metadata_2.clone(),
				tokendata_large.clone()
			),
			Error::<Runtime>::DataTooLarge
		);
	})
}

#[test]
fn transfer_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = vec![1];
		let classdata = vec![1, 2, 3];
		assert_ok!(NftFeatures::create_class(
			Origin::signed(ALICE),
			metadata.clone(),
			classdata.clone(),
		));

		let class_deposit = ClassDepositBase::get() +
			DataNftDepositPerByte::get() * (metadata.len() as u128 + classdata.len() as u128);

		let metadata_2 = vec![2];
		let tokendata = vec![1, 2];
		let token_deposit = TokenDepositBase::get() +
			DataNftDepositPerByte::get() * (metadata_2.len() as u128 + tokendata.len() as u128);
		// mint token for sub account
		assert_ok!(Balances::deposit_into_existing(&class_id_account(), token_deposit));
		assert_ok!(NftFeatures::mint(
			Origin::signed(class_id_account()),
			CLASS_ID,
			metadata_2.clone(),
			tokendata.clone()
		));

		// before tranfer to BOB
		assert_eq!(reserved_balance(&class_id_account()), class_deposit + token_deposit);
		// after transfer to BOB
		assert_ok!(NftFeatures::transfer(
			Origin::signed(class_id_account()),
			BOB,
			(CLASS_ID, TOKEN_ID)
		));

		assert_eq!(reserved_balance(&class_id_account()), class_deposit);
		assert_eq!(reserved_balance(&BOB), token_deposit);
		System::assert_last_event(Event::NftFeatures(crate::Event::TransferSuccessToken {
			from: class_id_account(),
			to: BOB,
			class_id: CLASS_ID,
			token_id: TOKEN_ID,
		}));
	})
}

#[test]
fn tranfer_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = vec![1];
		let classdata = vec![1, 2, 3];
		assert_ok!(NftFeatures::create_class(
			Origin::signed(ALICE),
			metadata.clone(),
			classdata.clone(),
		));

		let metadata_2 = vec![2];
		let tokendata = vec![1, 2];
		let token_deposit = TokenDepositBase::get() +
			DataNftDepositPerByte::get() * (metadata_2.len() as u128 + tokendata.len() as u128);
		// mint token for sub account
		assert_ok!(Balances::deposit_into_existing(&class_id_account(), token_deposit));
		assert_ok!(NftFeatures::mint(
			Origin::signed(class_id_account()),
			CLASS_ID,
			metadata_2.clone(),
			tokendata.clone()
		));

		// should be owner
		assert_noop!(
			NftFeatures::transfer(Origin::signed(ALICE), BOB, (CLASS_ID, TOKEN_ID)),
			pallet_nft::Error::<Runtime>::NoPermission
		);

		// class id must exist
		assert_noop!(
			NftFeatures::transfer(
				Origin::signed(class_id_account()),
				BOB,
				(CLASS_ID_NOT_EXIST, TOKEN_ID)
			),
			Error::<Runtime>::ClassIdNotExist
		);

		// token id must exist
		assert_noop!(
			NftFeatures::transfer(
				Origin::signed(class_id_account()),
				BOB,
				(CLASS_ID, TOKEN_ID_NOT_EXIST)
			),
			Error::<Runtime>::TokenIdNotExist
		);

		// BOB doesnt deposit
		assert_eq!(reserved_balance(&BOB), 0);
	})
}

#[test]
fn burn_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = vec![1];
		let classdata = vec![1, 2, 3];
		assert_ok!(NftFeatures::create_class(
			Origin::signed(ALICE),
			metadata.clone(),
			classdata.clone(),
		));

		let class_deposit = ClassDepositBase::get() +
			DataNftDepositPerByte::get() * (metadata.len() as u128 + classdata.len() as u128);

		let metadata_2 = vec![2];
		let tokendata = vec![1, 2];
		let token_deposit = TokenDepositBase::get() +
			DataNftDepositPerByte::get() * (metadata_2.len() as u128 + tokendata.len() as u128);
		// mint token for sub account
		assert_ok!(Balances::deposit_into_existing(&class_id_account(), token_deposit));
		assert_ok!(NftFeatures::mint(
			Origin::signed(class_id_account()),
			CLASS_ID,
			metadata_2.clone(),
			tokendata.clone()
		));
		assert_eq!(reserved_balance(&class_id_account()), class_deposit + token_deposit);
		assert_ok!(NftFeatures::burn_token(
			Origin::signed(class_id_account()),
			(CLASS_ID, TOKEN_ID)
		));

		assert_eq!(reserved_balance(&class_id_account()), class_deposit);
		System::assert_last_event(Event::NftFeatures(crate::Event::BurnSuccessToken {
			owner_id: class_id_account(),
			class_id: CLASS_ID,
			token_id: TOKEN_ID,
		}));

		// token should be remove
		let token_info = pallet_nft::Pallet::<Runtime>::tokens(CLASS_ID, TOKEN_ID);
		assert_eq!(token_info, None);
		// class should be exist
		let class_info = pallet_nft::Pallet::<Runtime>::classes(CLASS_ID);
		assert_eq!(class_info.is_some(), true);
	})
}

#[test]
fn burn_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = vec![1];
		let classdata = vec![1, 2, 3];
		assert_ok!(NftFeatures::create_class(
			Origin::signed(ALICE),
			metadata.clone(),
			classdata.clone(),
		));

		let metadata_2 = vec![2];
		let tokendata = vec![1, 2];
		let token_deposit = TokenDepositBase::get() +
			DataNftDepositPerByte::get() * (metadata_2.len() as u128 + tokendata.len() as u128);

		assert_ok!(Balances::deposit_into_existing(&class_id_account(), token_deposit));
		assert_ok!(NftFeatures::mint(
			Origin::signed(class_id_account()),
			CLASS_ID,
			metadata_2.clone(),
			tokendata.clone()
		));

		assert_noop!(
			NftFeatures::burn_token(
				Origin::signed(class_id_account()),
				(CLASS_ID_NOT_EXIST, TOKEN_ID)
			),
			Error::<Runtime>::ClassIdNotExist
		);

		assert_noop!(
			NftFeatures::burn_token(Origin::signed(BOB), (CLASS_ID, TOKEN_ID)),
			Error::<Runtime>::NotOwner
		);

		assert_noop!(
			NftFeatures::burn_token(
				Origin::signed(class_id_account()),
				(CLASS_ID, TOKEN_ID_NOT_EXIST)
			),
			Error::<Runtime>::TokenIdNotExist
		);
	})
}
