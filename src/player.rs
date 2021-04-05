use crate::packages::{Body, Coordinates};

pub struct Player {
    pub pos_x: usize,
    pub pos_y: usize,
    pub body: Body,
}

//  Implement movement and other features to `Player`
impl Player {
    //  Returns the coordinates of the head of the player
    pub fn xy(&self) -> Coordinates {
        (self.pos_x, self.pos_y)
    }
    //  Translate player up
    pub fn up(&mut self) {
        self.pos_y -= 1;
    }
    //  Translate player left
    pub fn left(&mut self) {
        self.pos_x -= 1
    }
    //  Translate player down
    pub fn down(&mut self) {
        self.pos_y += 1
    }
    //  Translate player right
    pub fn right(&mut self) {
        self.pos_x += 1
    }

    //  Append the last previous body coordinates
    pub fn grow(&mut self, prev: Coordinates) {
        self.body.push(prev);
    }

    //  Shift each tuple from `player.body`
    pub fn update(&mut self) {
        let len = self.body.len();

        if len > 1 {
            self.body.insert(0, self.xy());
            self.body.remove(len);
        } else {
            self.body[0] = self.xy()
        }
    }
}
