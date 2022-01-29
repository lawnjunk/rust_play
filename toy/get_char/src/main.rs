extern crate raw_tty;
use std::os::unix::io::AsRawFd;
use std::fs::{File};
use std::process;
use std::time::Duration;
use std::thread::{self,JoinHandle};
use std::sync::mpsc::{self,Receiver};
use std::io::{self,Read,Write,Result,BufReader,BufRead};
use std::rc::Rc;
use raw_tty::{IntoRawMode,GuardMode,TtyModeGuard};

#[derive(Debug)]
struct CharSource {
    buf: Vec<u8>,
    rx: Receiver<u8>,
    handle: Rc::<JoinHandle<()>>,
}

#[derive(Debug,Clone,Copy)]
enum KeyValue {
    Letter(char),
    Esc,
    Tab,
    Enter,
    Backspace,
    Del,
    Insert,
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

#[derive(Debug)]
struct Key {
    value: KeyValue,
    special: Option<KeyValue>,
    ctrl: bool,
    alt: bool,
}

impl CharSource {
    fn new() -> CharSource {
        let (tx, rx) = mpsc::channel::<u8>();

        let handle = thread::spawn(move || {
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            let mut local_buf = [0;10];

            loop {
                if let Ok(read_count) =  stdin.read(&mut local_buf) {
                    write_code("0G");
                    println!("\nread_count: {}", read_count);
                    for i in 0..read_count {
                        tx.send(local_buf[i]).unwrap();
                    }
                    write_code(&format!("0G{:?}", parse_key(&mut local_buf[..read_count], read_count)));
                    println!();
                }
            }
        });

        CharSource {
            buf: vec![],
            rx,
            handle: Rc::new(handle),
        }
    }

    fn read(&mut self) -> Option<u8> {
        loop {
            match self.rx.try_recv() {
                Ok(data) => {
                    self.buf.push(data);
                },
                Err(e) => break,
            }
        }

        return self.buf.pop();
    }
}

fn write_code(code: &str) {
    print!("\x1b[{}", code);
}

fn pares_key_byte(byte: u8, alt: bool) -> Option<Key> {
    if byte == 8 {
        return Some(Key {
            value: KeyValue::Letter((byte + 96) as char),
            special: Some(KeyValue::Backspace),
            ctrl: true,
            alt,
        })
    }
    if byte == 13 {
        return Some(Key {
            value: KeyValue::Letter((byte + 96) as char),
            special: Some(KeyValue::Enter),
            ctrl: true,
            alt,
        })
    }

    if byte == 9 {
        return Some(Key {
            value: KeyValue::Letter((byte + 96) as char),
            special: Some(KeyValue::Tab),
            ctrl: true,
            alt,
        })
    }

    // CTRL + CHAR
    if byte < 27 {
        return Some(Key {
            value: KeyValue::Letter((byte + 96) as char),
            special: None,
            ctrl: true,
            alt,
        })
    }

    // ESCAPE
    if byte == 27 {
        return Some(Key {
            value: KeyValue::Esc,
            special: None,
            ctrl: false,
            alt,
        })
    }

    // BACKSPACE 
    if byte == 127 {
        return Some(Key {
            value: KeyValue::Backspace,
            special: None,
            ctrl: false,
            alt,
        })
    }

    if byte > 31 {
        return Some(Key {
            value: KeyValue::Letter(byte as char),
            special: None,
            ctrl: false,
            alt,
        })
    }
    None
}

fn parse_key(buf: &mut [u8], count: usize) -> Vec<Option<Key>> {
    write_code("0G");
    println!("buf: {:?}", buf);
    let mut result: Vec<Option<Key>> = vec![];

    if count == 0  {
        return result
    }

    if count == 1 {
        result.push(pares_key_byte(buf[0], false));
    }

    if count == 2 {
        if buf[0] == 27 {
            result.push(pares_key_byte(buf[1], true));
        } else {
            result.push(pares_key_byte(buf[0], false));
            result.push(pares_key_byte(buf[1], false));
        }
    }


    let create_page_key = | result: &mut Vec<_>, buf: &[_], starts_with: &[_], value: KeyValue | {
        if buf.ends_with(&[starts_with, &[126]].concat()) {
            result.push(Some(Key {
                value: value,
                special: None,
                ctrl: false,
                alt: false,
            }))
        }

        if buf.starts_with(starts_with) && buf.ends_with(&[59, 53, 126]) {
            result.push(Some(Key {
                value,
                special: None,
                ctrl: true,
                alt: false,
            }))
        }

        if buf.starts_with(starts_with) && buf.ends_with(&[59, 51, 126]) {
            result.push(Some(Key {
                value,
                special: None,
                ctrl: false,
                alt: true,
            }))
        }

        if buf.starts_with(starts_with) && buf.ends_with(&[59, 55, 126]) {
            result.push(Some(Key {
                value,
                special: None,
                ctrl: true,
                alt: true,
            }))
        }
    };

    let create_nav_key = | result: &mut Vec<_>, buf: &[_], start: &[_], end: u8, value: KeyValue | {
        if buf.ends_with(&[start, &[end]].concat()) {
            result.push(Some(Key {
                value: value,
                special: None,
                ctrl: false,
                alt: false,
            }))
        }

        if buf.starts_with(&[start, &[49, 59, 53], &[end]].concat()) {
            result.push(Some(Key {
                value,
                special: None,
                ctrl: true,
                alt: false,
            }))
        }

        if buf.starts_with(&[start, &[49, 59, 51], &[end]].concat()) {
            result.push(Some(Key {
                value,
                special: None,
                ctrl: false,
                alt: true,
            }))
        }

        if buf.starts_with(&[start, &[49, 59, 55], &[end]].concat()) {
            result.push(Some(Key {
                value,
                special: None,
                ctrl: true,
                alt: true,
            }))
        }
    };

    // more than two byte
    if buf[0] == 27 {
        // page up 
        let page_up: [u8;3] = [27, 91, 53];


        create_page_key(&mut result, &buf, &[27, 91, 53], KeyValue::PageUp);
        create_page_key(&mut result, &buf, &[27, 91, 54], KeyValue::PageDown);

        create_page_key(&mut result, &buf, &[27, 91, 50], KeyValue::Insert);
        create_page_key(&mut result, &buf, &[27, 91, 51], KeyValue::Del);

        create_nav_key(&mut result, &buf, &[27, 79], 80, KeyValue::F1);
        create_nav_key(&mut result, &buf, &[27, 79], 81, KeyValue::F2);
        create_nav_key(&mut result, &buf, &[27, 79], 82, KeyValue::F3);

        create_nav_key(&mut result, &buf, &[27, 79], 82, KeyValue::F3);

        create_nav_key(&mut result, &buf, &[27, 91], 70, KeyValue::End);
        create_nav_key(&mut result, &buf, &[27, 91], 72, KeyValue::Home);
        create_nav_key(&mut result, &buf, &[27, 91], 65, KeyValue::Up);
        create_nav_key(&mut result, &buf, &[27, 91], 66, KeyValue::Down);
        create_nav_key(&mut result, &buf, &[27, 91], 67, KeyValue::Right);
        create_nav_key(&mut result, &buf, &[27, 91], 68, KeyValue::Left);
    }

    buf.fill(0);
    return result
}

fn main() {
    let tty = File::open("/dev/tty").unwrap();
    let mut guard = TtyModeGuard::new(tty.as_raw_fd()).unwrap();
    guard.set_raw_mode().unwrap();
    let render = thread::spawn(move || {
        let mut cs = CharSource::new();
        loop {
            if let Some(recent) = cs.read() {
                if recent == 113  || recent == 3 {
                    break;
                }
            } else {
            }
        }
    });

    render.join().unwrap();
    drop(guard);
}
