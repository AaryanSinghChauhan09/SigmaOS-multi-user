// Integration test for SigmaOS-multi-user
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use multi_user as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
