use bevy::{color::palettes::tailwind, prelude::*};

pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let material = materials.add(Color::WHITE);

    commands.spawn(MaterialMeshBundle {
        mesh: floor,
        material: material.clone(),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: cube.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(0.0, 0.25, -3.0),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: cube,
        material,
        transform: Transform::from_xyz(0.75, 1.75, 0.0),
        ..default()
    });
}

pub fn spawn_lights(mut commands: Commands) {
    commands.spawn((PointLightBundle {
        point_light: PointLight {
            color: Color::from(tailwind::ROSE_300),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-2.0, 4.0, -0.75),
        ..default()
    },));
}
