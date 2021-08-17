pub use {
    console::Term,
    device_query::{
        DeviceQuery, DeviceState, Keycode,
        Keycode::{A, D, S, W},
    },
    rand::Rng,
    std::{thread::sleep, time::Duration},
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

pub const WAIT: Duration = Duration::from_millis(10);

pub const MOVEABLE: [char; 2] = [EMPTY, FOOD];
pub const KEYS: [Keycode; 4] = [W, A, S, D];

pub const BORDER: char = 'X';
pub const EMPTY: char = ' ';
pub const FOOD: char = '+';
pub const BODY: char = 'o';
pub const HEAD: char = '*';
