use std::io::{Error, stdin};
use std::io::{Read, self};
use std::os::fd::AsRawFd;

use termios::*;

fn disable_raw_mod(raw: Termios) -> Result<(), Error> {
    let mut raw = raw;
    tcsetattr(stdin().as_raw_fd(), TCSAFLUSH, &mut raw)?;
    Ok(())
}

fn enable_raw_mode() -> Result<Termios, Error> {
    let fd  = stdin().as_raw_fd();
    let mut raw = Termios::from_fd(fd)?;
    let orig_raw = raw;

    tcgetattr(fd, &mut raw)?;
    raw.c_lflag &= !ECHO;
    tcsetattr(fd, TCSAFLUSH, &mut raw)?;

    Ok(orig_raw) 
}

fn main() {
    let mut stdin = io::stdin();
    let orig_raw = enable_raw_mode().unwrap();

    let mut c = [0; 1];
    while stdin.read(&mut c).unwrap() == 1 {
        if c[0] == b'q' {
            break;
        }
    }

    disable_raw_mod(orig_raw).unwrap();
}