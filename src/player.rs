use crate::packages::{Body, Coordinates};

pub struct Player {
    pub max_x: usize,
    pub max_y: usize,
    pub pos_x: usize,
    pub pos_y: usize,
    pub body: Body,
}

//  Implement movement and other features to `Player`
impl Player {
    //  Returns the coordinates of the head of the player
    pub fn head(&self) -> Coordinates {
        (self.pos_x, self.pos_y)
    }
    pub fn tail(&self) -> Coordinates {
        (|b: &Body| b[b.len() - 1])(&self.body)
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

    //  Check if the player can translate upwards
    pub fn can_up(&self, new_y: usize) -> bool {
        new_y > 0 && !self.body.contains(&(self.pos_x, new_y))
    }
    //  Check if the player can translate leftwards
    pub fn can_left(&self, new_x: usize) -> bool {
        new_x > 0 && !self.body.contains(&(new_x, self.pos_y))
    }
    //  Check if the player can translate downwards
    pub fn can_down(&self, new_y: usize) -> bool {
        new_y < self.max_y && !self.body.contains(&(self.pos_x, new_y))
    }
    //  Check if the player can translate rightwards
    pub fn can_right(&self, new_x: usize) -> bool {
        new_x < self.max_x && !self.body.contains(&(new_x, self.pos_y))
    }

    //  Append the last previous body coordinates
    pub fn grow(&mut self, prev: Coordinates) {
        self.body.push(prev);
    }

    //  Shift each tuple from `player.body`
    pub fn update(&mut self) {
        let len = self.body.len();

        if len > 1 {
            self.body.insert(0, self.head());
            self.body.remove(len);
        } else {
            self.body[0] = self.head()
        }
    }
}
