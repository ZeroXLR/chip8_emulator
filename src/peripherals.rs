extern crate minifb;

use minifb::{ Window, WindowOptions, Key };

const SCALE             : usize = 10;
const CHIP_SCREEN_WIDTH : usize = 64;
const CHIP_SCREEN_HEIGHT: usize = 32;
const WIDTH             : usize = CHIP_SCREEN_WIDTH * SCALE;
const HEIGHT            : usize = CHIP_SCREEN_HEIGHT * SCALE;

pub struct Peripherals {
	window: Window,
	buffer: [u32; WIDTH * HEIGHT]
}

impl Peripherals {
	pub fn new() -> Peripherals {
		Peripherals{
			window: Window::new("Rusty Emulator",
			                    WIDTH, HEIGHT,
			                    WindowOptions::default()).unwrap(),
			buffer: [0; WIDTH * HEIGHT]
		}
	}

	#[inline]
	pub fn display_is_open(&self) -> bool {
		self.window.is_open() && !self.window.is_key_down(Key::Escape)
	}

	pub fn clear_display(&mut self) {
		self.buffer.iter_mut().for_each(|pixel| *pixel = 0);
	}

	pub fn display_sprite(&mut self, sprite: &[u8], x: u8, y: u8) -> u8 {
		let (x, y) = (x as usize, y as usize);
		let mut pixel_unset = 0;
		for (j, &slice) in sprite.iter().enumerate() {
			let _y = (y + j) % CHIP_SCREEN_HEIGHT; // adjusted y coordinate for chip screen
			let __y = _y * SCALE; // initial y coordinate for actual window

			let mut shifty_slice = slice; // each byte encodes a horizontal slice of the sprite
			for i in 0 .. 8 {
				let _x = (x + i) % CHIP_SCREEN_WIDTH; // adjusted x coordinate for chip screen
				let __x = _x * SCALE; // initial x coordinate for actual window

				// Magnify every pixel on the chip screen to a SCALE x SCALE block of duplicates:
				for n in 0 .. SCALE {
					let initial = __x + (__y + n) * WIDTH;
					self.buffer[initial .. initial + SCALE]
							.iter_mut()
							.for_each(|pixel| *pixel ^= if shifty_slice & 0b1000_0000 == 0 {
								// or-assign to preserve previous value under multiple assigns
								pixel_unset |= (*pixel & 1) as u8;
								0x0
							} else {
								0xFFFF // turquoise hue
							});
				}

				shifty_slice <<= 1; // promote the next bit to the left for analysis
			}
		}
		pixel_unset
	}

	#[inline]
	pub fn display_update(&mut self) {
		self.window.update_with_buffer(&self.buffer).unwrap();
	}

	// TO IMPLEMENT
	pub fn is_key_pressed(&self, _key: u8) -> bool {
		true
	}
}