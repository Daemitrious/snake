use {
    crate::{area::Area, lock, player::Player, Lock, Object},
    rand::{prelude::IteratorRandom, thread_rng},
    std::{
        collections::{hash_map::DefaultHasher, HashMap},
        hash::{Hash, Hasher},
        io::{Error, Write},
        net::TcpStream,
        time::Duration,
    },
};

#[derive(Default)]
#[non_exhaustive]
pub struct Game {
    pub players: HashMap<u64, Lock<Player>>,
    pub streams: HashMap<u64, Lock<TcpStream>>,
    area: Area,
    len: usize,
}

impl Game {
    pub const TIMER: Duration = Duration::from_millis(250);

    pub fn get_player(&self, id: u64) -> Option<&Lock<Player>> {
        self.players.get(&id)
    }

    pub fn get_stream(&self, id: u64) -> Option<&Lock<TcpStream>> {
        self.streams.get(&id)
    }

    pub fn update(&mut self) {
        for player in self.players.values() {
            if let Ok(guard_player) = player.read() {
                if self.area.can_move(&guard_player) {
                    let index = self.area.next_move(&guard_player);
                    drop(guard_player);

                    if let Ok(mut guard_player) = player.write() {
                        if self.area.data()[index] == Object::Food as u8 {
                            guard_player.grow(index);
                            self.area.set(index, Object::Player)
                        } else {
                            self.area.set(guard_player.tail(), Object::Space);
                            guard_player.translate(index);
                            self.area.set(index, Object::Player)
                        }
                    }
                }
            }
        }
    }

    pub fn distribute(&mut self) -> Result<(), Error> {
        println!("\nDistributing");
        for stream in self.streams.values() {
            if let Ok(mut guard_stream) = stream.write() {
                println!("{:?}", guard_stream);

                guard_stream.write_all(&self.area.info())?;
                guard_stream.flush()?
            }
        }
        println!("Done\n");
        Ok(())
    }

    pub const fn area(&self) -> &Area {
        &self.area
    }

    pub fn spawn_food(&mut self) {
        if let Some(index) = (0..self.area.data().len())
            .filter(|i| self.area[*i] == Object::Space as u8)
            .choose(&mut thread_rng())
        {
            self.area.set(index, Object::Food);
        }
    }

    pub fn setup_client(
        &mut self,
        p: usize,
        mut stream: TcpStream,
    ) -> Result<(u64, Lock<TcpStream>), Error> {
        stream.write_all(&self.area.info())?;
        stream.flush()?;

        self.len += 1;

        let mut hasher = DefaultHasher::new();
        self.len.hash(&mut hasher);
        let id = hasher.finish();

        // stream.set_nonblocking(true)?;
        // stream.set_read_timeout(Some(Duration::from_millis(100)))?;

        let stream = lock(stream);

        self.players.insert(id, lock(Player::new(p)));
        self.streams.insert(id, stream.clone());

        Ok((id, stream))
    }

    pub fn remove_client(&mut self, id: u64) {
        if let Some(player) = self.players.remove(&id) {
            self.streams.remove(&id);

            if let Ok(guard_player) = player.read() {
                for index in guard_player.data.iter() {
                    self.area.set(*index, Object::Space)
                }
            }
        }
    }
}
