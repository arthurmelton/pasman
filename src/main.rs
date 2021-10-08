use directories::BaseDirs;
use std::path::Path;
use std::fs::File;
use clap::*;
use std::time::SystemTime;

fn main() {
    let matches = App::new("Nyson")
        .version("0.19")
        .about("a programing language made in rust")
        .arg(
            Arg::with_name("test")
                .short("t")
                .long("test")
                .help("test to see how many passwords you can crack per second")
                .takes_value(false),
        )
        .get_matches();
    if matches.is_present("test") {
        let mut num = 0;
        let now = SystemTime::now();
        while now.elapsed().unwrap().as_millis() < 1000 {
            format!("{:x}", md5::compute(num.to_string()));
            num += 1;
        }
        println!("your computer can do {} hashes per second (Some computers can do WAY more than this)\n1 chars = {} seconds\n2 chars = {} seconds\n3 chars = {} seconds\n4 chars = {} seconds\n5 chars = {} seconds\n6 chars = {} seconds\n7 chars = {} seconds\n8 chars = {} seconds\n9 chars = {} seconds\n10 chars = {} seconds\n11 chars = {} seconds\n12 chars = {} seconds\n13 chars = {} seconds\n14 chars = {} seconds\n15 chars = {} seconds\n16 chars = {} seconds\n17 chars = {} seconds\n18 chars = {} seconds\n19 chars = {} seconds\n20 chars = {} seconds\n", num, (72_u128.pow(1)/num/2), (72_u128.pow(2)/num/2), (72_u128.pow(3)/num/2), (72_u128.pow(4)/num/2), (72_u128.pow(5)/num/2), (72_u128.pow(6)/num/2), (72_u128.pow(7)/num/2), (72_u128.pow(8)/num/2), (72_u128.pow(9)/num/2), (72_u128.pow(10)/num/2), (72_u128.pow(11)/num/2), (72_u128.pow(12)/num/2), (72_u128.pow(13)/num/2), (72_u128.pow(14)/num/2), (72_u128.pow(15)/num/2), (72_u128.pow(16)/num/2), (72_u128.pow(17)/num/2), (72_u128.pow(18)/num/2), (72_u128.pow(19)/num/2), (72_u128.pow(20)/num/2));
    }
    else {
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
}
