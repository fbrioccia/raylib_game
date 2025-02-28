use raylib::prelude::*;
pub struct Player
{
    id: i32,
    pub position: Vector2
}

impl Player{
    pub fn new(id:i32, position: Vector2)-> Self{
        Self {id, position}
    }
}
