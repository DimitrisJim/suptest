extern crate suptest;
use suptest::{unit, integration};

// actually no issue with this, apparently.
#[test]
#[test]
#[integration]
#[unit]
fn test_test(){
    assert_eq!(1, 1);
}