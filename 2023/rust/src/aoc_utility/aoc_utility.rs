use std::fs;
use std::env;

pub fn get_input(filename : &str, day_folder : &str) -> String
{
    let mut cwd = env::current_dir().unwrap();
    cwd.push("src");
    cwd.push(day_folder);
    cwd.push(filename);
    // println!("CWD: \"{}\"", cwd.display());

    let input : String = fs::read_to_string( cwd )
        .expect("\"src\\day_1\\input.txt\" not found.");

    return input;
}