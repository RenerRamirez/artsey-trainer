use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    //setting up stdout and going into raw mode

    let mut stdout = stdout().into_raw_mode().unwrap();

    //detecting keydown events
    let mut i = 1;
    write!(
        stdout, 
        "{}", 
        termion::clear::All
    )
    .unwrap();
    stdout.flush().unwrap();
    write!(
        stdout,
        "{}",
        termion::cursor::Goto(i, 1),
    )
        .unwrap();
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}",
            termion::cursor::Goto(i, 1),
        )
        .unwrap();
        i += 1;

        //i reckon this speaks for itself
        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Char('w') => print!("s"),
            Key::Char('e') => print!("t"),
            Key::Char('r') => print!("r"),
            Key::Char('t') => print!("a"),
            Key::Char('s') => {
                print!("o"),
            }
            Key::Char('d') => print!("i"),
            Key::Char('f') => print!("y"),
            Key::Char('g') => print!("e"),
            Key::Char('x') => {
                i = 1;
                write!( stdout, "{}", termion::clear::All,) .unwrap();
                write!( stdout, "{}", termion::cursor::Goto(i, 1),) .unwrap();
            }
            Key::Backspace => {
                i = 1;
                write!(
                    stdout, 
                    "{}", 
                    termion::clear::All,
                )
                .unwrap();
                write!(
                    stdout,
                    "{}",
                    termion::cursor::Goto(i, 1),
                )
                .unwrap();
            }
            _ => (),
        }

        stdout.flush().unwrap();
    }
}
