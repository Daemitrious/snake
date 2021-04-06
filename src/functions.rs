use crate::{
    game::Game,
    packages::{
        Area, Coordinates, DeviceQuery, DeviceState, Dimensions, End, Keycode, Lose, Rng, Term,
        Win, A, BORDER, D, FOOD, KEYS, PLAYER, S, W,
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

//  Create a new `Player` struct with a set of random coordinates
pub fn new_player((max_x, max_y): Dimensions) -> Player {
    let pos_x = randint(1..max_x);
    let pos_y = randint(1..max_y);

    Player {
        pos_x: pos_x,
        pos_y: pos_y,
        body: vec![(pos_x, pos_y)],
    }
}

//  Select a `usize` integer from a specified Range
pub fn randint(range: std::ops::Range<usize>) -> usize {
    rand::thread_rng().gen_range(range)
}

fn overprint(content: String, tsize: usize) {
    drop(Term::stdout().clear_last_lines(tsize));
    println!("{}", content);
}

// Clear the terminal then print `array` in grid format
pub fn refresh(area: &Area) {
    let mut bag: Vec<String> = Vec::with_capacity(area.len());

    for v in area.iter() {
        bag.push(v.into_iter().map(|c| c.to_string() + " ").collect())
    }
    overprint(bag.join("\n"), area.len())
}

//  Initialize keyboard input then begin game loop
pub fn run(columns: usize, rows: usize) -> End {
    //  Initialize game board
    let mut game = new_game(columns, rows);

    //  Assign player from `PLAYER` struct
    let mut player: Player = new_player(game.size());
    let mut prev: Coordinates;

    //  Keyboard inputs
    let input = DeviceState::new();

    let mut _held_key: Vec<Keycode> = Vec::with_capacity(1);
    let mut prev_key: Vec<Keycode> = Vec::with_capacity(1);

    let mut key: &Keycode;

    // `area` setup // Might change
    game.to_player(player.xy());
    game.new_food();
    game.update(&player.body);

    //  Begin loop
    loop {
        _held_key = input.get_keys();
        //println!("{}", _held_key[0]);

        if _held_key.len() > 0 && _held_key != prev_key {
            key = &_held_key[0];

            if KEYS.contains(key) {
                prev = player.body[player.body.len() - 1];

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
                    break if game.over() { Win } else { Lose };
                }
            } else {
                continue;
            }
        }
        prev_key = _held_key;
    }
}
