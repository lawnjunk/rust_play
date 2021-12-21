use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("GUSS a num");

    let secret = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guss = String::new();
        println!("input your guss:");
        io::stdin()
            .read_line(&mut guss)
            .expect("read_line chucked up");

        let guss: u32 = guss.trim().parse().expect("garbage is not number");

        let judgement = match guss.cmp(&secret) {
            Ordering::Less => "Too smol",
            Ordering::Greater => "To honkin",
            Ordering::Equal => "such luck",
        };

        if judgement == "such luck" {
            println!("wow, great guss");
            break;
        } else {
            println!("guss {} is {}, try harder plz", guss, judgement);
        }
    }
}
