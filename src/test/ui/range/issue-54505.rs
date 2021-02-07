// run-rustfix

// Regression test for #54505 - range borrowing suggestion had
// incorrect syntax (missing parentheses).

use std::ops::RangeBounds;


// take a reference to any built-in range
fn take_range(_r: &impl RangeBounds<i8>) {}


fn main() {
    take_range(0..1);
    //~^ ERROR arguments to this function are incorrect [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(0..1)

    take_range(1..);
    //~^ ERROR arguments to this function are incorrect [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(1..)

    take_range(..);
    //~^ ERROR arguments to this function are incorrect [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(..)

    take_range(0..=1);
    //~^ ERROR arguments to this function are incorrect [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(0..=1)

    take_range(..5);
    //~^ ERROR arguments to this function are incorrect [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(..5)

    take_range(..=42);
    //~^ ERROR arguments to this function are incorrect [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(..=42)
}
