use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_create() {
	new_test_ext().execute_with(|| {
    assert_ok!(TemplateModule::create(Origin::signed(1)));
    let event = Event::TemplateModule(crate::Event::Created(
			1,
			1
		));
		assert_eq!(last_event(), event);

	});
}

#[test]
fn test_breed() {
	new_test_ext().execute_with(|| {
    assert_ok!(TemplateModule::create(Origin::signed(1)));
    let event = Event::TemplateModule(crate::Event::Created(
			1,
			1
		));
		assert_eq!(last_event(), event);

    System::set_block_number(2);
    assert_ok!(TemplateModule::create(Origin::signed(1)));
    let event = Event::TemplateModule(crate::Event::Created(
			1,
			2
		));
		assert_eq!(last_event(), event);

    assert_noop!(
			TemplateModule::breed(Origin::signed(1), 9, 10),
			Error::<Test>::InvalidKittyId
		);

    assert_ok!(TemplateModule::breed(Origin::signed(1), 1, 2));
    let event = Event::TemplateModule(crate::Event::Created(
			1,
			3
		));
		assert_eq!(last_event(), event);

	});
}

#[test]
fn test_transfer() {
	new_test_ext().execute_with(|| {
    assert_ok!(TemplateModule::create(Origin::signed(1)));
    let event = Event::TemplateModule(crate::Event::Created(
			1,
			1
		));
		assert_eq!(last_event(), event);

    assert_noop!(
			TemplateModule::transfer(
				Origin::signed(2),
        3,
        1
			),
			Error::<Test>::RequireOwner
		);

    assert_ok!(TemplateModule::transfer(Origin::signed(1), 2, 1));
    let event = Event::TemplateModule(crate::Event::Transferred(
			1,
			2,
      1
		));
		assert_eq!(last_event(), event);

	});
}

#[test]
fn test_ask() {
	new_test_ext().execute_with(|| {
    assert_ok!(TemplateModule::create(Origin::signed(1)));
    let event = Event::TemplateModule(crate::Event::Created(
			1,
			1
		));
		assert_eq!(last_event(), event);

    assert_noop!(
			TemplateModule::ask(Origin::signed(2), 1, Some(1)),
			Error::<Test>::RequireOwner
		);

    assert_ok!(TemplateModule::ask(Origin::signed(1), 1, Some(1)));
    let event = Event::TemplateModule(crate::Event::Ask(
			1,
			1,
      Some(1)
		));
		assert_eq!(last_event(), event);

	});
}

#[test]
fn test_buy() {
	new_test_ext().execute_with(|| {

    assert_ok!(TemplateModule::create(Origin::signed(1)));
    let event = Event::TemplateModule(crate::Event::Created(
			1,
			1
		));
		assert_eq!(last_event(), event);
    System::set_block_number(2);
    assert_ok!(TemplateModule::create(Origin::signed(1)));
    let event = Event::TemplateModule(crate::Event::Created(
			1,
			2
		));
		assert_eq!(last_event(), event);


    assert_ok!(TemplateModule::ask(Origin::signed(1), 1, Some(2)));
    let event = Event::TemplateModule(crate::Event::Ask(
			1,
			1,
      Some(2)
		));
		assert_eq!(last_event(), event);

    assert_noop!(
			TemplateModule::buy(Origin::signed(2), 9, 2),
			Error::<Test>::InvalidKittyId
		);

    assert_noop!(
			TemplateModule::buy(Origin::signed(2), 2, 2),
			Error::<Test>::NotForSale
		);

    assert_noop!(
			TemplateModule::buy(Origin::signed(2), 1, 1),
			Error::<Test>::PriceTooLow
		);

    assert_ok!(TemplateModule::buy(Origin::signed(2), 1, 2));
    let event = Event::TemplateModule(crate::Event::Sold(
			1,
			2,
      1,
      2
		));
		assert_eq!(last_event(), event);

	});
}