mod animation;
mod followcamera;
mod gamestate;
mod input;
mod level;
mod locomotion;
mod physics;
mod player;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;

use animation::AnimationPlugin;
use followcamera::FollowCameraPlugin;
use gamestate::GameState;
use input::InputPlugin;
use level::LevelPlugin;
use locomotion::LocomotionPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;

fn main() {
  App::new()
    .add_state::<GameState>()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugin(DebugLinesPlugin::default())
    .add_plugin(LdtkPlugin)
    .insert_resource(LevelSelection::Index(0))
    .insert_resource(LdtkSettings {
      level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
        load_level_neighbors: true,
      },
      ..Default::default()
    })
    .add_plugin(FollowCameraPlugin)
    .add_plugin(InputPlugin)
    .add_plugin(LevelPlugin)
    .add_plugin(LocomotionPlugin)
    .add_plugin(PhysicsPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(AnimationPlugin)
    .add_system(setup_camera.in_schedule(OnEnter(GameState::InGame)))
    .run();
}

fn setup_camera(mut commands: Commands) {
  commands.spawn((Camera2dBundle {
    projection: OrthographicProjection {
      scale: 0.25,
      ..default()
    },
    ..default()
  },));
}
