extern crate suptest;
use suptest::{unit, integration};

// okay, passing okay should be okay, okay?
// attributes are (for now) ignored.
#[unit(okay)]
fn test_test(){
    assert_eq!(1, 1);
}

// Simple application, fine.
#[unit]
fn test_another(){
    assert_eq!("What is", "What is");
}

// Apply ignore.
#[unit]
#[ignore]
fn ignored_sorry(){
    assert_eq!([1, 2, 3], [1, 2, 3]);
}

// reversed:
#[ignore]
#[unit]
fn ignored_sorry_again(){
    assert_eq!([1, 2, 3], [1, 2, 3]);
}

// Apply panic.
#[unit]
#[should_panic]
fn panic_please(){
    panic!("SEVERE PANICKING");
}

// reversed:
#[should_panic]
#[unit]
fn panic_please_again(){
    panic!("SEVERE PANICKING");
}

// Apply both
#[unit]
#[should_panic]
#[ignore]
fn panic_but_ill_ignore_you(){
    panic!("UNNOTICED PANICKING");
}

// reversed: one
#[should_panic]
#[ignore]
#[unit]
fn panic_but_ill_ignore_you_again(){
    panic!("UNNOTICED PANICKING");
}

// reversed: two
#[ignore]
#[should_panic]
#[unit]
fn panic_but_ill_ignore_you_again_again(){
    panic!("UNNOTICED PANICKING");
}

// -----------------
// -- integration --
// -----------------

// okay, passing okay should be okay, okay?
// attributes are (for now) ignored.
#[integration(okay)]
fn i_have_gotten_bored_of_writing_names(){
    assert_eq!(1, 1);
}

// Simple application, fine.
#[integration]
fn this_is_why_im_essentially_forming_sentences_here(){
    assert_eq!("What is", "What is");
}

// Apply ignore.
#[integration]
#[ignore]
fn maybe_i_should_generate_these(){
    assert_eq!([1, 2, 3], [1, 2, 3]);
}

// reversed:
#[ignore]
#[integration]
fn shouldnt_i(){
    assert_eq!([1, 2, 3], [1, 2, 3]);
}

// Apply panic.
#[integration]
#[should_panic]
fn yeah_that_would_probably_be_smarter(){
    panic!("SEVERE PANICKING");
}

// reversed:
#[should_panic]
#[integration]
fn sometimes_you_gotta_do_what_you_gotta_do(){
    panic!("SEVERE PANICKING");
}

// Apply both
#[integration]
#[should_panic]
#[ignore]
fn anyhow(){
    panic!("UNNOTICED PANICKING");
}

// reversed: one
#[should_panic]
#[ignore]
#[integration]
fn what_else(){
    panic!("UNNOTICED PANICKING");
}

// reversed: two
#[ignore]
#[should_panic]
#[integration]
fn whats_the_weather_like(){
    panic!("UNNOTICED PANICKING");
}

// ---- mod application -----

#[unit]
mod tests{
    fn test_mod_hi(){
        assert_eq!("to be", "to be");
    }

    // Apply ignore.
    #[ignore]
    fn ignored_sorry(){
        assert_eq!([1, 2, 3], [1, 2, 3]);
    }

    // Apply panic.
    #[should_panic]
    fn panic_please(){
        panic!("SEVERE PANICKING");
    }

    // Apply both
    #[should_panic]
    #[ignore]
    fn panic_but_ill_ignore_you(){
        panic!("UNNOTICED PANICKING");
    }
}

#[integration]
mod more_tests{
    fn test_mod_hi(){
        assert_eq!("to be", "to be");
    }

    // Apply ignore.
    #[ignore]
    fn ignored_sorry(){
        assert_eq!([1, 2, 3], [1, 2, 3]);
    }

    // Apply panic.
    #[should_panic]
    fn panic_please(){
        panic!("SEVERE PANICKING");
    }

    // Apply both
    #[should_panic]
    #[ignore]
    fn panic_but_ill_ignore_you(){
        panic!("UNNOTICED PANICKING");
    }
}