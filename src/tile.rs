pub mod base;
pub mod player;
pub mod air;

use std::any::Any;
use font_kit::font::Font;
use raqote::DrawTarget;
use crate::TilePosition;

pub trait Tile {
    fn render(&self, target: &mut DrawTarget, pos: &TilePosition, font: &Font);
    fn update(&mut self);
    fn get_type(&self) -> &TileType;

    fn as_any(&self) -> &dyn Any;
}


#[derive(PartialEq, Debug)]
pub enum TileType {
    Empty,
    Base, // TODO: Base(TileTexture)?
    Player
}