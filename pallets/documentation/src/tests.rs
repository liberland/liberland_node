use crate::mock::*;
use crate::*;
//use frame_system::ensure_signed;

#[test]
fn basic_documentation_test() {
    new_test_ext().execute_with(|| {
        // push document
        DocumentationPallet::push_document(
            "pasport".as_bytes().to_vec(),
            Document {
                content: "some content".as_bytes().to_vec(),
                submited: false,
            },
        );
        assert_eq!(
            Document {
                content: "some content".as_bytes().to_vec(),
                submited: false,
            },
            DocumentationPallet::get_document("pasport".as_bytes().to_vec()).unwrap()
        );

        // update document
        DocumentationPallet::update_document(
            "pasport".as_bytes().to_vec(),
            Document {
                content: "some content is updated".as_bytes().to_vec(),
                submited: false,
            },
        );

        assert_eq!(
            Document {
                content: "some content is updated".as_bytes().to_vec(),
                submited: false,
            },
            DocumentationPallet::get_document("pasport".as_bytes().to_vec()).unwrap()
        );

        // update non-existent document
        DocumentationPallet::update_document(
            "visa".as_bytes().to_vec(),
            Document {
                content: "some content is updated".as_bytes().to_vec(),
                submited: false,
            },
        );
        assert_eq!(
            None,
            DocumentationPallet::get_document("visa".as_bytes().to_vec())
        );
    });
}
