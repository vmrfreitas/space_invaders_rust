// Space Invaders desenvolvido em rust, utilizando o tutorial
// da biblioteca ggez como base.

extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::{ContextBuilder};
use ggez::event;
use std::env;
use std::path;


mod asse; // carrega os modulos com as funções e classes
mod ms;
mod go;

fn main(){

    let mut cb = ContextBuilder::new("space_invaders", "ggez")
        .window_setup(conf::WindowSetup::default().title("Best Space Invaders Ever"))
        .window_mode(conf::WindowMode::default().dimensions(480, 640));

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
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