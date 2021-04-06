pub use {
    console::Term,
    device_query::{
        DeviceQuery, DeviceState, Keycode,
        Keycode::{A, D, S, W},
    },
    rand::Rng,
    End::*,
};

pub enum End {
    Win,
    Lose,
}

pub type Coordinates = (usize, usize);
pub type Dimensions = (usize, usize);
pub type Body = Vec<Coordinates>;
pub type Area = Vec<Vec<char>>;

pub const MOVEABLE: [char; 2] = [EMPTY, FOOD];
pub const KEYS: [Keycode; 4] = [W, A, S, D];

pub const BORDER: char = 'X';
pub const PLAYER: char = 'o';
pub const EMPTY: char = ' ';
pub const FOOD: char = '+';
