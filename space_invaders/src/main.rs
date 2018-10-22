//! An Asteroids-ish example game to show off ggez.
//! The idea is that this game is simple but still
//! non-trivial enough to be interesting.

extern crate ggez;
extern crate rand;

use ggez::audio;
use ggez::conf;
use ggez::event::{self, EventHandler, Keycode, Mod};
use ggez::graphics;
use ggez::graphics::{Point2, Vector2};
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;

/// *********************************************************************
/// Basic stuff, make some helpers for vector functions.
/// ggez includes the nalgebra math library to provide lots of
/// math stuff  We just add some helpers.
/// **********************************************************************

/// Create a unit vector representing the
/// given angle (in radians)
fn vec_from_angle(angle: f32) -> Vector2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vector2::new(vx, vy)
}

/// Just makes a random `Vector2` with the given max magnitude.
fn random_vec(max_magnitude: f32) -> Vector2 {
    let angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
    let mag = rand::random::<f32>() * max_magnitude;
    vec_from_angle(angle) * (mag)
}

/// *********************************************************************
/// Now we define our Actor's.
/// An Actor is anything in the game world.
/// We're not *quite* making a real entity-component system but it's
/// pretty close.  For a more complicated game you would want a
/// real ECS, but for this it's enough to say that all our game objects
/// contain pretty much the same data.
/// **********************************************************************
#[derive(Debug)]
enum GameObjType {
    Player,
    Enemy,
    Barrier,
    Shot,
}

#[derive(Debug)]
struct GameObj {
    tag: GameObjType,
    pos: Point2,
    speed: f32,
    direction: Vector2,
    size: f32,
    hit_points: i32,
}


impl GameObj {
    fn new(tag: GameObjType, pos: Point2, speed: f32, direction: Vector2, size: f32, hit_points: i32) -> Self {
        GameObj{
            tag: tag,
            pos: pos,
            speed: speed,
            direction: direction,
            size: size,
            hit_points: hit_points,
        }
    }

    fn update_position(&mut self, dt: f32){

        self.pos += self.direction * self.speed * dt;
    }

}

fn enemy_pos_calculator(enemies: &mut Vec<GameObj>, sx: f32){
    
    let screen_bounds = sx / 2.0 - GAME_BOUNDS;
    let mut reached_corner = false;

    let enemy_speed = ENEMY_SPEED/(enemies.len() as f32); 

    for mut enemy in enemies{
        enemy.speed = enemy_speed;
        if enemy.pos.x > screen_bounds{
            reached_corner = true;
        }
        else if enemy.pos.x < -screen_bounds{
            reached_corner = true;
        }
    }

    if reached_corner{
        for mut enemy in enemies{
            enemy.pos.y -= 30.0;
            enemy.direction = Vector2::new(-enemy.direction.x, 0.0);
        }
    }
}

fn player_handle_input(player: &mut GameObj, input: &InputState) {
    player.direction = Vector2::new(input.xaxis, 0.0);
}


fn create_enemies(screen_width: u32, screen_height: u32) -> Vec<GameObj> {
    let mut vec = Vec::new();

    let spacing = ((screen_width as f32) - 40.0*2.0)/(ENEMY_NCOLUMN as f32);
    let initial_x_pos = 40.0 - (screen_width as f32/2.0) + spacing/2.0;
    let mut x_pos = initial_x_pos;
    let mut y_pos = 250.0;


    for j in 0..ENEMY_NLINE {
        x_pos = initial_x_pos;
        for i in 0..ENEMY_NCOLUMN {
            let mut enemy = GameObj::new(GameObjType::Enemy, 
                Point2::new(x_pos, y_pos), 
                ENEMY_SPEED, 
                Vector2::new(1.0, 0.0),
                ENEMY_SIZE, 
                ENEMY_HP);
            vec.push(enemy);
            x_pos += spacing; 
        }
        y_pos -= spacing;
    }
    return vec;
}

fn create_barriers(screen_width: u32, screen_height: u32) -> Vec<GameObj> {
    let mut vec = Vec::new();

    let spacing = ((screen_width as f32) - 60.0*2.0)/4.0;
    let mut x_pos = 60.0 - (screen_width as f32/2.0)  + spacing/2.0;
    let y_pos = -200.0;


    for i in 0..4 {
        let mut enemy = GameObj::new(GameObjType::Barrier, 
            Point2::new(x_pos, y_pos), 
            0.0, 
            na::zero(), 
            BARRIER_SIZE, 
            BARRIER_HP);
        vec.push(enemy);
        x_pos += spacing; 
    }
 
    return vec;
}


struct Assets {
    player_image: graphics::Image,
    enemy_image: graphics::Image,
    barrier_image: graphics::Image,
    shot_image: graphics::Image,
    font: graphics::Font,
    shot_sound: audio::Source,
    hit_sound: audio::Source,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;
        let enemy_image = graphics::Image::new(ctx, "/enemy.png")?;
        let barrier_image = graphics::Image::new(ctx, "/barrier.png")?;
        let shot_image = graphics::Image::new(ctx, "/shot.png")?;
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18)?;

        let shot_sound = audio::Source::new(ctx, "/pew.ogg")?;
        let hit_sound = audio::Source::new(ctx, "/boom.ogg")?;
        Ok(Assets {
            player_image,
            enemy_image,
            barrier_image,
            shot_image,
            font,
            shot_sound,
            hit_sound,
        })
    }

    fn game_obj_image(&mut self, game_obj: &GameObj) -> &mut graphics::Image {
        match game_obj.tag {
            GameObjType::Player => &mut self.player_image,
            GameObjType::Enemy => &mut self.enemy_image,
            GameObjType::Barrier => &mut self.barrier_image,
            GameObjType::Shot => &mut self.shot_image,
        }
    }
}

#[derive(Debug)]
struct InputState {
    xaxis: f32,
    is_firing: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            xaxis: 0.0,
            is_firing: false,
        }
    }
}

const PLAYER_HP: i32 = 1;
const ENEMY_HP: i32 = 1;
const BARRIER_HP: i32 = 4;
const SHOT_HP: i32 = 1;

const PLAYER_SIZE: f32 = 12.0;
const ENEMY_SIZE: f32 = 12.0;
const BARRIER_SIZE: f32 = 12.0;
const SHOT_SIZE: f32 = 6.0;

const PLAYER_SPEED: f32 = 300.0;
const ENEMY_SPEED: f32 = 1000.0;
const SHOT_SPEED: f32 = 300.0;
const PLAYER_STARTING_POS_Y: f32 = -290.0;

const PLAYER_SHOT_TIME: f32 = 0.5;
const ENEMY_SHOT_TIME: f32 = 0.5;
const ENEMY_NLINE: i32 = 5;
const ENEMY_NCOLUMN: i32 = 11;
const GAME_BOUNDS: f32 = 30.0;

struct MainState {
    player: GameObj,
    enemies: Vec<GameObj>,
    barriers: Vec<GameObj>,
    shots_player: Vec<GameObj>,
    shots_enemy: Vec<GameObj>,
    level: i32,
    score: i32,
    assets: Assets,
    screen_width: u32,
    screen_height: u32,
    input: InputState,
    player_shot_timeout: f32,
    gui_dirty: bool,
    score_display: graphics::Text,
    level_display: graphics::Text,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.print_resource_stats();
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());

        println!("Game resource path: {:?}", ctx.filesystem);

        //print_instructions();

        let assets = Assets::new(ctx)?;
        let score_disp = graphics::Text::new(ctx, "score", &assets.font)?;
        let level_disp = graphics::Text::new(ctx, "level", &assets.font)?;

        let mut player = GameObj::new(GameObjType::Player,
            Point2::new(0.0, PLAYER_STARTING_POS_Y), 
            PLAYER_SPEED, 
            na::zero(), 
            PLAYER_SIZE, 
            PLAYER_HP);
        
        let enemies = create_enemies(ctx.conf.window_mode.width, ctx.conf.window_mode.height);
        let barriers = create_barriers(ctx.conf.window_mode.width, ctx.conf.window_mode.height);

        let s = MainState {
            player,
            enemies,
            barriers,
            shots_player: Vec::new(),
            shots_enemy: Vec::new(),
            level: 0,
            score: 0,
            assets,
            screen_width: ctx.conf.window_mode.width,
            screen_height: ctx.conf.window_mode.height,
            input: InputState::default(),
            player_shot_timeout: 0.0,
            gui_dirty: true,
            score_display: score_disp,
            level_display: level_disp,
        };

        Ok(s)
    }

    fn activate_player_shot(&mut self) {
        self.player_shot_timeout = PLAYER_SHOT_TIME;

        let player = &self.player;
        let mut shot = GameObj::new(GameObjType::Shot, 
            player.pos+Vector2::new(0.0, 20.0), 
            SHOT_SPEED, 
            Vector2::new(0.0, 1.0), 
            SHOT_SIZE, 
            SHOT_HP);

        self.shots_player.push(shot);
        let _ = self.assets.shot_sound.play();
    }

    fn remove_objects(&mut self) {

        let mut index_list = Vec::new();

        for i in 0..( self.shots_player.len() as i32){
            if self.shots_player[i as usize].hit_points == 0
            {
                index_list.push(i);
            }
        }

        for i in 0..( index_list.len() as i32){
            self.shots_player.remove(index_list.pop().unwrap() as usize);
        }

         for i in 0..(self.shots_enemy.len() as i32){
            if self.shots_enemy[i as usize].hit_points == 0
            {
                index_list.push(i);
            }
        }

        for i in 0..( index_list.len() as i32){
            self.shots_enemy.remove(index_list.pop().unwrap() as usize);
        }


        for i in 0..(self.enemies.len() as i32){
            if self.enemies[i as usize].hit_points == 0
            {
                index_list.push(i);
            }
        }

        for i in 0..( index_list.len() as i32){
            self.enemies.remove(index_list.pop().unwrap() as usize);
        }

        for i in 0..(self.barriers.len() as i32){
            if self.barriers[i as usize].hit_points == 0
            {
                index_list.push(i);
            }
        }

        for i in 0..( index_list.len() as i32){
            self.barriers.remove(index_list.pop().unwrap() as usize);
        }
    }

    fn collisions(&mut self) {
        for shot_player in &mut self.shots_player {
            

            for enemy in &mut self.enemies {
                let distance = enemy.pos - shot_player.pos;
                if distance.norm() < (shot_player.size + enemy.size) {
                    shot_player.hit_points = 0;
                    enemy.hit_points = 0;
                    self.score += 1;
                    self.gui_dirty = true;
                    let _ = self.assets.hit_sound.play();
                }
            }

            for barrier in &mut self.barriers {
                let distance = barrier.pos - shot_player.pos;
                if distance.norm() < (shot_player.size + barrier.size) {
                    shot_player.hit_points = 0;
                    barrier.hit_points -= 1;
                    let _ = self.assets.hit_sound.play();
                }
            }

            for shot_enemy in &mut self.shots_enemy{
                let distance = shot_enemy.pos - shot_player.pos;
                if distance.norm() < (shot_player.size + shot_enemy.size) {
                    shot_player.hit_points = 0;
                    shot_enemy.hit_points = 0;
                    let _ = self.assets.hit_sound.play();
                }

            } 
        }

        for shot_enemy in &mut self.shots_enemy {
            let distance = shot_enemy.pos - self.player.pos;
            if distance.norm() < (self.player.size + shot_enemy.size) {
                self.player.hit_points = 0;
                shot_enemy.hit_points = 0;
            }

            for barrier in &mut self.barriers {
                let distance = barrier.pos - shot_enemy.pos;
                if distance.norm() < (shot_enemy.size + barrier.size) {
                    shot_enemy.hit_points = 0;
                    barrier.hit_points -= 1;
                    let _ = self.assets.hit_sound.play();
                }
            }
        }


        for enemy in &mut self.enemies {
            let distance = enemy.pos - self.player.pos;
            if distance.norm() < (self.player.size + enemy.size) {
                self.player.hit_points = 0;
            }

            for barrier in &mut self.barriers {
                let distance = barrier.pos - enemy.pos;
                if distance.norm() < (enemy.size + barrier.size) {
                    barrier.hit_points = 0;
                    let _ = self.assets.hit_sound.play();
                }
            }

        }
    }

    fn check_for_level_respawn(&mut self) {
        if self.enemies.is_empty() {
            self.level += 1;
            self.gui_dirty = true;
            let new_enemies = create_enemies(self.screen_width, self.screen_height);
            let new_barriers = create_barriers(self.screen_width, self.screen_height);
            self.enemies.extend(new_enemies);
            self.barriers = new_barriers;
        }
    }

    fn update_ui(&mut self, ctx: &mut Context) {
        let score_str = format!("Score: {}", self.score);
        let level_str = format!("Level: {}", self.level);
        let score_text = graphics::Text::new(ctx, &score_str, &self.assets.font).unwrap();
        let level_text = graphics::Text::new(ctx, &level_str, &self.assets.font).unwrap();

        self.score_display = score_text;
        self.level_display = level_text;
    }
}

fn draw_game_obj(
    assets: &mut Assets,
    ctx: &mut Context,
    game_obj: &GameObj,
    world_coords: (u32, u32),
) -> GameResult<()> {
    let (screen_w, screen_h) = world_coords;
    let pos = world_to_screen_coords(screen_w, screen_h, game_obj.pos);
    let image = assets.game_obj_image(game_obj);
    let drawparams = graphics::DrawParam {
        dest: pos,
        rotation: 0.0,
        offset: graphics::Point2::new(0.5, 0.5),
        ..Default::default()
    };
    graphics::draw_ex(ctx, image, drawparams)
}


fn check_player_bounds(player: &mut GameObj, sx: f32) {

    let screen_bounds = sx / 2.0 - GAME_BOUNDS;
    
    if player.pos.x > screen_bounds {
        player.pos.x = screen_bounds;
    } else if player.pos.x < -screen_bounds {
        player.pos.x = -screen_bounds;
    }
}

fn check_shot_bounds(shot: &mut GameObj, sy: f32) {

    let screen_bounds = sy / 2.0;
    
    if shot.pos.y > screen_bounds {
        shot.hit_points = 0;
    } else if shot.pos.y < -screen_bounds {
        shot.hit_points = 0;
    }
}

fn world_to_screen_coords(screen_width: u32, screen_height: u32, point: Point2) -> Point2 {
    let width = screen_width as f32;
    let height = screen_height as f32;
    let x = point.x + width / 2.0;
    let y = height - (point.y + height / 2.0);
    Point2::new(x, y)
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 30;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            // Update the player state based on the user input.
            player_handle_input(&mut self.player, &self.input);
            self.player_shot_timeout -= seconds;
            if self.input.is_firing && self.player_shot_timeout < 0.0 {
                self.activate_player_shot();
            }

            // Update the physics for all actors.
            // First the player...
            self.player.update_position(seconds);
            check_player_bounds(&mut self.player, self.screen_width as f32);
           
            // wrap_actor_position(
            //     &mut self.player,
            //     self.screen_width as f32,
            //     self.screen_height as f32,
            // );

            // Then the shots...
            for mut shot_player in &mut self.shots_player {
                shot_player.update_position(seconds);
                check_shot_bounds(&mut shot_player, self.screen_height as f32);
                //wrap_actor_position(act, self.screen_width as f32, self.screen_height as f32);
            }


            for shot_enemy in &mut self.shots_enemy {
                shot_enemy.update_position(seconds);
                //wrap_actor_position(act, self.screen_width as f32, self.screen_height as f32);
            }

            // And finally the rocks.
            enemy_pos_calculator(&mut self.enemies, self.screen_width as f32);

            for enemy in &mut self.enemies {

                enemy.update_position(seconds);
                //wrap_actor_position(act, self.screen_width as f32, self.screen_height as f32);
            }

            // Handle the results of things moving:
            // collision detection, object death, and if
            // we have killed all the rocks in the level,
            // spawn more of them.
            self.collisions();

            self.remove_objects();

            self.check_for_level_respawn();

            // Using a gui_dirty flag here is a little
            // messy but fine here.
            if self.gui_dirty {
                self.update_ui(ctx);
                self.gui_dirty = false;
            }

            // Finally we check for our end state.
            // I want to have a nice death screen eventually,
            // but for now we just quit.
            if self.player.hit_points <= 0 {
                println!("Game over!");
                let _ = ctx.quit();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Our drawing is quite simple.
        // Just clear the screen...
        graphics::clear(ctx);

        // Loop over all objects drawing them...
        {
            let assets = &mut self.assets;
            let coords = (self.screen_width, self.screen_height);

            let p = &self.player;
            draw_game_obj(assets, ctx, p, coords)?;

            for s in &self.shots_player {
                draw_game_obj(assets, ctx, s, coords)?;
            }

            for s in &self.shots_enemy {
                draw_game_obj(assets, ctx, s, coords)?;
            }

            for b in &self.barriers {
                draw_game_obj(assets, ctx, b, coords)?;
            }

            for e in &self.enemies {
                draw_game_obj(assets, ctx, e, coords)?;
            }
        }

        // And draw the GUI elements in the right places.
        let level_dest = graphics::Point2::new(10.0, 10.0);
        let score_dest = graphics::Point2::new(200.0, 10.0);
        graphics::draw(ctx, &self.level_display, level_dest, 0.0)?;
        graphics::draw(ctx, &self.score_display, score_dest, 0.0)?;

        // Then we flip the screen...
        graphics::present(ctx);

        // And yield the timeslice
        // This tells the OS that we're done using the CPU but it should
        // get back to this program as soon as it can.
        // This ideally prevents the game from using 100% CPU all the time
        // even if vsync is off.
        // The actual behavior can be a little platform-specific.
        timer::yield_now();
        Ok(())
    }

    // Handle key events.  These just map keyboard events
    // and alter our input state appropriately.
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left => {
                self.input.xaxis = -1.0;
            }
            Keycode::Right => {
                self.input.xaxis = 1.0;
            }
            Keycode::Space => {
                self.input.is_firing = true;
            }
            Keycode::Escape => ctx.quit().unwrap(),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left | Keycode::Right => {
                self.input.xaxis = 0.0;
            }
            Keycode::Space => {
                self.input.is_firing = false;
            }
            _ => (), // Do nothing
        }
    }
}



fn main(){

    let mut cb = ContextBuilder::new("space_invaders", "ggez")
        .window_setup(conf::WindowSetup::default().title("Best Space Invaders Ever"))
        .window_mode(conf::WindowMode::default().dimensions(480, 640));

    // We add the CARGO_MANIFEST_DIR/resources to the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        // We need this re-assignment alas, see
        // https://aturon.github.io/ownership/builders.html
        // under "Consuming builders"
        cb = cb.add_resource_path(path);
    } else {
        println!("Not building from cargo?  Ok.");
    }

    let ctx = &mut cb.build().unwrap();

    match MainState::new(ctx) {
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
        Ok(ref mut game) => {
            let result = event::run(ctx, game);
            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            } else {
                println!("Game exited cleanly.");
            }
        }
    }

}