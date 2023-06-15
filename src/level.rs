use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::{ prelude::*, rapier::prelude::ColliderBuilder };

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(spawn_level)
      .add_system(spawn_level_collisions)
      .add_system(spawn_doors)
      .add_system(monitor_doors);
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
  tiles: Query<(Entity, &TileEnumTags), Added<TileEnumTags>>
) {
  for (entity, enum_tags) in tiles.iter() {
    if enum_tags.tags.contains(&String::from("Wall")) {
      commands.entity(entity).insert(Collider::cuboid(8.0, 8.0));
    }
  }
}

fn spawn_doors(
  mut commands: Commands,
  entity_query: Query<(Entity, &EntityInstance), Added<EntityInstance>>
) {
  for (entity, entity_instance) in entity_query.iter() {
    if entity_instance.identifier == *"LevelDoor" {
      commands.entity(entity).insert(Collider::cuboid(8.0, 8.0)).insert(Sensor);
    }
  }
}

fn monitor_doors(active_events: Query<(&EntityInstance, &ActiveEvents)>) {
  for (entity_instance, &active_event) in active_events.iter() {
    if entity_instance.identifier == *"LevelDoor" {
      if active_event == ActiveEvents::COLLISION_EVENTS {
        println!("thing collided with LevelDoor");
      }
    }
  }
}
