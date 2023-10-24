pub mod base;
pub mod player;
pub mod air;

use font_kit::font::Font;
use minifb::Window;
use raqote::DrawTarget;
use strum_macros::Display;
use crate::Position;

pub trait Tile {
    fn render(&self, target: &mut DrawTarget, pos: &Position, font: &Font);
    fn update(&self, window: &mut Window);
    fn get_type(&self) -> &TileType;
}

#[derive(PartialEq)]
pub enum TileType {
    Empty,
    Base,
    Player
}