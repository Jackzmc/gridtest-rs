use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use font_kit::font::Font;
use minifb::{Key, Window};
use raqote::{Color, DrawOptions, DrawTarget, Point, SolidSource, Source};
use crate::{EntityPosition, GRID_SIZE, Position};
use crate::entity::Entity;
use crate::entity::player::PlayerEntity;
use crate::world::World;

pub struct Game {
    pub window: Window,
    pub target: DrawTarget,
    pub font: Font,
    size: (usize, usize),
    current_world: Rc<RefCell<World>>,
    player: Rc<RefCell<Box<dyn Entity>>>
}

const MOVE_SPEED: f32 = 10.0;

impl Game {
    pub fn new(window: Window, target: DrawTarget, font: Font) -> Game {
        let world = World::new(GRID_SIZE, GRID_SIZE);
        let default_world = Rc::new(RefCell::new(world));
        let player_pos = EntityPosition(40.0, 220.0);
        let player = default_world.borrow_mut().add_entity(PlayerEntity::new(Some(player_pos)));
        let size = window.get_size();
        Game {
            window,
            target,
            font,
            size,
            player,
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
        self.window.get_keys_pressed(minifb::KeyRepeat::Yes).iter().for_each(|key|
            match key {
                Key::W => {
                    self.player.borrow_mut().mv_rel((0.0, MOVE_SPEED));
                },
                Key::S => {
                    self.player.borrow_mut().mv_rel((0.0, -MOVE_SPEED));
                },
                Key::A => {
                    self.player.borrow_mut().mv_rel((-MOVE_SPEED, 0.0));
                }
                ,Key::D => {
                    self.player.borrow_mut().mv_rel((MOVE_SPEED, 0.0));
                }
                _ => (),
            }
        );

        self.current_world.borrow_mut().update();
    }
}