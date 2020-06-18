extern crate suptest;
use suptest::{unit, integration};

#[integration("test")]
fn test_test(){
    assert_eq!(1, 1);
}

#[unit("test")]
fn test_two_test(){
    assert_eq!(2, 2)
}