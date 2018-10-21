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

const T_PLAYER: i32 = 0;
const T_ENEMY: i32 = 1;
const T_BARRIER: i32 = 2;
const T_SHOT: i32 = 3;

const PLAYER_MAX_LIFE: i32 = 1;
const PLAYER_SPEED: i32 = 1;
const PLAYER_SIZE: f32 = 1.0;

const ENEMY_MAX_LIFE: i32 = 1;
const ENEMY_SPEED: i32 = 1;
const ENEMY_SIZE: f32 = 1.0;

const BARRIER_MAX_LIFE: i32 = 1;
const BARRIER_SPEED: i32 = 1;
const BARRIER_SIZE: f32 = 1.0;

const SHOT_SPEED: i32 = 1;
const SHOT_SIZE: f32 = 1.0;


struct Info {
	pos: Point2,
	speed: i32,
	direction: Point2,
	size: f32
}

struct Player {
	hit_points: i32,
	info: Info
}

struct Enemy {
	hit_points: i32,
	info: Info
}

struct Barrier {
	hit_points: i32,
	info: Info
}

struct Shot {
	info: Info
}

trait GameObject {
	fn new(pos: Point2, direction: Point2) -> Self;

	fn get_type() -> i32;
}

struct GameObjStruct {
    game_obj: Box<GameObject>,
}

impl GameObject for Player {
	fn new(pos: Point2, direction: Point2) -> Player {
		Player { hit_points: PLAYER_MAX_LIFE,
		info: Info{ pos: pos, 
		speed: PLAYER_SPEED,
		direction: direction,
		size: PLAYER_SIZE }
		}
	}

	fn get_type() -> i32 {
		return T_PLAYER;
	}

}

impl GameObject for Enemy {
	fn new(pos: Point2, direction: Point2) -> Enemy {
		Enemy {hit_points: ENEMY_MAX_LIFE,
		info: Info{ pos: pos, 
		speed: ENEMY_SPEED,
		direction: direction,
		size: ENEMY_SIZE }
		}
	}

	fn get_type() -> i32 {
		return T_ENEMY;
	}

}

impl GameObject for Barrier {
	fn new(pos: Point2, direction: Point2) -> Barrier {
		Barrier {hit_points: BARRIER_MAX_LIFE,
		info: Info{ pos: pos, 
		speed: BARRIER_SPEED,
		direction: direction,
		size: BARRIER_SIZE }
		}
	}

	fn get_type() -> i32 {
		return T_BARRIER;
	}

}

impl GameObject for Shot {
	fn new(pos: Point2, direction: Point2) -> Shot {
		Shot {
		info: Info { pos: pos, 
		speed: SHOT_SPEED,
		direction: direction,
		size: SHOT_SIZE }
		}
	}

	fn get_type() -> i32 {
		return T_SHOT;
	}

}


struct Assets {
    player_image: graphics::Image,
    shot_image: graphics::Image,
    enemy_image: graphics::Image,
    barrier_image: graphics::Image,
    font: graphics::Font,
    shot_sound: audio::Source,
    hit_sound: audio::Source,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;
        let shot_image = graphics::Image::new(ctx, "/shot.png")?;
        let enemy_image = graphics::Image::new(ctx, "/enemy.png")?;
        let barrier_image = graphics::Image::new(ctx, "/barrier.png")?;
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18)?;

        let shot_sound = audio::Source::new(ctx, "/pew.ogg")?;
        let hit_sound = audio::Source::new(ctx, "/boom.ogg")?;
        Ok(Assets {
            player_image,
            shot_image,
            enemy_image,
            barrier_image,
            font,
            shot_sound,
            hit_sound,
        })
    }


    fn actor_image(&mut self, game_obj: &GameObjStruct) -> &mut graphics::Image {

    	obj_type = game_obj.get_type();

        match obj_type {
            T_PLAYER => &mut self.player_image,
           	T_ENEMY => &mut self.enemy_image,
            T_BARRIER => &mut self.barrier_image,
            T_SHOT => &mut self.shot_image,
        }
    }

}

/*

struct MainState {
    player: Player,
    shots: Vec<Shot>,
    enemies: Vec<Enemy>,
    level: i32,
    score: i32,
    assets: Assets,
    screen_width: u32,
    screen_height: u32,
    input: InputState,
    gui_dirty: bool,
    score_display: graphics::Text,
    level_display: graphics::Text,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.print_resource_stats();

        graphics::set_background_color(ctx, (0, 0, 0, 255).into());

        println!("Game resource path: {:?}", ctx.filesystem);

        print_instructions();

        let assets = Assets::new(ctx)?;
        let score_disp = graphics::Text::new(ctx, "score", &assets.font)?;
        let level_disp = graphics::Text::new(ctx, "level", &assets.font)?;

        let player = create_player();
        let rocks = create_rocks(5, player.pos, 100.0, 250.0);

        let s = MainState {
            player,
            shots: Vec::new(),
            rocks,
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

    fn fire_player_shot(&mut self) {
        self.player_shot_timeout = PLAYER_SHOT_TIME;

        let player = &self.player;
        let mut shot = create_shot();
        shot.pos = player.pos;
        shot.facing = player.facing;
        let direction = vec_from_angle(shot.facing);
        shot.velocity.x = SHOT_SPEED * direction.x;
        shot.velocity.y = SHOT_SPEED * direction.y;

        self.shots.push(shot);
        let _ = self.assets.shot_sound.play();
    }

    fn clear_dead_stuff(&mut self) {
        self.shots.retain(|s| s.life > 0.0);
        self.rocks.retain(|r| r.life > 0.0);
    }

    fn handle_collisions(&mut self) {
        for rock in &mut self.rocks {
            let pdistance = rock.pos - self.player.pos;
            if pdistance.norm() < (self.player.bbox_size + rock.bbox_size) {
                self.player.life = 0.0;
            }
            for shot in &mut self.shots {
                let distance = shot.pos - rock.pos;
                if distance.norm() < (shot.bbox_size + rock.bbox_size) {
                    shot.life = 0.0;
                    rock.life = 0.0;
                    self.score += 1;
                    self.gui_dirty = true;
                    let _ = self.assets.hit_sound.play();
                }
            }
        }
    }

    fn check_for_level_respawn(&mut self) {
        if self.rocks.is_empty() {
            self.level += 1;
            self.gui_dirty = true;
            let r = create_rocks(self.level + 5, self.player.pos, 100.0, 250.0);
            self.rocks.extend(r);
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




*/


fn main(){
	let mut player1: Player = GameObject::new(Point2::origin(), Point2::origin());

}