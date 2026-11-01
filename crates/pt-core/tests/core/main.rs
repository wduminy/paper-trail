use rstest;

use pt_core::*;

#[test]
fn it_works() {
    let text = extract("../../samples/Pulicken-2026.pdf").unwrap();
    assert!(!text.is_empty());
}
