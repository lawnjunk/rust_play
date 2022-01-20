extern crate sys;
extern crate rand;
mod render;

use std::io::{self,Read};
use std::thread;
use std::sync::mpsc::{self,Receiver};
use rand::{Rng,thread_rng};
use std::time::Duration;
use std::fs;

fn get_tty() -> io::Result<fs::File> {
    fs::OpenOptions::new().read(true).write(true).open("/dev/tty")
}

struct KeyStream {
    rx: Receiver<Option<u8>>,
}

impl KeyStream {
    fn new() -> KeyStream {
        let (tx, rx)= mpsc::channel();

        thread::spawn(move || {
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            let mut buf = [0;100];
            handle.read(&mut buf).unwrap();

            for byte in buf {
            tx.send(Some(byte)).unwrap();
            }
        });

        KeyStream {
            rx,
        }
    }

    fn read(&mut self, buf: &mut [u8]) -> Option<usize> {
        let mut total = 0;

        loop {
            if total >= buf.len() {
                break;
            }

            match self.rx.try_recv() {
                Ok(Some(byte)) => {
                    buf[total] = byte;
                    total += 1;
                },
                Ok(None) => buf[0] = 2,
                Err(_) => break,
            }
        }

        None
    }
}

enum Action {
    Quit,
    Error,
    None,
    MoveVector((i32, i32)),
}

fn main() {
    let (tx, rx) = mpsc::channel::<Action>();

    let event_therad = thread::spawn(move || {
        loop {
            let mut buf: [u8;3] = [0,0,0];
            let mut ks = KeyStream::new();


            ks.read(&mut buf);

            render::write_raw(&format!("input {:?}", buf));
            thread::sleep(Duration::from_millis(500));
            tx.send(Action::MoveVector((1, 1))).unwrap_or(());
        }
    });

    let render_thread = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        render::screen_reset();
        let screen_size = render::screen_size().unwrap();

        let mut cursor = (0, 0);
        loop {

            let action = rx.recv().unwrap();
            match action {
                Action::MoveVector((x, y)) => {
                    cursor.0 += x;
                    cursor.0 = cursor.0.clamp(0, screen_size.width as i32);
                    cursor.1 += y;
                    cursor.0 = cursor.1.clamp(1, screen_size.height as i32);
                },
                Action::Quit => {
                    std::process::exit(0);
                },
                Action::Error => {
                    println!("error recv action");
                    std::process::exit(1);
                },
                Action::None => (),
            }

            render::cursor_set_pos(cursor.0 as u32, cursor.1 as u32);
        }
    });

    event_therad.join().unwrap();
    render_thread.join().unwrap();

    render::color_fg(197);
    println!("Hello, world!");
    render::mode_reset();

}
