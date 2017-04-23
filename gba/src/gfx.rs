use core::slice;
use core::ptr::{write_volatile};

pub const MODE3: u32 = 0x3;
pub const BG2: u32 = 0x400;
pub const PTR_MODE: u32 = 0x4000000;
pub const PTR_SCREEN: u32 = 0x06000000;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub struct Color (pub u16);

impl Color {
    /// Creates a color with 16 bits, 5 bits for each channel.
    pub const fn rgb15(red: u32, green: u32, blue: u32) -> Color {
        Color((red | (green << 5) | (blue << 10)) as u16)
    }

    pub const BLACK:  Color = Color(0x0000);
    pub const RED:    Color = Color(0x001F);
    pub const LIME:   Color = Color(0x03E0);
    pub const YELLOW: Color = Color(0x03FF);
    pub const BLUE:   Color = Color(0x7C00);
    pub const MAG:    Color = Color(0x7C1F);
    pub const CYAN:   Color = Color(0x7FE0);
    pub const WHITE:  Color = Color(0x7FFF);
}

pub struct Mode3;

impl Mode3 {
    /// Calling this invalidates all other modes and enters Mode3.
    pub fn activate () -> Self {
        unsafe {
            write_volatile(PTR_MODE as *mut u32, MODE3 | BG2);
        }
        Mode3
    }
}

pub struct Screen;

impl Screen {
    pub const WIDTH : usize = 240;
    pub const HEIGHT : usize = 160;
    pub const SIZE : usize = Self::WIDTH * Self::HEIGHT;

    pub fn draw_dot(x: i32, y: i32, color: Color) {
        let buff : &mut [u16] = unsafe {
            slice::from_raw_parts_mut(PTR_SCREEN as *mut u16, Self::SIZE)
        };

        buff[(x+y*Self::WIDTH as i32) as usize] = color.0;
    }


}
