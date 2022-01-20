extern crate rand; 

use std::thread::{self,JoinHandle};
use std::process;
use std::sync::mpsc;
use std::time::Duration;
use rand::{thread_rng, Rng};

enum Action {
    Quit(String),
    Move(String),
}

fn join_all(handle_list: Vec<JoinHandle<()>>) {
    for handle in handle_list {
        match handle.join() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("something went wrong joining a thread");
            }
        }

    }
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let atx = tx.clone();
    let a = thread::spawn(move || {
        let mut rng = thread_rng();
        let delay_list: [u32;10] = rng.gen::<[u32;10]>()
            .map(|value| { value % 1000});

        for (i, delay) in delay_list.iter().enumerate() {
            thread::sleep(Duration::from_nanos(*delay as u64));
            atx.send(Action::Move(format!("a-{}-{} ", i, delay))).unwrap_or(());
        }

        // use unwrap_or incase the rx loop is no longer running
        atx.send(Action::Quit("A won".to_owned())).unwrap_or(());
    });

    let btx = tx.clone();
    let b = thread::spawn(move || {
        let mut rng = thread_rng();
        for i in 0..10 {
            let delay = rng.gen_range(0..1000);
            thread::sleep(Duration::from_nanos(delay as u64));
            btx.send(Action::Move(format!("b_{}_{} ", i, delay))).unwrap_or(());
        }

        // use unwrap_or incase the rx loop is no longer running
        btx.send(Action::Quit("B won".to_owned())).unwrap_or(());
    });

    let c = thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(action) => {
                    match action {
                        Action::Quit(message) => {
                            println!("\n{}", message);
                            break;
                        },
                        Action::Move(message) => {
                            println!("{}", message);
                        }
                    }
                },
                Err(_) => {
                    eprintln!("something went wrong");
                    process::exit(1)
                }
            };
        }
    });

    join_all(vec![a, b, c]);
    println!("bubye");
}
