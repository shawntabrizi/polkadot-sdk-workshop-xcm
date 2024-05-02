use crate::fundamentals::instruction::*;
use xcm::latest::prelude::*;

#[test]
fn test_clear_origin_message() {
    let message = clear_origin_message();

    assert_eq!(message, Xcm(vec![ClearOrigin,]));
}
