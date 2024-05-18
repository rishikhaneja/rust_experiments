// extern crate libc;
// use libc::c_int;

// #[link(name = "mylibrary", kind = "static")]
// extern {
//     fn add_numbers(a: c_int, b: c_int) -> c_int;
// }

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!("../gen/bindings.rs"));

fn main() {
    let a = 5;
    let b = 10;

    let result = unsafe { mylibrary_add_numbers(a, b) };

    println!("cxx result: {}", result);
}
