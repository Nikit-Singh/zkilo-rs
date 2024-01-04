use std::io::{self, Error, Read, Write};
use std::os::fd::{AsRawFd, RawFd};
use std::process::exit;

use termios::*;

struct EditorConfig {
    screenrows: u16,
    screencols: u16,
    term: Termios,
    fd: RawFd,
}

impl Default for EditorConfig {
    fn default() -> Self {
        let fd = io::stdin().as_raw_fd();
        let mut raw: Termios = Termios::from_fd(fd).unwrap();
        tcgetattr(fd, &mut raw).unwrap();

        let (screenrows, screencols) = get_window_size().unwrap();

        EditorConfig {
            screenrows,
            screencols,
            term: raw,
            fd,
        }
    }
}

fn get_window_size() -> Option<(u16, u16)> {
    let mut winsize = libc::winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let mut out = io::stdout();

    unsafe {
        // if libc::ioctl(out.as_raw_fd(), libc::TIOCGWINSZ, &mut winsize) == -1 {
            if out.write(b"\x1b[999C\x1b[999B").unwrap() != 12 {
                return None;
            }
            editor_read_key();
        // }
    }

    Some((winsize.ws_row, winsize.ws_col))
}

fn ctrl_key(k: char) -> u8 {
    k as u8 & 0x1f //00011111
}

fn enable_raw_mode(cfg: &EditorConfig) -> Result<(), Error> {
    let mut raw = cfg.term;
    tcgetattr(cfg.fd, &mut raw)?;
    raw.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    raw.c_cflag |= CS8;
    raw.c_oflag &= !(OPOST);
    raw.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 1;
    tcsetattr(cfg.fd, TCSAFLUSH, &mut raw)?;

    Ok(())
}

fn disable_raw_mod(raw: &Termios) -> Result<(), Error> {
    let mut raw = raw;
    tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, &mut raw)?;
    Ok(())
}

fn editor_read_key() -> char {
    let byte = io::stdin().by_ref().bytes().next();

    match byte {
        Some(ch) => ch.ok().unwrap() as char,
        None => '\0',
    }
}

fn editor_process_key(cfg: &EditorConfig) {
    let c = editor_read_key();
    let ctrl_q = ctrl_key('q');

    match c as u8 {
        x if x == ctrl_q => {
            term_refresh();
            disable_raw_mod(&cfg.term).unwrap();
            exit(0);
        }
        d => println!("{}", d as char),
    }
}

fn term_refresh() {
    let mut out = io::stdout();
    let out = out.by_ref();
    out.write(b"\x1b[2J").unwrap();
    out.write(b"\x1b[H").unwrap();
}

fn editor_refresh_screen(cfg: &EditorConfig) {
    let mut out = io::stdout();
    let out = out.by_ref();
    out.write(b"\x1b[2J").unwrap();
    out.write(b"\x1b[H").unwrap();

    editor_draw_rows(cfg);

    out.write(b"\x1b[H").unwrap();
}

fn editor_draw_rows(cfg: &EditorConfig) {
    for _ in 0..cfg.screenrows {
        io::stdout().write(b"~\r\n").unwrap();
    }
}

fn main() {
    let cfg = EditorConfig::default();
    enable_raw_mode(&cfg).unwrap();
    loop {
        editor_refresh_screen(&cfg);
        editor_process_key(&cfg);
    }
}
