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

pub enum StringColor {
    // Standard Colors
    Black,
    Red,
    Green,
    Orange,
    Blue,
    Pink,
    Cyan,
    White,

    // Bright Colors
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightOrange,
    BrightBlue,
    BrightPink,
    BrightCyan,
    BrightWhite,
}

impl StringColor {
    pub fn val(&self) -> u32 {
        match *self {
            Self::Black  => 30, 
            Self::Red    => 31, 
            Self::Green  => 32, 
            Self::Orange => 33, 
            Self::Blue   => 34, 
            Self::Pink   => 35, 
            Self::Cyan   => 36, 
            Self::White  => 37, 

            Self::BrightBlack  => 90, 
            Self::BrightRed    => 91, 
            Self::BrightGreen  => 92, 
            Self::BrightOrange => 93, 
            Self::BrightBlue   => 94, 
            Self::BrightPink   => 95, 
            Self::BrightCyan   => 96, 
            Self::BrightWhite  => 97, 
        }
    }

    pub fn get_all_colors() -> Vec<Self> {
        return vec![
            Self::Black,
            Self::Red,
            Self::Green,
            Self::Orange,
            Self::Blue,
            Self::Pink,
            Self::Cyan,
            Self::White,

            Self::BrightBlack,
            Self::BrightRed,
            Self::BrightGreen,
            Self::BrightOrange,
            Self::BrightBlue,
            Self::BrightPink,
            Self::BrightCyan,
            Self::BrightWhite,
        ];
    }

    pub fn colorize_string(s: &str, c: StringColor) -> String {
        let mut modifier = String::new();
        modifier.push_str("\x1b[");
        modifier.push_str(c.val().to_string().as_str());
        modifier.push('m');
        modifier.push_str(s);
        modifier.push_str("\x1b[0m");

        return modifier; 
    }
}

pub struct Printer {
    print_buf: Vec<String>,
}

impl Printer {
    pub fn new() -> Self {
        return Self { print_buf: Vec::new() };
    }
    /*
    pub fn print_in_place(data: &str) {
        // Clear console
        println!("\x1b[2J");

        // Move cursor to home pos
        println!("\x1b[H");

        println!("{}", data);
    } 
    */

    fn clear_console() {
        // Clear console
        println!("\x1b[2J");

        // Move cursor to home pos
        println!("\x1b[H");
    }

    pub fn dyn_print(&self, frame_num: u32) {
        Self::clear_console();

        if frame_num as usize >= self.print_buf.len() {
            // Out of bounds
            return;
        }

        println!("{}", self.print_buf[frame_num as usize]);
    }
}


mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn colorize_string_test() {
        let expected = String::from("\x1b[30mTEST\x1b[0m");

        assert_eq!(expected, StringColor::colorize_string("TEST", StringColor::Black));
    }

    #[test]
    fn get_all_colors_test() {
        let expected = vec![30, 31, 32, 33, 34, 35, 36, 37, 90, 91, 92, 93, 94, 95, 96, 97];
        let actual: Vec<u32> = StringColor::get_all_colors().iter().map(|c| c.val()).collect();

        for color in StringColor::get_all_colors() {
            println!("{}", StringColor::colorize_string("TEST", color));
        }

        assert_eq!(expected, actual);
    }

    #[test]
    fn clear_console_test() {
        let data = "testbegin\nThis is a test of\nSeems to be working...\nprinting in place2\ntestend";
        let mut i: usize = 0;

        while i < data.len() {
            Printer::clear_console();
            for c in data.char_indices() {
                let r = rand::thread_rng().gen_range(0..20);
                let mut color = StringColor::White;
                if i % 2 == 0 && c.0 % 2 == 0 && r == 10 {
                    color = StringColor::BrightWhite;
                }


                if c.0 == i {
                    color = StringColor::BrightRed;
                }

                print!("{}", StringColor::colorize_string(c.1.to_string().as_str(), color));
            }
            println!();
            std::thread::sleep(std::time::Duration::from_millis(100));
            i += 1;
        }
    }
}