use std::net::TcpStream;

use {
    snake::{game::Game, lock, Lock, Movement},
    std::{
        io::{Read, Result},
        net::TcpListener,
        thread::{sleep, spawn},
    },
};

pub fn handle_client(
    stream: Lock<TcpStream>,
    movement: &mut Movement,
    open: Lock<bool>,
) -> Result<()> {
    loop {
        if let Ok(guard_open) = open.read() {
            if *guard_open {
                drop(guard_open);

                if let Ok(guard_game) = game.read() {
                    if let Some(stream) = guard_game.get_stream(id) {
                        if let Ok(mut guard_stream) = stream.write() {
                            let mut buf = [0; 1];
                            guard_stream.read_exact(&mut buf)?;
                            drop(guard_stream);

                            if let Some(movement) = Movement::from_u8(buf[0]) {
                                if let Some(player) = guard_game.get_player(id) {
                                    if let Ok(mut guard_player) = player.write() {
                                        guard_player.direction = movement
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() -> Result<()> {
    //  Initiate TcpListener
    let listener = TcpListener::bind("127.0.0.1:8888")?;

    let game = lock(Game::default());
    let open = lock(false);

    let thread_game = game.clone();
    let thread_open = open.clone();

    let _timer = spawn(move || 'timer: loop {
        println!("Writing...");
        if let Ok(mut guard_game) = thread_game.write() {
            println!("Done\nUpdating");
            guard_game.update();
            println!("Done\nWriting");

            drop(guard_game);

            if let Ok(mut guard_open) = thread_open.write() {
                *guard_open = false;

                drop(guard_open);

                if let Ok(mut guard_game) = thread_game.write() {
                    if let Err(error) = guard_game.distribute() {
                        println!("{:?}", error);
                        break 'timer;
                    }
                }
                println!("Done\n");

                if let Ok(mut guard_open) = thread_open.write() {
                    *guard_open = true;
                }
            }
        }
        sleep(Game::TIMER);
    });

    for stream in listener.incoming().filter_map(Result::ok) {
        if let Ok(guard_game) = game.read() {
            if let Some(p) = guard_game.area().vacancy() {
                drop(guard_game);

                if let Ok(mut guard_game) = game.write() {
                    if let Ok((id, stream)) = guard_game.setup_client(p, stream) {
                        drop(guard_game);

                        let thread_game = game.clone();
                        let thread_open = open.clone();

                        spawn(move || {
                            handle_client(stream, thread_open).unwrap_err();
                            if let Ok(mut guard_game) = thread_game.write() {
                                guard_game.remove_client(id)
                            }
                        });
                    }
                }
            }
        }
    }
    Ok(())
}
