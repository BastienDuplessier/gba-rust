#![feature(lang_items)]

#![no_std]
#![no_main]

extern crate gba;

mod lang;

pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    loop{}
}
