use crate::{
    game::Game,
    packages::{
        sleep, Area, Coordinates, DeviceQuery, DeviceState, Dimensions, Keycode, Rng, Term, A,
        BORDER, D, EMPTY, FOOD, KEYS, S, W, WAIT,
    },
    player::Player,
};

//  Create a new `Game` struct with a specified `Area` size
pub fn new_game(columns: usize, rows: usize) -> Game {
    let columns = columns + 2;
    let rows = rows + 2;

    let max_x = columns - 1;
    let max_y = rows - 1;

    let mut area: Area = (0..rows)
        .map(|_| (0..columns).map(|_| EMPTY).collect())
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
    Game { max_x, max_y, area }
}

//  Create a new `Player` struct with a set of random coordinates
pub fn new_player((max_x, max_y): Dimensions) -> Player {
    let pos_x = randint(1..max_x);
    let pos_y = randint(1..max_y);

    Player {
        max_x,
        max_y,
        pos_x,
        pos_y,
        body: vec![(pos_x, pos_y)],
    }
}

//  Select a `usize` integer from a specified Range
pub fn randint(range: std::ops::Range<usize>) -> usize {
    rand::thread_rng().gen_range(range)
}

// Clear the terminal then print `array` in grid format
pub fn refresh(area: &Area) {
    let mut bag: Vec<String> = Vec::with_capacity(area.len());

    for v in area.iter() {
        bag.push(v.into_iter().map(|c| c.to_string() + " ").collect())
    }
    Term::stdout().clear_last_lines(area.len()).unwrap();
    println!("{}", bag.join("\n"));
}

//  Initialize keyboard input then begin game loop
pub fn run() -> bool {
    let (rows, columns) = (randint(5..15), randint(5..15));

    //  Clear screen buffer before-hand
    Term::stdout().clear_screen().unwrap();

    //  Initialize game board
    let mut game = new_game(columns, rows);

    //  Assign player from `PLAYER` struct
    let mut player: Player = new_player(game.size());

    //  Initiate previous coordinate variable for player
    let mut prev: Coordinates;

    //  Keyboard inputs
    let input = DeviceState::new();

    //  Initiate keyboard event arrays
    let mut _held_key: Vec<Keycode> = Vec::with_capacity(1);
    let mut prev_key: Vec<Keycode> = Vec::with_capacity(1);

    //  Initiate current key variable
    let mut key: Keycode;

    //  Setup game
    game.to_player(player.head());
    game.new_food();
    game.update(&player.body);

    let mut direction = if player.max_x > player.max_y {
        if player.pos_x < player.max_x / 2 {
            D
        } else {
            A
        }
    } else {
        if player.pos_y < player.max_y / 2 {
            S
        } else {
            W
        }
    };

    //  Begin loop
    loop {
        for _ in 0..10 {
            _held_key = input.get_keys();

            //  Check for any keyboard events
            if !_held_key.is_empty() && _held_key != prev_key {
                key = _held_key[0].clone();

                //  Check if event is for movement
                if KEYS.contains(&key) {
                    if !(direction == W && key == S
                        || direction == S && key == W
                        || direction == A && key == D
                        || direction == D && key == A)
                        || player.body.len() == 1
                    {
                        direction = key
                    }
                } else {
                    continue;
                }
            } else {
                sleep(WAIT);
            }
            prev_key = _held_key;
        }
        prev = player.tail();

        match direction {
            W => {
                if player.can_up(player.pos_y - 1) {
                    player.up()
                } else {
                    break false;
                }
            }
            A => {
                if player.can_left(player.pos_x - 1) {
                    player.left()
                } else {
                    break false;
                }
            }
            S => {
                if player.can_down(player.pos_y + 1) {
                    player.down()
                } else {
                    break false;
                }
            }
            D => {
                if player.can_right(player.pos_x + 1) {
                    player.right()
                } else {
                    break false;
                }
            }
            _ => unreachable!(),
        };
        player.update();

        //  Check if player ate food
        if game.at(player.head()) == FOOD {
            player.grow(prev);
            game.new_food();
        } else {
            game.to_empty(prev);
        }
        game.update(&player.body);

        //  Check if player can't move
        if !game.can_move(&player.head()) {
            break if game.is_over() { true } else { false };
        }
    }
}
