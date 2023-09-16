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
    chunk::{ir::ChunkIR, new_example_chunk, Chunk},
    PanOrbitCameraPlugin,
};

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_cam_and_light, setup_mesh))
        .add_systems(Update, draw_gizmos)
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
    chunk: Chunk,
    chunk_ir: ChunkIR,
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

    // setup chunk mesh:
    let chunk = new_example_chunk();
    let chunk_ir = ChunkIR::construct_from_chunk(&chunk);
    let chunk_mesh = chunk_ir.construct_mesh();

    commands.insert_resource(ChunkResource { chunk, chunk_ir });
    let chunk_mesh_handle: Handle<Mesh> = meshes.add(chunk_mesh);

    commands.spawn(PbrBundle {
        mesh: chunk_mesh_handle,
        material: material_handle,
        ..default()
    });
}

fn draw_gizmos(mut gizmos: Gizmos, chunk: Res<ChunkResource>) {
    chunk.chunk_ir.draw_gizmos(&mut gizmos);
}
