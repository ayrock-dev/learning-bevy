use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
  animation::Animation, gamestate::GameState, input::Controllable, locomotion::Locomotor,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(player_added.in_set(OnUpdate(GameState::InGame)))
      .add_system(drive_player_animations.in_set(OnUpdate(GameState::InGame)));
  }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Default, Bundle)]
struct PlayerBundle {
  player: Player,
  locomotor: Locomotor,
  controllable: Controllable,
}

fn player_added(
  mut commands: Commands,
  entity_query: Query<(Entity, &Transform, &EntityInstance), Added<EntityInstance>>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  asset_server: Res<AssetServer>,
) {
  for (entity, transform, entity_instance) in entity_query.iter() {
    if entity_instance.identifier == *"Player" {
      let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("sprites/character/adult/body/idle/character_black_idle_body.png"),
        Vec2::new(11.0, 16.0),
        2,
        4,
        Some(Vec2::new(53.0, 48.0)),
        Some(Vec2::new(26.0, 26.0)),
      ));

      commands
        .entity(entity)
        .insert(PlayerBundle::default())
        .insert(SpriteSheetBundle {
          texture_atlas,
          transform: *transform,
          sprite: TextureAtlasSprite {
            custom_size: Some(Vec2::new(22.0, 32.0)),
            ..default()
          },
          ..default()
        })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::capsule_y(5.0, 5.0))
        .insert(KinematicCharacterController {
          autostep: None,
          snap_to_ground: None,
          slide: false,
          ..default()
        });
    }
  }
}

fn drive_player_animations(mut query: Query<(&mut Animation, &Locomotor)>) {
  for (mut animation, locomotor) in query.iter_mut() {
    if locomotor.direction.length() > 0.0 {
      animation.set(String::from("is_moving"), 1);

      if locomotor.direction.angle_between(Vec2::new(0.0, 1.0)).abs() < PI / 4.0 {
        animation.set(String::from("direction"), 1);
      } else if locomotor
        .direction
        .angle_between(Vec2::new(0.0, -1.0))
        .abs()
        < PI / 4.0
      {
        animation.set(String::from("direction"), 0);
      } else if locomotor.direction.angle_between(Vec2::new(1.0, 0.0)).abs() < PI / 4.0 {
        animation.set(String::from("direction"), 3);
      } else {
        animation.set(String::from("direction"), 2);
      }
    } else {
      animation.set(String::from("is_moving"), 0);
    }
  }
}
