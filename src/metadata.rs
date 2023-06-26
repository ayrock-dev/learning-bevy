use bevy::prelude::*;
use serde::Deserialize;

#[derive(Resource, Deref, DerefMut)]
pub struct GameHandle(pub Handle<GameMeta>);

#[derive(Resource, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GameMeta {
  pub start_level: String,
}
