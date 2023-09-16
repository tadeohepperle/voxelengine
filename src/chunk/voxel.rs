use bevy::prelude::default;

#[derive(Debug, Clone, Copy)]
pub struct Voxel {
    /// Some if x_side is filled
    pub x_side: Option<Matter>,
    /// Some if y_side is filled
    pub y_side: Option<Matter>,
    /// Some if z_side is filled
    pub z_side: Option<Matter>,
    /// Some if faces inside the voxel (e.g diagonal faces, weird triags) should be filled
    pub inner: Option<Matter>,
    pub corner: Corner,
}

impl Voxel {
    pub fn new(
        x_side: Option<Matter>,
        y_side: Option<Matter>,
        z_side: Option<Matter>,
        corner: Corner,
    ) -> Self {
        Self {
            x_side,
            y_side,
            z_side,
            inner: None,
            corner: corner,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Corner {
    #[default]
    Air,
    Weak,
    Strong,
}

impl Corner {
    #[inline]
    pub fn strong(&self) -> bool {
        matches!(self, Corner::Strong)
    }

    #[inline]
    pub fn weak(&self) -> bool {
        matches!(self, Corner::Weak)
    }

    #[inline]
    pub fn air(&self) -> bool {
        matches!(self, Corner::Air)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    matter: Option<Matter>,
    kind: EdgeKind,
}

#[derive(Debug, Clone, Copy)]
/// 33 possible edge kinds
pub enum EdgeKind {
    /// 1,0,0
    X,
    /// 0,1,0
    Y,
    /// 0,0,1
    Z,
    /// 1,1,0
    XY,
    /// 1,-1,0
    XYm,
    /// 1,0,1
    XZ,
    /// 1,0,-1
    XZm,
    /// 0,1,1
    YZ,
    /// 0,1,-1
    YZm,
    /// 2,1,0
    XextY,
    /// 1,2,0
    XYext,
    /// 2,-1,0
    XextYm,
    /// 1,-2,0
    XYmext,
    /// 2,0,1
    XextZ,
    /// 1,0,2
    XZext,
    /// 2,0,-1
    XextZm,
    /// 1,0,-2
    XZmext,
    /// 0,2,1
    YextZ,
    /// 0,1,2
    YZext,
    /// 0,2,-1
    YextZm,
    /// 0,1,-2
    YZmext,
    /// 1,1,1
    XYZ,
    /// 1,1,-1
    XYZm,
    /// 1,-1,1
    XYmZ,
    /// 1,-1,-1
    XYmZm,
    /// 2,1,1
    XextYZ,
    /// 1,2,1
    XYextZ,
    /// 1,1,2
    XYZext,
    /// 2,1,-1
    XextYZm,
    /// 1,2,-1
    XYextZm,
    /// 1,1,-2
    XYZmext,
    /// 2,-1,1
    XextYmZ,
    /// 1,-2,1
    XYmextZ,
    /// 1,-1,2
    XYmZext,
    /// 2,-1,-1
    XextYmZm,
    /// 1,-2,-1
    XYmextZm,
    /// 1,-1,-2
    XYmZmext,
}

#[derive(Debug, Clone, Copy)]
/// material something is made of
pub enum Matter {
    Dirt,
    Wood,
}
