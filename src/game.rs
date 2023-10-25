use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::time::{Duration, Instant};
use font_kit::font::Font;
use minifb::{Key, Window};
use raqote::{Color, DrawOptions, DrawTarget, Point, SolidSource, Source};
use crate::{EntityPosition, GRID_SIZE, Position};
use crate::entity::Entity;
use crate::world::World;

/// 1/N where N is the amount of updates / second
const UPDATE_RATE: f32 = 1.0 / 30.0;
pub struct Game {
    pub window: Window,
    pub target: DrawTarget,
    pub font: Font,
    last_update: Instant,
    size: (usize, usize),
    current_world: Rc<RefCell<World>>,
    player_pos: Position
}

impl Game {
    pub fn new(window: Window, target: DrawTarget, font: Font) -> Game {
        let world = World::new(GRID_SIZE, GRID_SIZE);
        let default_world = Rc::new(RefCell::new(world));
        // let player = default_world.borrow_mut().add_entity(PlayerEntity::new());
        let size = window.get_size();
        println!("update rate: {}", UPDATE_RATE);
        Game {
            window,
            target,
            font,
            last_update: Instant::now(),
            size,
            player_pos: Position(1, 10),
            current_world: default_world
        }
    }

    pub fn draw_text(&mut self, pos: Point, scale: f32, text: &str, src: &Source, options: Option<DrawOptions>) {
        self.target.draw_text(&self.font, scale, text, pos, src, &options.unwrap_or_default());
    }
    pub fn draw_text_simple(&mut self, pos: Point, scale: f32, text: &str, color: Color) {
        let source = Source::Solid(SolidSource::from(color));
        self.draw_text(pos, scale, text, &source, None);
    }

    pub fn world(&self) -> Rc<RefCell<World>> {
        self.current_world.clone()
    }

    pub fn world_mut(&mut self) -> RefMut<'_, World> {
        self.current_world.borrow_mut()
    }

    pub fn set_world(&mut self, world: World) {

    }

    pub fn render(&mut self) {
        self.target.clear(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff));
        self.current_world.borrow().render(&mut self.target, &self.font);
        self.window.update_with_buffer(self.target.get_data(), self.size.0, self.size.1).unwrap();
    }

    pub fn update(&mut self) {
        // Only run 1/UPDATE_RATE times a second
        if self.last_update.elapsed().as_secs_f32() < UPDATE_RATE {
            return;
        }
        self.last_update = Instant::now();
        self.window.get_keys_pressed(minifb::KeyRepeat::Yes).iter().for_each(|key|
            match key {
                Key::W => {
                    self.current_world.borrow_mut().mv_tile_rel(&mut self.player_pos, (0, 1));
                },
                Key::S => {
                    self.current_world.borrow_mut().mv_tile_rel(&mut self.player_pos, (0, -1));
                },
                Key::A => {
                    self.current_world.borrow_mut().mv_tile_rel(&mut self.player_pos, (-1, 0));
                }
                ,Key::D => {
                    self.current_world.borrow_mut().mv_tile_rel(&mut self.player_pos, (1, 0));
                }
                _ => (),
            }
        );
    }
}