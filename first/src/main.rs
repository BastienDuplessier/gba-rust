#![feature(lang_items)]

#![no_std]
#![no_main]

extern crate gba;

mod lang;

pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};

use gba::gfx;
use gba::gfx::Color;

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    let mut m = gfx::Mode3::new();
    m.dot(120, 80, Color::rgb15(31, 0, 0));
    m.dot(136, 80, Color::rgb15(0, 31, 0));
    m.dot(120, 96, Color::rgb15(0, 0, 31));

    loop{}
}
