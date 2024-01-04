use std::io::{stdin, Write};
use std::io::{self, Error, Read};
use std::os::fd::AsRawFd;
use std::process::exit;

use termios::*;

// #define CTRL_KEY(k) ((k) & 0x1f)
fn ctrl_key(k: char) -> u8 {
    k as u8 & 0x1f //00011111
}

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

fn disable_raw_mod(raw: &Termios) -> Result<(), Error> {
    let mut raw = raw;
    tcsetattr(stdin().as_raw_fd(), TCSAFLUSH, &mut raw)?;
    Ok(())
}

fn editor_read_key() -> char {
    let byte = stdin().by_ref().bytes().next();

    match byte {
        Some(ch) => ch.ok().unwrap() as char,
        None => '\0',
    }
}

fn editor_process_key(raw: &Termios) {
    let c = editor_read_key();
    let ctrl_q = ctrl_key('q');

    match c as u8 {
        x if x == ctrl_q => {
            editor_refresh_screen();
            disable_raw_mod(&raw).unwrap();
            exit(0);
        },
        d => println!("{}", d as char),
    }
}

fn editor_refresh_screen() {
    let mut out = io::stdout();
    let out = out.by_ref();
    out.write(b"\x1b[2J").unwrap();
    out.write(b"\x1b[H").unwrap();
}

fn main() {
    let raw = enable_raw_mode().unwrap();
    loop {
        editor_refresh_screen();
        editor_process_key(&raw);
    }
}
