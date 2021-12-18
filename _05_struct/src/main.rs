#[derive(Debug)]
struct Note {
    title: String,
    description: String,
    complete: bool,
}

impl Note {
    fn new(title: &str, description: Option<&str>) -> Note {
        let description = description.unwrap_or(title);
        Note {
            title: title.to_owned(),
            description: description.to_owned(),
            complete: false,
        }
    }

    fn finish(&mut self) -> &mut Note {
        self.complete = true;
        self
    }

    fn reset(&mut self) -> &mut Note {
        self.complete = false;
        self
    }
}

fn main() {
    let mut note = Note::new("get eggs", Some("to to pxMart and get some eggs"));

    note.finish().reset();
    println!("note: {:#?}", note);

    let title = String::from("get milk");
    let mut note = Note::new(&title, None);

    note.finish();
    println!("note: {:#?}\ntitle {}", note, title);
}
