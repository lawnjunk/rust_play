// borrow a strign and create a new string that gets moved trough a return

fn suckify(txt: &str) -> String {
    String::from("sucks".to_owned() + &txt.to_owned())
}

fn suckify2(txt: &str) -> String {
    format!("{} sucks", txt)
}

fn suckify3(txt: &str) -> String {
    // clone will not work because &str.clone() just makes an &str
    // to_owned creats a string from a &str
    txt.to_owned() + &" sucks sucks sucks".to_owned()
}

// borrow string
fn suckify4(txt: &String) -> String {
    // clone works because its a string
    txt.to_owned() + &" is a sucka".to_owned()
}

// borrow
fn suckify5(text: String) -> String {
    format!("{} sucked", text)
}

fn btor(txt: &str) -> String {
    txt.to_string().replace("b", "r")
}

// get last index of first word
fn first_word(txt: &str) -> &str {
    let bytes = txt.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &txt[..index];
        }
    }
    txt
}

fn main() {
    let someone = "bob";
    println!("{}", suckify(&someone));
    println!("{}", suckify2(&someone));
    println!("{}", suckify3(&someone));
    println!("{}", suckify3(&someone.to_owned()));
    println!("{}", suckify3("daren"));
    println!("{}", suckify4(&someone.to_owned()));

    let someone = suckify5("cool".to_string());
    suckify5(someone.clone()); // clone doesn't borrow
    println!("{}", someone);
    let someone = suckify5(someone); //real borrow
    println!("{}", someone);

    let x = "hello ".repeat(5);

    println!("{}", x);
    println!("{}", btor("bob"));
    println!("Hello, world!");

    // env var names
    for (name, _value) in std::env::vars() {
        println!("env {}", name);
    }

    // cmd line args
    for arg in std::env::args() {
        println!("env {}", arg);
    }

    println!("temp dir: {}", std::env::temp_dir().display());
    println!(
        "temp dir: {:?}",
        std::env::current_dir().expect("should have dir")
    );

    println!("first word: {}", first_word("this is cool i guess"));
}
