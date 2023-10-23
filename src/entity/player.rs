use font_kit::font::Font;
use minifb::Window;
use raqote::DrawTarget;
use crate::entity::Entity;
use crate::{Position, Velocity};

pub struct PlayerEntity {
    vel: Velocity,
    health: usize
}

impl PlayerEntity {
    pub fn new() -> PlayerEntity {
        PlayerEntity {
            vel: Velocity(0.0, 0.0),
            health: 100
        }
    }
}

impl Entity for PlayerEntity {
    fn render(&self, target: &mut DrawTarget, pos: &Position, font: &Font) {

    }

    fn update(&self, window: &mut Window) {

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
}