use raylib::prelude::*;

pub struct Player
{
    pub position: Vector2,
    pub color: core::color::Color,
    speed: f32
}

impl Player{
    pub fn new(position: Vector2, color: core::color::Color, speed:f32)-> Self{
        Self {position, color,speed}
    }
}

impl super::Object for Player{
    fn update(&mut self, handle:&RaylibHandle){
        let mut direction:Vector2 = Vector2::zero();
        if handle.is_key_down(KeyboardKey::KEY_LEFT){
            direction = direction+Vector2::new(-1f32,0f32);
        }
        if handle.is_key_down(KeyboardKey::KEY_RIGHT){
            direction = direction+Vector2::new(1f32,0f32);
        }
        if handle.is_key_down(KeyboardKey::KEY_UP){
            direction = direction+Vector2::new(0f32,-1f32);
        }
        if handle.is_key_down(KeyboardKey::KEY_DOWN){
            direction = direction+Vector2::new(0f32,1f32);
        }

        let delta = handle.get_frame_time();
        self.position = self.position+direction*self.speed*delta;
        
    }

    fn draw(&self, draw_handle:&mut RaylibDrawHandle)
    {
        draw_handle.draw_circle_v(self.position, 15f32, self.color);
    }
}
