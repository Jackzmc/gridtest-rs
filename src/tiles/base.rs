use font_kit::font::Font;
use minifb::Window;
use raqote::{DrawOptions, DrawTarget, SolidSource, Source};
use crate::{Position, TILE_SIZE};
use crate::tiles::{Tile, TileType};

pub struct BaseTile {
    tile_type: TileType
}

impl Tile for BaseTile {
    fn render(&self, target: &mut DrawTarget, pos: &Position, font: &Font) {
        target.fill_rect(pos.0 as f32 * TILE_SIZE, pos.1 as f32 * TILE_SIZE, TILE_SIZE, TILE_SIZE, &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0xff, 0)), &DrawOptions::new());
    }

    fn update(&self, window: &mut Window) {

    }

    fn get_type(&self) -> &TileType {
        &self.tile_type
    }
}

impl BaseTile {
    pub fn new(tile_type: TileType) -> Box<impl Tile> {
        Box::new(BaseTile {
            tile_type
        })
    }
    pub fn set_type(&mut self, tile_type: TileType) {
        self.tile_type = tile_type;
    }
}

