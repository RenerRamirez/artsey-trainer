use std::io::{stdin, stdout, Write};
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use timerfd::{TimerFd, TimerState, SetTimeFlags};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut tfd = TimerFd::new().unwrap();

    let mut i = 1;
    write!(stdout, "{}", termion::clear::All).unwrap();
    write!(stdout, "{}", termion::cursor::Goto(i, 1)).unwrap();
    stdout.flush().unwrap();

    // each key press fills up the .keys() "buffer"
    tfd.set_state(TimerState::Oneshot(Duration::new(0, 500)), SetTimeFlags::Default);
    for c in stdin.keys() {
        write!(stdout, "{}", termion::cursor::Goto(1, 1),).unwrap();
        stdout.flush().unwrap();

        print!("Keys pressed: [ ");
        match c {
            Ok(Key::Char(k)) => print!("{k}"),
            _ => (),
        }
        println!(" ] ");
        
        match tfd.get_state() {
            TimerState::Oneshot(d) => println!("Remaining: {:?}", d),
            _ => (),
        }

        // move cursor to top-left of the terminal 
        write!(stdout, "{}", termion::cursor::Goto(i, 5),).unwrap();

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
                write!(stdout, "{}", termion::cursor::Goto(i, 5)).unwrap();
            }
            Key::Backspace => {
                i = 1;
                write!(stdout, "{}", termion::clear::All,).unwrap();
                write!(stdout, "{}", termion::cursor::Goto(i, 5)).unwrap();
            }
            _ => (),
        }

        stdout.flush().unwrap();
    }
}
