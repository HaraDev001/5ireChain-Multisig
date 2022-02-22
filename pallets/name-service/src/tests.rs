#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok, traits::Currency};
use mock::{Event, *};
use sp_core::H256;
#[test]
fn it_works_for_domain_zero() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_eq!(NameService::domains(), 0);
	});
}

#[test]
fn test_register_domain() {
	new_test_ext().execute_with(|| {
		let dummy_hash: H256 = [2; 32].into();
		let domain_name = "5ire.org".as_bytes().to_vec();
		assert_ok!(NameService::register_domain(Origin::signed(ALICE), dummy_hash, domain_name));
		assert_eq!(NameService::resolver(dummy_hash).source, ALICE);
	});
}

// TODO: Test other functions with features
// - Catching events after the event
// - Set balance of the test account

#[test]
fn test_set_ipv4() {
	new_test_ext().execute_with(|| {
		let dummy_hash: H256 = [1; 32].into();
		let domain_name = "5ire.org".as_bytes().to_vec();
		let ipv4: [u16; 4] = [0; 4];
		assert_ok!(NameService::register_domain(Origin::signed(ALICE), dummy_hash, domain_name));
		assert_ok!(NameService::set_ipv4(Origin::signed(ALICE), dummy_hash, ipv4));
	})
}

#[test]
fn test_renew() {
	new_test_ext().execute_with(|| {
		let dummy_hash: H256 = [1; 32].into();
		let domain_name = "5ire.org".as_bytes().to_vec();
		assert_ok!(NameService::register_domain(Origin::signed(ALICE), dummy_hash, domain_name));
		assert_ok!(NameService::renew(Origin::signed(ALICE), dummy_hash));
	})
}
#[test]
fn test_claim_auction() {
	new_test_ext().execute_with(|| {
		let dummy_hash: H256 = [1; 32].into();
		let domain_name = "5ire.org".as_bytes().to_vec();
		assert_ok!(NameService::register_domain(Origin::signed(ALICE), dummy_hash, domain_name));
		assert_ok!(NameService::claim_auction(Origin::signed(ALICE), dummy_hash));
	})
}

#[test]
fn test_finalize_auction() {
	new_test_ext().execute_with(|| {
		let dummy_hash: H256 = [1; 32].into();
		let domain_name = "5ire.org".as_bytes().to_vec();
		System::set_block_number(1);
		assert_ok!(NameService::register_domain(Origin::signed(ALICE), dummy_hash, domain_name));
		assert_ok!(NameService::claim_auction(Origin::signed(ALICE), dummy_hash));
		let resolve = Resolver::<Test>::get(&dummy_hash);
		// after 1 hours 60*60
		let auction_closed = System::block_number() + 600u64;
		assert_eq!(resolve.auction_closed, auction_closed);
		System::set_block_number(602);
		assert_ok!(NameService::finalize_auction(Origin::signed(ALICE), dummy_hash));
	})
}
