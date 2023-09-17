use bevy::{
    prelude::{Color, Gizmos, Mesh, Vec3},
    render::{mesh::Indices, render_graph::Edge, render_resource::PrimitiveTopology},
    utils::HashMap,
};

use crate::chunk::voxel::{Corner, Matter};

use self::{pos::Pos, voxel::Voxel};

pub mod examples;
pub mod ir;
pub mod pos;

pub mod voxel;

use Corner::*;

/// This is only the size in X and Z direction. Chunks are 256 voxels high.
pub const CHUNK_SIZE: u8 = 32;

/// x,y,z as chunk indexes
pub type ChunkPos = (isize, isize, isize);

pub struct ChunkWorld {
    pub chunks: HashMap<ChunkPos, Chunk>,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub voxels: HashMap<Pos, Voxel>,
    pub edges: HashMap<Pos, Vec<Edge>>,
}

impl Chunk {
    fn get_voxel_corner_info(&self, o: Pos, o_corner: Corner) -> VoxelCornerInfo {
        let x = o.plus_x();
        let y = o.plus_y();
        let z = o.plus_z();
        let xy = o.plus_xy();
        let xz = o.plus_xz();
        let yz = o.plus_yz();
        let xyz = o.plus_xyz();

        let cor: Corners<Corner> = {
            Corners {
                o: o_corner,
                x: self.get_voxel_corner(&x),
                y: self.get_voxel_corner(&y),
                z: self.get_voxel_corner(&z),
                xy: self.get_voxel_corner(&xy),
                xz: self.get_voxel_corner(&xz),
                yz: self.get_voxel_corner(&yz),
                xyz: self.get_voxel_corner(&xyz),
            }
        };

        let pos = Corners {
            o,
            x,
            y,
            z,
            xy,
            xz,
            yz,
            xyz,
        };
        VoxelCornerInfo { cor, pos }
    }

    fn get_voxel_corner(&self, pos: &Pos) -> Corner {
        self.voxels.get(pos).map(|e| e.corner).unwrap_or_default()
    }
}

struct Corners<T> {
    /// should never be air!
    o: T,
    x: T,
    y: T,
    z: T,
    xy: T,
    xz: T,
    yz: T,
    xyz: T,
}

struct VoxelCornerInfo {
    cor: Corners<Corner>,
    pos: Corners<Pos>,
}

const W: Corner = Weak;
const S: Corner = Strong;

impl VoxelCornerInfo {
    pub fn x_side(&self) -> Side {
        let VoxelCornerInfo { pos, cor } = &self;
        match (cor.o, cor.y, cor.yz, cor.z) {
            (S, S, S, S) => Side::Quad(pos.o, pos.y, pos.yz, pos.z),
            (W, S, S, S) => Side::Triag(pos.y, pos.yz, pos.z),
            (S, W, S, S) => Side::Triag(pos.o, pos.yz, pos.z),
            (S, S, W, S) => Side::Triag(pos.o, pos.y, pos.z),
            (S, S, S, W) => Side::Triag(pos.o, pos.y, pos.yz),
            _ => Side::None,
        }
    }

    pub fn y_side(&self) -> Side {
        let VoxelCornerInfo { pos, cor } = &self;
        match (cor.o, cor.x, cor.xz, cor.z) {
            (S, S, S, S) => Side::Quad(pos.o, pos.x, pos.xz, pos.z),
            (W, S, S, S) => Side::Triag(pos.x, pos.xz, pos.z),
            (S, W, S, S) => Side::Triag(pos.o, pos.xz, pos.z),
            (S, S, W, S) => Side::Triag(pos.o, pos.x, pos.z),
            (S, S, S, W) => Side::Triag(pos.o, pos.x, pos.xz),
            _ => Side::None,
        }
    }

    pub fn z_side(&self) -> Side {
        let VoxelCornerInfo { pos, cor } = &self;
        match (cor.o, cor.x, cor.xy, cor.y) {
            (S, S, S, S) => Side::Quad(pos.o, pos.x, pos.xy, pos.y),
            (W, S, S, S) => Side::Triag(pos.x, pos.xy, pos.y),
            (S, W, S, S) => Side::Triag(pos.o, pos.xy, pos.y),
            (S, S, W, S) => Side::Triag(pos.o, pos.x, pos.y),
            (S, S, S, W) => Side::Triag(pos.o, pos.x, pos.xy),
            _ => Side::None,
        }
    }

    pub fn inner_side(&self) -> InnerSide {
        let VoxelCornerInfo { pos, cor } = &self;
        match (cor.o, cor.z, cor.xz, cor.x, cor.y, cor.yz, cor.xyz, cor.xz) {
            // exactly one corner is weak, all others strong: -> single triangle
            (W, S, S, S, S, S, S, S) => InnerSide::Triag(pos.x, pos.y, pos.z),
            (S, W, S, S, S, S, S, S) => InnerSide::Triag(pos.o, pos.xz, pos.yz),
            (S, S, W, S, S, S, S, S) => InnerSide::Triag(pos.x, pos.xyz, pos.z),
            (S, S, S, W, S, S, S, S) => InnerSide::Triag(pos.o, pos.xz, pos.x),
            // (S, S, S, S, W, S, S, S) => InnerSide::Triag(),
            // (S, S, S, S, S, W, S, S) => InnerSide::Triag(),
            // (S, S, S, S, S, S, W, S) => InnerSide::Triag(),
            // (S, S, S, S, S, S, S, W) => InnerSide::Triag(),
            _ => InnerSide::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Side {
    None,
    Triag(Pos, Pos, Pos),
    Quad(Pos, Pos, Pos, Pos),
}

#[derive(Debug, Clone)]
pub enum InnerSide {
    None,
    Triag(Pos, Pos, Pos),
    DoubleTriag {
        p1: Pos,
        crease: (Pos, Pos),
        p2: Pos,
    },
}

impl Chunk {
    pub fn draw_gizmos(&self, gizmos: &mut Gizmos) {
        const AIR_CORNER_COLOR: Color = Color::ALICE_BLUE;
        const WEAK_CORNER_COLOR: Color = Color::PINK;
        const STRONG_CORNER_COLOR: Color = Color::DARK_GREEN;

        const SIDE_COLOR: Color = Color::DARK_GREEN;

        for (pos, voxel) in self.voxels.iter() {
            // draw corner point:
            let color = match &voxel.corner {
                Corner::Air => AIR_CORNER_COLOR,
                Corner::Weak => WEAK_CORNER_COLOR,
                Corner::Strong => STRONG_CORNER_COLOR,
            };
            gizmos.sphere((*pos).into(), Default::default(), 0.05, color);

            if let Some(matter) = voxel.x_side {
                let mut a = Vec3::from(*pos);
                a.y += 0.4;
                a.z += 0.4;
                let mut b = a;
                b.z += 0.2;
                let mut c = a;
                c.z += 0.2;
                c.y += 0.2;
                let mut d = a;
                d.y += 0.2;
                gizmos.line(a, b, SIDE_COLOR);
                gizmos.line(b, c, SIDE_COLOR);
                gizmos.line(c, d, SIDE_COLOR);
                gizmos.line(d, a, SIDE_COLOR);
            }

            if let Some(matter) = voxel.y_side {
                let mut a = Vec3::from(*pos);
                a.x += 0.4;
                a.z += 0.4;
                let mut b = a;
                b.z += 0.2;
                let mut c = a;
                c.z += 0.2;
                c.x += 0.2;
                let mut d = a;
                d.x += 0.2;
                gizmos.line(a, b, SIDE_COLOR);
                gizmos.line(b, c, SIDE_COLOR);
                gizmos.line(c, d, SIDE_COLOR);
                gizmos.line(d, a, SIDE_COLOR);
            }

            if let Some(matter) = voxel.z_side {
                let mut a = Vec3::from(*pos);
                a.x += 0.4;
                a.y += 0.4;
                let mut b = a;
                b.y += 0.2;
                let mut c = a;
                c.y += 0.2;
                c.x += 0.2;
                let mut d = a;
                d.x += 0.2;
                gizmos.line(a, b, SIDE_COLOR);
                gizmos.line(b, c, SIDE_COLOR);
                gizmos.line(c, d, SIDE_COLOR);
                gizmos.line(d, a, SIDE_COLOR);
            }
        }
    }
}
