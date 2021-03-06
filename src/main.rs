mod cli;

use std::io::{stdout, Write, Error, Stdout};
use crossterm::{
    QueueableCommand,
    cursor::{
        Show,
        Hide,
        MoveTo,
    },
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Colorize, Print, PrintStyledContent},
    terminal::{
        EnterAlternateScreen, 
        LeaveAlternateScreen,
        enable_raw_mode,
        disable_raw_mode,
        Clear,
        ClearType,
        size,
    }
};

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(i) = matches.value_of("SESSION") {
        println!("Session: {}", i);
        run(); 
    }
}

fn run () {
    let mut stdout = stdout();

    enable_raw_mode().expect("failed to enable raw mode");

    stdout
        .queue(Hide).unwrap()
        .queue(EnterAlternateScreen).unwrap()
        .queue(Clear(ClearType::All)).unwrap();

    stdout.flush().unwrap();

    let terminal_size = crossterm::terminal::size().unwrap();

    loop {

        draw(&mut stdout, width / 2, height / 2).unwrap();
        draw(&mut stdout, width / 2, height / 2).unwrap();
        draw(&mut stdout, width / 2, height).unwrap();

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            }) => break,
            _ => (),
        }
    }

    stdout
        .queue(Show).unwrap()
        .queue(LeaveAlternateScreen).unwrap();

    stdout.flush().unwrap();

    disable_raw_mode().unwrap();
}

fn draw(stdout: &mut Stdout, width: u16, height: u16) -> Result<(), Error> {
    stdout
        .queue(MoveTo(0, 0)).unwrap()
        .queue(PrintStyledContent("❯ ".green())).unwrap()
        .queue(Print(format!("{:width$}", "", width=width as usize))).unwrap();

    stdout.flush().unwrap();

    Ok(())
}
