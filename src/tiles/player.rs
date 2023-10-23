use minifb::Window;
use raqote::{DrawOptions, DrawTarget, SolidSource, Source};
use crate::{Position, TILE_SIZE};
use crate::tiles::{Tile, TileType};

pub struct PlayerTile {
    pos: Position,
}

impl PlayerTile {
    pub fn new(pos: Position) -> Box<impl Tile> {
        Box::new(PlayerTile {
            pos
        })
    }
}

impl Tile for PlayerTile {
    fn render(&self, window: &mut Window, target: &mut DrawTarget) {
        target.fill_rect(self.pos.0 as f32 * TILE_SIZE, self.pos.1 as f32 * TILE_SIZE, TILE_SIZE, TILE_SIZE, &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)), &DrawOptions::new());
    }

    fn set_pos(&mut self, pos: Position) {
        self.pos = pos;
    }

    fn get_pos(&self) -> &Position {
        &self.pos
    }

    fn get_type(&self) -> &TileType {
        &TileType::Player
    }
}
