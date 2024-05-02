// Fundamentals Lesson 3

use xcm::latest::prelude::*;

#[allow(dead_code)]
pub fn clear_origin_message() -> Xcm<()> {
    let message = Xcm(vec![ClearOrigin]);

    message
}
