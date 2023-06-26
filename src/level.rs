use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(spawn_level)
      .add_system(spawn_level_collisions);
  }
}

fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(LdtkWorldBundle {
    ldtk_handle: asset_server.load("game.ldtk"),
    ..default()
  });
}

fn spawn_level_collisions(
  mut commands: Commands,
  tiles: Query<(Entity, &TileEnumTags), Added<TileEnumTags>>,
) {
  for (entity, enum_tags) in tiles.iter() {
    if enum_tags.tags.contains(&String::from("Wall")) {
      commands.entity(entity).insert(Collider::cuboid(8.0, 8.0));
    }
  }
}
