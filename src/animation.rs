use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::{HashMap, Uuid};

use serde::Deserialize;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(run_animations);
  }
}

#[derive(Default, Deserialize, Debug)]
struct DriverDictionary(HashMap<String, usize>);

impl DriverDictionary {
  fn get(&self, k: String) -> usize {
    if let Some(value) = self.0.get(&k) {
      return *value;
    } else {
      return 0;
    }
  }

  fn insert(&mut self, k: String, v: usize) -> Option<usize> {
    self.0.insert(k, v)
  }

  fn merge(&mut self, value: &DriverDictionary) {
    for (k, v) in value.0.iter() {
      self.insert(k.to_string(), *v);
    }
  }

  fn clear(&mut self) {
    self.0.clear()
  }
}

#[derive(Deserialize, Debug)]
struct ControlDriver {
  name: String,
  auto: bool,
}

impl ControlDriver {
  fn resolve(&self, prev: &DriverDictionary, next: &mut DriverDictionary, size: usize) -> usize {
    if size == 0 {
      return 0;
    }

    let value = prev.get(self.name.to_string()) % size;

    if self.auto {
      next.insert(self.name.to_string(), value + 1);
    }

    return value;
  }
}

impl Default for ControlDriver {
  fn default() -> Self {
    ControlDriver {
      name: Uuid::new_v4().to_string(),
      auto: true,
    }
  }
}

#[derive(Deserialize, Debug)]
enum Node {
  Leaf(AnimationNode),
  Branch(SwitchNode),
}

#[derive(Deserialize, Debug)]
struct Frame {
  index: usize,
  #[serde(skip)]
  drivers: DriverDictionary,
}

impl Frame {
  fn apply(&self, /*next: &mut DriverDictionary,*/ sprite: &mut TextureAtlasSprite) {
    //next.merge(&self.drivers);

    sprite.index = self.index;
  }
}

#[derive(Deserialize, Debug)]
struct AnimationNode {
  frames: Vec<Frame>,
  driver: ControlDriver,
}

impl AnimationNode {
  fn resolve_frame_index(&self, prev: &DriverDictionary, next: &mut DriverDictionary) -> usize {
    self.driver.resolve(prev, next, self.frames.len())
  }
}

#[derive(Deserialize, Debug)]
struct SwitchNode {
  driver: ControlDriver,
  nodes: Vec<Node>,
}

impl SwitchNode {
  fn resolve(&self, prev: &DriverDictionary, next: &mut DriverDictionary) -> &AnimationNode {
    let next_index = &self.driver.resolve(prev, next, self.nodes.len());

    match &self.nodes[*next_index] {
      Node::Branch(node) => node.resolve(prev, next), // recurse
      Node::Leaf(node) => node,
    }
  }
}

#[derive(Component, Deserialize, Debug)]
pub struct Animation {
  name: String,
  fps: i32,
  root: SwitchNode,
  #[serde(skip)]
  prev: DriverDictionary,
  #[serde(skip)]
  next: DriverDictionary,
  #[serde(skip)]
  timer: Timer,
}

impl Animation {
  fn animate(&mut self, sprite: &mut TextureAtlasSprite) {
    self.prev.merge(&self.next);
    self.next.clear();

    let leaf = self.root.resolve(&self.prev, &mut self.next);
    let index = leaf.resolve_frame_index(&self.prev, &mut self.next);
    // println!("playing leaf {} at {}", leaf.driver.name, index);

    leaf.frames[index].apply(/*&mut self.next, */ sprite);
  }

  pub fn update(&mut self, dt: Duration, sprite: &mut TextureAtlasSprite) {
    self.timer.tick(dt);

    if self.timer.just_finished() {
      self.animate(sprite);
    }
  }

  pub fn set(&mut self, k: String, v: usize) {
    self.next.insert(k, v);
  }

  pub fn get(&mut self, k: String) -> usize {
    self.next.get(k)
  }
}

impl From<&str> for Animation {
  fn from(yaml: &str) -> Self {
    let mut animation: Animation = serde_yaml::from_str(yaml).unwrap();

    let seconds_per_frame = 1.0 / (animation.fps as f32);

    animation.timer = Timer::from_seconds(seconds_per_frame, TimerMode::Repeating);
    animation.prev = DriverDictionary(HashMap::new());
    animation.next = DriverDictionary(HashMap::new());

    animation
  }
}

pub fn run_animations(
  time: Res<Time>,
  mut query: Query<(&mut Animation, &mut TextureAtlasSprite)>,
) {
  for (mut animation, mut atlas) in query.iter_mut() {
    animation.update(time.delta(), &mut atlas);
  }
}
