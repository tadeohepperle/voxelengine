use crate::chunk::{
    pos::Pos,
    voxel::{Corner, Matter, Voxel},
};

use super::Chunk;

const D: Option<Matter> = Some(Matter::Dirt);
const N: Option<Matter> = None;
use Corner::*;

macro_rules! v {
    ($x:expr,$y:expr,$z:expr,$corner:expr) => {
        Voxel::new($x, $y, $z, $corner)
    };
}

pub fn example_chunks() -> Vec<Chunk> {
    vec![
        solid_cube(),
        solid_cube_weak_corner(),
        solid_cube_on_plane(),
        solid_cube_3weak_corners(),
    ]
}

pub fn solid_cube() -> Chunk {
    #[rustfmt::skip]
    let voxels = [
        // // the cube:
        (1,1,1, v!(D,D,D,Strong)),
        (1,1,2, v!(N,N,D,Strong)),
        (2,1,1, v!(D,N,N,Strong)),
        (1,2,1, v!(N,D,N,Strong)),
        // the support corners:
        (2,1,2, v!(N,N,N,Strong)),
        (2,2,2, v!(N,N,N,Strong)),
        (2,2,1, v!(N,N,N,Strong)),
        (1,2,2, v!(N,N,N,Strong)),

    ];

    Chunk {
        voxels: voxels
            .into_iter()
            .map(|(x, y, z, v)| (Pos::new(x, y, z), v))
            .collect(),
        edges: Default::default(),
    }
}

pub fn solid_cube_weak_corner() -> Chunk {
    #[rustfmt::skip]
    let voxels = [
        // // the cube:
        (1,1,1, v!(D,D,D,Strong)),
        (1,1,2, v!(N,N,D,Strong)),
        (2,1,1, v!(D,N,N,Strong)),
        (1,2,1, v!(N,D,N,Strong)),
        // the support corners:
        (2,1,2, v!(N,N,N,Strong)),
        (2,2,2, v!(N,N,N,Strong)),
        (2,2,1, v!(N,N,N,Weak)),
        (1,2,2, v!(N,N,N,Strong)),
    ];

    Chunk {
        voxels: voxels
            .into_iter()
            .map(|(x, y, z, v)| (Pos::new(x, y, z), v))
            .collect(),
        edges: Default::default(),
    }
}

pub fn solid_cube_3weak_corners() -> Chunk {
    #[rustfmt::skip]
    let voxels = [
        // // the cube:
        (1,1,1, v!(D,D,D,Strong)),
        (1,1,2, v!(N,N,D,Strong)),
        (2,1,1, v!(D,N,N,Strong)),
        (1,2,1, v!(N,D,N,Strong)),
        // the support corners:
        (2,1,2, v!(N,N,N,Strong)),
        (2,2,2, v!(N,N,N,Weak)),
        (2,2,1, v!(N,N,N,Weak)),
        (1,2,2, v!(N,N,N,Weak)),
    ];

    Chunk {
        voxels: voxels
            .into_iter()
            .map(|(x, y, z, v)| (Pos::new(x, y, z), v))
            .collect(),
        edges: Default::default(),
    }
}

pub fn solid_cube_on_plane() -> Chunk {
    #[rustfmt::skip]
    let voxels = [
        // // lower row:
        (0,0,0, v!(D,N,N,Strong)),
        (0,0,1, v!(D,N,N,Strong)),
        (0,0,2, v!(D,N,N,Strong)),
        (0,0,3, v!(D,N,N,Strong)),
        (0,0,4, v!(N,N,D,Strong)),
        // // actual field:
        (0,1,0, v!(N,D,N,Strong)),
        (0,1,1, v!(N,D,N,Strong)),
        (0,1,2, v!(N,D,N,Strong)),
        (0,1,3, v!(N,D,N,Strong)),
        (0,1,4, v!(N,N,N,Strong)),
        (1,1,0, v!(N,D,N,Strong)),
        (2,1,0, v!(N,D,N,Strong)),
        (2,1,1, v!(N,D,N,Strong)),
        (2,1,2, v!(N,D,N,Strong)),
        (3,1,0, v!(N,D,N,Strong)),
        // // the cube on top:
        (1,1,1, v!(D,D,D,Strong)),
        (1,1,2, v!(N,N,D,Strong)),
        (2,1,1, v!(D,N,N,Strong)),
        (1,2,1, v!(N,D,N,Strong)),
        // the support corners:
        (2,1,2, v!(N,N,N,Strong)),
        (2,2,2, v!(N,N,N,Strong)),
        (2,2,1, v!(N,N,N,Strong)),
        (1,2,2, v!(N,N,N,Strong)),
        // some weak corners:

        (1,1,3, v!(N,N,N,Weak)),
    ];

    Chunk {
        voxels: voxels
            .into_iter()
            .map(|(x, y, z, v)| (Pos::new(x, y, z), v))
            .collect(),
        edges: Default::default(),
    }
}
