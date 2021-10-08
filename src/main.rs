use directories::BaseDirs;
use std::path::Path;
use std::fs::File;
use clap::*;
use std::time::SystemTime;
use std::io::{Write, Read};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};
use rand::seq::SliceRandom;
use std::str::FromStr;

fn main() {
    let matches = App::new("pasman")
        .version("0.01")
        .about("A password manager")
        .arg(
            Arg::with_name("test")
                .short("t")
                .long("test")
                .help("test to see how many passwords you can crack per second")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .help("pass through your password")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("lists all the passwords you have")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("generate")
                .short("g")
                .long("generate")
                .help("generates a password with the x chars long")
                .takes_value(true),
        )
        .get_matches();
    if matches.is_present("test") {
        let mut num = 0;
        let now = SystemTime::now();
        while now.elapsed().unwrap().as_millis() < 1000 {
            format!("{:x}", md5::compute(num.to_string()));
            num += 1;
        }
        println!("your computer can do {} hashes per second (Some computers can do WAY more than this)\n1 chars = {} seconds\n2 chars = {} seconds\n3 chars = {} seconds\n4 chars = {} seconds\n5 chars = {} seconds\n6 chars = {} seconds\n7 chars = {} seconds\n8 chars = {} seconds\n9 chars = {} seconds\n10 chars = {} seconds\n11 chars = {} seconds\n12 chars = {} seconds\n13 chars = {} seconds\n14 chars = {} seconds\n15 chars = {} seconds\n16 chars = {} seconds\n17 chars = {} seconds\n18 chars = {} seconds\n19 chars = {} seconds\n20 chars = {} seconds\n", num, (72_u128.pow(1)/num), (72_u128.pow(2)/num), (72_u128.pow(3)/num), (72_u128.pow(4)/num), (72_u128.pow(5)/num), (72_u128.pow(6)/num), (72_u128.pow(7)/num), (72_u128.pow(8)/num), (72_u128.pow(9)/num), (72_u128.pow(10)/num), (72_u128.pow(11)/num), (72_u128.pow(12)/num), (72_u128.pow(13)/num), (72_u128.pow(14)/num), (72_u128.pow(15)/num), (72_u128.pow(16)/num), (72_u128.pow(17)/num), (72_u128.pow(18)/num), (72_u128.pow(19)/num), (72_u128.pow(20)/num));
    }
    else if matches.is_present("generate") {
        let chars = vec!['0','1','2','3','4','5','6','7','8','9','!','@','#','$','%','^','&','*','(',')','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
        let mut password:String = String::new();
        for _x in 0..FromStr::from_str(matches.value_of("generate").unwrap()).unwrap() {
            password.push_str(chars.choose(&mut rand::thread_rng()).unwrap().to_string().as_str())
        }
        println!("{}", password);
    }
    else {
        if let Some(base_dirs) = BaseDirs::new() {
            if !Path::new(&[base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join("")).exists() {
                File::create(&[base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join(""));
            }
            let mut line = String::new();
            if matches.is_present("password") {
                line = matches.value_of("password").unwrap().trim().to_string();
            }
            else {
                println!("What is your master password? (you can run the --test on the executable to see how long it would take to crack the password)");
                std::io::stdin().read_line(&mut line).unwrap();
                line = line.trim().to_string();
            }
            if line.contains(" ") {
                println!("sorry your master password can not have a space in it.");
                std::process::exit(1);
            }
            let password = format!("{:x}", md5::compute(line));
            let key = Key::from_slice(password.as_ref());
            let cipher = Aes256Gcm::new(key);
            let nonce = Nonce::from_slice(b"unique nonce");
            let mut contents = String::new();
            File::open(&[base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join("")).unwrap().read_to_string(&mut contents);
            if contents.len() == 0 {
                println!("Lets add a password what do you want the name to be?");
                let mut new_accont:String = "".to_string();
                std::io::stdin().read_line(&mut new_accont).unwrap();
                new_accont = new_accont.trim().to_string();
                if new_accont.contains(" ") {
                    println!("sorry your account cant have a space in it try something like www.google.com");
                    std::io::stdin().read_line(&mut new_accont).unwrap();
                    new_accont = new_accont.trim().to_string();
                }
                println!("Lets add a password to it");
                let mut new_pass:String = "".to_string();
                std::io::stdin().read_line(&mut new_pass).unwrap();
                new_pass = new_pass.trim().to_string();
                if new_accont.contains(" ") {
                    println!("sorry your password can not have a space in it");
                    std::io::stdin().read_line(&mut new_pass).unwrap();
                    new_pass = new_pass.trim().to_string();
                }
                let ciphertext = cipher.encrypt(nonce, [new_accont, " : ".to_string(), new_pass].join("").as_ref())
                    .expect("encryption failure!"); // NOTE: handle this error to avoid panics!
                write_file([base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join(""), hex::encode(ciphertext));
            }
            else {
                if matches.is_present("list") {
                    let decoded = hex::decode(contents.clone()).unwrap();
                    let plaintext = cipher.decrypt(nonce, decoded.as_ref())
                        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
                    println!("{}", String::from_utf8_lossy(&*plaintext).to_string());
                    std::process::exit(1);
                }
                println!("Type the one you want\n1: add a password\n2: find a password\n3: list all passwords (UNSECURE)");
                let mut type_pick:String = "".to_string();
                std::io::stdin().read_line(&mut type_pick).unwrap();
                type_pick = type_pick.trim().to_string();
                if !["1", "2", "3"].contains(&&*type_pick) {
                    println!("Sorry but you have to pick on of the above type 1 etc");
                    std::io::stdin().read_line(&mut type_pick).unwrap();
                    type_pick = type_pick.trim().to_string();
                }
                if type_pick == "1" {
                    println!("what do you want the password account to be?");
                    let mut new_accont:String = "".to_string();
                    std::io::stdin().read_line(&mut new_accont).unwrap();
                    new_accont = new_accont.trim().to_string();
                    if new_accont.contains(" ") {
                        println!("sorry your account cant have a space in it try something like www.google.com");
                        std::io::stdin().read_line(&mut new_accont).unwrap();
                        new_accont = new_accont.trim().to_string();
                    }
                    println!("what do you want the password to it");
                    let mut new_pass:String = "".to_string();
                    std::io::stdin().read_line(&mut new_pass).unwrap();
                    new_pass = new_pass.trim().to_string();
                    if new_accont.contains(" ") {
                        println!("sorry your password can not have a space in it");
                        std::io::stdin().read_line(&mut new_pass).unwrap();
                        new_pass = new_pass.trim().to_string();
                    }
                    let decoded = hex::decode(contents.clone()).unwrap();
                    let plaintext = cipher.decrypt(nonce, decoded.as_ref())
                        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
                    let ciphertext = cipher.encrypt(nonce, [String::from_utf8_lossy(&*plaintext).to_string(), "\n".to_string(), new_accont, " : ".to_string(), new_pass].join("").as_ref())
                        .expect("encryption failure!"); // NOTE: handle this error to avoid panics!
                    write_file([base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join(""), hex::encode(ciphertext));
                }
                else if type_pick == "2" {
                    let decoded = hex::decode(contents.clone()).unwrap();
                    let plaintext = cipher.decrypt(nonce, decoded.as_ref())
                        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
                    println!("what account are you looking for?");
                    loop {
                        let mut account: String = "".to_string();
                        std::io::stdin().read_line(&mut account).unwrap();
                        account = account.trim().to_string();
                        if account.contains(" ") {
                            println!("Sorry the account you are looking for can not have a space in it");
                            std::io::stdin().read_line(&mut account).unwrap();
                            account = account.trim().to_string();
                        }
                        let split = String::from_utf8_lossy(&*plaintext).to_string().replace("\n", " : ");
                        let split: Vec<&str> = split.split(" : ").collect();
                        let mut account_names: Vec<String> = Vec::new();
                        let mut passwords: Vec<String> = Vec::new();
                        for x in 0..split.len() {
                            if x % 2 == 0 {
                                account_names.push(split[x].to_string());
                            } else {
                                passwords.push(split[x].to_string());
                            }
                        }
                        let mut name = Vec::new();
                        for x in account_names.clone() {
                            if x.contains(&account) {
                                name.push(x);
                            }
                        }
                        if name.len() == 0 {
                            println!("I have not fount any passwords with that name");
                            std::process::exit(1);
                        } else if name.len() == 1 {
                            for x in 0..account_names.clone().len() {
                                if name.first().unwrap() == &account_names[x] {
                                    println!("{}", passwords[x]);
                                }
                            }
                            std::process::exit(1);
                        } else {
                            println!("I have found\n{}\nWhat password do you want?", name.join("\n"));
                        }
                    }
                }
                else if type_pick == "3" {
                    let decoded = hex::decode(contents.clone()).unwrap();
                    let plaintext = cipher.decrypt(nonce, decoded.as_ref())
                        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
                    println!("{}", String::from_utf8_lossy(&*plaintext).to_string());
                }
            }
        }
    }
}

fn write_file(file:String, text:String) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write_all(text.as_ref())?;
    Ok(())
}