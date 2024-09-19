use avian3d::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, (spawn_world_model, spawn_lights, spawn_shapes));
}

fn spawn_world_model(mut commands: Commands, assets: Res<AssetServer>) {
    let scene = assets.load("untitled.glb#Scene0");

    commands.spawn((
        SceneBundle { scene, ..default() },
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        RigidBody::Static,
    ));
}

fn spawn_shapes(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube_mesh = meshes.add(Cuboid::default());
    let cone_mesh = meshes.add(Cone::default());

    commands.spawn((
        PbrBundle {
            mesh: cube_mesh.clone(),
            material: materials.add(Color::from(tailwind::GRAY_100)),
            transform: Transform::from_xyz(2.0, 9.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: cube_mesh.clone(),
            material: materials.add(Color::from(tailwind::GRAY_100)),
            transform: Transform::from_xyz(4.0, 9.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::sphere(0.5),
    ));

    commands.spawn((
        PbrBundle {
            mesh: cone_mesh.clone(),
            material: materials.add(Color::from(tailwind::GRAY_100)),
            transform: Transform::from_xyz(6.0, 9.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cone(0.5, 1.),
    ));
}

fn spawn_lights(mut commands: Commands) {
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
