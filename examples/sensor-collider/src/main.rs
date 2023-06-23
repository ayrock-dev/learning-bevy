use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default, Clone)]
struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct PlayerBundle {
  #[sprite_bundle("player.png")]
  #[bundle]
  sprite_bundle: SpriteBundle,
  #[worldly]
  worldly: Worldly,

  player: Player,
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(LdtkPlugin)
    .insert_resource(LevelSelection::Uid(4))
    .insert_resource(LdtkSettings {
      level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
        load_level_neighbors: true,
      },
      set_clear_color: SetClearColor::FromLevelBackground,
      ..Default::default()
    })
    .register_ldtk_entity::<PlayerBundle>("Player")
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_startup_system(setup)
    .add_system(movement)
    .add_system(follow_camera)
    .add_system(update_level_selection)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  // Add a camera
  commands.spawn(Camera2dBundle::default());

  let ldtk_handle = asset_server.load("game.ldtk");
  commands.spawn(LdtkWorldBundle {
    ldtk_handle,
    ..Default::default()
  });
}

fn follow_camera(
  mut cameras: Query<&mut Transform, With<Camera>>,
  players: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
  for player_transform in &players {
    for mut camera_transform in &mut cameras {
      camera_transform.translation.x = player_transform.translation.x;
      camera_transform.translation.y = player_transform.translation.y;
    }
  }
}

fn movement(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
  for mut transform in &mut query {
    let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
    let left = if input.pressed(KeyCode::A) { 1. } else { 0. };

    transform.translation.x += (right - left) * 8.;

    let up = if input.pressed(KeyCode::W) { 1. } else { 0. };
    let down = if input.pressed(KeyCode::S) { 1. } else { 0. };

    transform.translation.y += (up - down) * 8.;
  }
}

fn update_level_selection(
  level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
  player_query: Query<&Transform, With<Player>>,
  mut level_selection: ResMut<LevelSelection>,
  ldtk_levels: Res<Assets<LdtkLevel>>,
) {
  for (level_handle, level_transform) in &level_query {
    if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
      let level_bounds = Rect {
        min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
        max: Vec2::new(
          level_transform.translation.x + ldtk_level.level.px_wid as f32,
          level_transform.translation.y + ldtk_level.level.px_hei as f32,
        ),
      };

      for player_transform in &player_query {
        if player_transform.translation.x < level_bounds.max.x
          && player_transform.translation.x > level_bounds.min.x
          && player_transform.translation.y < level_bounds.max.y
          && player_transform.translation.y > level_bounds.min.y
          && !level_selection.is_match(&0, &ldtk_level.level)
        {
          *level_selection = LevelSelection::Iid(ldtk_level.level.iid.clone());
        }
      }
    }
  }
}
