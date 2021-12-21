use std::io;

const FIVE_MINUTES_IN_SECONDS: u32 = 60 * 5;

fn mutable_number() {
    let mut x = 5;
    println!("x: {}", x);
    x = 6;
    println!("x: {}", x);
}

fn print_a_constant() {
    println!("five minutes in seconds: {}", FIVE_MINUTES_IN_SECONDS)
}

fn shadow_number() {
    // shadow also allows for type change
    let x = 5;
    println!("x: {}", x);
    let x = 6;
    println!("x: {}", x);

    let mut x: f32 = x as f32;

    x += 0.1;
    println!("x: {}", x);
}

fn tuple_play() {
    // touples can store multiple types
    let player = ("shark", 2.5, 100, 2);
    // printing the whole tuple
    println!("player (:?) {:?}", player);
    // pretty printing the whole tuple
    println!("player (:#?) {:#?}", player);

    // accessing the contents through destructureing
    // vars that start with _ (_y_cord) won't have warnings when not used
    let (name, x_cord, _y_cord, hearts) = player;
    println!("name (:?) {:?}", name);
    println!("name (:#?) {:#?}", name);
    println!("x_cord (:?) {:?}", x_cord);
    println!("x_cord (:#?) {:#?}", x_cord);
    println!("hearts (:?) {:?}", hearts);
    println!("hearts (:#?) {:#?}", hearts);

    // let (name, x_cord) = player; // not allowed, due to mismatch elements
    println!("accessing a tuple through offset: player.0 {}", player.0);
    println!("accessing a tuple through offset: player.1 {}", player.1);
    println!("accessing a tuple through offset: player.2 {}", player.2);
    println!("accessing a tuple through offset: player.3 {}", player.3);
    // println!("accessing a tuple through offset: player.4 {}", player.4); not allowed due to out
    // of bounds
}

fn runtime_error() {
    let number_list = [2, 4, 6, 8];

    println!("dont fuckup, enter a number (0-4):");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("input must be a number");

    let index: usize = index.trim().parse().expect("input must be an integer");

    let result = number_list[index];
    println!("2 to the {} is {}", index + 1, result);
}

fn safe_access(list: &[i32], index: i32) -> i32 {
    let index: usize = index.clamp(0, (list.len() as i32) - 1) as usize;
    list[index]
}

fn no_runtime_error() {
    let number_list: [i32; 4] = [2, 4, 6, 8];
    println!("\nnumber_list.len() {}", number_list.len());

    println!("feel free to fuckup, enter a number (0-4):");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("unable to read input");

    let index: i32 = index.trim().parse().expect("input must be an integer");

    let result = safe_access(&number_list, index);

    println!("2 to the {} is {}", index + 1, result);
    println!("first number in list: {}", number_list[0]);
}

fn loop_until_quit() {
    println!("try and quit the loop");
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("unable to read input");

        if input.trim() == "q" {
            break;
        } else if input.trim() == "quit" {
            break;
        } else {
            println!("you chose to continue\ntry to quit again");
        }
    }
    println!("you quit");
}

fn fizz_buzz_with_range() {
    for number in (0..5) {
        if number % 4 == 0 {
            println!("fizz");
        } else if number % 2 == 0 {
            println!("buzz");
        } else {
            println!("gross");
        }
    }
}

fn base_two_power(times: u32) {
    let mut count = 0;
    let mut result = 2;
    let result = loop {
        count += 1;
        result *= 2;
        if count == times {
            break result;
        }
    };
    println!("two to the {}: {}", times, result);
}

fn main() {
    mutable_number();
    print_a_constant();
    shadow_number();
    tuple_play();
    fizz_buzz_with_range();
    base_two_power(5);
    no_runtime_error();
    runtime_error();
    loop_until_quit();

    println!("Hello, world!");
}
