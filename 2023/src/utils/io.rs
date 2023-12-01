use std::fs;

pub fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(data) => return data,
        Err(err) => {
            println!("Could not read file due to: {}", err);
            return String::from("");
        }
    }
}

#[allow(dead_code)]
pub fn read_file_with_split(path: &str, delim: char) -> Vec<String> {
    return read_file(path).split(delim).map(String::from).collect();
}

#[allow(dead_code)]
pub fn read_file_split_newlines(path: &str) -> Vec<String> {
    return read_file_with_split(path, '\n');
}
