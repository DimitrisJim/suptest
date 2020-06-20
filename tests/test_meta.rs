extern crate suptest;
use suptest::{unit, integration};

#[unit(okay)]
fn test_test(){
    assert_eq!(1, 1);
}


#[integration("", "")]
fn test_two_test(){
    assert_eq!(2, 2)
}
