use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);
#[derive(Resource)]
struct GreetTimer(Timer);
#[derive(Clone, Default, Component)]
struct Player;
#[derive(Component)]
struct DoorUser;
#[derive(Component)]
struct Door(String);

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct PlayerBundle {
  #[sprite_bundle("player.png")]
  #[bundle]
  pub sprite_bundle: SpriteBundle,
  #[worldly]
  pub worldly: Worldly,

  pub player: Player,
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
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
        // .add_plugin(HelloPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        //.add_startup_system(setup_physics)
        //.add_system(loop_ball_altitude)
        //.add_system(display_door_use)
        //.add_system(mark_door_users)
        .add_system(movement)
        .add_system(follow_camera)
        .add_system(update_level_selection)
        .run();
}

fn hello_world() {
    println!("Hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());

    let ldtk_handle = asset_server.load("game.ldtk");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}

fn follow_camera(mut cameras: Query<&mut Transform, With<Camera>>, players: Query<&Transform, (With<Player>, Without<Camera>)>) {
  for player_transform in players.iter() {
    for mut camera_transform in cameras.iter_mut() {
      camera_transform.translation.x = player_transform.translation.x;
      camera_transform.translation.y = player_transform.translation.y;
    }
  }
}

fn movement(
  input: Res<Input<KeyCode>>,
  mut query: Query<(&mut Transform), With<Player>>,
) {
  for (mut transform) in &mut query {
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

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
        .insert(Sensor)
        .insert(Door(String::from("Level 1")));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
        .insert(Player)
        .insert(DoorUser);
}

fn loop_ball_altitude(mut positions: Query<&mut Transform, With<RigidBody>>) {
    for mut transform in positions.iter_mut() {
        if transform.translation.y < -200.0 {
            transform.translation.y = 400.0;
        }
    }
}

fn display_door_use(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    doors: Query<(Entity, &Door)>,
    players: Query<Entity, (With<Player>, With<DoorUser>)>,
) {
    let (entity, door) = doors.single(); // A first entity with a collider attached.
    if let Ok(player) = players.get_single() {
        // A second entity with a collider attached.

        /* Find the intersection pair, if it exists, between two colliders. */
        if rapier_context.intersection_pair(entity, player) == Some(true) {
            println!(
                "The entities {:?} and {:?} have intersecting colliders!",
                door.0, player
            );
            commands.entity(player).remove::<DoorUser>();
        }
    }
}

fn mark_door_users(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    doors: Query<(Entity, &Door)>,
    players: Query<Entity, (With<Player>, Without<DoorUser>)>,
) {
    let (entity, door) = doors.single(); // A first entity with a collider attached.
    if let Ok(player) = players.get_single() {
        // A second entity with a collider attached.

        /* Find the intersection pair, if it exists, between two colliders. */
        if rapier_context.intersection_pair(entity, player) != Some(true) {
            println!(
                "The entities {:?} and {:?} no longer have intersecting colliders!",
                door.0, player
            );
            commands.entity(player).insert(DoorUser);
        }
    }
}
