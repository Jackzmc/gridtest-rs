use std::any::Any;
use font_kit::font::Font;
use minifb::Window;
use raqote::{Color, DrawOptions, DrawTarget, SolidSource, Source};
use crate::{Position, TILE_SIZE};
use crate::tile::{Tile, TileType};


#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum TileTexture {
    Stone,
    Bedrock,
    Dirt,
    Grass
}

impl TileTexture {
    pub fn get_color(&self) -> Color {
        match self {
            TileTexture::Stone => Color::new(255, 89, 89, 87),
            TileTexture::Bedrock => Color::new(255, 46, 46, 45),
            TileTexture::Dirt => Color::new(255, 138, 90, 32),
            TileTexture::Grass => Color::new(255, 12, 207, 67),
            _ => Color::new(255, 255, 255, 255)
        }
    }
}

pub struct BaseTile {
    texture: TileTexture
}

impl Tile for BaseTile {
    fn render(&self, target: &mut DrawTarget, pos: &Position, font: &Font) {
        target.fill_rect(pos.0 as f32 * TILE_SIZE, pos.1 as f32 * TILE_SIZE, TILE_SIZE, TILE_SIZE, &Source::Solid(SolidSource::from(self.texture.get_color())), &DrawOptions::new());
    }

    fn update(&self, window: &mut Window) {

    }
    fn get_type(&self) -> &TileType {
        &TileType::Base
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl BaseTile {
    pub fn new(texture: TileTexture) -> Box<impl Tile> {
        Box::new(BaseTile {
            texture
        })
    }

    pub fn get_texture(&self) -> &TileTexture {
        &self.texture
    }
}

