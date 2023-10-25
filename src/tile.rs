pub mod base;
pub mod player;
pub mod air;

use std::any::Any;
use font_kit::font::Font;
use minifb::Window;
use raqote::DrawTarget;
use crate::Position;

pub trait Tile {
    fn render(&self, target: &mut DrawTarget, pos: &Position, font: &Font);
    fn update(&self, window: &mut Window);
    fn get_type(&self) -> &TileType;

    fn as_any(&self) -> &dyn Any;
}


#[derive(PartialEq, Debug)]
pub enum TileType {
    Empty,
    Base, // TODO: Base(TileTexture)?
    Player
}