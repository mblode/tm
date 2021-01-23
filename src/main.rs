// mod cli;

// use std::io::{Read, Write, Error};
// use std::env;
// use std::fs::File;

use termion::screen::AlternateScreen;
use std::io::{Write, stdout};

fn main() {
    // let matches = cli::build_cli().get_matches();

    // if let Some(i) = matches.value_of("SESSION") {
    //     let current_dir = env::current_dir().unwrap();
    //     println!("Value for input: {}", i);
    //     println!("Current dir: {:#?}", current_dir);
    //     println!("Terminal size: {:#?}", termion::terminal_size());
    //     println!("Terminal size: {:#?}", termion::terminal_size());
    //     println!("Terminal size: {:#?}", termion::terminal_size());
    //     println!("Terminal size: {:#?}", termion::terminal_size());
    // }

    {
        let mut screen = AlternateScreen::from(stdout());
        write!(screen, "Writing to alternate screen!").unwrap();
        screen.flush().unwrap();
    }
    println!("Writing to main screen.");

}
