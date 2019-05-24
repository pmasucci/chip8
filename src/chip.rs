use crate::cpu::Cpu;
use std::{thread, time};

pub fn emulate(rom: &str) {
  let mut cpu = Cpu::new();
  println!("chippy {}", rom);
  cpu.reset();
  cpu.load(rom);
  loop {
    cpu.execute_cycle();
    thread::sleep(time::Duration::from_millis(16));
  }
}
