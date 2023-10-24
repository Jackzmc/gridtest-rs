use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use minifb::Window;
use raqote::{DrawOptions, DrawTarget, Point, SolidSource, Source};
use crate::{Position, TILE_SIZE};
use crate::tile::{Tile, TileType};

pub struct PlayerTile {
}

impl PlayerTile {
    pub fn new() -> Box<impl Tile> {
        Box::new(PlayerTile {
        })
    }
}

impl Tile for PlayerTile {
    fn render(&self, target: &mut DrawTarget, pos: &Position, font: &Font) {
        let (x, y) = (pos.0 as f32 * TILE_SIZE, pos.1 as f32 * TILE_SIZE);
        target.draw_text(&font, 14., "Player", Point::new(x - (TILE_SIZE/2.0), y - 5.0),
                     &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)),
                     &DrawOptions::new(),
        );
        target.fill_rect(x, y, TILE_SIZE, TILE_SIZE, &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)), &DrawOptions::new());

    }

    fn update(&self, window: &mut Window) {

    }


    fn get_type(&self) -> &TileType {
        &TileType::Player
    }
}
