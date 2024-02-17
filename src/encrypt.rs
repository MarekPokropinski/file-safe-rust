mod safe;

pub use crate::safe::Safe;

fn encrypt() -> Result<(), String> {
    let home_path = match home::home_dir() {
        Some(path) => path,
        None => panic!(),
    };

    let safe_path = home_path.join("safe");

    let safe = Safe::new(&safe_path);

    match safe.encrypt() {
        Ok(_) => (),
        Err(x) => return Err(x.to_string())
    }

    Ok(())
}


fn main() {
    if let Some(x) = encrypt().err() {
        println!("Error: {}!", x);
    }
}
