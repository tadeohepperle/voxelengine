use bevy::{
    prelude::{Color, Gizmos, Mesh, Vec3},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use super::{
    pos::{self, Pos},
    voxel::Matter,
    Chunk, Side,
};

#[derive(Debug, Clone)]
pub struct ChunkIR {
    pub quads: Vec<QuadIR>,
    pub triags: Vec<TriagIR>,
    pub edges: Vec<EdgeIR>,
}

#[derive(Debug, Clone)]
pub struct QuadIR {
    pub matter: Matter,
    pub a: Pos,
    pub b: Pos,
    pub c: Pos,
    pub d: Pos,
}

pub fn calculate_triag_normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    let ab = b - a;
    let ac = c - a;
    ab.cross(ac).normalize()
}

#[derive(Debug, Clone)]
pub struct TriagIR {
    pub matter: Matter,
    pub a: Pos,
    pub b: Pos,
    pub c: Pos,
}

#[derive(Debug, Clone)]
pub struct EdgeIR {
    pub matter: Matter,
    pub a: Pos,
    pub b: Pos,
}

impl ChunkIR {
    pub fn construct_from_chunk(chunk: &Chunk) -> Self {
        let mut quads: Vec<QuadIR> = vec![];
        let mut triags: Vec<TriagIR> = vec![];
        let mut edges: Vec<EdgeIR> = vec![];

        let mut add_side = |side: Side, matter: Matter| match side {
            super::Side::None => {}
            super::Side::Triag(a, b, c) => {
                let triag = TriagIR { matter, a, b, c };
                triags.push(triag);
            }
            super::Side::Quad(a, b, c, d) => {
                let quad = QuadIR { matter, a, b, c, d };
                quads.push(quad);
            }
        };

        for (pos, voxel) in chunk.voxels.iter() {
            if !voxel.corner.air() {
                let corner_info = chunk.get_voxel_corner_info(*pos, voxel.corner);
                // add x side:
                if let Some(matter) = voxel.x_side {
                    add_side(corner_info.x_side(), matter);
                }
                // add y side:
                if let Some(matter) = voxel.y_side {
                    add_side(corner_info.y_side(), matter);
                }
                // add z side:
                if let Some(matter) = voxel.z_side {
                    add_side(corner_info.z_side(), matter);
                }
            }
        }

        // todo!("add edges");

        Self {
            quads,
            triags,
            edges,
        }
    }

    pub fn draw_gizmos(&self, gizmos: &mut Gizmos) {
        const QUAD_COLOR: Color = Color::GREEN;
        const EDGE_COLOR: Color = Color::RED;

        let mut draw_triangle = |a: Vec3, b: Vec3, c: Vec3| {
            gizmos.line(a, b, QUAD_COLOR);
            gizmos.line(b, c, QUAD_COLOR);
            gizmos.line(c, a, QUAD_COLOR);
        };

        for quad in self.quads.iter() {
            let a: Vec3 = quad.a.into();
            let b: Vec3 = quad.b.into();
            let c: Vec3 = quad.c.into();
            let d: Vec3 = quad.d.into();
            draw_triangle(a, b, c);
            draw_triangle(a, c, d);
        }

        for triag in self.triags.iter() {
            let a: Vec3 = triag.a.into();
            let b: Vec3 = triag.b.into();
            let c: Vec3 = triag.c.into();
            draw_triangle(a, b, c);
        }

        for edge in self.edges.iter() {
            let a: Vec3 = edge.a.into();
            let b: Vec3 = edge.b.into();
            gizmos.line(a, b, EDGE_COLOR);
        }
    }

    pub fn construct_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut verts: Vec<[f32; 3]> = vec![];
        let mut normals: Vec<[f32; 3]> = vec![];
        let mut uvs: Vec<[f32; 2]> = vec![];
        let mut indices: Vec<u32> = vec![];

        // const TO_X: [f32; 3] = [1.0, 0.0, 0.0];
        // const TO_Y: [f32; 3] = [0.0, 1.0, 0.0];
        // const TO_Z: [f32; 3] = [0.0, 0.0, 1.0];

        let mut add_triangle = |a: [f32; 3],
                                b: [f32; 3],
                                c: [f32; 3],
                                a_uv: [f32; 2],
                                b_uv: [f32; 2],
                                c_uv: [f32; 2],
                                normal: [f32; 3]| {
            let i = verts.len() as u32;
            verts.push(a);
            verts.push(b);
            verts.push(c);
            normals.push(normal);
            normals.push(normal);
            normals.push(normal);
            uvs.push(a_uv);
            uvs.push(b_uv);
            uvs.push(c_uv);
            indices.push(i);
            indices.push(i + 1);
            indices.push(i + 2);
        };

        for quad in self.quads.iter() {
            let a: Vec3 = quad.a.into();
            let b: Vec3 = quad.b.into();
            let c: Vec3 = quad.c.into();
            let d: Vec3 = quad.d.into();
            // todo!("calculation of normal might be overkill here, we can just cache the set of normals that is possible")
            let normal = calculate_triag_normal(a, b, c);
            let neg_normal: [f32; 3] = (-normal).into();
            let normal: [f32; 3] = normal.into();
            // we just trust that d lies in the same plane and will have the same normal.

            // a . . d
            // . .   .
            // .   . .
            // b . . c

            // todo!("uv must depend on material");
            let a_uv = [0.0, 0.0];
            let b_uv = [0.0, 1.0];
            let c_uv = [1.0, 1.0];
            let d_uv = [1.0, 0.0];

            let a: [f32; 3] = a.into();
            let b: [f32; 3] = b.into();
            let c: [f32; 3] = c.into();
            let d: [f32; 3] = d.into();

            // draw double sided triangles:

            add_triangle(a, b, c, a_uv, b_uv, c_uv, normal);
            add_triangle(a, c, b, a_uv, c_uv, b_uv, neg_normal);
            add_triangle(a, c, d, a_uv, c_uv, d_uv, normal);
            add_triangle(a, d, c, a_uv, d_uv, c_uv, neg_normal);
        }

        for triag in self.triags.iter() {
            let a: Vec3 = triag.a.into();
            let b: Vec3 = triag.b.into();
            let c: Vec3 = triag.c.into();

            let normal = calculate_triag_normal(a, b, c);
            let neg_normal: [f32; 3] = (-normal).into();
            let normal: [f32; 3] = normal.into();

            let a: [f32; 3] = a.into();
            let b: [f32; 3] = b.into();
            let c: [f32; 3] = c.into();

            let a_uv = [0.0, 0.0];
            let b_uv = [0.0, 1.0];
            let c_uv = [1.0, 1.0];
            // add two triangles to achieve double sided look
            add_triangle(a, b, c, a_uv, b_uv, c_uv, normal);
            add_triangle(a, c, b, a_uv, c_uv, b_uv, neg_normal);
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(Indices::U32(indices)));

        mesh
    }
}
