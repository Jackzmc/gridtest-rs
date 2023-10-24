mod world;
mod tile;
mod util;
mod game;
mod entity;

use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};
use minifb::{MouseMode, Window, WindowOptions, ScaleMode, Scale, Key};
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder, Point, Transform, StrokeStyle, Color};
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use crate::entity::player::PlayerEntity;
use crate::game::Game;
use crate::tile::player::PlayerTile;
use crate::world::World;

#[derive(Clone, Debug)]
pub struct Position(usize, usize);
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}
#[derive(Clone, Debug)]
pub struct EntityPosition(isize, isize);

#[derive(Clone, Debug)]
pub struct Velocity(f32, f32);



/// The size of the tile (TILE_SIZExTILE_SIZE)
const TILE_SIZE: f32 = 20f32;
const GRID_SIZE: usize = 20;


fn main() {
    let dim = TILE_SIZE as usize * GRID_SIZE;
    let window = Window::new("Grid Test", dim, dim, WindowOptions {
        ..WindowOptions::default()
    }).unwrap();
    let size = window.get_size();
    let dt = DrawTarget::new(size.0 as i32, size.1 as i32);
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();

    let mut game = Game::new(window, dt, font);
    let player_pos = Position(1, 1);
    game.world_mut().set_tile(&player_pos, PlayerTile::new());
    game_loop(&mut game);
}

fn game_loop(game: &mut Game) {
    loop {
        game.update();
        game.render();
    }
}

