use rand::Rng;
use crate::Position;
use crate::tile::TileType;

pub fn get_random_tile_type() -> TileType {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=2) {
        0 => TileType::Stone,
        _ => TileType::Air,
    }
}

pub fn get_random_coordinate(dim: usize) -> Position {
    let mut rng = rand::thread_rng();
    Position(rng.gen_range(0..dim), rng.gen_range(0..dim))
}