use std::cell::RefCell;
use std::rc::Weak;
use font_kit::font::Font;
use minifb::Window;
use raqote::{DrawOptions, DrawTarget, Point, SolidSource, Source};
use crate::entity::Entity;
use crate::{EntityPosition, Position, TILE_SIZE, Velocity};
use crate::world::World;

pub struct PlayerEntity {
    pos: EntityPosition,
    world: Option<Weak<RefCell<World>>>,
    vel: Velocity,
    health: usize
}

impl PlayerEntity {
    pub fn new() -> Box<PlayerEntity> {
        Box::new(PlayerEntity {
            world: None,
            pos: EntityPosition(0,0),
            vel: Velocity(0.0, 0.0),
            health: 100
        })
    }
}

impl Entity for PlayerEntity {
    fn render(&self, target: &mut DrawTarget, font: &Font) {
        let (x, y) = (self.pos.0 as f32 * TILE_SIZE, self.pos.1 as f32 * TILE_SIZE);
        target.draw_text(&font, 14., &format!("{} HP", self.health), Point::new(x - (TILE_SIZE/2.0), y - 5.0),
                         &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)),
                         &DrawOptions::new(),
        );
        target.fill_rect(x, y, TILE_SIZE, TILE_SIZE, &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)), &DrawOptions::new());

    }
    fn update(&self, window: &mut Window) {
    }

    fn get_pos(&self) -> &EntityPosition {
        &self.pos
    }

    fn set_pos(&mut self, new_pos: EntityPosition) {
        self.pos = new_pos;
    }

    fn mv_rel(&mut self, offset: (isize, isize)) {
        self.pos.0 += offset.0;
        self.pos.1 += offset.1;
    }

    fn get_health(&self) -> usize {
        self.health
    }

    fn set_health(&mut self, value: usize) {
        self.health = value;
    }

    fn get_vel(&self) -> &Velocity {
        &self.vel
    }

    fn set_vel(&mut self, vel: Velocity) {
        self.vel = vel;
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn respawn(&mut self) {
        self.health = 100;
        self.vel.0 = 0.0;
        self.vel.1 = 0.0;
    }

    fn set_world(&mut self, world: Weak<RefCell<World>>) {
        self.world = Some(world);
    }
}