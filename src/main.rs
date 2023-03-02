pub mod entities;
pub mod components;
pub mod systems;
pub mod resources;
pub mod constants;

use bevy::prelude::*;
use resources::Game;





fn main() {
    App::new()
        // .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::game::setup)
        .run();
}
