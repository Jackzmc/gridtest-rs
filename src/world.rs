use font_kit::font::Font;
use minifb::Window;
use raqote::DrawTarget;
use crate::{GRID_SIZE, Position, util};
use crate::tiles::base::BaseTile;
use crate::tiles::{Tile, TileType};

pub struct World {
    tiles: Vec<Vec<Box<dyn Tile>>>
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        // Initalize the tiles
        let mut rows = vec![];
        for y in 0..height {
            let mut columns: Vec<Box<dyn Tile>> = vec![];
            for x in 0..width {
                let mut tile = BaseTile::new(TileType::Air);
                columns.push(tile);
            }
            rows.push(columns);
        }
        let mut world = World {
            tiles: rows
        };
        world.generate();
        world
    }

    /// Removes the tile at position (replacing with air), returning the tile
    pub fn remove_tile(&mut self, pos: &Position) -> Box<dyn Tile> {
        let replacement_tile = BaseTile::new(TileType::Air);
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
        self.get_tile(pos).is_some_and(|t| t.get_type() != &TileType::Air)
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
        let mut tile = self.swap_in_tile(from, BaseTile::new(TileType::Air));
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
        for y in 0..GRID_SIZE - 1 {
            for x in 0..GRID_SIZE {
                let tile_type = util::get_random_tile_type();
                if tile_type != TileType::Air {
                    let tile = BaseTile::new(tile_type);
                    let pos = Position(x, y);
                    self.set_tile(&pos, tile);
                }
            }
        }

        // // Force bottom layer to have solid
        let row = self.tiles.get_mut(GRID_SIZE - 1).unwrap();
        for c in 0..GRID_SIZE {
            let tile = BaseTile::new(TileType::Stone);
            let pos = Position(c, GRID_SIZE - 1);
            self.set_tile(&pos, tile);
        }
    }

    fn print(&self) {
        self._print_line(1);
        for row in self.tiles.iter() {
            for tile in row.iter() {
                print!("|{}", tile.get_type());
            }
            println!("|");
            self._print_line(1);
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
                if tile.get_type() != &TileType::Air {
                    tile.render(target, &Position(x, y), font);
                }
            }
        }
    }
}