use minifb::Window;
use raqote::DrawTarget;
use crate::{Position, TileType};

pub trait Tile {
    fn render(&self, window: &mut Window, target: &mut DrawTarget) -> ();
    fn set_pos(&mut self, pos: Position);
    fn get_pos(&self) -> &Position;
    fn get_type(&self) -> &TileType;
}