extern crate rand;
use crate::display::{Display, FONT_SET};
use rand::Rng;

pub struct Cpu {
  ram: [u8; 4096],
  display: Display,
  v: [u8; 16],
  dt: u8,
  st: u8,
  pc: u16,
  sp: u8,
  stack: [u16; 16],
  i: u16,
}

fn read_word(memory: [u8; 4096], index: u16) -> u16 {
  (memory[index as usize] as u16) << 8 | (memory[(index + 1) as usize] as u16)
  }

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      i: 0,
      pc: 0,
      ram: [0; 4096],
      v: [0; 16],
      display: Display::new(),
      stack: [0; 16],
      sp: 0,
      dt: 0,
      st: 0,
    }
  }

  pub fn reset(&mut self) {
    self.i = 0;
    self.pc = 0x200;
    self.ram = [0; 4096];
    self.v = [0; 16];
    self.stack = [0; 16];
    self.sp = 0;
    self.dt = 0;
    self.display.cls();
    for i in 0..80 {
      self.ram[i] = FONT_SET[i];
    }
  }

  pub fn execute_cycle(&mut self) {
    let opcode: u16 = read_word(self.ram, self.pc);
    self.process_opcode(opcode);
  }

  fn generate_random() -> u8 {
    rand::random::<u8>()
  }

  fn process_opcode(&mut self, opcode: u16) {
    let first_nibble = opcode >> 12;
    let nnn = opcode & 0x0fff;
    let x = ((opcode & 0x0f00) >> 8) as usize;
    let y = ((opcode & 0x00f0) >> 4) as usize;
    let vx = self.v[x];
    let vy = self.v[y];
    let kk = (opcode & 0x00ff) as u8;
    let n = (opcode & 0x000f) as u8;
    self.pc += 2;

    match first_nibble {
    0x0 => match opcode & 0x00ff {
      0xE0 => println!("UNIMPLEMENTED CLS"),
      0xEE => {
        self.pc = self.stack[self.sp as usize]; 
        self.sp -= 1;
      },
      _ => println!("UNIMPLEMENTED"),
    },
    0x1 => self.pc = nnn,
    0x2 => {
      self.sp += 1; 
      self.stack[self.sp as usize] = self.pc;
      self.pc = nnn;
    },
    0x3 => {
      if self.v[x] == kk{
        self.pc += 2;
      }
    },
    0x4 => {
      if !self.v[x] == kk {
        self.pc += 2;
      }
    },
    0x5 => {
      if self.v[x] == self.v[y] {
        self.pc += 2;
      }
    },
    0x6 => {
      self.v[x] = kk;
    },
    0x7 => {
      self.v[x] += kk;
    },
    0x8 => match opcode & 0x000f {
      0x0 => {
        self.v[x] = self.v[y];
      }
      0x1 => {
        self.v[x] = self.v[x] | self.v[y];
      },
      0x2 => {
        self.v[x] = self.v[x] & self.v[y];
      },
      0x3 => {
        self.v[x] = self.v[x] ^ self.v[y];
      },
      0x4 => {
        let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
        match overflow {
          true => self.v[0xF] = 1,
          false => self.v[0xF] = 0,
        }
        self.v[x] = res;
      },
      0x5 => {
        let (res, overflow) = self.v[x].overflowing_sub(self.v[y]);
        match overflow {
          true => self.v[0xF] = 0,
          false => self.v[0xF] = 1,
        }
        self.v[x] = res;
      },
      0x6 => {
        let last_bit = self.v[x] & 0b00000001;
        self.v[0xF] = last_bit;
        self.v[x] >>= 1;
      },
      0x7 => {
        let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);
        match overflow {
          true => self.v[0xF] = 0,
          false => self.v[0xF] = 1,
        }
        self.v[x] = res;
      },
      0xE => {
        let first_bit = self.v[x] & 0b10000000;
        self.v[0xF] = first_bit;
        self.v[x] <<= 1;
      },
      _ => println!("UNIMPLEMENTED"),
    },
    0x9 => {
      if self.v[x] != self.v[y] {
        self.pc += 2;
      }
    },
    0xA => {
      self.i = nnn;
    },
    0xB => {
      self.pc = nnn + self.v[0] as u16;
    },
    0xC => {
      self.v[x] = Cpu::generate_random() & kk;
    }
    0xD => {
      let collision = self.display.draw(vx as usize, vy as usize, &self.ram[self.i as usize .. (self.i + n as u16) as usize]);
      self.v[0xF] = if collision { 1 } else { 0 };
    },
    0xE => match opcode & 0x00ff {
      0x9E => {
        if let self.keyboard.keys[self.v[x]] {
          self.pc += 2;
        }
      },
      0xA1 => {
        if let !self.keyboard.keys[self.v[x]] {
          self.pc += 2;
        }
      },
      _ => println!("UNIMPLEMENTED"),
    },
    0xF => match opcode & 0x00ff {
      0x07 => {
        self.v[x] = self.dt;
      },
      0x0A => {
        self.pc -= 2;
        for(i, key) in self.keyboard.keys.iter().enumerate() {
          if *key == true {
            self.v[x] = i;
            self.pc += 2;
          }
        }
      },
      0x15 => {
        self.dt = vx;
      },
      0x18 => {
        self.st = vx;
      },
      0x1E => {
        self.i += vx as u16;
      },
      0x29 => {
        self.i = vx as u16 * 5;
      },
      0x33 => {
        let hundreds = vx/100;
        let tens = (vx/10) % 10;
        let ones = vx % 10;
        self.ram[self.i as usize] = hundreds;
        self.ram[self.i as usize + 1] = tens;
        self.ram[self.i as usize + 2] = ones;
      },
      0x55 => {
        for (i, register) in self.v[0 .. (x as usize)] {
          self.ram[self.i + i] = register;
        }
      },
      0x65 => {
        for (i, register) in self.v[0 .. x] {
          register = self.ram[self.i + i];
        }
      },
      _ => println!("UNIMPLEMENTED"),
    },
    _ => "instruction not recognized".to_string(),
  };
  }
}
