pub trait News {
    fn get_headline(&self) -> &str;
    fn get_article(&self) -> &str;
    fn get_word_count(&self) -> usize;
}

pub trait Like {
    fn inc_like(&mut self) -> i32;
}

#[derive(Debug)]
pub struct Article<'a> {
    title: &'a str,
    content: &'a str,
    like_count: i32,
}

impl<'a> Article<'a> {
    fn new(title: &'a str, content: &'a str) -> Article<'a> {
        Article {
            title,
            content,
            like_count: 0,
        }
    }
}

impl Like for Article<'_> {
    fn inc_like(&mut self) -> i32 {
        self.like_count += 1;
        self.like_count
    }
}

impl<'a> News for Article<'a> {
    // return the first word of the content
    fn get_headline(&self) -> &'a str {
        self.title
    }

    // return all content after first word
    fn get_article(&self) -> &'a str {
        self.content
    }

    fn get_word_count(&self) -> usize {
        let headline = self.get_headline();
        let article = self.get_article();

        let mut count: usize = 0;
        let headline_word_count = article.split(' ').into_iter().map(|_| 1).reduce(|a, b| {
            println!("a: {} b: {}", a, b);
            a + b
        });
        for _ in article.split(' ').into_iter() {
            count += 1
        }
        println!("headline_word_count: {:?}", headline_word_count);
        count + (headline_word_count.unwrap_or(0) as usize)
    }
}

fn print_news<'a, T: News>(data: T) -> T {
    println!(
        "headline: {}\narticle: {}",
        data.get_headline(),
        data.get_article()
    );
    data
}

fn main() {
    let content = String::from("cool beans");
    let mut note = Article::new("cool dude", &content);
    println!("note {:?}", note);
    note.inc_like();
    println!("note {:?}", note);
    println!("note {:?}", note.get_headline());

    println!("word count: {}", note.get_word_count());
    print_news(note);
}
