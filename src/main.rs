use mcfind::cli::{self, CliError};
use std::error::Error;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {}", e);
        chain(&e)
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

fn chain(error: &dyn Error) -> ErrorChain {
    ErrorChain { next: Some(error) }
}

struct ErrorChain<'a> {
    next: Option<&'a dyn Error>,
}

impl<'a> Iterator for ErrorChain<'a> {
    type Item = &'a dyn Error;

    fn next(&mut self) -> Option<Self::Item> {
        let error = self.next?;
        self.next = error.source();
        Some(error)
    }
}
