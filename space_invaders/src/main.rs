// Space Invaders desenvolvido em rust, utilizando o tutorial
// da biblioteca ggez como base.

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
use rand::Rng;

use std::env;
use std::path;


const PLAYER_HP: i32 = 3;
const ENEMY_HP: i32 = 1;
const BARRIER_HP: i32 = 4;
const SHOT_HP: i32 = 1;

const PLAYER_SIZE: f32 = 12.0;
const ENEMY_SIZE: f32 = 6.0;
const BARRIER_SIZE: f32 = 12.0;
const SHOT_SIZE: f32 = 6.0;

const PLAYER_SPEED: f32 = 300.0;
const ENEMY_SPEED: f32 = 600.0;
const SHOT_SPEED: f32 = 300.0;
const PLAYER_STARTING_POS_Y: f32 = -290.0;

const PLAYER_SHOT_TIME: f32 = 0.5;
const ENEMY_SHOT_TIME: f32 = 1.0;
const ENEMY_NLINE: i32 = 5;
const ENEMY_NCOLUMN: i32 = 11;
const GAME_BOUNDS: f32 = 30.0;
const MAX_DIFF_LEVEL: i32 = 7;


// Valores possíveis assumidos pela struct "GameObj"

#[derive(Debug)]
enum GameObjType {
    Player,
    Enemy,
    Barrier,
    Shot,
}

// Struct de um objeto de jogo genérico, funciona como uma classe

#[derive(Debug)]
struct GameObj {
    tag: GameObjType,
    curr_sprite: i32,
    pos: Point2,
    speed: f32,
    direction: Vector2,
    size: f32,
    hit_points: i32,
}

// Implementação dos "métodos" da "classe" GameObj

impl GameObj {
    fn new(tag: GameObjType, pos: Point2, speed: f32, direction: Vector2, size: f32, hit_points: i32) -> Self {
        GameObj{
            tag: tag,               // Funciona como um ID
            curr_sprite: 0,         // Sprite atual do objeto
            pos: pos,               // Posição do objeto na tela
            speed: speed,           // Velocidade do objeto
            direction: direction,   // Direção em que o objeto está se movimentando
            size: size,             // Tamanho da hitbox do objeto
            hit_points: hit_points, // HP do objeto
        }
    }

    fn update_position(&mut self, time_var: f32){ // Atualiza a posição do objeto de acordo com a velocidade e direção

        self.pos += self.direction * self.speed * time_var;
    }

}

fn enemy_pos_calculator(enemy: &mut GameObj, // Atualiza a posição de um inimigo
    player: &mut GameObj, 
    reached_corner: bool, 
    screen_bounds_x: f32, 
    screen_bounds_y: f32) -> bool{

    let mut reached_corner_1 = reached_corner; // Se um inimigo encostar no limite horizontal da tela

    if enemy.pos.x > screen_bounds_x{ 
        reached_corner_1 = true;
    }
    else if enemy.pos.x < -screen_bounds_x{
        reached_corner_1 = true;  
    }

    if enemy.pos.y < -screen_bounds_y{ // Se um inimigo sair da tela por baixo, o jogo acaba
        player.hit_points = 0;
    }

    return reached_corner_1;
}

fn create_enemies(screen_width: u32, diff_level: i32) -> Vec<GameObj> { // Cria os inimigos nas suas posições corretas
    let mut vec = Vec::new();

    let spacing = ((screen_width as f32) - 40.0*2.0)/(ENEMY_NCOLUMN as f32); // Espaço entre inimigos
    let initial_x_pos = 40.0 - (screen_width as f32/2.0) + spacing/2.0; 
    let mut x_pos;
    let mut y_pos = 250.0 - (diff_level as f32) * 35.0;
    let mut enemy_type = 1;


    for _j in 0..ENEMY_NLINE {
        x_pos = initial_x_pos;
        for _i in 0..ENEMY_NCOLUMN {
            let mut enemy = GameObj::new(GameObjType::Enemy, 
                Point2::new(x_pos, y_pos), 
                ENEMY_SPEED, 
                Vector2::new(1.0, 0.0),
                ENEMY_SIZE, 
                ENEMY_HP);
            enemy.curr_sprite = enemy_type; // Define qual sprite de inimigo utilizar
            vec.push(enemy);
            x_pos += spacing; 
        }
        if enemy_type == 1{ // Cada coluna de inimigo possui uma sprite diferente
            enemy_type = 2;
        }
        else {
            enemy_type = 1;
        }
        y_pos -= spacing;
    }
    return vec;
}

fn create_barriers(screen_width: u32) -> Vec<GameObj> { // Cria as barreiras nas suas posições corretas
    let mut vec = Vec::new();

    let spacing = ((screen_width as f32) - 60.0*2.0)/4.0;
    let mut x_pos = 60.0 - (screen_width as f32/2.0)  + spacing/2.0;
    let y_pos = -200.0; // Posição da barreira é hardcoded 


    for _i in 0..4 {
        let mut enemy = GameObj::new(GameObjType::Barrier, 
            Point2::new(x_pos, y_pos), 
            0.0,        // Barreira não precisa de velocidade
            na::zero(), // Barreira não precisa de direção
            BARRIER_SIZE, 
            BARRIER_HP);
        vec.push(enemy);
        x_pos += spacing; 
    }
 
    return vec;
}


struct Assets { // Classe que possui todos os assets necessários para o jogo
    player_image: graphics::Image,
    enemy_image_1: graphics::Image,
    enemy_image_2: graphics::Image,
    enemy_image_3: graphics::Image,
    enemy_image_4: graphics::Image,
    barrier_image_1: graphics::Image,
    barrier_image_2: graphics::Image,
    barrier_image_3: graphics::Image,
    barrier_image_4: graphics::Image,
    shot_image: graphics::Image,
    font: graphics::Font,
    shot_sound: audio::Source,
    player_hit_sound: audio::Source,
    enemy_hit_sound: audio::Source,
}

impl Assets { // Implementação dos métodos da classe de assets
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;
        let enemy_image_1 = graphics::Image::new(ctx, "/enemy1.png")?;
        let enemy_image_2 = graphics::Image::new(ctx, "/enemy2.png")?;
        let enemy_image_3 = graphics::Image::new(ctx, "/enemy3.png")?;
        let enemy_image_4 = graphics::Image::new(ctx, "/enemy4.png")?;
        let barrier_image_1 = graphics::Image::new(ctx, "/barrier1.png")?;
        let barrier_image_2 = graphics::Image::new(ctx, "/barrier2.png")?;
        let barrier_image_3 = graphics::Image::new(ctx, "/barrier3.png")?;
        let barrier_image_4 = graphics::Image::new(ctx, "/barrier4.png")?;
        let shot_image = graphics::Image::new(ctx, "/shot.png")?;
        let font = graphics::Font::new(ctx, "/slkscr.ttf", 12)?;

        let shot_sound = audio::Source::new(ctx, "/shoot.ogg")?;
        let player_hit_sound = audio::Source::new(ctx, "/explosion.ogg")?;
        let enemy_hit_sound = audio::Source::new(ctx, "/invaderkilled.ogg")?;
        Ok(Assets {
            player_image,
            enemy_image_1,
            enemy_image_2,
            enemy_image_3,
            enemy_image_4,
            barrier_image_1,
            barrier_image_2,
            barrier_image_3,
            barrier_image_4,
            shot_image,
            font,
            shot_sound,
            player_hit_sound,
            enemy_hit_sound,
        })
    }

    fn game_obj_sprite(&mut self, game_obj: &GameObj) -> &mut graphics::Image { // Seleciona a sprite correta para o obj
        match game_obj.tag {
            GameObjType::Player => &mut self.player_image,
            GameObjType::Enemy => {
                match game_obj.curr_sprite {
                    1 => &mut self.enemy_image_2,
                    -1 => &mut self.enemy_image_1,
                    2 => &mut self.enemy_image_4,
                    -2 => &mut self.enemy_image_3,
                    _ => &mut self.enemy_image_2 // O rust exige essa opção "_"
                }
            },
            GameObjType::Barrier => {
                match game_obj.curr_sprite {
                    0 => &mut self.barrier_image_1,
                    1 => &mut self.barrier_image_2,
                    2 => &mut self.barrier_image_3,
                    3 => &mut self.barrier_image_4,
                    _ => &mut self.barrier_image_1 // O rust exige essa opção "_"
                }
            },
            GameObjType::Shot => &mut self.shot_image,
        }
    }
}

#[derive(Debug)]
struct InputState { // Classe que contem o estado dos inputs do usuário
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

struct MainState { // Classe do estado atual do jogo
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
    player_shot_timeout: f32, // Tempo de espera entre tiros do player
    enemy_shot_timeout: f32,  // Tempo de espera entre tiros dos inimigos
    enemy_sprite_timer: f32,
    gui_dirty: bool,          // Flag de atualização da GUI
    score_display: graphics::Text,
    level_display: graphics::Text,
    hp_display: graphics::Text,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());

        println!();
        println!("Bem vindo ao melhor Space Invaders da existência");
        println!();
        println!("Como jogar:");
        println!("Setinhas para esquerda e direita, espaço para atirar, esc para sair, boa sorte");
        println!();

        let assets = Assets::new(ctx)?;
        let score_disp = graphics::Text::new(ctx, "score", &assets.font)?;
        let level_disp = graphics::Text::new(ctx, "level", &assets.font)?;
        let hp_disp = graphics::Text::new(ctx, "hp", &assets.font)?;

        // Criação dos objetos do jogo

        let player = GameObj::new(GameObjType::Player,
            Point2::new(0.0, PLAYER_STARTING_POS_Y), 
            PLAYER_SPEED, 
            na::zero(), 
            PLAYER_SIZE, 
            PLAYER_HP);

        let enemies = create_enemies(ctx.conf.window_mode.width, 0);
        let barriers = create_barriers(ctx.conf.window_mode.width);

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
            enemy_shot_timeout: 0.0,
            enemy_sprite_timer: 0.0,
            gui_dirty: true,
            score_display: score_disp,
            level_display: level_disp,
            hp_display: hp_disp,
        };

        Ok(s)
    }

    fn activate_enemy_shot(&mut self) { // Função que dispara um tiro do inimigo
        self.enemy_shot_timeout = ENEMY_SHOT_TIME;

        // Escolhe um inimigo aleatório
        let enemy_shooter = (rand::thread_rng().gen_range(0, self.enemies.len() as i32)) as usize;

        let shot = GameObj::new(GameObjType::Shot, 
            self.enemies[enemy_shooter].pos+Vector2::new(0.0, -20.0), 
            SHOT_SPEED, 
            Vector2::new(0.0, -1.0), 
            SHOT_SIZE, 
            SHOT_HP);

        self.shots_enemy.push(shot);
    }

    fn activate_player_shot(&mut self) { // Função que dispara um tiro do player
        self.player_shot_timeout = PLAYER_SHOT_TIME;

        let player = &self.player;
        let shot = GameObj::new(GameObjType::Shot, 
            player.pos+Vector2::new(0.0, 20.0), 
            SHOT_SPEED, 
            Vector2::new(0.0, 1.0), 
            SHOT_SIZE, 
            SHOT_HP);

        self.shots_player.push(shot);
        let _ = self.assets.shot_sound.play();
    }

    fn remove_objects(&mut self) { // Remove do jogo os objetos que estão com o HP zerado

        let mut index_list = Vec::new(); // Lista de elementos a serem removidos

        for i in 0..( self.shots_player.len() as i32){
            if self.shots_player[i as usize].hit_points == 0
            {
                index_list.push(i);
            }
        }

        for _i in 0..( index_list.len() as i32){ // Remove um elemento por vez de acordo com a lista
            self.shots_player.remove(index_list.pop().unwrap() as usize); 
        }

        for i in 0..(self.shots_enemy.len() as i32){
            if self.shots_enemy[i as usize].hit_points == 0
            {
                index_list.push(i);
            }
        }

        for _i in 0..( index_list.len() as i32){
            self.shots_enemy.remove(index_list.pop().unwrap() as usize);
        }


        for i in 0..(self.enemies.len() as i32){
            if self.enemies[i as usize].hit_points == 0
            {
                index_list.push(i);
            }
        }

        for _i in 0..( index_list.len() as i32){
            self.enemies.remove(index_list.pop().unwrap() as usize);
        }

        for i in 0..(self.barriers.len() as i32){
            if self.barriers[i as usize].hit_points == 0
            {
                index_list.push(i);
            }
        }

        for _i in 0..( index_list.len() as i32){
            self.barriers.remove(index_list.pop().unwrap() as usize);
        }
    }

    fn collisions(&mut self) { // Função que gerencia as colisões
        
        for shot_player in &mut self.shots_player {
            for enemy in &mut self.enemies {
                let distance = enemy.pos - shot_player.pos; // Tiro do player com inimigo
                if distance.norm() < (shot_player.size + enemy.size) {
                    shot_player.hit_points = 0;
                    enemy.hit_points = 0;
                    self.score += 1;
                    self.gui_dirty = true;
                    let _ = self.assets.enemy_hit_sound.play();
                }
            }

            for barrier in &mut self.barriers {
                let distance = barrier.pos - shot_player.pos; // Tiro do player com a barreira
                if distance.norm() < (shot_player.size + barrier.size) {
                    shot_player.hit_points = 0;
                    barrier.hit_points -= 1; 
                    barrier.curr_sprite += 1; // Atualiza o sprite da barreira para um mais "destruído"
                }
            }

            for shot_enemy in &mut self.shots_enemy{
                let distance = shot_enemy.pos - shot_player.pos; // Tiro do player com tiro do inimigo
                if distance.norm() < (shot_player.size + shot_enemy.size) {
                    shot_player.hit_points = 0;
                    shot_enemy.hit_points = 0;
                }

            } 
        }

        for shot_enemy in &mut self.shots_enemy {
            let distance = shot_enemy.pos - self.player.pos; // Tiro do inimigo com o player
            if distance.norm() < (self.player.size + shot_enemy.size) {
                self.player.hit_points -= 1;
                self.gui_dirty = true;
                shot_enemy.hit_points = 0;
                let _ = self.assets.player_hit_sound.play();
            }

            for barrier in &mut self.barriers {
                let distance = barrier.pos - shot_enemy.pos; // Tiro do inimigo com a barreira
                if distance.norm() < (shot_enemy.size + barrier.size) {
                    shot_enemy.hit_points = 0;
                    barrier.hit_points -= 1;
                    barrier.curr_sprite += 1;
                }
            }
        }


        for enemy in &mut self.enemies {
            let distance = enemy.pos - self.player.pos; // Inimigo com o player
            if distance.norm() < (self.player.size + enemy.size) {
                self.player.hit_points = 0;
            }

            for barrier in &mut self.barriers {
                let distance = barrier.pos - enemy.pos; // Inimigo com a barreira
                if distance.norm() < (enemy.size + barrier.size) {
                    barrier.hit_points = 0; // Simplesmente destrói a barreira
                }
            }
        }
    }

    fn check_for_level_respawn(&mut self) { // Recarrega os inimigos e as barreiras caso o level passe
        if self.enemies.is_empty() {
            self.level += 1;
            let mut diff_level = self.level;
            self.gui_dirty = true;
            if self.level > MAX_DIFF_LEVEL
            {
                diff_level = MAX_DIFF_LEVEL;  
            }
            let new_enemies = create_enemies(self.screen_width, diff_level);
            let new_barriers = create_barriers(self.screen_width);
            self.enemies.extend(new_enemies);
            self.barriers = new_barriers;
        }
    }

    fn update_ui(&mut self, ctx: &mut Context) { // Faz o update da user interface
        let score_str = format!("Score: {}", self.score);
        let level_str = format!("Level: {}", self.level);
        let hp_str = format!("Lives: {}", self.player.hit_points);
        let score_text = graphics::Text::new(ctx, &score_str, &self.assets.font).unwrap();
        let level_text = graphics::Text::new(ctx, &level_str, &self.assets.font).unwrap();
        let hp_text = graphics::Text::new(ctx, &hp_str, &self.assets.font).unwrap();

        self.score_display = score_text;
        self.level_display = level_text;
        self.hp_display = hp_text;
    }
}

fn draw_game_obj( // Desenha os sprites dos objetos
    assets: &mut Assets,
    ctx: &mut Context,
    game_obj: &GameObj,
    world_coords: (u32, u32),
) -> GameResult<()> {

    let (screen_w, screen_h) = world_coords;
    let pos = world_to_screen_coords(screen_w, screen_h, game_obj.pos);
    let image = assets.game_obj_sprite(game_obj);
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

fn check_shot_bounds(shot: &mut GameObj, sy: f32) { // Não deixa o jogador sair da tela

    let screen_bounds = sy / 2.0;
    
    if shot.pos.y > screen_bounds {
        shot.hit_points = 0;
    } else if shot.pos.y < -screen_bounds {
        shot.hit_points = 0;
    }
}


// Passa as coordenadas do "mundo" para a tela
fn world_to_screen_coords(screen_width: u32, screen_height: u32, point: Point2) -> Point2 {
    let width = screen_width as f32;
    let height = screen_height as f32;
    let x = point.x + width / 2.0;
    let y = height - (point.y + height / 2.0);
    Point2::new(x, y)
}

impl EventHandler for MainState { // Loop principal do jogo, onde tudo é atualizado
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 30;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            self.enemy_sprite_timer -= seconds;
            self.player.direction = Vector2::new(self.input.xaxis, 0.0);
            self.player_shot_timeout -= seconds;
            if self.input.is_firing && self.player_shot_timeout < 0.0 {
                self.activate_player_shot();
            }
            self.enemy_shot_timeout -= seconds;
            if self.enemy_shot_timeout < 0.0 {
                self.activate_enemy_shot();
            }

            self.player.update_position(seconds);
            check_player_bounds(&mut self.player, self.screen_width as f32);
           
            for mut shot_player in &mut self.shots_player {
                shot_player.update_position(seconds);
                check_shot_bounds(&mut shot_player, self.screen_height as f32);
            }


            for mut shot_enemy in &mut self.shots_enemy {
                shot_enemy.update_position(seconds);
                check_shot_bounds(&mut shot_enemy, self.screen_height as f32);
            }

            let screen_bounds_x = (self.screen_width as f32) / 2.0 - GAME_BOUNDS;
            let screen_bounds_y = (self.screen_height as f32) / 2.0 - GAME_BOUNDS;
            let mut reached_corner = false;

            // Velocidade dos inimigos aumenta inversamente proporcional ao número de inimigos
            let enemy_speed = ENEMY_SPEED/(self.enemies.len() as f32);

            for mut enemy in &mut self.enemies{

                if self.enemy_sprite_timer < 0.0 {
                    enemy.curr_sprite *= -1;
                }

                enemy.speed = enemy_speed;
                reached_corner = enemy_pos_calculator(enemy, 
                                                    &mut self.player, 
                                                    reached_corner, 
                                                    screen_bounds_x, 
                                                    screen_bounds_y);
            }

            if self.enemy_sprite_timer < 0.0 { // Troca os sprites mais rapido quando os inimigos são mais rapidos
                self.enemy_sprite_timer = 10.0/enemy_speed;        
            }

            if reached_corner{ // Move os inimigos pra baixo e troca sua direção
                for mut enemy in &mut self.enemies{
                    enemy.pos.y -= 15.0;
                    enemy.direction = Vector2::new(-enemy.direction.x, 0.0);
                }
            }

            for enemy in &mut self.enemies {
                enemy.update_position(seconds);
            }

            self.collisions();
            self.remove_objects();
            self.check_for_level_respawn();
            if self.gui_dirty {
                self.update_ui(ctx);
                self.gui_dirty = false;
            }

            if self.player.hit_points <= 0 {
                println!("Você perdeu, que pena. Pontuação: {}", self.score);
                let _ = ctx.quit();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> { // Função que desenha na tela tudo
        graphics::clear(ctx);
        
        { // Desenha os objetos
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

        // Desenha a user interface

        let level_dest = graphics::Point2::new(10.0, 10.0);
        let score_dest = graphics::Point2::new(180.0, 10.0);
        let hp_dest = graphics::Point2::new(360.0, 10.0);
        graphics::draw(ctx, &self.level_display, level_dest, 0.0)?;
        graphics::draw(ctx, &self.score_display, score_dest, 0.0)?;
        graphics::draw(ctx, &self.hp_display, hp_dest, 0.0)?;

        graphics::present(ctx);

        timer::yield_now();
        Ok(())
    }

    // Mapeia as teclas apertadas para a classe input
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
            _ => (),
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
            _ => (),
        }
    }
}



fn main(){

    let mut cb = ContextBuilder::new("space_invaders", "ggez")
        .window_setup(conf::WindowSetup::default().title("Best Space Invaders Ever"))
        .window_mode(conf::WindowMode::default().dimensions(480, 640));

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        // We need this re-assignment alas, see
        // https://aturon.github.io/ownership/builders.html
        // under "Consuming builders"
        cb = cb.add_resource_path(path);
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
                println!("Erro durante execução: {}", e);
            }
        }
    }

}