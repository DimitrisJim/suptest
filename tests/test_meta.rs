extern crate suptest;
use suptest::{unit, integration};

#[unit(okay)]
fn test_test(){
    assert_eq!(1, 1);
}

#[unit]
fn test_another(){
    assert_eq!("What is", "What is");
}

#[unit]
fn test_another_again(){
    assert_eq!([1, 2, 3], [1, 2, 3]);
}

#[integration("", "")]
fn test_two_test(){
    assert_eq!(2, 2)
}

#[integration("", "")]
fn test_another_test(){
    assert_eq!(vec![2], vec![5-3]);
}


#[unit]
mod tests{
    fn test_mod_hi(){
        assert_eq!("to be", "to be");
    }
}