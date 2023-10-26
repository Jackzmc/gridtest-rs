mod world;
mod tile;
mod util;
mod game;
mod entity;

use std::fmt::{Display, Formatter};
use std::ops::Deref;
use minifb::{MouseMode, Window, WindowOptions, ScaleMode, Scale, Key};
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder, Point, Transform, StrokeStyle, Color};
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use crate::entity::player::PlayerEntity;
use crate::game::Game;
use crate::tile::player::PlayerTile;

#[derive(Clone, Debug)]
pub struct Position(usize, usize);
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}
#[derive(Clone, Debug)]
pub struct FloatVector2D(f32, f32);

impl Display for FloatVector2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2},{:.2})", self.0, self.1)
    }
}

use FloatVector2D as EntityPosition;
use FloatVector2D as Velocity;


/// The size of the tile (TILE_SIZExTILE_SIZE)
const TILE_SIZE: f32 = 20f32;
const GRID_SIZE: usize = 20;

const WINDOW_SIZE: usize = TILE_SIZE as usize * GRID_SIZE;
const RENDER_BOUND: f32 = TILE_SIZE * GRID_SIZE as f32 - TILE_SIZE;

fn main() {
    let dim = TILE_SIZE as usize * GRID_SIZE;
    let window = Window::new("Grid Test", WINDOW_SIZE, WINDOW_SIZE, WindowOptions {
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
    game_loop(&mut game);
}


fn game_loop(game: &mut Game) {
    'main_loop: loop {
        game.update();
        game.render();
    }
}

