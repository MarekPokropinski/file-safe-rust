mod safe;

use std::env;

pub use crate::safe::Safe;

fn init() -> Result<(), String> {
    let home_path = match home::home_dir() {
        Some(path) => path,
        None => panic!(),
    };

    let safe_path = home_path.join("safe");

    let safe = Safe::new(&safe_path);

    let args: Vec<String> = env::args().collect();

    let password = match args.get(1) {
        Some(x)=> x,
        None => return Err(String::from("Start program with password as an argument: safe-init <password>"))
    };

    match safe.init(password.clone()) {
        Ok(_) => (),
        Err(x) => return Err(x.to_string())
    }

    Ok(())
}

fn main() {
    if let Some(x) = init().err() {
        println!("Error: {}!", x);
    }
}
