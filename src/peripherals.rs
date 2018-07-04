extern crate minifb;

use minifb::{ Window, WindowOptions, Key, KeyRepeat };

const SCALE             : usize = 10;
const CHIP_SCREEN_WIDTH : usize = 64;
const CHIP_SCREEN_HEIGHT: usize = 32;
const WIDTH             : usize = CHIP_SCREEN_WIDTH * SCALE;
const HEIGHT            : usize = CHIP_SCREEN_HEIGHT * SCALE;

pub struct Peripherals {
	key   : Option<u8>,
	window: Window,
	buffer: [u32; WIDTH * HEIGHT]
}

impl Peripherals {
	pub fn new() -> Peripherals {
		Peripherals{
			key   : None,
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

	pub fn update(&mut self) {
		self.key = match self.window.get_keys_pressed(KeyRepeat::Yes) {
			Some(keys) => if keys.is_empty() {
					None
				} else {
					match keys[0] {
						Key::Key1 => Some(0x1),
						Key::Key2 => Some(0x2),
						Key::Key3 => Some(0x3),
						Key::Key4 => Some(0xC),

						Key::Q => Some(0x4),
						Key::W => Some(0x5),
						Key::E => Some(0x6),
						Key::R => Some(0xD),

						Key::A => Some(0x7),
						Key::S => Some(0x8),
						Key::D => Some(0x9),
						Key::F => Some(0xE),

						Key::Z => Some(0xA),
						Key::X => Some(0x0),
						Key::C => Some(0xB),
						Key::V => Some(0xF),

						_ => None
					}
				},
			None => None
		};

		self.window.update_with_buffer(&self.buffer).unwrap();
	}

	#[inline]
	pub fn get_key_pressed(&self) -> Option<u8> {
		self.key
	}

	pub fn is_key_pressed(&self, key_code: u8) -> bool {
		match self.key {
			Some(self_key_code) => self_key_code == key_code,
			None => false
		}
	}
}