extern crate ggez;
extern crate rand;

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

use ggez::graphics::{Point2, Vector2};
use ggez::nalgebra as na;

impl Movement for GameObj {
    fn update_position(&mut self, time_var: f32){ // Atualiza a posição do objeto de acordo com a velocidade e direção
        self.pos += self.direction * self.speed * time_var;
    }   

    fn get_pos(&self) -> Point2{
        return self.pos;
    }

    fn get_pos_x(&self) -> f32{
        return self.pos.x;
    }

    fn get_pos_y(&self) -> f32{
        return self.pos.y;
    }

    fn set_pos(&mut self, pos: Point2){
        self.pos = pos;
    }

    fn set_pos_x(&mut self, pos: f32){
        
        self.pos.x = pos;
    }

    fn set_pos_y(&mut self, pos: f32){
        self.pos.y = pos;
    }

    fn get_speed(&self) -> f32{
        return self.speed;
    }

    fn set_speed(&mut self, speed: f32){
        self.speed = speed;
    }

    fn get_direction(&self) ->  Vector2{
        return self.direction;
    }

    fn get_direction_x(&self) -> f32{
        return self.direction.x;
    }

    fn get_direction_y(&self) -> f32{
        return self.direction.y;
    }

    fn set_direction(&mut self, dir: Vector2){
        self.direction = dir;
    }

    fn set_direction_x(&mut self, dir: f32){
        self.direction.x = dir;
    }

    fn set_direction_y(&mut self, dir: f32){
        self.direction.y = dir;
    }
}


pub trait Movement {
    fn update_position(&mut self, time_var: f32);
    fn get_pos(&self) -> Point2;
    fn get_pos_x(&self) -> f32;
    fn get_pos_y(&self) -> f32;
    fn set_pos(&mut self, pos: Point2);
    fn set_pos_x(&mut self, pos: f32);
    fn set_pos_y(&mut self, pos: f32);
    fn get_speed(&self) -> f32;
    fn set_speed(&mut self, speed: f32);
    fn get_direction(&self) ->  Vector2;
    fn get_direction_x(&self) -> f32;
    fn get_direction_y(&self) -> f32;
    fn set_direction(&mut self, dir: Vector2);
    fn set_direction_x(&mut self, dir: f32);
    fn set_direction_y(&mut self, dir: f32);
}


// Valores possíveis assumidos pela struct "GameObj"

#[derive(Debug)]
pub enum GameObjType {
    Player,
    Enemy,
    Barrier,
    Shot,
}

// Struct de um objeto de jogo genérico, funciona como uma classe

#[derive(Debug)]
pub struct GameObj {
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
    pub fn new(tag: GameObjType, pos: Point2, speed: f32, direction: Vector2, size: f32, hit_points: i32) -> Self {
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

    pub fn new_player() -> Self { // construtor alternativo 
        
        return GameObj::new(GameObjType::Player, 
            Point2::new(0.0, PLAYER_STARTING_POS_Y), 
            PLAYER_SPEED, 
            na::zero(),
            PLAYER_SIZE,
            PLAYER_HP);
    }
    pub fn new_shot(pos: Point2, direction: Vector2) -> Self { // construtor alternativo 
        
        return GameObj::new(GameObjType::Shot, 
            pos, 
            SHOT_SPEED, 
            direction,
            SHOT_SIZE,
            SHOT_HP);
    }

     pub fn new_enemy(pos: Point2) -> Self { // construtor alternativo 
        
        return GameObj::new(GameObjType::Enemy, 
            pos,
            ENEMY_SPEED, 
            Vector2::new(1.0, 0.0),
            ENEMY_SIZE,
            ENEMY_HP);
    }

    pub fn new_barrier(pos: Point2) -> Self { // construtor alternativo 
        
        return GameObj::new(GameObjType::Barrier,
            pos,
            0.0, 
            na::zero(),
            BARRIER_SIZE,
            BARRIER_HP);
    }


    pub fn get_tag(&self) -> &GameObjType {
        return &self.tag;
    }
    pub fn get_curr_sprite(&self) -> i32{
        return self.curr_sprite;
    }

    pub fn set_curr_sprite(&mut self, curr_sprite: i32){
        if curr_sprite < -2 {
            self.curr_sprite = -2;
        }
        else if curr_sprite>3 {
            self.curr_sprite = 3;
        }
        else{
            self.curr_sprite = curr_sprite;
        }
    }

    pub fn get_size(&self) -> f32{
        return self.size;
    }

    pub fn get_hit_points(&self) -> i32{
        return self.hit_points;
    }

    pub fn set_hit_points(&mut self, hp: i32){
        if hp < 0 {
            self.hit_points = 0;
        }
        else if hp > 3 {
            self.hit_points = 3;
        }
        else{
            self.hit_points = hp;
        }
    }


    pub fn sub_hit_points(&mut self){
        self.hit_points -= 1;
    }


}
