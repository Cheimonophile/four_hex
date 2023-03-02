
use bevy::prelude::*;
 
use crate::components::map::Map;




#[derive(Resource, Default)]
pub struct Game {
  pub map: Map
}