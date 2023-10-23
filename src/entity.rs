pub mod player;

use font_kit::font::Font;
use minifb::Window;
use raqote::DrawTarget;
use strum_macros::Display;
use crate::{Position, Velocity};

pub trait Entity {
    fn render(&self, target: &mut DrawTarget, pos: &Position, font: &Font);
    fn update(&self, window: &mut Window);
    fn get_health(&self) -> usize;
    fn set_health(&mut self, value: usize);
    fn get_vel(&self) -> &Velocity;
    fn set_vel(&mut self, vel: Velocity);
}
