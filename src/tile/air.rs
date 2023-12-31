use std::any::Any;
use font_kit::font::Font;
use minifb::Window;
use raqote::DrawTarget;
use crate::TilePosition;
use crate::tile::{Tile, TileType};

#[derive(Copy,Clone)]
pub struct EmptyTile;

impl EmptyTile {
    pub fn new() -> Box<EmptyTile> {
        Box::new(EmptyTile {})
    }
}
impl Tile for EmptyTile {
    fn render(&self, target: &mut DrawTarget, pos: &TilePosition, font: &Font) {
        // Do nothing
    }

    fn update(&mut self) {
        // Do nothing
    }

    fn get_type(&self) -> &TileType {
        &TileType::Empty
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}