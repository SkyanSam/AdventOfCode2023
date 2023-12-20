use std::env;
use std::fs;
//use substring::Substring;
use scan_fmt::scan_fmt;

fn main() {
    let file_path = format!("{}{}", std::env::current_dir().unwrap().display(), "/src/data.txt");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let substrings: Vec<&str> = contents.split(',').collect();

    for substring in substrings {
        let parts: Vec<&str> = substring.split(':').collect();
        let id : i32;
        if let Some((game_string, game_id)) = scan_fmt!(parts[0], "{s} {d}", &str, i32) {
            id = game_id;
        }
        println!("{id}");
    }
}
