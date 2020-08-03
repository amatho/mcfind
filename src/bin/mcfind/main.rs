mod cli;

use cli::Error;
use std::error::Error as StdError;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {}", e);

        chain(&e)
            .skip(1)
            .take(1)
            .for_each(|c| eprintln!("\nCaused by:\n    {}", c));

        match e {
            Error::NoCommand | Error::MissingArgumentError(_) => println!("\n{}", cli::usage()),
            _ => (),
        }
    }
}

fn chain(error: &dyn StdError) -> Chain {
    Chain { next: Some(error) }
}

struct Chain<'a> {
    next: Option<&'a dyn StdError>,
}

impl<'a> Iterator for Chain<'a> {
    type Item = &'a dyn StdError;

    fn next(&mut self) -> Option<Self::Item> {
        let error = self.next?;
        self.next = error.source();
        Some(error)
    }
}
