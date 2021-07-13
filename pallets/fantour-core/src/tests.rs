#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{Event, *};

#[test]
fn test_whitelist() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(None, FantourCore::account_whitelist(ALICE));
		assert_eq!(None, FantourCore::account_whitelist(BOB));
		assert_ok!(FantourCore::add_whitelist(Origin::root(), ALICE));
		assert_eq!(last_event(), Event::fantour_core(crate::Event::AddWhitelist(ALICE)));
		assert_eq!(Some(()), FantourCore::account_whitelist(ALICE));
		assert_noop!(
			FantourCore::add_whitelist(Origin::signed(BOB), BOB),
			DispatchError::BadOrigin,
		);

		assert_ok!(FantourCore::remove_whitelist(Origin::root(), ALICE));
		assert_eq!(last_event(), Event::fantour_core(crate::Event::RemoveWhitelist(ALICE)));
		assert_eq!(None, FantourCore::account_whitelist(ALICE));
		assert_eq!(None, FantourCore::account_whitelist(BOB));
	});
}
