use bevy::prelude::Vec3;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Pos {
    pub fn new(x: i8, y: i8, z: i8) -> Self {
        Pos { x, y, z }
    }

    pub fn plus_x(&self) -> Self {
        Pos {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        }
    }

    pub fn plus_y(&self) -> Self {
        Pos {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        }
    }

    pub fn plus_z(&self) -> Self {
        Pos {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        }
    }

    pub fn plus_xy(&self) -> Self {
        Pos {
            x: self.x + 1,
            y: self.y + 1,
            z: self.z,
        }
    }

    pub fn plus_xz(&self) -> Self {
        Pos {
            x: self.x + 1,
            y: self.y,
            z: self.z + 1,
        }
    }

    pub fn plus_yz(&self) -> Self {
        Pos {
            x: self.x,
            y: self.y + 1,
            z: self.z + 1,
        }
    }

    pub fn plus_xyz(&self) -> Self {
        Pos {
            x: self.x + 1,
            y: self.y + 1,
            z: self.z + 1,
        }
    }
}

impl From<Pos> for [f32; 3] {
    fn from(pos: Pos) -> Self {
        [pos.x as f32, pos.y as f32, pos.z as f32]
    }
}

impl From<Pos> for Vec3 {
    fn from(value: Pos) -> Self {
        Vec3::new(value.x as f32, value.y as f32, value.z as f32)
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// #[macro_export]
// macro_rules! pos {
//     ($x:expr, $y:expr, $z:expr ) => {
//         i83 {
//             x: $x,
//             y: $y,
//             z: $z,
//         }
//     };
// }
