use bevy::{
    ecs::component,
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_texture)
        .run();
}

#[derive(Component)]
pub struct CustomUV;

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let custom_texture_handle: Handle<Image> = asset_server.load("textures/seamless_stone.png");

    let cube_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh());

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(custom_texture_handle),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: cube_mesh_handle,
            material,
            ..default()
        },
        CustomUV,
    ));

    let camera_and_light_transform =
        Transform::from_xyz(1.8, 1.8, 1.8).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn(Camera3dBundle {
        transform: camera_and_light_transform,
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            range: 100.0,
            ..default()
        },
        transform: camera_and_light_transform,
        ..Default::default()
    });
}

fn create_cube_mesh() -> Mesh {
    let mut cube_mesh = Mesh::new(PrimitiveTopology::TriangleList);

    #[rustfmt::skip]
   cube_mesh.insert_attribute(
       Mesh::ATTRIBUTE_POSITION,
       // Each array is an [x, y, z] coordinate in local space.
       // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
       // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
       vec![
           // top (facing towards +y)
           [-0.5, 0.5, -0.5], // vertex with index 0
           [0.5, 0.5, -0.5], // vertex with index 1
           [0.5, 0.5, 0.5], // etc. until 23
           [-0.5, 0.5, 0.5],
           // bottom   (-y)
           [-0.5, -0.5, -0.5],
           [0.5, -0.5, -0.5],
           [0.5, -0.5, 0.5],
           [-0.5, -0.5, 0.5],
           // right    (+x)
           [0.5, -0.5, -0.5],
           [0.5, -0.5, 0.5],
           [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
           [0.5, 0.5, -0.5],
           // left     (-x)
           [-0.5, -0.5, -0.5],
           [-0.5, -0.5, 0.5],
           [-0.5, 0.5, 0.5],
           [-0.5, 0.5, -0.5],
           // back     (+z)
           [-0.5, -0.5, 0.5],
           [-0.5, 0.5, 0.5],
           [0.5, 0.5, 0.5],
           [0.5, -0.5, 0.5],
           // forward  (-z)
           [-0.5, -0.5, -0.5],
           [-0.5, 0.5, -0.5],
           [0.5, 0.5, -0.5],
           [0.5, -0.5, -0.5],
       ],
   );

    // Set-up UV coordinated to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    #[rustfmt::skip]
    cube_mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.25],
            // Assigning the UV coords for the bottom side.
            [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
            // Assigning the UV coords for the right side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the left side. 
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the back side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
            // Assigning the UV coords for the forward side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
        ],
    );

    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    #[rustfmt::skip]
    cube_mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    );

    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    #[rustfmt::skip]
    cube_mesh.set_indices(Some(Indices::U32(vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y) 
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ])));

    cube_mesh
}

fn animate_texture(
    mesh_query: Query<&Handle<Mesh>, With<CustomUV>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
) {
    let mesh_handle = mesh_query.get_single().expect("should have mesh");
    let mesh = meshes.get_mut(mesh_handle).unwrap();

    let uv_attribute = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();
    let VertexAttributeValues::Float32x2(uv_attribute) = uv_attribute else {
        panic!("Unexpected vertex format, expected Float32x2.");
    };

    for uv_cords in uv_attribute.iter_mut() {
        uv_cords[1] += time.delta_seconds() * 0.1;
        if uv_cords[1] > 1.0 {
            uv_cords[1] -= 1.0;
        }
        // uv_cords[0] %= 1.0;
    }
}

fn toggle_texture(mesh_to_change: &mut Mesh) {
    // Get a mutable reference to the values of the UV attribute, so we can iterate over it.
    let uv_attribute = mesh_to_change.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();
    // The format of the UV coordinates should be Float32x2.
    let VertexAttributeValues::Float32x2(uv_attribute) = uv_attribute else {
        panic!("Unexpected vertex format, expected Float32x2.");
    };

    // Iterate over the UV coordinates, and change them as we want.
    for uv_coord in uv_attribute.iter_mut() {
        // If the UV coordinate points to the upper, "dirt+grass" part of the texture...
        if (uv_coord[1] + 0.5) < 1.0 {
            // ... point to the equivalent lower, "sand+water" part instead,
            uv_coord[1] += 0.5;
        } else {
            // else, point back to the upper, "dirt+grass" part.
            uv_coord[1] -= 0.5;
        }
    }
}
