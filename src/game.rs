use std::cell::{Cell, RefCell, RefMut};
use std::rc::Rc;
use font_kit::font::Font;
use minifb::{Key, MouseMode, Window};
use raqote::{Color, DrawOptions, DrawTarget, Point, SolidSource, Source};
use crate::{GRID_SIZE, Position};
use crate::world::World;

pub struct Game {
    pub window: Window,
    pub target: DrawTarget,
    pub font: Font,
    size: (usize, usize),
    current_world: Rc<RefCell<World>>,
    player_pos: Position
}

impl Game {
    pub fn new(window: Window, target: DrawTarget, font: Font) -> Game {
        let world = World::new(GRID_SIZE, GRID_SIZE);
        let default_world = Rc::new(RefCell::new(world));
        let size = window.get_size();
        Game {
            window,
            target,
            font,
            size,
            player_pos: Position(0, 0),
            current_world: default_world.clone(),
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
        if let Some(pos) = self.window.get_mouse_pos(MouseMode::Clamp) {
            let pos_string = format!("Player({},{})", self.player_pos.0, self.player_pos.1);
            self.draw_text_simple(Point::new(pos.0, pos.1), 15., &pos_string, Color::new(255, 0, 0, 0));
        }

        self.window.update_with_buffer(self.target.get_data(), self.size.0, self.size.1).unwrap();
    }

    pub fn update(&mut self) {
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