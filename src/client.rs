use std::net::TcpStream;

use crate::{player::Player, Lock};

pub struct Client {
    stream: Lock<TcpStream>,
    player: Lock<Player>,
    id: u64,
}

impl Client {
    fn stream(&self) -> Lock<TcpStream> {
        self.stream.clone()
    }

    fn player(&self) -> Lock<Player> {
        self.player.clone()
    }

    const fn id(&self) -> u64 {
        self.id
    }
}
