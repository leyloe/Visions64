use avian3d::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, (spawn_world_model, spawn_lights));
}

fn spawn_world_model(mut commands: Commands, assets: Res<AssetServer>) {
    let scene = assets.load("untitled.glb#Scene0");

    commands.spawn((
        SceneBundle { scene, ..default() },
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        RigidBody::Static,
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
