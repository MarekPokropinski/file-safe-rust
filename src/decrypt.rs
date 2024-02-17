mod safe;

use std::env;

pub use crate::safe::Safe;

fn decrypt() -> Result<(), String> {
    let home_path = match home::home_dir() {
        Some(path) => path,
        None => panic!(),
    };

    let args: Vec<String> = env::args().collect();

    let password = match args.get(1) {
        Some(x)=> x,
        None => return Err(String::from("Start program with password as an argument: safe-init <password>"))
    };

    let safe_path = home_path.join("safe");

    let safe = Safe::new(&safe_path);
    match safe.decrypt(password.clone()) {
        Ok(_) => (),
        Err(x) => return Err(x.to_string())
    }

    Ok(())
}

fn main() {
    if let Some(x) = decrypt().err() {
        println!("Error: {}!", x);
    }
}
