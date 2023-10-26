pub mod player;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Weak;
use font_kit::font::Font;
use minifb::Window;
use raqote::DrawTarget;
use strum_macros::Display;
use crate::{EntityPosition, Position, Velocity};
use crate::world::World;

#[derive(Debug)]
pub enum EntityType {
    Player
}
pub trait Entity {
    fn render(&self, target: &mut DrawTarget, font: &Font);
    fn update(&mut self);
    fn get_type(&self) -> &EntityType;
    fn get_pos(&self) -> &EntityPosition;
    fn set_pos(&mut self, new_pos: EntityPosition);
    fn mv_rel(&mut self, offset: (f32, f32));
    fn get_health(&self) -> usize;
    fn set_health(&mut self, value: usize);
    fn get_vel(&self) -> &Velocity;
    fn set_vel(&mut self, vel: Velocity);
    fn is_alive(&self) -> bool;
    fn respawn(&mut self);
    fn set_world(&mut self, world: Weak<RefCell<World>>);
    fn as_any(&self) -> &dyn Any;
}
