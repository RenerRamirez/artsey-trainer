use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut i = 1;
    write!(stdout, "{}", termion::clear::All).unwrap();
    write!(stdout, "{}", termion::cursor::Goto(i, 1)).unwrap();
    stdout.flush().unwrap();

    // each key press fills up the .keys() "buffer"
    for c in stdin.keys() {
        // move cursor to top-left of the terminal 
        write!(stdout, "{}", termion::cursor::Goto(i, 1),).unwrap();

        i += 1;

        match c.unwrap() {
            Key::Ctrl('q') => {
                write!(stdout, "{}", termion::clear::All).unwrap();
                write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
                break;
            }
            Key::Char('w') => print!("s"),
            Key::Char('e') => print!("t"),
            Key::Char('r') => print!("r"),
            Key::Char('t') => print!("a"),
            Key::Char('s') => print!("o"),
            Key::Char('d') => print!("i"),
            Key::Char('f') => print!("y"),
            Key::Char('g') => print!("e"),
            Key::Char('x') => {
                // clear whole screen
                i = 1;
                write!(stdout, "{}", termion::clear::All,).unwrap();
                write!(stdout, "{}", termion::cursor::Goto(i, 1)).unwrap();
            }
            Key::Backspace => {
                i = 1;
                write!(stdout, "{}", termion::clear::All,).unwrap();
                write!(stdout, "{}", termion::cursor::Goto(i, 1)).unwrap();
            }
            _ => (),
        }

        stdout.flush().unwrap();
    }
}
