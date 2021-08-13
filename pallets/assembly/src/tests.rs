use frame_support::assert_ok;
use pallet_documentation::Document;

use crate::mock::*;
use crate::*;
//use frame_system::ensure_signed;

#[test]
fn basic_assembly_test() {
    new_test_ext().execute_with(|| {
        DocumentationPallet::push_document(
            "pasport".as_bytes().to_vec(),
            Document {
                content: "some content".as_bytes().to_vec(),
                submited: false,
            },
        );

        assert_ok!(AssemblyPallet::submit_document(
            "pasport".as_bytes().to_vec()
        ));

        assert_eq!(
            DocumentationPallet::get_document("pasport".as_bytes().to_vec()).unwrap(),
            Document {
                content: "some content".as_bytes().to_vec(),
                submited: true
            }
        );
    });
}
