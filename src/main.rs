extern crate minifb;
extern crate rand;

use std::fs::File;
use std::io::Read;

use chip8::Chip8;

mod memory;
mod internals;
mod peripherals;
mod chip8;

fn main() -> std::io::Result<()> {
	let mut rom  = File::open("games/UFO")?;
	let mut code = Vec::<u8>::new();

	rom.read_to_end(&mut code)?;

	let mut chip8 = Chip8::new();
	chip8.load_rom(&code);
	chip8.run_game();

	Ok(())
}
