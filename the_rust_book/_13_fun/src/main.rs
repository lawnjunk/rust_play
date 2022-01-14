use std::thread;
use std::time::Duration;

pub fn slow_as_funk(seed: u32) -> u32 {
    println!("wait jus a minute");
    thread::sleep(Duration::from_secs(2));
    println!("thx luv");
    seed
}

struct Cache<A,T>
where
    A: Copy,
    T: Fn(A) -> A,
{
    calculation: T,
    value: Option<A>
}

impl<A,T> Cache<A, T> 
where
    A: Copy,
    T: Fn(A) -> A
{
    fn new(calculation: T) -> Cache<A,T> {
        Cache {
            calculation,
            value: None,
        }
    }
    fn get_value(&mut self, arg: A) -> A {
        if let Some(value) = self.value {
            return value
        }

        let result = (self.calculation)(arg);
        self.value = Some(result);
        result
    }
}


fn  do_useless(seed: u32, random: u32) {
    let mut cache = Cache::new(|x:u32| {
        println!("wait jus a minute");
        thread::sleep(Duration::from_secs(5));
        println!("thx luv");
        x
    });

    if  seed > 25 {
        let result  = cache.get_value(seed);
        println!("garbage: {}", result);
        let result  = cache.get_value(seed);
        println!("more garbage: {}", result);
        return
    }

    if random == 3 {
        println!("nothing for you this time");
        return
    }

    let result = cache.get_value(seed);
    println!("trash: {}", result);
}

fn main() {
    println!("Hello, world!");
    let magic_seed = 50;
    let magic_random = 7;

    do_useless(magic_seed, magic_random);

    println!("thnks bub, come back again");
}
