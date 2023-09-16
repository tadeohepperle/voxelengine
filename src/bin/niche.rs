use bevy::prelude::dbg;

pub fn main() {
    let e: Option<ABC> = Some(ABC::B);
    let e1: u8 = unsafe { std::mem::transmute(e) };
    dbg!(e1);
    let f: Option<ABC> = None;
    let f1: u8 = unsafe { std::mem::transmute(f) };
    dbg!(f1);
    let s = std::mem::size_of::<Option<ABC>>();
    dbg!(s);
}

pub enum ABC {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
}
