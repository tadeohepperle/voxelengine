use bevy::{
    prelude::Mesh,
    render::{mesh::Indices, render_graph::Edge, render_resource::PrimitiveTopology},
    utils::HashMap,
};

use crate::chunk::voxel::{Corner, Matter};

use self::{pos::Pos, voxel::Voxel};

pub mod ir;
pub mod pos;

mod voxel;

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
    fn get_voxel_corners(&self, pos: Pos) -> Corners {
        let x_pos = pos.plus_x();
        let y_pos = pos.plus_y();
        let z_pos = pos.plus_z();
        let xy_pos = pos.plus_xy();
        let xz_pos = pos.plus_xz();
        let yz_pos = pos.plus_yz();
        let xyz_pos = pos.plus_xyz();

        Corners {
            x: self.get_voxel_corner(&x_pos),
            x_pos,
            y: self.get_voxel_corner(&y_pos),
            y_pos,
            z: self.get_voxel_corner(&z_pos),
            z_pos,
            xy: self.get_voxel_corner(&xy_pos),
            xy_pos,
            xz: self.get_voxel_corner(&xz_pos),
            xz_pos,
            yz: self.get_voxel_corner(&yz_pos),
            yz_pos,
            xyz: self.get_voxel_corner(&xyz_pos),
            xyz_pos,
        }
    }

    fn get_voxel_corner(&self, pos: &Pos) -> Corner {
        self.voxels.get(pos).map(|e| e.corner).unwrap_or_default()
    }
}

struct Corners {
    x: Corner,
    x_pos: Pos,
    y: Corner,
    y_pos: Pos,
    z: Corner,
    z_pos: Pos,
    xy: Corner,
    xy_pos: Pos,
    xz: Corner,
    xz_pos: Pos,
    yz: Corner,
    yz_pos: Pos,
    xyz: Corner,
    xyz_pos: Pos,
}

pub fn new_example_chunk() -> Chunk {
    const D: Option<Matter> = Some(Matter::Dirt);
    const N: Option<Matter> = None;

    macro_rules! v {
        ($x:expr,$y:expr,$z:expr,$corner:expr) => {
            Voxel::new($x, $y, $z, $corner)
        };
    }
    use Corner::*;

    #[rustfmt::skip]
    let voxels = [
        // // lower row:
        // ((0,0,0), v!(D,N,N,Strong)),
        // ((0,0,1), v!(D,N,N,Strong)),
        // ((0,0,2), v!(D,N,N,Strong)),
        // ((0,0,3), v!(D,N,N,Strong)),
        // ((0,0,4), v!(N,N,D,Strong)),
        // // actual field:
        // ((0,1,0), v!(N,D,N,Strong)),
        // ((0,1,1), v!(N,D,N,Strong)),
        // ((0,1,2), v!(N,D,N,Strong)),
        // ((0,1,3), v!(N,D,N,Strong)),
        // ((0,1,4), v!(N,N,N,Strong)),
        // ((1,1,0), v!(N,D,N,Strong)),
        // ((2,1,0), v!(N,D,N,Strong)),
        // ((2,1,1), v!(N,D,N,Strong)),
        // ((2,1,2), v!(N,D,N,Strong)),
        // ((3,1,0), v!(N,D,N,Strong)),
        // // the cube on top:
        (Pos::new(1,1,1), v!(D,D,D,Strong)),
        (Pos::new(1,1,2), v!(N,N,D,Strong)),
        (Pos::new(2,1,1), v!(D,N,N,Strong)),
        (Pos::new(1,2,1), v!(N,D,N,Strong)),
        // just the support corners:
        (Pos::new(2,1,2), v!(N,N,N,Strong)),
        (Pos::new(2,2,2), v!(N,N,N,Strong)),
        (Pos::new(2,2,1), v!(N,N,N,Strong)),
        (Pos::new(1,2,2), v!(N,N,N,Strong)),

    ];

    Chunk {
        voxels: voxels.into_iter().collect(),
        edges: Default::default(),
    }
}
