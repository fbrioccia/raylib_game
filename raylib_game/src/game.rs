use crate::objects;
use crate::objects::*;
use raylib::prelude::*;

pub struct Game {
    pub objects: Vec<Box<dyn objects::Object>>,
    //thread:  &'a RaylibThread,
    //raylib_handle: &'a RaylibHandle
}

impl Game {
    pub fn new() -> Self {
        //handle:&'a RaylibHandle, thread:&'a RaylibThread
        let mut game = Game {
            objects: vec![],
            //thread: thread,
            //raylib_handle: handle
        };

        let player = player::Player::new( Vector2 { x: 50f32, y: 50f32 }, Color::RED, 100.);
        //let player2 = player::Player::new(0, Vector2{x:100f32,y:50f32}, Color::GREEN,100.);
        game.objects.push(Box::new(player));
        return game;
    }

    pub fn spawn_alien(&mut self)
    {
        let alien = alien::Alien::new(
            Vector2::new(100.,100.),
            color::Color::ORANGE,
            50.
         );

         self.objects.push(Box::new(alien));
    }
}
