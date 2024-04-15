use std::io::{stdin, stdout, Write};
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use timerfd::{ClockId, SetTimeFlags, TimerFd, TimerState};

#[derive(PartialEq, Debug)]
enum Press {
    S,
    T,
    R,
    A,
    O,
    I,
    Y,
    E,
}

#[derive(PartialEq, Debug)]
enum TwoChord {
    C,
    F,
    J,
    N,
    G,
    U,
}

trait PushIfNew {
    fn push_if_new(&mut self, p: Press);
}

impl PushIfNew for Vec<Press> {
    fn push_if_new(&mut self, p: Press) {
        if !self.contains(&p) {
            self.push(p)
        }
    }
}

trait FromPress {
    fn from_press(left: Press, right: Press) -> Self;
}

impl FromPress for TwoChord {
    fn from_press(left: Press, right: Press) -> Self {
        match (left, right) {
            (Press::E, Press::Y) | (Press::Y, Press::E) => TwoChord::C,
            (Press::A, Press::R) | (Press::R, Press::A) => TwoChord::F,
            (Press::T, Press::S) | (Press::S, Press::T) => TwoChord::J,
            (Press::I, Press::O) | (Press::O, Press::I) => TwoChord::N,
            (Press::R, Press::T) | (Press::T, Press::R) => TwoChord::G,
            (Press::Y, Press::I) | (Press::I, Press::Y) => TwoChord::U,
            _ => TwoChord::C,
        }
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut i = 1;

    // clear screen on startup
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut tfd = TimerFd::new_custom(ClockId::Monotonic, false, false).unwrap();
    let mut key_array: Vec<Press> = vec![];

    // each key press fills up the .keys() "buffer"
    for c in stdin.keys() {
        // set timer
        match tfd.get_state() {
            TimerState::Disarmed => {
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1, 5),
                    termion::clear::CurrentLine,
                )
                .unwrap();
                stdout.flush().unwrap();

                write!(stdout, "{}", termion::cursor::Goto(i, 1),).unwrap();
                stdout.flush().unwrap();

                tfd.set_state(
                    TimerState::Oneshot(Duration::from_millis(500)),
                    SetTimeFlags::Default,
                );
                // pop all keys from key_array and check chord
                match key_array.len() {
                    1 => print!("{:#?}", key_array.get(0).unwrap()),
                    2 => {
                        print!(
                            "{:#?}",
                            TwoChord::from_press(key_array.remove(0), key_array.remove(0))
                        );
                    }
                    _ => (),
                }
                key_array.clear();
            }
            _ => {}
        }

        i += 1;

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('w') => key_array.push_if_new(Press::S),
            Key::Char('e') => key_array.push_if_new(Press::T),
            Key::Char('r') => key_array.push_if_new(Press::R),
            Key::Char('t') => key_array.push_if_new(Press::A),
            Key::Char('s') => key_array.push_if_new(Press::O),
            Key::Char('d') => key_array.push_if_new(Press::I),
            Key::Char('f') => key_array.push_if_new(Press::Y),
            Key::Char('g') => key_array.push_if_new(Press::E),
            Key::Char('x') | Key::Backspace => {
                // key_array.push_if_new(Press::BS);
                // clear whole screen
                write!(stdout, "{}", termion::clear::All,).unwrap();

                i = 1;
            }
            _ => (),
        }

        write!(stdout, "{}", termion::cursor::Goto(1, 5),).unwrap();
        stdout.flush().unwrap();

        println!("{:?}", key_array);

        // move to previous cursor position
        write!(stdout, "{}", termion::cursor::Goto(i, 1),).unwrap();
        stdout.flush().unwrap();
    }
}
