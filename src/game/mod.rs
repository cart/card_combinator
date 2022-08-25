pub mod animate;
pub mod camera;
pub mod card;
pub mod progress_bar;
pub mod tile;

use std::f32::consts::PI;

use self::camera::PlayerCameraPlugin;
use crate::game::{
    card::{Card, CardBundle, CardPlugin, CardType},
    progress_bar::{ProgressBar, ProgressBarBundle, ProgressBarPlugin},
};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CardPlugin)
            .add_plugin(PlayerCameraPlugin)
            .add_plugin(ProgressBarPlugin)
            .add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    asset_server.watch_for_changes().unwrap();

    commands.spawn_bundle(CardBundle {
        transform: Transform::from_xyz(-0.5, 0.0, 0.0),
        card: Card {
            card_type: CardType::Villager,
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(CardBundle {
        transform: Transform::from_xyz(0.5, 0.0, 0.0),
        card: Card {
            card_type: CardType::Villager,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(CardBundle {
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        card: Card {
            card_type: CardType::Log,
            ..default()
        },
        ..default()
    });

    let tile_mesh = meshes.add(
        shape::Quad {
            size: Vec2::new(3.0, 3.0),
            ..default()
        }
        .into(),
    );

    let tile_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("tile_woods.png")),
        base_color: Color::rgb_u8(90, 110, 90),
        unlit: true,
        depth_bias: -10.0,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(3.0, 0.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(-3.0, 0.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(-3.0, -3.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(3.0, -3.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(3.0, 3.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(0.0, -3.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });
}
