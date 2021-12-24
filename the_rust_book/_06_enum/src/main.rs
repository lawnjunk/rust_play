use std::collections::HashMap;
use std::io;
use std::vec::Vec;

const HELP_MESSAGE: &str = "Actions:
    help           - print this help
    quit           - terminate program
    list           - list notes
    add <message>  - add a note
    delete <id>    - delete a note
    get <id>       - get a note
";

type NoteId = u32;

#[derive(Clone, Debug)]
struct Note {
    id: NoteId,
    title: String,
}

type IdInput = Result<NoteId, String>;

#[derive(Debug)]
struct App {
    note_count: u32,
    store: HashMap<u32, Note>,
}

impl App {
    fn new() -> App {
        App {
            note_count: 0,
            store: HashMap::new(),
        }
    }

    fn add_note(&mut self, title: &str) -> &App {
        self.store.insert(
            self.note_count,
            Note {
                id: self.note_count as NoteId,
                title: title.to_owned(),
            },
        );
        self.note_count = self.note_count + 1;
        self
    }

    fn get_note(&mut self, id: NoteId) -> Option<Note> {
        if let Some(note) = self.store.get(&id) {
            Some(note.clone())
        } else {
            None
        }
    }

    fn delete_note(&mut self, id: NoteId) -> Option<Note> {
        self.store.remove(&id)
    }

    fn get_note_id_list(&self) -> Vec<NoteId> {
        let mut result: Vec<NoteId> = Vec::new();
        for key in self.store.keys() {
            result.push(*key as NoteId);
        }
        result.sort();
        result
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    Add(String),
    Get(IdInput),
    Delete(IdInput),
    List,
    Help,
    Quit,
    Unknown,
}

fn parse_id_input(input: String) -> IdInput {
    if let Ok(id) = input.parse() {
        IdInput::Ok(id)
    } else {
        IdInput::Err(input)
    }
}

fn fetch_input() -> Action {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let input = input.trim();
        return if input == "quit" {
            Action::Quit
        } else if input == "help" {
            Action::Help
        } else if input.starts_with("add") {
            Action::Add(input.replace("add ", ""))
        } else if input == "list" {
            Action::List
        } else if input.starts_with("get") {
            let input = input.replace("get ", "");
            Action::Get(parse_id_input(input))
        } else if input.starts_with("delete") {
            let input = input.replace("delete ", "");
            Action::Delete(parse_id_input(input))
        } else {
            Action::Unknown
        };
    } else {
        println!("failed to get input, try again");
        Action::Unknown
    }
}

fn handle_action(action: &Action, app: &mut App) {
    match action {
        Action::Quit => {
            println!("bye bye");
        }
        Action::Add(title) => {
            println!("adding note ({}): {}", app.note_count, title);
            app.add_note(&title);
        }
        Action::List => {
            let id_list = app.get_note_id_list();
            if id_list.len() == 0 {
                println!("there are no notes");
                return;
            }
            for id in id_list {
                if let Some(note) = app.get_note(id) {
                    println!("({}): {}", note.id, note.title);
                }
            }
        }
        Action::Get(id) => match id {
            IdInput::Ok(id) => {
                if let Some(note) = app.get_note(*id) {
                    println!("note: {}", note.title);
                } else {
                    println!("note with id {} not found", id);
                }
            }
            IdInput::Err(id) => {
                println!("[{}] is not a valid id", id);
            }
        },
        Action::Delete(id) => match id {
            IdInput::Ok(id) => {
                if let Some(note) = app.delete_note(*id) {
                    println!("deleted note ({}): {}", note.id, note.title);
                } else {
                    println!("no note with id {} was found", id);
                }
            }
            IdInput::Err(id) => {
                println!("[{}] is not a valid id", id);
            }
        },
        Action::Help => {
            println!("{}", HELP_MESSAGE);
        }
        Action::Unknown => {
            println!("not sure what to do with that");
        }
    }
}

fn main() {
    let mut app = App::new();
    println!("app {:#?}", app);

    loop {
        println!("");
        let action = fetch_input();
        handle_action(&action, &mut app);
        if action == Action::Quit {
            break;
        }
        println!("app {:#?}", app);
    }
}
