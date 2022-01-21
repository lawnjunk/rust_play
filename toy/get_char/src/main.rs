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

impl CharSource {
    fn new() -> CharSource {
        let (tx, rx) = mpsc::channel::<u8>();

        let handle = thread::spawn(move || {
            println!("hahah ");
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            let mut local_buf = [0;100];

            loop {
                if let Ok(read_count) =  stdin.read(&mut local_buf) {
                    println!("\nread_count: {}", read_count);
                    for i in 0..read_count {
                        tx.send(local_buf[i]).unwrap();
                    }
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

fn main() {
    let tty = File::open("/dev/tty").unwrap();
    let mut guard = TtyModeGuard::new(tty.as_raw_fd()).unwrap();
    guard.set_raw_mode().unwrap();
    let render = thread::spawn(move || {
        let mut cs = CharSource::new();
        loop {
            if let Some(recent) = cs.read() {
                write_code("0G");
                println!("code: {:?}", recent);
                if recent == b'\x1b' {
                    println!("wwwwww");
                }
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
