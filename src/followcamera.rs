use bevy::prelude::*;

use crate::{gamestate::GameState, player::Player};

#[derive(Component)]
pub struct FollowCamera;

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(tag_follow_camera.in_set(OnUpdate(GameState::InGame)))
      .add_system(follow_camera.in_set(OnUpdate(GameState::InGame)));
  }
}

fn tag_follow_camera(mut commands: Commands, cameras: Query<Entity, Added<Camera>>) {
  for camera in cameras.iter() {
    commands.entity(camera).insert(FollowCamera);
  }
}

fn follow_camera(
  mut cameras: Query<&mut Transform, (With<FollowCamera>, Without<Player>)>,
  players: Query<&Transform, With<Player>>,
) {
  for player in players.iter() {
    for mut camera in cameras.iter_mut() {
      camera.translation.x = player.translation.x;
      camera.translation.y = player.translation.y;
    }
  }
}
