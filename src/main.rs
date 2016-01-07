#[macro_use]
extern crate hlua;
extern crate glium;

mod scripts;

fn main() {
 use glium::{DisplayBuild, Surface};
 scripts::hello_world();
 let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
 loop{
  let mut target = display.draw();
  target.clear_color(0.0, 0.0, 1.0, 1.0);
  target.finish().unwrap();

  for ev in display.poll_events(){
   match ev{
    glium::glutin::Event::Closed => return,
    _ => ()
   }
  }
 }
}
