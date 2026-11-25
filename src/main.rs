use macroquad::{input::KeyCode::Back, prelude::*};
struct Controller {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    speed: f32,
}

struct Wall {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    speed: f32,
}

impl Wall {
    fn mov(&mut self, dt: f32){
        self.y += self.speed * dt;
    }
    fn draw(&self, texture: &Texture2D ){
        draw_texture(&texture,
             self.x,
             self.y,
             WHITE
            );
        
    }
}

fn generator(vec: &mut Vec<Wall>, timer: &mut f32, dt: f32){

     let spawn_delay = 1.5;
     let dice = macroquad::rand::gen_range(1, 13);
     let x = match dice {
        1 => 16.0,
        2 => 80.0,
        3 => 144.0,
        4 => 208.0,
        5 => 272.0,
        6 => 336.0,
        7 => 400.0,
        8 => 464.0,
        9 => 528.0,
        10 => 592.0,
        11 => 656.0,
        12 => 720.0,
    _ => unreachable!()};

    *timer += dt;
   
    if *timer >= spawn_delay { // не мое
        *timer = 0.0; // не мое
        vec.push(Wall { x: x, y: -50.0, w: 64.0, h: 150.0, speed: 400.0});

}   }   

fn colission(player: &Controller, wall: &Wall) -> bool { // не мое
    player.x < wall.x + wall.w &&
    player.x + player.w > wall.x &&
    player.y < wall.y + wall.h &&
    player.y + player.h > wall.y
} 

#[macroquad::main("Crystal collector")]
async fn main() {
    let crystal: Texture2D = load_texture("assets/crystal.png").await.unwrap();
    let basket: Texture2D = load_texture("assets/basket.png").await.unwrap();
    let background: Texture2D = load_texture("assets/background.png").await.unwrap();

    let mut count = 0;
    let mut spawn_timer = 0.0;
    let mut player = Controller {
        x: 0.0,
        y: 0.0,
        w: 64.0,
        h: 64.0,
        speed: 400.0,
    };
    let mut walls: Vec<Wall> = Vec::new();
    
    loop {
        let dt = get_frame_time();
        for wall in &mut walls {
            wall.mov(dt);
            if colission(&player, wall){
                count = count + 1;
            }
            
        }
        
        generator(&mut walls, &mut spawn_timer, dt);
        walls.retain(|wall| !colission(&player, wall)); // не мое
        walls.retain(|wall| !(wall.y + wall.h> 750.0));  // взял код выше и переделал так что мое на половину

        if is_key_down(KeyCode::D) {
            player.x += player.speed * dt;
        }
        if is_key_down(KeyCode::A) {
            player.x -= player.speed * dt;
        }
        if is_key_down(KeyCode::S) {
            player.y += player.speed * dt;
        }
        if is_key_down(KeyCode::W) {
            player.y -= player.speed * dt;
        }
          
        
        clear_background(BLACK);

        draw_texture(&background,
            0.0,
            0.0,
            WHITE
        );
        for draw in &walls{
                draw.draw(&crystal);
            }

        draw_texture(&basket,
            player.x,
            player.y,
            WHITE
        );
        

        
       draw_text(format!("Score: {}", count), 0.0, 590.0, 32.0, WHITE);
       next_frame().await;

    }
}
