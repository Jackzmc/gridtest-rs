use std::cell::RefCell;
use std::cmp::max;
use std::ptr;
use std::rc::{Rc};
use font_kit::font::Font;
use rand::Rng;
use rand::rngs::ThreadRng;
use raqote::DrawTarget;
use crate::{GRID_SIZE, Position};
use crate::entity::Entity;
use crate::tile::base::{BaseTile, TileTexture};
use crate::tile::{Tile, TileType};
use crate::tile::air::EmptyTile;

pub struct World {
    tiles: Vec<Vec<Box<dyn Tile>>>,
    entities: Vec<Rc<RefCell<Box<dyn Entity>>>>
}

const TILE_LAYERS: [TileTexture; 4] = [TileTexture::Bedrock, TileTexture::Stone, TileTexture::Dirt, TileTexture::Grass];

impl World {
    pub fn new(width: usize, height: usize) -> World {
        // Initialize the tile
        let mut rows = vec![];
        for _ in 0..height {
            let mut columns: Vec<Box<dyn Tile>> = vec![];
            for _ in 0..width {
                let mut tile = EmptyTile::new();
                columns.push(tile);
            }
            rows.push(columns);
        }
        let mut world = World {
            tiles: rows,
            entities: vec![],
        };
        world.generate();
        world
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) -> Rc<RefCell<Box<dyn Entity>>> {
        let c = Rc::new(RefCell::new(entity));
        self.entities.push(c.clone());
        c
    }

    pub fn remove_entity(&mut self, entity: &Box<dyn Entity>) {
        // if let Some(i) = self.entities.iter().position(|e| e == entity) {
        //     self.entities.remove(i);
        // }
    }

    /// Removes the tile at position (replacing with air), returning the tile
    pub fn remove_tile(&mut self, pos: &Position) -> Box<dyn Tile> {
        let replacement_tile = EmptyTile::new();
        self.swap_in_tile(pos, replacement_tile)
    }

    pub fn get_tile(&self, pos: &Position) -> Option<&Box<dyn Tile>> {
        if let Some(row) = self.tiles.get(pos.1) {
            return row.get(pos.0);
        }
        None
    }

    pub fn get_tile_mut(&mut self, pos: &Position) -> Option<&mut Box<dyn Tile>> {
        if let Some(row) = self.tiles.get_mut(pos.1) {
            if let Some(tile) = row.get_mut(pos.0) {
                return Some(tile);
            }
        }
        None
    }

    /// Swaps in tile into position, returning the replaced tile
    pub fn swap_in_tile(&mut self, pos: &Position, mut tile: Box<dyn Tile>) -> Box<dyn Tile> {
        let row = self.tiles.get_mut(pos.1).unwrap();
        std::mem::replace(&mut row[pos.0], tile)
    }

    pub fn swap_tile(&mut self, a: &Position, b: &Position) {
        // This is unsafe code because we are taking two mut references so we can swap them around.
        unsafe {
            let tile_a: *mut Box<dyn Tile> = &mut self.tiles[a.1][a.0];
            let tile_b: *mut Box<dyn Tile> = &mut self.tiles[b.1][b.0];
            ptr::swap(tile_a, tile_b);
        }
    }

    pub fn is_occupied(&self, pos: &Position) -> bool {
        self.get_tile(pos).is_some_and(|t| t.get_type() != &TileType::Empty)
    }

    /// Sets the tile at position, returning a reference to it.
    pub fn set_tile(&mut self, pos: &Position, tile: Box<dyn Tile>) -> &Box<dyn Tile> {
        self.swap_in_tile(pos, tile);
        let row = self.tiles.get(pos.1).unwrap();
        row.get(pos.0).unwrap()
    }

    /// Moves a tile from pos 'from' to 'to'. Returns false if out of bounds
    pub fn mv_tile(&mut self, from: &Position, to: &Position) -> bool {
        if to.0 >= GRID_SIZE || to.1 >= GRID_SIZE {
            return false;
        } else if self.is_occupied(to) {
            return false;
        }
        // Replace current tile with air,
        self.swap_tile(from, to);
        true
    }

    /// Move tile at pos by offset, returns bool if successful / in bounds
    pub fn mv_tile_rel(&mut self, pos: &mut Position, offset: (isize, isize)) -> bool {
        let new_coords = (pos.0 as isize + offset.0, pos.1 as isize + offset.1);
        // Check that we don't go below 0
        if new_coords.0 < 0 || new_coords.1 < 0 {
            return false;
        }
        let new_pos = Position(new_coords.0 as usize, new_coords.1 as usize);
        println!("mv_tile_rel = {}", new_pos);
        if self.mv_tile(pos, &new_pos) {
            println!("success");
            pos.0 = new_pos.0;
            pos.1 = new_pos.1;
            return true
        }
        false
    }

    /// Generates the grid
    fn generate(&mut self) {
        // for y in 1..GRID_SIZE - 1 {
        //     for x in 1..GRID_SIZE {
        //         let tile_type = util::get_random_tile_type();
        //         if tile_type != TileType::Empty {
        //             let tile = BaseTile::new(util::get_random_tile_texture());
        //             let pos = Position(x, y);
        //             self.set_tile(&pos, tile);
        //         }
        //     }
        // }

        // Make the floor
        let mut rng = rand::thread_rng();
        for x in 0..GRID_SIZE {
            let tile = BaseTile::new(TileTexture::Bedrock);
            let pos = Position(x, 0);
            self.set_tile(&pos, tile);
        }
        // For each layer, generate N amount of tiles starting from bottom up
        self._generate_layer(&mut rng, TileTexture::Stone, vec![TileTexture::Bedrock, TileTexture::Stone], 0, 5);
        self._generate_layer(&mut rng, TileTexture::Dirt, vec![TileTexture::Stone, TileTexture::Dirt], 1, 4);
        self._generate_layer(&mut rng, TileTexture::Grass, vec![TileTexture::Dirt, TileTexture::Grass], 2, 2);
        // TODO: for every layer, do a floating height [min, max] that can fluctuate += some amount, so that neighbors arent very spikey.
        // self._generate_ground(TileTexture::Stone, 0.9, vec![TileTexture::Bedrock, TileTexture::Stone]);
        // self._generate_ground(TileTexture::Dirt, 0.5, vec![TileTexture::Bedrock, TileTexture::Stone]);
        // self._generate_ground(TileTexture::Grass, 0.8, vec![TileTexture::Stone, TileTexture::Grass, TileTexture::Dirt]);
    }

    fn _generate_col(&mut self, rng: &mut ThreadRng, x: usize) {
        // [0, GRID_SIZE-1] as bottom row is bedrock and is skipped
        let height = rng.gen_range(0..GRID_SIZE-1);
        let mut y = GRID_SIZE - 2;
        println!("generating column x={}", x);
        while y > height {
            let pos = Position(x, y);
            println!("generating tile ({})", pos);
            if !self._generate_tile(rng, &pos) {
                return;
            }
            y -= 1;
        }
        println!("end column: max height")
    }

    fn _generate_tile(&mut self, rng: &mut ThreadRng, pos: &Position) -> bool {
        let bottom_tile = &self.tiles[pos.1+1][pos.0];
        if bottom_tile.get_type() != &TileType::Base {
            println!("ending column: tile is {:?}, not Base", bottom_tile.get_type());
            return false;
        }
        let bottom_variant = bottom_tile.as_any().downcast_ref::<BaseTile>().unwrap()
            .get_texture();
        if let Some(i) = TILE_LAYERS.iter().position(|l| l == bottom_variant) {
            let mut new_variant = bottom_variant;
            // Use current variant if 50% or if at the end of the layer list
            if rng.gen_bool(0.4) {
                if let Some(next_variant) = TILE_LAYERS.get(i + 1) {
                    println!("new_variant switching {:?} -> {:?}", new_variant, next_variant);
                    new_variant = next_variant;
                } else {
                    return false;
                }
            }
            let tile = BaseTile::new(new_variant.clone());
            self.set_tile(&pos, tile);
            return true;
        }
        println!("ending column: bottom variant ({:?}) is not in TILE_LAYERS", bottom_variant);
        return false;
    }

    fn _generate_layer(&mut self, rng: &mut ThreadRng, texture: TileTexture, valid_bottoms: Vec<TileTexture>, bottom_y: usize, max_height: usize){
        for x in 0..GRID_SIZE {
            let height = rng.gen_range(0..max_height);
            println!("layer {:?} | X={} | max_height={} | {} <= y <= {}", texture, x, max_height, bottom_y + height, bottom_y);
            for y in bottom_y..bottom_y+height {
                // If the tile is empty and the tile below is a solid:
                let pos = Position(x, y);
                if self.is_occupied(&pos) {
                    continue;
                }
                let bottom_tile = &self.tiles[y][x];
                if bottom_tile.get_type() == &TileType::Base {
                    let bottom_base = bottom_tile.as_any()
                        .downcast_ref::<BaseTile>()
                        .unwrap();
                    if !valid_bottoms.contains(bottom_base.get_texture()) {
                        continue;
                    }
                    // println!("type at {} = {:?} (variant={:?})", pos, tile_type, tile.get_texture());
                } else {
                    println!("type at {} = {:?}", pos, self.tiles[y][x].get_type());
                }

                let tile = BaseTile::new(texture.clone());
                self.set_tile(&pos, tile);
            }
        }
    }
    /// Renders every tile
    pub fn render(&self, target: &mut DrawTarget, font: &Font) {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                let tile = &self.tiles[y][x];
                tile.render(target, &Position(x, y), font);
            }
        }
    }
}