use raylib::prelude::*;
pub mod alien;
pub mod player;

pub trait Object {
    fn update(&mut self, raylib_handle: &RaylibHandle);
    fn draw(&self, draw_handle: &mut RaylibDrawHandle);
}
