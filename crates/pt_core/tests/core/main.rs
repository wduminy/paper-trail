use rstest::*;

use pt_core::*;

#[rstest]
fn it_works() {
    let mut pdf = ProjectService::default();
    assert!(pdf.add_pdf("../../samples/Pulicken-2026.pdf").is_ok());
}
