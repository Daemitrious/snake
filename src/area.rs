use {
    crate::{player::Player, Movement, Object},
    rand::{prelude::IteratorRandom, thread_rng},
    std::ops::{Index, IndexMut},
};

pub struct Area {
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<u8>,
}

impl Area {
    pub const fn rows(&self) -> usize {
        self.rows
    }

    pub const fn columns(&self) -> usize {
        self.columns
    }

    pub const fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn available(&self) -> bool {
        for v in self.data.iter() {
            if *v == Object::Space as u8 {
                return true;
            }
        }
        false
    }

    pub fn vacancy(&self) -> Option<usize> {
        (0..self.data.len())
            .filter(|i| self.data[*i] == 32)
            .choose(&mut thread_rng())
    }

    pub fn set(&mut self, index: usize, object: Object) {
        self[index] = object as u8
    }

    pub fn can_move(&self, player: &Player) -> bool {
        let head = player.data[0];

        match *player.direction {
            Movement::W => head / self.rows > 0,
            Movement::A => head % self.columns > 0,
            Movement::S => head / self.rows < self.rows - 1,
            Movement::D => head % self.columns < self.columns - 1,
        }
    }

    pub fn next_move(&self, player: &Player) -> usize {
        let head = player.data[0];

        match *player.direction {
            Movement::W => head - self.rows,
            Movement::A => head - 1,
            Movement::S => head + self.rows,
            Movement::D => head + 1,
        }
    }

    pub fn info(&self) -> Vec<u8> {
        let mut data = self.data.clone();
        data.insert(0, self.columns() as u8);
        data.insert(0, self.rows() as u8);
        data
    }
}

impl Default for Area {
    fn default() -> Self {
        let rows = 10;
        let columns = 10;
        Self {
            rows,
            columns,
            data: (0..rows * columns).map(|_| Object::Space as u8).collect(),
        }
    }
}

impl Index<usize> for Area {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Area {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
