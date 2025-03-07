use raylib::prelude::*;
mod game;
mod objects;

fn main() {
    
    
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
    let mut game = game::Game::new();
    game.spawn_alien();
     
    while !rl.window_should_close() {
        
        for obejct in &mut game.objects{
            obejct.update(&rl);
        }
        let mut d = rl.begin_drawing(&thread);
        
        for obejct in &game.objects{
            obejct.draw(&mut d);
        }
        


        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}


