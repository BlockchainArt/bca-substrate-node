use super::*;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok, traits::tokens::nonfungibles::Inspect};

#[test]
fn setup() {
    new_test_ext().execute_with(|| {
        assert_eq!(1, NextCollectionId::<Test>::get());
    });
}

#[test]
fn minting() {
    new_test_ext().execute_with(|| {
        // create class
        assert_ok!(Bca::create_collection(
            Origin::signed(1),
            Default::default(),
            Edition {
                proofs: 1,
                prints: 3
            },
            Default::default(),
            0,
            Default::default(),
        ));
        assert_eq!(2, NextCollectionId::<Test>::get());
        assert_eq!(1, Collections::<Test>::get::<Vec<u8>>(Default::default()));
        assert_eq!(
            CollectionDetails {
                edition: Edition {
                    proofs: 1,
                    prints: 3
                },
                next_proof: 1,
                next_print: 1,
                next_id: 1
            },
            Uniques::typed_class_attribute(&1, b"bca").unwrap()
        );

        // basic mint
        assert_ok!(Bca::create_print(Origin::signed(1), 1, false, 1));
        assert_eq!(1, Uniques::owner(1, 1).unwrap());
        assert_eq!(
            CollectionDetails {
                edition: Edition {
                    proofs: 1,
                    prints: 3
                },
                next_proof: 1,
                next_print: 2,
                next_id: 2
            },
            Uniques::typed_class_attribute(&1, b"bca").unwrap()
        );
        assert_eq!(None, Uniques::attribute(&1, &2, b"proof"));

        // proof mint
        assert_ok!(Bca::create_print(Origin::signed(1), 1, true, 1));
        assert_eq!(
            CollectionDetails {
                edition: Edition {
                    proofs: 1,
                    prints: 3
                },
                next_proof: 2,
                next_print: 2,
                next_id: 3
            },
            Uniques::typed_class_attribute(&1, b"bca").unwrap()
        );
        assert_eq!(
            true,
            Uniques::typed_attribute::<_, bool>(&1, &2, b"proof").unwrap()
        );

        //transfer mint
        assert_ok!(Bca::create_print(Origin::signed(1), 1, false, 2));
        assert_eq!(
            CollectionDetails {
                edition: Edition {
                    proofs: 1,
                    prints: 3
                },
                next_proof: 2,
                next_print: 3,
                next_id: 4
            },
            Uniques::typed_class_attribute(&1, b"bca").unwrap()
        );
        assert_eq!(2, Uniques::owner(1, 3).unwrap());
    });
}

#[test]
fn transfer() {
    new_test_ext().execute_with(|| {
        assert_ok!(Bca::create_collection(
            Origin::signed(1),
            Default::default(),
            Edition {
                proofs: 1,
                prints: 3
            },
            Default::default(),
            0,
            Default::default(),
        ));
        assert_ok!(Bca::create_print(Origin::signed(1), 1, false, 1));
        assert_ok!(Bca::transfer_print(Origin::signed(1), 1, 1, 2));
        assert_eq!(2, Uniques::owner(1, 1).unwrap());
    });
}

#[test]
fn collection_unavailable() {
    new_test_ext().execute_with(|| {
        assert_ok!(Bca::create_collection(
            Origin::signed(1),
            Default::default(),
            Edition {
                proofs: 1,
                prints: 3
            },
            Default::default(),
            0,
            Default::default(),
        ));
        assert_noop!(
            Bca::create_collection(
                Origin::signed(1),
                Default::default(),
                Edition {
                    proofs: 1,
                    prints: 3
                },
                Default::default(),
                0,
                Default::default(),
            ),
            Error::<Test>::CollectionUnavailable
        );
    });
}

#[test]
fn collection_not_found() {
    new_test_ext().execute_with(|| {
        assert_ok!(Bca::create_collection(
            Origin::signed(1),
            Default::default(),
            Edition {
                proofs: 1,
                prints: 3
            },
            Default::default(),
            0,
            Default::default(),
        ));
        // collection doesn't exist
        assert_noop!(
            Bca::create_print(Origin::signed(1), 2, false, 1),
            Error::<Test>::CollectionNotFound
        );
        // not collection owner
        assert_noop!(
            Bca::create_print(Origin::signed(2), 1, false, 2),
            Error::<Test>::CollectionNotFound
        );
    });
}

#[test]
fn print_unavailable() {
    new_test_ext().execute_with(|| {
        assert_ok!(Bca::create_collection(
            Origin::signed(1),
            Default::default(),
            Edition {
                proofs: 0,
                prints: 0
            },
            Default::default(),
            0,
            Default::default(),
        ));
        assert_noop!(
            Bca::create_print(Origin::signed(1), 1, false, 1),
            Error::<Test>::PrintUnavailable
        );
        assert_noop!(
            Bca::create_print(Origin::signed(1), 1, true, 1),
            Error::<Test>::PrintUnavailable
        );
    });
}

#[test]
fn print_not_found() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Bca::transfer_print(Origin::signed(1), 1, 1, 2),
            Error::<Test>::PrintNotFound
        );
    });
}
