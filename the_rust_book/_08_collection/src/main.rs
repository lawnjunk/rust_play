use std::collections::HashMap;

fn main() {
    let mut store: HashMap<String, Vec<String>> = HashMap::new();

    store.insert("pomelo".to_owned(), Vec::new());

    if let Some(list) = store.get_mut("pomelo") {
        list.push("create error handler".to_owned());
        list.push("encrypt payload".to_owned());
        list.push("define supervisor spec".to_owned());
    }

    store.insert("chore".to_owned(), Vec::new());

    if let Some(list) = store.get_mut("chore") {
        let todo_list = [
            "get eggs",
            "get flour",
            "get shooger",
            "bake caake",
            "make wish",
            "blow out candles",
            "eat cake",
        ];

        for todo in todo_list {
            list.push(String::from(todo));
        }
    }

    println!("store: {:#?}", store);

    for (list_name, list) in store.iter() {
        for task in list {
            println!("{} task: {}", list_name, task);
        }
    }
}
