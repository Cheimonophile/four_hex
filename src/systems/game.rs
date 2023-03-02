use bevy::prelude::*;

use crate::constants::*;
use crate::components::map;




/// set up a simple 3D scene
pub fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {

  // hex
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::RegularPolygon {
      sides: 6,
      radius: UNIT,
      ..default()
    })),
    material: materials.add(Color::GREEN.into()),
    ..default()
  });

  // light
  commands.spawn(PointLightBundle {
      point_light: PointLight {
          intensity: 1500.0,
          shadows_enabled: true,
          ..default()
      },
      transform: Transform::from_xyz(4.0, 8.0, 4.0),
      ..default()
  });


  // camera
  commands.spawn(Camera3dBundle {
      transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
  });
}