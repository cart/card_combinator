pub mod animate;
pub mod camera;
pub mod card;

use self::camera::PlayerCameraPlugin;
use crate::game::card::{Card, CardBundle, CardColor, CardPlugin};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CardPlugin)
            .add_plugin(PlayerCameraPlugin)
            .add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // stage
    commands.spawn_bundle(PbrBundle {
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.6, 0.6, 0.6),
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, -0.1),
        mesh: meshes.add(
            shape::Quad {
                size: Vec2::new(8.0, 5.0),
                ..default()
            }
            .into(),
        ),
        ..default()
    });

    commands.spawn_bundle(CardBundle {
        transform: Transform::from_xyz(-1.0, 0.6, 0.0),
        card: Card {
            color: CardColor::Gray,
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(CardBundle {
        transform: Transform::from_xyz(0.0, 0.6, 0.0),
        card: Card {
            color: CardColor::Gray,
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(CardBundle {
        transform: Transform::from_xyz(1.0, 0.6, 0.0),
        card: Card {
            color: CardColor::Gray,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(CardBundle {
        transform: Transform::from_xyz(-0.5, -0.6, 0.0),
        card: Card {
            color: CardColor::Blue,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(CardBundle {
        transform: Transform::from_xyz(0.5, -0.6, 0.0),
        card: Card {
            color: CardColor::Yellow,
            ..default()
        },
        ..default()
    });

    // let shadow_projection_size = Vec2::new(30.0, 30.0);
    // commands.spawn_bundle(DirectionalLightBundle {
    //     transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, -0.6, 0.0, -1.0)),
    //     directional_light: DirectionalLight {
    //         shadows_enabled: true,
    //         shadow_projection: OrthographicProjection {
    //             left: -shadow_projection_size.x / 2.0,
    //             right: shadow_projection_size.x / 2.0,
    //             bottom: -shadow_projection_size.y / 2.0,
    //             top: shadow_projection_size.y / 2.0,
    //             near: 0.01,
    //             far: 100.0,
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     ..default()
    // });
}
