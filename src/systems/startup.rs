use bevy::prelude::*;
use bevy_fpc::FpcBundle;
use bevy_rapier3d::prelude::*;

use crate::components::Scene0;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("scenes/untitled.glb#Scene0"),
            ..default()
        },
        Scene0,
    ));

    commands
        .spawn(FpcBundle::default())
        .insert(bevy_fpc::Player)
        .insert(TransformBundle::from(Transform::from_xyz(0., 1., 0.)))
        .insert(Collider::cuboid(0.5, 1.0, 0.5));

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0., 0., 0.)),
        Collider::cuboid(10.0, 0.1, 10.0),
    ));
}
