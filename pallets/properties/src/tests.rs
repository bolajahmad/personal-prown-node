use crate::{mock::*, Event};
use frame_support::assert_ok;

#[test]
#[should_panic]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// define function calls
		let property_id = 10;
		let property_owner = 10_u64;
		let asset_cid: Vec<u8> = "hdyv36ghvxh".as_bytes().to_vec();
		// Dispatch a signed extrinsic.
		assert_ok!(PropertiesMod::create_new_property(
			RuntimeOrigin::signed(1),
			property_id,
			property_owner,
			asset_cid
		));
		// Read pallet storage and assert an expected result.
		assert_eq!(PropertiesMod::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(
			Event::PropertyCreated {
				property_id: 10,
				owner: 100,
				proposal_cid: "hdyv36ghvxh".as_bytes().to_vec(),
			}
			.into(),
		);
	});
}
