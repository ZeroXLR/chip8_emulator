use internals::{ PROGRAM_START, Internals };
use peripherals::Peripherals;

pub struct Chip8 {
	peripherals: Peripherals,
	internals  : Internals
}

impl Chip8 {
	pub fn new() -> Chip8 {
		Chip8 {
			peripherals: Peripherals::new(),
			internals: Internals::new()
		}
	}

	#[inline]
	pub fn load_rom(&mut self, rom_code: &[u8]) {
		self.internals.ram.write_data_at(PROGRAM_START, rom_code);
	}

	pub fn run_game(&mut self) {
		while self.peripherals.display_is_open() {
			self.internals.run_next(&mut self.peripherals);
			self.peripherals.display_update();
		}
	}
}