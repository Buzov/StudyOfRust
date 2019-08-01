mod structure;
mod shapes;
mod types;

use crate::structure::test_struct_tuple;
use crate::structure::zero_sized_types;
use crate::shapes::point::{test_point, test_partial_eq};
use crate::types::*;

//fn main {}  ==  fn main() -> ()
fn main() {

    test_sleep();
    number_type();
    test_fn_tuple();
    tuple();
    loops();
    links();

    test_life_time_in_fn();
    test_closures();

    test_point();
    test_partial_eq();

    test_struct_tuple();
//    println!("tuple_fnin: {}", tuple_fn(tuple_struct));
    zero_sized_types();

}

