use mcfind::{slime, world::PlayerPos};
use std::num::ParseIntError;
use thiserror::Error;

pub fn run() -> Result<(), Error> {
    let command = Command::create_from_args()?;
    command.execute();

    Ok(())
}

pub const fn usage() -> &'static str {
    "\
usage: mcfind <command> [<args>]

    slime <seed> <x> <z> [<chunk radius>]
"
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("no command specified")]
    NoCommand,
    #[error("missing argument `{0}`")]
    MissingArgumentError(&'static str),
    #[error("argument was not an integer")]
    ParseError(#[from] ParseIntError),
}

enum Command {
    Slime {
        seed: i64,
        pos: PlayerPos,
        radius: Option<u32>,
    },
}

impl Command {
    fn create_from_args() -> Result<Command, Error> {
        let mut args = std::env::args();
        args.next();

        match args.next() {
            Some(s) if s.eq_ignore_ascii_case("slime") => {
                let seed = args
                    .next()
                    .map(|s| s.parse())
                    .ok_or(Error::MissingArgumentError("seed"))??;

                let x = args
                    .next()
                    .map(|s| s.parse())
                    .ok_or(Error::MissingArgumentError("x"))??;

                let z = args
                    .next()
                    .map(|s| s.parse())
                    .ok_or(Error::MissingArgumentError("z"))??;

                let radius = args.next().map(|s| s.parse()).transpose().unwrap_or(None);

                Ok(Command::Slime {
                    seed,
                    pos: PlayerPos::new(x, 0, z),
                    radius,
                })
            }
            _ => Err(Error::NoCommand),
        }
    }

    fn execute(self) {
        match self {
            Command::Slime { seed, pos, radius } => {
                let chunks = slime::nearby_slimes(
                    seed,
                    pos,
                    radius.unwrap_or_else(|| {
                        println!("Info: using default radius 2\n");
                        2
                    }),
                );

                for chunk in chunks {
                    println!("Slime chunk from {}", chunk.display_world_coords());
                }
            }
        }
    }
}
