use bevy::{
    ecs::component,
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};
use bevy_flycam::prelude::*;
use voxelengine::{
    chunk::{examples::example_chunks, ir::ChunkIR, Chunk},
    PanOrbitCameraPlugin,
};

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_cam_and_light, setup_mesh))
        .add_systems(Update, (draw_gizmos, show_hide_chunks, switch_chunks))
        .add_plugins(PanOrbitCameraPlugin)
        .run();
}

fn setup_cam_and_light(mut commands: Commands) {
    let light_transform = Transform::from_xyz(30., 30., 10.0).looking_at(Vec3::ZERO, Vec3::Y);

    // commands
    //     .spawn(Camera3dBundle {
    //         transform: camera_and_light_transform,
    //         ..default()
    //     })
    //     .insert(FlyCam);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000.0,
            range: 200.0,
            ..default()
        },
        transform: light_transform,
        ..Default::default()
    });
}

#[derive(Resource, Debug, Clone)]
pub struct ChunkResource {
    chunks: Vec<(Chunk, ChunkIR)>,
    current_index: usize,
}

impl ChunkResource {
    pub fn current_chunk(&self) -> (&Chunk, &ChunkIR) {
        let (chunk, chunk_ir) = &self.chunks[self.current_index];
        (chunk, chunk_ir)
    }

    pub fn increment_index(&mut self) {
        self.current_index += 1;
        if self.current_index >= self.chunks.len() {
            self.current_index = 0;
        }
    }

    pub fn decrement_index(&mut self) {
        if self.current_index == 0 {
            self.current_index = self.chunks.len();
        }
        self.current_index -= 1;
    }
}

#[derive(Component, Debug, Clone)]
pub struct ChunkMesh {
    index: usize,
}

fn setup_mesh(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // setup the material:
    let texture_handle: Handle<Image> = asset_server.load("textures/seamless_stone.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..default()
    });

    let chunks: Vec<(Chunk, ChunkIR)> = example_chunks()
        .into_iter()
        .map(|c| {
            let ir = ChunkIR::construct_from_chunk(&c);
            (c, ir)
        })
        .collect();

    for (index, (_chunk, chunk_ir)) in chunks.iter().enumerate() {
        let chunk_mesh = chunk_ir.construct_mesh();
        let chunk_mesh_handle: Handle<Mesh> = meshes.add(chunk_mesh);
        commands.spawn((
            PbrBundle {
                mesh: chunk_mesh_handle,
                material: material_handle.clone(),
                visibility: if index == 0 {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
                ..default()
            },
            ChunkMesh { index },
        ));
    }

    commands.insert_resource(ChunkResource {
        chunks,
        current_index: 0,
    });
}

fn show_hide_chunks(
    mut query: Query<(&mut Visibility, &ChunkMesh)>,
    mut chunks: ResMut<ChunkResource>,
) {
    for (mut v, chunk_mesh) in query.iter_mut() {
        if chunks.current_index == chunk_mesh.index {
            *v = Visibility::Visible;
        } else {
            *v = Visibility::Hidden;
        }
    }
}

fn switch_chunks(input: Res<Input<KeyCode>>, mut chunks: ResMut<ChunkResource>) {
    if input.just_pressed(KeyCode::Left) {
        chunks.increment_index();
    }
    if input.just_pressed(KeyCode::Right) {
        chunks.decrement_index();
    }
}

fn draw_gizmos(mut gizmos: Gizmos, chunks: Res<ChunkResource>) {
    let (chunk, chunk_ir) = chunks.current_chunk();
    chunk.draw_gizmos(&mut gizmos);
    chunk_ir.draw_gizmos(&mut gizmos);
}
