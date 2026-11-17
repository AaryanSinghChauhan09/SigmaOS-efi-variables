// Integration test for SigmaOS-efi-variables
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use efi_variables as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
