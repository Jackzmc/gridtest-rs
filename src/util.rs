use rand::Rng;
use crate::TilePosition;
use crate::tile::base::TileTexture;
use crate::tile::TileType;

pub fn get_random_tile_texture() -> TileTexture {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=2) {
        0 => TileTexture::Grass,
        _ => TileTexture::Stone,
    }
}

pub fn get_random_tile_type() -> TileType {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=2) {
        0 => TileType::Base,
        _ => TileType::Empty,
    }
}

pub fn get_random_coordinate(dim: usize) -> TilePosition {
    let mut rng = rand::thread_rng();
    TilePosition(rng.gen_range(0..dim), rng.gen_range(0..dim))
}