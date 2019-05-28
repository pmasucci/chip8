extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use crate::cpu::Cpu;
use std::{thread, time};
const WIDTH: usize = 1280;
const HEIGHT: usize = 640;
const PIXEL_WIDTH: usize = 20;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub fn emulate(rom: &str) {
  let mut cpu = Cpu::new();
  println!("chippy {}", rom);
  cpu.reset();
  cpu.load(rom);

  let opengl = OpenGL::V3_3;
  let mut window: Window = WindowSettings::new("CHIP-8", [WIDTH as u32, HEIGHT as u32])
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();
  let mut gl = GlGraphics::new(opengl);
  let mut events = Events::new(EventSettings::new());

  while let Some(e) = events.next(&mut window) {
    if let Some(r) = e.render_args() {
      gl.draw(r.viewport(), |c, gl| {
        use graphics::*;
        clear(BLACK, gl);
        for (i, &pixel) in cpu.display.memory.iter().enumerate() {
          if pixel == 1 {
            let transform = c.transform.trans(
              ((i * PIXEL_WIDTH) % WIDTH) as f64,
              ((i / 64) * PIXEL_WIDTH) as f64,
            );
            let rect = rectangle::square(0.0, 0.0, PIXEL_WIDTH as f64);
            rectangle(RED, rect, transform, gl);
          }
        }
      })
    }

    if let Some(u) = e.update_args() {
      cpu.execute_cycle();
    }
  }
}
