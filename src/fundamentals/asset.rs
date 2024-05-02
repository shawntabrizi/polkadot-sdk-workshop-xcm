// Fundamentals Lesson 2

use xcm::latest::prelude::*;

#[allow(dead_code)]
pub fn empty_assets() -> Assets {
    let assets = vec![];
    Assets::from(assets)
}
