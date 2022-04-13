use std::sync::Arc;

use super::Movement;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Player {
    pub direction: Arc<Movement>,
    pub data: Vec<usize>,
}

impl Player {
    pub fn new(p: usize) -> Self {
        Self {
            direction: Arc::new(Movement::rand()),
            data: vec![p],
        }
    }

    pub fn head(&self) -> usize {
        self.data[0]
    }

    pub fn tail(&self) -> usize {
        let tail_index = self.data.len() - 1;
        self.data[tail_index]
    }

    pub fn translate(&mut self, index: usize) {
        self.data.insert(0, index);
        self.data.remove(self.data.len() - 1);
    }

    pub fn grow(&mut self, index: usize) {
        self.data.insert(0, index);
    }
}
