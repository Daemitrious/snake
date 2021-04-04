use keyboard_query::{DeviceQuery, DeviceState};
use rand::Rng;

const EMPTY: &str = " ";
const BORDER: &str = "X";
const PLAYER: &str = "o";

const W: u16 = 87;
const A: u16 = 65;
const S: u16 = 83;
const D: u16 = 68;

const WASD: [u16; 4] = [W, A, S, D];

type Area = Vec<Vec<&'static str>>;
type Dimensions = (usize, usize);
type Coordinates = (usize, usize);

struct Game {
    max_x: usize,
    max_y: usize,
    area: Area,
}

struct Player {
    pos_x: usize,
    pos_y: usize,
}

impl Game {
    fn to_empty(&mut self, (pos_x, pos_y): Coordinates) {
        self.area[pos_y][pos_x] = EMPTY
    }
    fn to_player(&mut self, (pos_x, pos_y): Coordinates) {
        self.area[pos_y][pos_x] = PLAYER
    }

    fn update(&self) {
        refresh(&self.area)
    }
    fn size(&self) -> Dimensions {
        (self.max_x, self.max_y)
    }
}

impl Player {
    fn up(&mut self) {
        self.pos_y -= 1
    }
    fn left(&mut self) {
        self.pos_x -= 1
    }
    fn down(&mut self) {
        self.pos_y += 1
    }
    fn right(&mut self) {
        self.pos_x += 1
    }

    fn xy(&self) -> Coordinates {
        (self.pos_x, self.pos_y)
    }
}

fn check(num: u16) -> bool {
    for int in WASD.iter() {
        if num == *int {
            return true;
        }
    }
    false
}

fn randint(range: std::ops::Range<usize>) -> usize {
    rand::thread_rng().gen_range(range)
}

fn new_player((max_x, max_y): Dimensions) -> Player {
    Player {
        pos_x: randint(1..max_x),
        pos_y: randint(1..max_y),
    }
}

fn new_game(columns: usize, rows: usize) -> Game {
    let columns = columns + 2;
    let rows = rows + 2;

    let max_x = columns - 1;
    let max_y = rows - 1;

    let mut area: Area = (0..rows)
        .map(|_| (0..columns).map(|_| " ").collect())
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

fn run(game: &mut Game) {
    //  Assign first player
    let mut player: Player = new_player(game.size());

    //  Keyboard inputs
    let input = DeviceState::new();
    let mut prev_keys = Vec::with_capacity(1);

    //  Setup first screen
    game.to_player(player.xy());
    game.update();

    //  Begin loop
    loop {
        let keys = input.get_keys();

        if keys != prev_keys && keys.len() == 1 {
            let key = keys[0];

            if check(key) {
                game.to_empty(player.xy());

                match key {
                    W => {
                        if player.pos_y - 1 > 0 {
                            player.up();
                        }
                    }

                    A => {
                        if player.pos_x - 1 > 0 {
                            player.left();
                        }
                    }

                    S => {
                        if player.pos_y + 1 < game.max_y {
                            player.down();
                        }
                    }

                    D => {
                        if player.pos_x + 1 < game.max_x {
                            player.right()
                        }
                    }
                    _ => unreachable!(),
                }

                game.to_player(player.xy());
                game.update();
            } else {
                continue;
            }
        }
        prev_keys = keys;
    }
}

fn main() {
    run(&mut new_game(5, 5));
}
