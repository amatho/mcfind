mod cli;

use cli::Error as CliError;
use std::error::Error;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {}", e);

        e.chain()
            .skip(1)
            .take(1)
            .for_each(|c| eprintln!("\nCaused by:\n    {}", c));

        match e {
            CliError::NoCommand | CliError::MissingArgumentError(_) => {
                println!("\n{}", cli::usage())
            }
            _ => (),
        }
    }
}

trait ErrorChain {
    fn chain(&self) -> Chain;
}

impl<T: Error> ErrorChain for T {
    fn chain(&self) -> Chain {
        Chain { next: Some(self) }
    }
}

struct Chain<'a> {
    next: Option<&'a dyn Error>,
}

impl<'a> Iterator for Chain<'a> {
    type Item = &'a dyn Error;

    fn next(&mut self) -> Option<Self::Item> {
        let error = self.next?;
        self.next = error.source();
        Some(error)
    }
}
