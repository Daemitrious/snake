use keyboard_query::{DeviceQuery, DeviceState};
use rand::Rng;

const BORDER: char = 'X';
const PLAYER: char = 'o';
const EMPTY: char = ' ';
const FOOD: char = '+';

const W: u16 = 87;
const A: u16 = 65;
const S: u16 = 83;
const D: u16 = 68;

const WASD: [u16; 4] = [W, A, S, D];
const MOVEABLE: [char; 2] = [EMPTY, FOOD];

type Coordinates = (usize, usize);
type Dimensions = (usize, usize);
type Body = Vec<Coordinates>;
type Area = Vec<Vec<char>>;

struct Game {
    max_x: usize,
    max_y: usize,
    area: Area,
}

struct Player {
    body: Body,
    pos_x: usize,
    pos_y: usize,
}

//  Implement self-mutable and informational functions
impl Game {
    fn at(&self, (pos_x, pos_y): Coordinates) -> char {
        self.area[pos_y][pos_x]
    }

    //  Checks if the player can move
    fn can_move(&self, &(pos_x, pos_y): &Coordinates) -> bool {
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
    fn over(&self) -> bool {
        for r in 1..self.max_y {
            for c in 1..self.max_x {
                if self.area[r][c] != PLAYER {
                    return false;
                }
            }
        }
        true
    }

    //  Modifies a specified location to `EMPTY`
    fn to_empty(&mut self, (pos_x, pos_y): Coordinates) {
        self.area[pos_y][pos_x] = EMPTY
    }
    //  Modifies a specified location to `PLAYER`
    fn to_player(&mut self, (pos_x, pos_y): Coordinates) {
        self.area[pos_y][pos_x] = PLAYER
    }
    //  Modifies a specified location to `FOOD`
    fn to_food(&mut self, (pos_x, pos_y): Coordinates) {
        self.area[pos_y][pos_x] = FOOD
    }

    //  Modifies a random `EMPTY` location to a `FOOD`
    fn new_food(&mut self) {
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

    //  Places each coordinate from `player.body` as a `PLAYER`
    fn update(&mut self, body: &Body) {
        for (pos_x, pos_y) in body.iter() {
            self.area[*pos_y][*pos_x] = PLAYER;
        }
        refresh(&self.area)
    }
    //  Returns the size of the `Game.Area`
    fn size(&self) -> Dimensions {
        (self.max_x, self.max_y)
    }
}

//  Implement movement and other features to `Player`
impl Player {
    //  Returns the coordinates of the head of the player
    fn xy(&self) -> Coordinates {
        (self.pos_x, self.pos_y)
    }
    //  Translate player up
    fn up(&mut self) {
        self.pos_y -= 1;
    }
    //  Translate player left
    fn left(&mut self) {
        self.pos_x -= 1
    }
    //  Translate player down
    fn down(&mut self) {
        self.pos_y += 1
    }
    //  Translate player right
    fn right(&mut self) {
        self.pos_x += 1
    }

    //  Append the last previous body coordinates
    fn grow(&mut self, prev: Coordinates) {
        self.body.push(prev);
    }

    //  Shift each tuple from `player.body`
    fn update(&mut self) {
        let len = self.body.len();

        if len > 1 {
            self.body.insert(0, self.xy());
            self.body.remove(len);
        } else {
            self.body[0] = self.xy()
        }
    }
}

//  Create a new `Game` struct with a specified `Area` size
fn new_game(columns: usize, rows: usize) -> Game {
    let columns = columns + 2;
    let rows = rows + 2;

    let max_x = columns - 1;
    let max_y = rows - 1;

    let mut area: Area = (0..rows)
        .map(|_| (0..columns).map(|_| ' ').collect())
        .collect();

    for r in 0..rows {
        let edge: bool = r == 0 || r == max_y;

        for c in 0..columns {
            if edge {
                area[r][c] = BORDER;
            } else {
                if c == 0 || c == max_x {
                    area[r][c] = BORDER;
                }
            }
        }
    }
    Game {
        max_x: max_x,
        max_y: max_y,
        area: area,
    }
}

//  Select a `usize` integer from a specified Range
fn randint(range: std::ops::Range<usize>) -> usize {
    rand::thread_rng().gen_range(range)
}

//  Create a new `Player` struct with a set of random coordinates
fn new_player((max_x, max_y): Dimensions) -> Player {
    let pos_x = randint(1..max_x);
    let pos_y = randint(1..max_y);

    Player {
        pos_x: pos_x,
        pos_y: pos_y,
        body: vec![(pos_x, pos_y)],
    }
}

// Clear the terminal then print `array` in grid format
fn refresh(area: &Area) {
    //  Clear the terminal
    print!("\x1B[2J\x1B[1;1H");

    for row in area.iter() {
        for column in row.iter() {
            print!("{} ", column);
        }
        println!();
    }
}

//  Initialize keyboard input then begin game loop
fn run(game: &mut Game) {
    //  Assign first player
    let mut player: Player = new_player(game.size());

    //  Keyboard inputs
    let input = DeviceState::new();
    let mut prev_keys = Vec::with_capacity(1);

    //////////////////////////////////////
    game.to_player(player.xy());
    game.new_food();
    game.update(&player.body);
    //////////////////////////////////////

    //  Begin loop
    let ending = loop {
        let keys = input.get_keys();

        if keys != prev_keys && keys.len() == 1 {
            let key = keys[0];

            if WASD.contains(&key) {
                let prev = player.body[player.body.len() - 1];

                match key {
                    W => {
                        if player.pos_y - 1 > 0
                            && game.at((player.pos_x, player.pos_y - 1)) != PLAYER
                        {
                            player.up()
                        } else {
                            continue;
                        }
                    }
                    A => {
                        if player.pos_x - 1 > 0
                            && game.at((player.pos_x - 1, player.pos_y)) != PLAYER
                        {
                            player.left()
                        } else {
                            continue;
                        }
                    }
                    S => {
                        if player.pos_y + 1 < game.max_y
                            && game.at((player.pos_x, player.pos_y + 1)) != PLAYER
                        {
                            player.down()
                        } else {
                            continue;
                        }
                    }
                    D => {
                        if player.pos_x + 1 < game.max_x
                            && game.at((player.pos_x + 1, player.pos_y)) != PLAYER
                        {
                            player.right()
                        } else {
                            continue;
                        }
                    }
                    _ => unreachable!(),
                };
                player.update();

                if game.at(player.xy()) == FOOD {
                    player.grow(prev);
                    game.new_food();
                } else {
                    game.to_empty(prev);
                }
                game.update(&player.body);

                if !game.can_move(&player.xy()) {
                    break if game.over() { "You Win" } else { "You Lose" };
                }
            } else {
                continue;
            }
        }
        prev_keys = keys;
    };
    println!("\n{}", ending);
}

//  Begin `run` with newly initalized game with specified size
fn main() {
    run(&mut new_game(10, 10));
}
