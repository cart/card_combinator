use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileData>()
            .add_startup_system(spawn_tiles)
            .add_system_to_stage(CoreStage::PostUpdate, on_spawn_tile);
    }
}

fn spawn_tiles(mut commands: Commands, tile_data: Res<TileData>) {
    for x in -1..2 {
        for y in -1..2 {
            commands.spawn_bundle(TileBundle {
                tile: Tile::Woods,
                transform: Transform::from_translation(Tile::grid_to_translation(IVec2::new(x, y))),
                ..default()
            });
        }
    }
}

#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    #[default]
    Woods,
}
impl Tile {
    pub const SIZE: Vec2 = Vec2::from_array([3.0, 3.0]);
    pub const OFFSET: Vec2 = Vec2::from_array([-0.05, -0.05]);
    pub const TILE_SLOT_ASPECT_RATIO: f32 = 50.0 / 60.0;
    pub const TILE_SLOT_SIZE: f32 = 1.2;

    pub fn grid_to_translation(grid_location: IVec2) -> Vec3 {
        (grid_location.as_vec2() * (Self::SIZE + Self::OFFSET)).extend(0.0)
    }
}

#[derive(Bundle, Default)]
pub struct TileBundle {
    pub tile: Tile,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibiltiy: ComputedVisibility,
}

pub struct TileData {
    mesh: Handle<Mesh>,
    woods_material: Handle<StandardMaterial>,
    tile_slot_mesh: Handle<Mesh>,
    tile_slot_material: Handle<StandardMaterial>,
}

impl FromWorld for TileData {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let asset_server = world.resource::<AssetServer>();
        Self {
            mesh: meshes.add(
                shape::Quad {
                    size: Vec2::new(3.0, 3.0),
                    ..default()
                }
                .into(),
            ),
            tile_slot_mesh: meshes.add(
                shape::Quad {
                    size: Tile::TILE_SLOT_SIZE * Vec2::new(Tile::TILE_SLOT_ASPECT_RATIO, 1.0),
                    ..default()
                }
                .into(),
            ),
            woods_material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("tile_woods.png")),
                base_color: Color::rgb_u8(90, 110, 90),
                unlit: true,
                depth_bias: -10.0,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            tile_slot_material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("tile_slot.png")),
                base_color: Color::rgba_u8(255, 255, 255, 100),
                unlit: true,
                depth_bias: -9.0,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
        }
    }
}

fn on_spawn_tile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tile_data: Res<TileData>,
    tiles: Query<(Entity, &Tile), Added<Tile>>,
) {
    for (entity, tile) in &tiles {
        commands.entity(entity).with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                material: match tile {
                    Tile::Woods => tile_data.woods_material.clone(),
                },
                mesh: tile_data.mesh.clone(),
                ..default()
            });
            parent.spawn_bundle(PbrBundle {
                material: tile_data.tile_slot_material.clone(),
                mesh: tile_data.tile_slot_mesh.clone(),
                transform: Transform::from_xyz(0.0, -0.08, 0.001),
                ..default()
            });
        });
    }
}
