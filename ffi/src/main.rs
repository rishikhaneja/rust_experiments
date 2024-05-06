extern crate libc;
use libc::c_int;

#[link(name = "mylibrary", kind = "static")]
extern {
    fn add_numbers(a: c_int, b: c_int) -> c_int;
}

fn main() {
    let a = 5;
    let b = 10;

    let result = unsafe { add_numbers(a, b) };

    println!("The result is: {}", result);
}
