use minifb::Window;
use raqote::{DrawOptions, DrawTarget, SolidSource, Source};
use crate::{Position, TILE_SIZE};
use crate::tiles::{Tile, TileType};

pub struct BaseTile {
    pos: Position,
    tile_type: TileType
}

impl Tile for BaseTile {
    fn render(&self, window: &mut Window, target: &mut DrawTarget) {
        target.fill_rect(self.pos.0 as f32 * TILE_SIZE, self.pos.1 as f32 * TILE_SIZE, TILE_SIZE, TILE_SIZE, &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0xff, 0)), &DrawOptions::new());
    }

    fn set_pos(&mut self, pos: Position) {
        self.pos = pos;
    }

    fn get_pos(&self) -> &Position {
        &self.pos
    }

    fn get_type(&self) -> &TileType {
        &self.tile_type
    }
}

impl BaseTile {
    pub fn new(tile_type: TileType) -> Box<impl Tile> {
        Box::new(BaseTile {
            pos: Position(0, 0),
            tile_type
        })
    }
    pub fn set_type(&mut self, tile_type: TileType) {
        self.tile_type = tile_type;
    }
}
