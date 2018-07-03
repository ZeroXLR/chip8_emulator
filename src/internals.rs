use memory::Memory;
use peripherals::Peripherals;

use std::fmt;

use rand;
use rand::distributions::Distribution;
use rand::distributions::uniform::Uniform;

pub const PROGRAM_START: usize = 0x200;

struct RandomByteGenerator {
	rng    : rand::ThreadRng,
	uniform: Uniform<u8>
}

impl RandomByteGenerator {
	fn new() -> RandomByteGenerator {
		RandomByteGenerator {
			rng    : rand::thread_rng(),
			uniform: Uniform::new_inclusive(0, 255)
		}
	}

	fn random_byte(&mut self) -> u8 {
		self.uniform.sample(&mut self.rng)
	}
}

pub struct Internals {
	pub ram: Memory,           // RAM Part

	v    : [u8; 16],           //
	pc   : usize,              //
	i    : usize,              // CPU Part
	stack: Vec<usize>,         //
	rbg  : RandomByteGenerator //
}

impl Internals {
	pub fn new() -> Internals {
		Internals {
			ram  : Memory::new(),

			v    : [0; 16],
			pc   : PROGRAM_START,
			i    : 0,
			stack: Vec::new(),
			rbg  : RandomByteGenerator::new()
		}
	}

	pub fn run_next(&mut self, peripherals: &mut Peripherals) {
		let (hi, lo) = (self.ram[self.pc], self.ram[self.pc+1]);

		let top = hi & 0xF0;
		let x   = (hi & 0x0F) as usize;
		let nnn = ((x as usize) << 8) | lo as usize;
		let nn  = lo;
		let n   = lo & 0x0F;
		let y   = ((lo & 0xF0) >> 4) as usize;

		match top {
			0x00 => match nn {
				0xE0 => {
					peripherals.clear_display();
					self.pc += 2;
				}
				0xEE => self.pc = self.stack.pop().expect("No subroutine to return!"),
				_    => panic!("Illegal instruction at {:#X}", self.pc)
			}
			0x10 => self.pc = nnn,
			0x20 => {
				self.stack.push(self.pc+2);
				self.pc = nnn;
			}
			0x30 => self.pc += if self.v[x] == nn { 4 } else { 2 },
			0x40 => self.pc += if self.v[x] != nn { 4 } else { 2 },
			0x50 => self.pc += if self.v[x] == self.v[y] { 4 } else { 2 },
			0x60 => {
				self.v[x] = nn;
				self.pc += 2;
			}
			0x70 => {
				self.v[x] = self.v[x].wrapping_add(nn);
				self.pc += 2;
			}
			0x80 => {
				let vx = self.v[x];
				match n {
					0x0 => self.v[x] = self.v[y],
					0x1 => self.v[x] = vx | self.v[y],
					0x2 => self.v[x] = vx & self.v[y],
					0x3 => self.v[x] = vx ^ self.v[y],
					0x4 => {
						let (sum, overflowed) = vx.overflowing_add(self.v[y]);
						self.v[x] = sum;
						self.v[15] = if overflowed { 1 } else { 0 };
					}
					0x5 => {
						let (difference, overflowed) = vx.overflowing_sub(self.v[y]);
						self.v[x] = difference;
						self.v[15] = if overflowed { 0 } else { 1 };
					}
					0x6 => {
						self.v[15] = vx & 0b0000_0001;
						self.v[x] = vx >> 1;
					}
					0x7 => {
						let (difference, overflowed) = self.v[y].overflowing_sub(vx);
						self.v[x] = difference;
						self.v[0xF] = if overflowed { 0 } else { 1 };
					}
					0xE => {
						self.v[0xF] = (vx & 0b1000_0000) >> 7;
						self.v[x] = vx << 1;
					}
					_   => panic!("Illegal instruction at {:#X}", self.pc)
				}
				self.pc += 2;
			}
			0x90 => self.pc += if self.v[x] != self.v[y] { 4 } else { 2 },
			0xA0 => {
				self.i = nnn;
				self.pc += 2;
			}
			0xB0 => self.pc = nnn + self.v[0] as usize,
			0xC0 => {
				self.v[x] = self.rbg.random_byte() & nn;
				self.pc += 2;
			}
			0xD0 => {
				let (vx, vy) = (self.v[x], self.v[y]);
				self.v[0xF] = peripherals.display_sprite(
					&self.ram[self.i .. self.i + n as usize], vx, vy
				);
				self.pc += 2;
			}
			0xE0 => match nn {
				0x9E => self.pc += if peripherals.is_key_pressed(self.v[x]) { 4 } else { 2 },
				0xA1 => self.pc += if peripherals.is_key_pressed(self.v[x]) { 2 } else { 4 },
				_    => panic!("Illegal instruction at {:#X}", self.pc)
			}
			0xF0 => {
				match nn {
					0x07 => {
					}
					0x0A => {
					}
					0x15 => {
					}
					0x18 => {
					}
					0x1E => self.i += self.v[x] as usize,
					0x29 => self.i = self.v[x] as usize * 5,
					0x33 => {
						let (vx, i) = (self.v[x], self.i);
						let h = vx / 100;
						let u = vx % 10;
						let d = (vx % 100) - u;
						self.ram[i] = h;
						self.ram[i + 1] = d;
						self.ram[i + 2] = u;
					}
					0x55 => {
						let upto = x + 1;
						self.ram.write_data_at(self.i, &self.v[ .. upto]);
						self.i += upto;
					}
					0x65 => {
						let (prev_i, upto) = (self.i, x + 1);
						self.i += upto;
						self.v[ .. upto].copy_from_slice(&self.ram[prev_i .. self.i]);
					}
					_ => panic!("Illegal instruction at {:#X}", self.pc)
				}
				self.pc += 2;
			}
			_ => panic!("Illegal instruction at {:#X}", self.pc)
		}
	}
}

impl fmt::Debug for Internals {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"GPRs: {}\nPC: {:#X}\nRI: {:#X}\nStack: {}\n",
			self.v.iter().map( |vx| format!("{:#X} ", vx) ).collect::<String>(),
			self.pc,
			self.i,
			self.stack.iter().rev().map( |address| format!("{:#X} ", address) ).collect::<String>()
		)
	}
}