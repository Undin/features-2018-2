// If some type has implementation of `FromStr` trait
// we can use to convert string to the type.

use std::num::ParseIntError;

fn process_result(str_result: &str) -> Result<(), ParseIntError> {
    process_int_result(str_result);
    Ok(())
}



fn process_int_result(int_result: i32) {

}

