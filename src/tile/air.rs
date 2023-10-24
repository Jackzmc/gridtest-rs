use std::io::Empty;
use font_kit::font::Font;
use minifb::Window;
use raqote::DrawTarget;
use crate::Position;
use crate::tile::{Tile, TileType};

pub struct EmptyTile;

impl EmptyTile {
    pub fn new() -> Box<EmptyTile> {
        Box::new(EmptyTile {})
    }
}
impl Tile for EmptyTile {
    fn render(&self, target: &mut DrawTarget, pos: &Position, font: &Font) {
        // Do nothing
    }

    fn update(&self, window: &mut Window) {
        // Do nothing
    }

    fn get_type(&self) -> &TileType {
        &TileType::Empty
    }
}