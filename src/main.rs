mod world;
mod tiles;
mod util;

use minifb::{MouseMode, Window, WindowOptions, ScaleMode, Scale, Key};
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder, Point, Transform, StrokeStyle};
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use crate::tiles::player::PlayerTile;
use crate::world::World;

#[derive(Clone, Debug)]
pub struct Position(usize, usize);



/// The size of the tiles (TILE_SIZExTILE_SIZE)
const TILE_SIZE: f32 = 20f32;
const GRID_SIZE: usize = 20;
const WIN_WIDTH: usize = 400;
const WIN_HEIGHT: usize = 400;


fn main() {
    let mut window = Window::new("Grid Test", WIN_WIDTH, WIN_HEIGHT, WindowOptions {
        ..WindowOptions::default()
    }).unwrap();
    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();

    let mut world = World::new(GRID_SIZE, GRID_SIZE);
    let mut player_pos = Position(0, 0);
    world.set_tile(&player_pos, PlayerTile::new(Position(0, 0)));
    loop {
        dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff));
        window.get_keys_pressed(minifb::KeyRepeat::Yes).iter().for_each(|key|
            match key {
                Key::W => {
                    world.mv_tile_rel(&mut player_pos, (0, 1));
                },
                Key::S => {
                    world.mv_tile_rel(&mut player_pos, (0, -1));
                },
                Key::A => {
                    world.mv_tile_rel(&mut player_pos, (-1, 0));
                }
                ,Key::D => {
                    world.mv_tile_rel(&mut player_pos, (1, 0));
                }
                _ => (),
            }
        );
        world.render(&mut window, &mut dt);

        if let Some(pos) = window.get_mouse_pos(MouseMode::Clamp) {
            let pos_string = format!("Player={:?}", &player_pos);
            dt.draw_text(&font, 36., &pos_string, Point::new(0., 100.),
                         &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0)),
                         &DrawOptions::new(),
            );
            window.update_with_buffer(dt.get_data(), size.0, size.1).unwrap();
        }
    }
}

