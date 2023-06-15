use bevy::{ prelude::*, input::gamepad::*, app::AppExit };

use crate::{ locomotion::Locomotor };

pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems((gamepad_connections, gamepad_input));
  }
}

#[derive(Component, Default)]
pub struct Controllable;

#[derive(Resource)]
struct GamepadResource(Gamepad);

fn gamepad_connections(
  mut commands: Commands,
  gamepad: Option<Res<GamepadResource>>,
  mut connection_events: EventReader<GamepadConnectionEvent>
) {
  for event in connection_events.iter() {
    println!("New connection event {:?}", event.connection);
    match &event.connection {
      GamepadConnection::Connected(info) => {
        println!("New gamepad connected with ID: {:?}, name: {}", event.gamepad.id, info.name);

        // if we don't have any gamepad yet, use this one
        if gamepad.is_none() {
          commands.insert_resource(GamepadResource(event.gamepad));
        }
      }
      GamepadConnection::Disconnected => {
        println!("Lost gamepad connection with ID: {:?}", event.gamepad.id);

        // if it's the one we previously recognized, forget it
        if let Some(GamepadResource(old_gamepad)) = gamepad.as_deref() {
          if old_gamepad.id == event.gamepad.id {
            commands.remove_resource::<GamepadResource>();
          }
        }
      }
    }
  }
}

fn gamepad_input(
  keys: Res<Input<KeyCode>>,
  gamepad_resource: Option<Res<GamepadResource>>,
  axes: Res<Axis<GamepadAxis>>,
  mut locomotors: Query<&mut Locomotor, With<Controllable>>,
  mut exit: EventWriter<AppExit>
) {
  let gamepad = if let Some(resource) = gamepad_resource {
    resource.0
  } else {
    return; // no gamepad
  };

  let axis_lx = GamepadAxis {
    gamepad,
    axis_type: GamepadAxisType::LeftStickX,
  };
  let axis_ly = GamepadAxis {
    gamepad,
    axis_type: GamepadAxisType::LeftStickY,
  };

  if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
    let left_stick_pos = Vec2::new(x, y);

    for mut locomotor in locomotors.iter_mut() {
      locomotor.direction = left_stick_pos;
    }
  }

  if keys.just_pressed(KeyCode::Escape) {
    exit.send(AppExit);
  }
}
