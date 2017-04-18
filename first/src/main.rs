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
    for x in 0..240 {
        m.dot(x, 80, Color::rgb15(31, 0, 0));
        m.dot(x, 20, Color::rgb15(31, 0, 0));
        m.dot(x, 83, Color::rgb15(0, 31, 0));
        m.dot(x, 96, Color::rgb15(0, 0, 31));

    }

    loop{}
}
