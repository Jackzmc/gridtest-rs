use std::any::Any;
use std::cell::RefCell;
use std::rc::Weak;
use font_kit::font::Font;
use minifb::Window;
use rand::Rng;
use raqote::{DrawOptions, DrawTarget, Point, SolidSource, Source};
use crate::entity::{Entity, EntityType, Health};
use crate::{EntityPosition, TilePosition, RENDER_BOUND, TILE_SIZE, Velocity, WINDOW_SIZE};
use crate::world::World;

pub struct PlayerEntity {
    pos: EntityPosition,
    world: Option<Weak<RefCell<World>>>,
    vel: Velocity,
    health: Health
}

const FRICTION_VALUE: f32 = 2.0;
const GRAVITY_FACTOR: f32 = 5.0;

impl PlayerEntity {
    pub fn new(pos: Option<EntityPosition>) -> Box<PlayerEntity> {
        Box::new(PlayerEntity {
            world: None,
            pos: pos.unwrap_or(EntityPosition(0.0,0.0)),
            vel: Velocity(0.0, 0.0),
            health: 100
        })
    }
}

const PLAYER_SIZE: f32 = 20f32;
const HALF_PLAYER_SIZE: f32 = PLAYER_SIZE / 2.0;

impl Entity for PlayerEntity {
    fn render(&self, target: &mut DrawTarget, font: &Font) {
        let (x, y) = (self.pos.0, RENDER_BOUND - (self.pos.1));
        target.draw_text(&font, 14., &format!("{} HP", self.health), Point::new(x - (TILE_SIZE/2.0), y - 5.0),
                         &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)),
                         &DrawOptions::new(),
        );
        target.draw_text(&font, 13., &format!("pos={} vel={}", self.pos, self.vel), Point::new(20.0, 100.0), &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)),
                         &DrawOptions::new(),
        );
        target.draw_text(&font, 13., &format!("tile_pos={}", self.pos.to_tile_coords()), Point::new(20.0, 120.0), &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)),
                         &DrawOptions::new(),
        );
        target.fill_rect(x, y, PLAYER_SIZE, PLAYER_SIZE, &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)), &DrawOptions::new());

    }
    fn update(&mut self, world: &World) {
        if self.vel.0 > 0.0 {
            // going left
            if !world.is_occupied(&self.pos.offset((HALF_PLAYER_SIZE, 0.0)).to_tile_coords()) {
                self.pos.0 += 1.0;
            } else {
                println!("collision=RIGHT");
            }
        } else if self.vel.0 < 0.0 {
            if !world.is_occupied(&self.pos.offset((-HALF_PLAYER_SIZE, 0.0)).to_tile_coords()) {
                self.pos.0 -= 1.0;
            } else {
                println!("collision=LEFT");
            }
        }
        if self.vel.1 > 0.0 {
            if !world.is_occupied(&self.pos.offset((0.0, HALF_PLAYER_SIZE)).to_tile_coords()) {
                self.pos.1 += 1.0;
            } else {
                println!("collision=UP");
            }
        } else if self.vel.1 < 0.0 {
            if !world.is_occupied(&self.pos.offset((0.0, -HALF_PLAYER_SIZE)).to_tile_coords()) {
                self.pos.1 -= 1.0;
            } else {
                println!("collision=DOWN");
            }
        }

        self.pos.0 = self.pos.0.clamp(0.0, RENDER_BOUND);
        self.pos.1 = self.pos.1.clamp(0.0, RENDER_BOUND);

        self.vel.0 /= FRICTION_VALUE;
        if self.vel.0.abs() < 0.01 {
            self.vel.0 = 0.0;
        }
        self.vel.1 /= FRICTION_VALUE;
        if self.vel.1.abs() < 0.01 {
            self.vel.1 = 0.0;
        }

        // TODO: collision and gravity! fun!
    }

    fn get_type(&self) -> &EntityType {
        &EntityType::Player
    }

    fn get_pos(&self) -> &EntityPosition {
        &self.pos
    }

    fn set_pos(&mut self, new_pos: EntityPosition) {
        self.pos = new_pos;
    }

    fn mv_rel(&mut self, offset: (f32, f32)) {
        self.vel.0 += offset.0;
        self.vel.1 += offset.1;
    }

    fn get_health(&self) -> Health {
        self.health
    }

    fn set_health(&mut self, value: Health) {
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn take_damage(&mut self, damage: Health) {
        self.health -= damage;
    }
}