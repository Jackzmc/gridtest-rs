mod world;
mod tile;
mod util;
mod game;
mod entity;

use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::OnceLock;
use minifb::{MouseMode, Window, WindowOptions, ScaleMode, Scale, Key};
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder, Point, Transform, StrokeStyle, Color};
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use crate::entity::player::PlayerEntity;
use crate::game::{DEFAULT_MAX_FPS, DEFAULT_TICK_RATE, Game};
use crate::tile::player::PlayerTile;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// The tickrate at how many times a second the game updates
    #[arg(long)]
    tickrate: Option<u8>,

    /// The maximum fps to achieve. 0 for unlimited
    #[arg(long)]
    max_fps: Option<u8>,
}

#[derive(Clone, Debug)]
pub struct TilePosition(usize, usize);
impl Display for TilePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}
impl EntityPosition {
    fn to_tile_coords(&self) -> TilePosition {
        TilePosition((self.0 / TILE_SIZE).round() as usize, (self.1 / TILE_SIZE).round() as usize)
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


pub static TICK_RATE: OnceLock<f32> = OnceLock::new();
pub static MAX_FPS: OnceLock<f32> = OnceLock::new();
fn main() {
    let args = Args::parse();
    TICK_RATE.set(1.0 / args.tickrate.unwrap_or(DEFAULT_TICK_RATE) as f32).unwrap();
    let max_fps = args.tickrate.unwrap_or(DEFAULT_MAX_FPS);
    if max_fps == 0 {
        // Set the limit to 1000 for "unlimited" fps
        MAX_FPS.set(1.0 / 1000.0).unwrap();
    } else {
        MAX_FPS.set(1.0 / args.tickrate.unwrap_or(DEFAULT_MAX_FPS) as f32).unwrap();
    }

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
        // End the game when closed
        if !game.window.is_open() {
            return;
        }
        game.update();
        game.render();
    }
}

