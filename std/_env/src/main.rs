use std::env;
use std::path::Path;

fn main() {
    let args = env::args();

    for arg in args {
        println!("cli arg: {}", arg);
    }

    for (var, _value) in env::vars() {
        println!("env key: {}", var);
    }

    if let Ok(current_dir) = env::current_dir() {
        println!("current dir: {:?}", current_dir);
        if let Some(path) = current_dir.to_str() {
            println!("current dir: {}", path);
        }
    }

    if let Ok(current_exe) = env::current_exe() {
        println!("path to current exe {:?}", current_exe);
    }

    // HOME DIR IS DEPRECATED
    // if let Some(home_dir) = env::home_dir() {
    //     println!("home dir: {:?}", home_dir);
    // }

    if let Some(os_path) = env::var_os("PATH") {
        for path in env::split_paths(&os_path) {
            println!("$PATH has {}", path.to_str().unwrap_or("PATH_ERROR"));
        }
    }

    // JOIN PATHs
    if let Ok(home) = env::var("HOME") {
        let workspace_exec = format!("{}/workspace/exec", home);
        let path_list = [Path::new(&workspace_exec), Path::new("/bin")];
        if let Ok(money_path) = env::join_paths(path_list) {
            println!("path {}", money_path.to_str().unwrap());
        }
    }

    let tmp_dir = env::temp_dir();
    println!("temp dir {:?}", tmp_dir);
    println!("Hello, world!");
}
