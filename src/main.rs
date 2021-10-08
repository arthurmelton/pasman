use directories::BaseDirs;
use std::path::Path;
use std::fs::File;

fn main() {
    if let Some(base_dirs) = BaseDirs::new() {
        if !Path::new(&[base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join("")).exists() {
            File::create(&[base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join(""));
        }
        println!("What is your password?");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        let password = format!("{:x}", md5::compute(matches.value_of(line).unwrap().to_string()));
    }
}
