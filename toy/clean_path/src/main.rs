use std::env;
fn main() {
    use std::collections::HashSet;

    if let Ok(path) = env::var("PATH") {
        let path_list = path.split(':');
        let mut path_set: HashSet<&str> = HashSet::new();
        let mut result: Vec<&str> = Vec::new();
        for path in path_list {
            if path_set.contains(path) {
                continue;
            }
            path_set.insert(path);
            result.push(path);
        }
        let path = result.join(":");
        println!("{}", path);
    } else {
        eprintln!("ERROR: unable to read $PATH");
    }
}
