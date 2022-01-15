extern crate rand;
extern crate tui_util;

use std::env;
use std::thread;
use std::time::Duration;
use noise::{Perlin, NoiseFn};
use tui_util::*;

const FPS:u64 = 5;

fn main() {
    screen_reset();

    let message:Vec<String> = env::args().into_iter().skip(1).collect();
    if message.len() < 1 {
        eprintln!("ERROR: you didn't provide anything to scatter");
        eprintln!();
        eprintln!("\tUSAGE: scatter (what evere you want to say as args)");
        std::process::exit(1);
    }

    // let perlin = Perlin::new();
    // let wat_pos_x = [16.0, 3.0];
    // let wat_pos_y = [1.5, 5.0];

    // let nx = perlin.get([0.2, 0.1]);
    // let ny = perlin.get(wat_pos_y);
    // println!("1) {} {}", nx, ny);

//     std::process::exit(0);
    // TODO @slugbyte impl safe word zones instead of lines
    let mut safe_line_list:Vec<u32> = Vec::new();
    for word in message.iter() {
        let screen = screen_size().unwrap();
        let char_list = ["+", "-", ":", "~", "_", ".", ",", "*", " ", " "];


        cursor_hide();

        for x in 0..screen.width {
            for y in 0..(screen.height -1) {
                if safe_line_list.contains(&y) {
                    continue;
                }
                if rand::random::<u8>() > 250 {
                    color_fg(rand::random::<u8>());
                } else {
                    color_fg(233);
                }

                cursor_set_pos(x, y);
                write_raw(char_list.get(rand::random::<usize>() % char_list.len()).unwrap());
                mode_reset();
            }
        }

        let safe_line = rand::random::<u32>() % screen.height;
        let horizontal_pos = rand::random::<u32>() % screen.width;

        let horizontal_pos = if horizontal_pos > screen.width - word.len() as u32 {
            horizontal_pos - word.len() as u32
        } else {
            horizontal_pos
        };

        safe_line_list.push(safe_line);
        cursor_set_pos(horizontal_pos, safe_line);
        write_raw(&format!("    {}    ", word));

        thread::sleep(Duration::from_millis(1000 / FPS));
    }

    thread::sleep(Duration::from_millis(1000));

    let screen  = screen_size().unwrap();
    cursor_set_pos(screen.width, screen.height);
    println!();
    cursor_show();
}
