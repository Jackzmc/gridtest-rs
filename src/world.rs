use std::cell::RefCell;
use std::rc::{Rc, Weak};
use font_kit::font::Font;
use minifb::Window;
use rand::Rng;
use raqote::DrawTarget;
use crate::{GRID_SIZE, Position, util};
use crate::entity::Entity;
use crate::game::Game;
use crate::tile::base::{BaseTile, TileTexture};
use crate::tile::{Tile, TileType};
use crate::tile::air::EmptyTile;

pub struct World {
    tiles: Vec<Vec<Box<dyn Tile>>>,
    entities: Vec<Rc<RefCell<Box<dyn Entity>>>>
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        // Initialize the tile
        let mut rows = vec![];
        for y in 0..height {
            let mut columns: Vec<Box<dyn Tile>> = vec![];
            for x in 0..width {
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
        let row = self.tiles.get_mut(a.1).unwrap();
        // TODO: implement
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
        let tile = self.swap_in_tile(from, EmptyTile::new());
        self.set_tile(to, tile);

        /*if let Some(from_tile) = self.get_tile_mut(from) {
            // let tile_type = from_tile.get_type().clone();
            // from_tile.set_type(TileType::Air);
            if let Some(to_tile) = self.get_tile_mut(to) {
                // to_tile.set_type(tile_type);
                // TODO: optimize out clone
                self.set_tile(to, from_tile.clone());
                return true;
            }
        }*/
        true
    }

    /// Move tile at pos by offset, returns bool if successful / in bounds
    pub fn mv_tile_rel(&mut self, pos: &mut Position, offset: (isize, isize)) -> bool {
        let new_coords = (pos.0 as isize + offset.0, pos.1 as isize - offset.1);
        // Check that we don't go below 0
        if new_coords.0 < 0 || new_coords.1 < 0 {
            return false;
        }
        let new_pos = Position(new_coords.0 as usize, new_coords.1 as usize);
        if self.mv_tile(pos, &new_pos) {
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
        for x in 0..GRID_SIZE {
            let tile = BaseTile::new(TileTexture::Bedrock);
            let pos = Position(x, GRID_SIZE - 1);
            self.set_tile(&pos, tile);
        }

        self._generate_ground(TileTexture::Stone, 0.9, vec![TileTexture::Bedrock, TileTexture::Stone]);
        self._generate_ground(TileTexture::Grass, 0.4, vec![TileTexture::Stone, TileTexture::Grass]);
    }

    fn _generate_ground(&mut self, texture: TileTexture, chance: f32, valid_textures: Vec<TileTexture>) {
        let mut rng = rand::thread_rng();
        for y in (0..GRID_SIZE-1).rev() {
            for x in 0..GRID_SIZE {
                // If the tile is empty and the tile below is a solid:
                let pos = Position(x, y);
                let tile_type = self.tiles[y+1][x].get_type();
                if tile_type == &TileType::Base {
                    let tile = self.tiles[y+1][x].as_any()
                        .downcast_ref::<BaseTile>()
                        .unwrap();
                    println!("type at {} = {:?} (variant={:?})", pos, tile_type, tile.get_texture());
                } else {
                    println!("type at {} = {:?}", pos, self.tiles[y][x].get_type());
                }
                // Check the tile below us
                if self.tiles[y+1][x].get_type() == &TileType::Base {
                    let tile = self.tiles[y+1][x].as_any()
                        .downcast_ref::<BaseTile>()
                        .unwrap();
                    println!("texture = {:?}. our pos = {}", tile.get_texture(), pos);
                    if valid_textures.contains(tile.get_texture()) {
                        if rng.gen_range(0.0..1.0) <= chance {
                            let tile = BaseTile::new(texture.clone());
                            self.set_tile(&pos, tile);
                        }
                    }
                }
            }
        }
    }

    pub fn print_coords(&self) {
        // Multiply by 2 for X and Y
        let coord_size = GRID_SIZE.to_string().len() * 2;
        // Add 3 for chars: (,)\
        let cell_size = coord_size + 3;

        self._print_line(cell_size);
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                print!("|({},{})", x, y);

            }
            println!("|");
            self._print_line(cell_size);
        }
    }

    fn _print_line(&self, cell_width: usize) {
        println!("{}+", format!("+{}", "-".repeat(cell_width)).repeat(GRID_SIZE));
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