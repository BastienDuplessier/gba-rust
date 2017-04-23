#![feature(lang_items)]

#![no_std]
#![no_main]

extern crate gba;

use gba::gfx;

mod lang;

pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    gfx::Mode3::activate();
    let colors = [gfx::Color::RED,
                  gfx::Color::BLUE,
                  gfx::Color::YELLOW,
                  gfx::Color::LIME,
                  gfx::Color::MAG,
                  gfx::Color::WHITE,
                  gfx::Color::CYAN,
                  gfx::Color::BLACK];

    for x in 0..239 {
        let color = colors[x / 10 % 8];
        for y in 0..159 {
            gfx::Screen::fill_dot(x as i32, y as i32, color);
        }
    }

    loop{}
}
