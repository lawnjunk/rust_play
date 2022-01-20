use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Error as IOError, Read};
use std::path::Path;
use std::process;

pub fn help() {
    println!(
        "USAGE:
    $ gerp filepath query"
    )
}

#[derive(Debug)]
struct Line<'a> {
    pub line_number: usize,
    pub text: &'a str,
}

#[derive(Debug)]
pub struct Context {
    filepath: String,
    query: String,
    content: Option<String>,
}

impl Context {
    fn new(arg_list: Vec<String>) -> Self {
        if arg_list.len() != 3 {
            panic!("usage error")
        }
        Context {
            filepath: arg_list[1].clone(),
            query: arg_list[2].clone(),
            content: None,
        }
    }
}

fn read_file(context: Context) -> Result<Context, IOError> {
    let mut context = context;
    let mut file = File::open(Path::new(&context.filepath))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    context.content = Some(content);
    Ok(context)
}

fn get_valid_lines<'a>(content: &'a str, query: &str) -> Vec<Line<'a>> {
    let mut result: Vec<Line<'a>> = Vec::new();

    let line_list: Vec<&str> = content.split('\n').collect();

    for (index, text) in line_list.into_iter().enumerate() {
        if text.contains(query) {
            result.push(Line {
                line_number: index,
                text,
            })
        }
    }
    result
}

fn gerp(context: &Context) -> Result<(), Box<dyn Error>> {
    if let Some(content) = &context.content {
        let line_list = get_valid_lines(&content, &context.query);
        for line in line_list {
            println!(
                "{} line {}: {}",
                context.filepath, line.line_number, line.text
            );
        }
    }
    Ok(())
}

fn get_args() -> Result<Vec<String>, Box<dyn Error>> {
    let arg_list: Vec<String> = env::args().collect();
    if arg_list.len() != 3 {
        return Err(Box::from(String::from("USAGE error three args expected)")));
    }
    Ok(arg_list)
}

fn run() -> Result<(), Box<dyn Error>> {
    let arg_list = get_args()?;
    let context = Context::new(arg_list);
    let context = read_file(context)?;
    gerp(&context)
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("error: {:?}", err);
            process::exit(1);
        }
    };
}
