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


// aqui tava os gameobjects

mod asse;
mod ms;
mod go;


// aqui tavam os creates

// AQUI TAVA OS ASSETS




// aqui tava o mainstate




// aqui tava o eventhandler


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

    match ms::MainState::new(ctx) {
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