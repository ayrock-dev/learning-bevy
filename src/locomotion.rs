use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::gamestate::GameState;

pub struct LocomotionPlugin;

impl Plugin for LocomotionPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(locomotion.in_set(OnUpdate(GameState::InGame)));
  }
}

#[derive(Component, Debug)]
pub struct Locomotor {
  pub direction: Vec2,
  pub speed: f32,
}

impl Default for Locomotor {
  fn default() -> Self {
    Locomotor {
      direction: Vec2::ZERO,
      speed: 64.0, // two tiles (16px) per second
    }
  }
}

pub fn locomotion(
  time: Res<Time>,
  mut query: Query<(&mut KinematicCharacterController, &Locomotor)>,
) {
  for (mut kinematics, locomotor) in query.iter_mut() {
    kinematics.translation = Some(locomotor.direction * locomotor.speed * time.delta_seconds());
  }
}
