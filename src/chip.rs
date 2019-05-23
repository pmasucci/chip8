use crate::cpu::Cpu;

pub fn emulate(rom: &str) {
  let cpu = Cpu::new();
  println!("chippy {}", rom);
}
