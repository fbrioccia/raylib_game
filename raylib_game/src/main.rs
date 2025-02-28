use raylib::prelude::*;
mod player;


fn main() {
    let mut player = player::Player::new(0, Vector2{x:50f32,y:50f32});
    let mut player2 = player::Player::new(0, Vector2{x:500f32,y:50f32});

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
     
    while !rl.window_should_close() {
        udpate(&mut player, Vector2::one());
        udpate(&mut player2, Vector2{x:-1f32,y:1f32});

        let mut d = rl.begin_drawing(&thread);
        d.draw_circle_v(player.position, 15f32,Color::RED);
        d.draw_circle_v(player2.position, 15f32,Color::GREEN);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}

fn udpate(player: &mut player::Player, direction: Vector2){
    if player.position.x < 640f32 && player.position.y < 480f32 {
        player.position = Vector2{
            x:player.position.x+direction.x,
            y:player.position.y+direction.y};
    }
}
