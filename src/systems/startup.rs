use bevy::prelude::*;
use bevy_fpc::FpcBundle;

use crate::components::Scene0;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(Color::hsl(203., 0.51, 0.51)));

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
        .insert(TransformBundle::from(Transform::from_xyz(0., 1., 0.)));
}
