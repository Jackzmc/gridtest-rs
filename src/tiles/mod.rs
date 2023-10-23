pub mod base;
pub mod player;

use minifb::Window;
use raqote::DrawTarget;
use strum_macros::Display;
use crate::Position;

pub trait Tile {
    fn render(&self, window: &mut Window, target: &mut DrawTarget) -> ();
    fn set_pos(&mut self, pos: Position);
    fn get_pos(&self) -> &Position;
    fn get_type(&self) -> &TileType;
}

#[derive(Display, Clone, Debug, PartialOrd, PartialEq)]
pub enum TileType {
    #[strum(serialize = " ")]
    Air,
    #[strum(serialize = "S")]
    Stone,
    #[strum(serialize = "•")]
    Player,
}