use ggez::graphics;
use ggez::graphics::{Point2, Vector2};
use ggez::audio;
use ggez::{Context, ContextBuilder, GameResult};

use go;


pub struct Assets { // Classe que possui todos os assets necessários para o jogo
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
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
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

    pub fn get_font(&self) -> &graphics::Font {
        return &self.font;
    }

    pub fn get_shot_sound(&self) -> &audio::Source {
        return &self.shot_sound;
    }

    pub fn get_player_hit_sound(&self) -> &audio::Source {
        return &self.player_hit_sound;
    }

    pub fn get_enemy_hit_sound(&self) -> &audio::Source {
        return &self.enemy_hit_sound;
    }



    pub fn game_obj_sprite(&mut self, game_obj: &go::GameObj) -> &mut graphics::Image { // Seleciona a sprite correta para o obj
        
        let game_obj_tag = game_obj.get_tag();

        match game_obj_tag {
            go::GameObjType::Player => &mut self.player_image,
            go::GameObjType::Enemy => {
                match game_obj.get_curr_sprite() {
                    1 => &mut self.enemy_image_2,
                    -1 => &mut self.enemy_image_1,
                    2 => &mut self.enemy_image_4,
                    -2 => &mut self.enemy_image_3,
                    _ => &mut self.enemy_image_2 // O rust exige essa opção "_"
                }
            },
            go::GameObjType::Barrier => {
                match game_obj.get_curr_sprite()  {
                    0 => &mut self.barrier_image_1,
                    1 => &mut self.barrier_image_2,
                    2 => &mut self.barrier_image_3,
                    3 => &mut self.barrier_image_4,
                    _ => &mut self.barrier_image_1 // O rust exige essa opção "_"
                }
            },
            go::GameObjType::Shot => &mut self.shot_image,
        }
    }
}