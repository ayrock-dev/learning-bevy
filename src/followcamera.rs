use bevy::{ prelude::* };

use crate::{ player::Player };

#[derive(Component)]
pub struct FollowCamera;

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(setup_follow_camera).add_system(follow_camera);
  }
}

fn setup_follow_camera(mut commands: Commands, cameras: Query<Entity, With<Camera>>) {
  for camera in cameras.iter() {
    commands.entity(camera).insert(FollowCamera);
  }
}

fn follow_camera(
  mut cameras: Query<&mut Transform, (With<FollowCamera>, Without<Player>)>,
  players: Query<&Transform, With<Player>>
) {
  for player in players.iter() {
    for mut camera in cameras.iter_mut() {
      camera.translation.x = player.translation.x;
      camera.translation.y = player.translation.y;
    }
  }
}
