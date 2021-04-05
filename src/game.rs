use crate::{
    functions::{randint, refresh},
    packages::{Area, Body, Coordinates, Dimensions, EMPTY, FOOD, MOVEABLE, PLAYER},
};

pub struct Game {
    pub max_x: usize,
    pub max_y: usize,
    pub area: Area,
}

//  Implement self-mutable and informational functions
impl Game {
    //  Modifies a specified location to `FOOD`
    fn to_food(&mut self, (pos_x, pos_y): Coordinates) {
        self.area[pos_y][pos_x] = FOOD
    }
    //  Modifies a specified location to `EMPTY`
    pub fn to_empty(&mut self, (pos_x, pos_y): Coordinates) {
        self.area[pos_y][pos_x] = EMPTY
    }
    //  Modifies a specified location to `PLAYER`
    pub fn to_player(&mut self, (pos_x, pos_y): Coordinates) {
        self.area[pos_y][pos_x] = PLAYER
    }
    //  Returns the `char` value of the item at the specified coordinates
    pub fn at(&self, (pos_x, pos_y): Coordinates) -> char {
        self.area[pos_y][pos_x]
    }
    //  Returns the size of the `Game.Area`
    pub fn size(&self) -> Dimensions {
        (self.max_x, self.max_y)
    }
    //  Modifies a random `EMPTY` location to a `FOOD`
    pub fn new_food(&mut self) {
        let mut available: Body = Vec::new();

        for r in 1..self.max_y {
            for c in 1..self.max_x {
                let v: char = self.area[r][c];

                if v != PLAYER && v != FOOD {
                    available.push((c, r));
                }
            }
        }
        let len = available.len();

        if len > 0 {
            self.to_food(available[if len > 1 { randint(0..len - 1) } else { 0 }])
        }
    }
    //  Checks if the player can move
    pub fn can_move(&self, &(pos_x, pos_y): &Coordinates) -> bool {
        for v in [
            (pos_x, pos_y - 1),
            (pos_x - 1, pos_y),
            (pos_x, pos_y + 1),
            (pos_x + 1, pos_y),
        ]
        .iter()
        {
            if MOVEABLE.contains(&self.at(*v)) {
                return true;
            }
        }
        false
    }

    //  Checks if every `at` of `area` is a PLAYER signifying the end of the game
    pub fn over(&self) -> bool {
        for r in 1..self.max_y {
            for c in 1..self.max_x {
                if self.area[r][c] != PLAYER {
                    return false;
                }
            }
        }
        true
    }

    //  Places each coordinate from `player.body` as a `PLAYER`
    pub fn update(&mut self, body: &Body) {
        for (pos_x, pos_y) in body.into_iter() {
            self.area[*pos_y][*pos_x] = PLAYER;
        }
        refresh(&self.area)
    }
}
