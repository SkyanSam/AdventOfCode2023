use std::env;
use std::fs;

fn is_num(character : char) -> bool {
    if (
        character == '0' || character == '1' || 
        character == '2' || character == '3' || 
        character == '4' || character == '5' ||
        character == '6' || character == '7' ||
        character == '8' || character == '9' 
    ) {
        return true;
    }
    return false;
}
fn main() {
    println!("Hello, world!");

    let file_path = format!("{}{}", std::env::current_dir().unwrap().display(), "/src/data.txt");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let mut first_char : char = '\0';
    let mut second_char : char = '\0';
    let mut num_string : [char; 2] = ['\0', '\0'];
    let mut num : i64 = 0;
    let mut total_num : i64 = 0;
    let mut is_number : bool;
    for (index, c) in contents.chars().enumerate() {
        is_number = is_num(c);
        if is_number {
            if first_char == '\0' {
                first_char = c;
            } else {
                second_char = c;
            }
        }
        if c == '\n' || index == contents.len() - 1 {
            if first_char != '\0' {
                if second_char == '\0' {
                    second_char = first_char;
                }
                num_string[0] = first_char;
                num_string[1] = second_char;
                let string_from_char_array: String = num_string.iter().collect();
                println!("String: {}", string_from_char_array);
                let parsed_string = string_from_char_array.parse();

                match parsed_string {
                    Ok(value) => {
                        num = value;
                    }
                    Err(err) => {
                        println!("Error parsing to i64: {}", err);
                    }
                }
                total_num += num;
            }
            first_char = '\0';
            second_char = '\0';
            num_string[0] = '\0';
            num_string[1] = '\0';
            num = 0;
        }
    }
    println!("Total number is {total_num}");
}
