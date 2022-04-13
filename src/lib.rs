pub mod area;
pub mod game;

pub mod client;
pub mod player;

use {
    rand::{distributions::uniform::SampleUniform, thread_rng, Rng},
    std::{
        ops::Range,
        sync::{Arc, RwLock},
    },
};

pub type Lock<T> = Arc<RwLock<T>>;

pub fn lock<T>(element: T) -> Lock<T> {
    Arc::new(RwLock::new(element))
}

pub fn randint<T: SampleUniform + PartialOrd>(range: Range<T>) -> T {
    thread_rng().gen_range(range)
}

pub enum Object {
    Player = 111,
    Space = 32,
    Food = 42,
}

impl Object {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            111 => Some(Self::Player),
            32 => Some(Self::Space),
            42 => Some(Self::Food),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Movement {
    W = 119,
    A = 97,
    S = 115,
    D = 100,
}

impl Movement {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            119 => Some(Self::W),
            97 => Some(Self::A),
            115 => Some(Self::S),
            100 => Some(Self::D),
            _ => None,
        }
    }

    pub fn rand() -> Self {
        match randint(0..4) {
            0 => Self::W,
            1 => Self::A,
            2 => Self::S,
            _ => Self::D,
        }
    }
}
