use std::io::stdin;
use std::os::fd::AsRawFd;
use std::io::{self, Read, Error};

use termios::*;

fn enable_raw_mode() -> Result<Termios, Error> {
    let fd = stdin().as_raw_fd();
    let mut raw = Termios::from_fd(fd)?;
    let orig_raw = raw;

    tcgetattr(fd, &mut raw)?;
    raw.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    raw.c_cflag |= CS8;
    raw.c_oflag &= !(OPOST);
    raw.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 1;
    tcsetattr(fd, TCSAFLUSH, &mut raw)?;

    Ok(orig_raw)
}

fn disable_raw_mod(raw: Termios) -> Result<(), Error> {
    let mut raw = raw;
    tcsetattr(stdin().as_raw_fd(), TCSAFLUSH, &mut raw)?;
    Ok(())
}

fn main() {
    let mut stdin = io::stdin();
    let orig_raw = enable_raw_mode().unwrap();

    loop {
        let byte = stdin.by_ref().bytes().next();

        let c = match byte {
            Some(ch) => ch.ok().unwrap() as char,
            None => '\0',
        };

        if c.is_ascii_control() {
            println!("{}\r", c as u8);
        } else {
            println!("{},({})\r", c as u8, c);
        }

        if c == 'q' {
            break;
        }
    }

    disable_raw_mod(orig_raw).unwrap();
}
