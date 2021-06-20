use crate::mock::*;
use crate::*;
use frame_support::{assert_err, assert_ok};
use frame_system::ensure_signed;

#[test]
fn basic_identity_test() {
    new_test_ext().execute_with(|| {});
}
