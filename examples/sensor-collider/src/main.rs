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
