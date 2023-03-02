
use bevy::prelude::*;
use serde;




#[derive(Resource, Default)]
pub struct Map {
  pub tiles: Vec<Vec<Tile>>,
}




#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile;



#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
  pub lat: u8,
  pub lon: u8,
}