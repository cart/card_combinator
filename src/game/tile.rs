use bevy::prelude::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileData>()
            .add_startup_system(spawn_tiles);
    }
}

fn spawn_tiles(mut commands: Commands, tile_data: Res<TileData>) {
    for x in -1..2 {
        for y in -1..2 {
            tile_data.spawn(&mut commands, IVec2::new(x, y));
        }
    }
}

pub struct TileData {
    mesh: Handle<Mesh>,
    woods_material: Handle<StandardMaterial>,
}

impl FromWorld for TileData {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let asset_server = world.resource::<AssetServer>();
        TileData {
            mesh: meshes.add(
                shape::Quad {
                    size: Vec2::new(3.0, 3.0),
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
        }
    }
}

impl TileData {
    pub fn spawn(&self, commands: &mut Commands, location: IVec2) -> Entity {
        let tile_size = Vec2::new(3.0, 3.0);
        let position = location.as_vec2() * tile_size;
        commands
            .spawn_bundle(PbrBundle {
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                material: self.woods_material.clone(),
                mesh: self.mesh.clone(),
                ..default()
            })
            .id()
    }
}
