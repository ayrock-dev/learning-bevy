use std::f32::consts::PI;

use bevy::{ prelude::* };
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{ locomotion::Locomotor, input::Controllable, animation::Animation };

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(player_added).add_system(drive_player_animations);
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
  asset_server: Res<AssetServer>
) {
  for (entity, transform, entity_instance) in entity_query.iter() {
    if entity_instance.identifier == *"Player" {
      let texture_atlas = texture_atlases.add(
        TextureAtlas::from_grid(
          asset_server.load("sprites/character/adult/body/idle/character_black_idle_body.png"),
          Vec2::new(11.0, 16.0),
          2,
          4,
          Some(Vec2::new(53.0, 48.0)),
          Some(Vec2::new(26.0, 26.0))
        )
      );

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
        .insert(build_player_animation())
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

fn build_player_animation() -> Animation {
  Animation::from(
    "
      name: player
      fps: 6
      root:
        driver:
          name: is_moving
          auto: false
        nodes:
          - !Branch
            atlas:
              path: sprites/character/adult/body/idle/character_black_idle_body.png
              tile_width: 11
              tile_height: 16
              columns: 2,
              rows: 4,
              padding_x: 53
              padding_y: 48
              offset_x: 26
              offset_y: 26
            driver:
              name: direction
              auto: false
            nodes:
                - !Leaf
                  frames:
                    - index: 4
                    - index: 5
                  driver:
                    name: idle_south
                    auto: true
                - !Leaf
                  frames:
                    - index: 6
                    - index: 7
                  driver:
                    name: idle_north
                    auto: true
                - !Leaf
                  frames:
                    - index: 2
                    - index: 3
                  driver:
                    name: idle_west
                    auto: true
                - !Leaf
                  frames:
                    - index: 0
                    - index: 1
                  driver:
                    name: idle_east
                    auto: true
          - !Branch
            atlas:
              path: sprites/character/adult/body/run/character_black_run_body.png
              tile_width: 11
              tile_height: 16
              columns: 2,
              rows: 4,
              padding_x: 53
              padding_y: 48
              offset_x: 26
              offset_y: 26
            driver:
              name: direction
              auto: false
            nodes:
                - !Leaf
                  frames:
                    - index: 4
                    - index: 5
                  driver:
                    name: moving_south
                    auto: true
                - !Leaf
                  frames:
                    - index: 6
                    - index: 7
                  driver:
                    name: moving_north
                    auto: true
                - !Leaf
                  frames:
                    - index: 2
                    - index: 3
                  driver:
                    name: moving_west
                    auto: true
                - !Leaf
                  frames:
                    - index: 0
                    - index: 1
                  driver:
                    name: moving_east
                    auto: true
    "
  )
}

fn drive_player_animations(mut query: Query<(&mut Animation, &Locomotor)>) {
  for (mut animation, locomotor) in query.iter_mut() {
    if locomotor.direction.length() > 0.0 {
      animation.set(String::from("is_moving"), 1);

      if locomotor.direction.angle_between(Vec2::new(0.0, 1.0)).abs() < PI / 4.0 {
        animation.set(String::from("direction"), 1);
      } else if locomotor.direction.angle_between(Vec2::new(0.0, -1.0)).abs() < PI / 4.0 {
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