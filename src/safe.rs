use std::path::Path;
use std::path::PathBuf;
use std::fs::create_dir_all;
use std::fs::remove_dir_all;
use std::fs::remove_file;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use simple_crypt::{encrypt_directory, decrypt_directory};


pub struct Safe {
    encrypted_path: PathBuf,
    decrypted_path: PathBuf,
    password_path: PathBuf,
}

impl Safe {
    pub fn new(path: &Path) -> Safe{
        Safe {
            encrypted_path: path.join("encrypted"),
            decrypted_path: path.join("decrypted safe"),
            password_path: path.join("password"),
        }
    }
    pub fn init(&self, password: String) -> std::io::Result<()> {
        if self.encrypted_path.exists() || self.decrypted_path.exists() || self.password_path.exists() {
            return Err(Error::other(String::from("Safe is already initialized.")));
        }

        match create_dir_all(&self.decrypted_path) {
            Ok(_) => (),
            Err(_) => return Err(Error::other(String::from(format!("Failed to create decrypted directory at path: {}", self.decrypted_path.display()))))
        }

        let mut file = File::create(&self.password_path)?;        
        file.write_all(password.as_bytes())?;

        Ok(())
    }

    pub fn encrypt(&self) -> std::io::Result<()> {
        if !self.decrypted_path.exists() || !self.password_path.exists() || self.encrypted_path.exists() {
            return Err(Error::other("Safe not initialized correctly or already encrypted."));
        }

        let mut password_file = File::open(&self.password_path)?;
        let mut password = String::new();
        password_file.read_to_string(&mut password)?;

        match encrypt_directory(&self.decrypted_path, &self.encrypted_path, password.as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err(Error::other("Failed to encrypt directory"))
        }

        remove_dir_all(&self.decrypted_path)?;
        remove_file(&self.password_path)?;

        Ok(())
    }

    pub fn decrypt(&self, password: String) -> std::io::Result<()> {
        if !self.encrypted_path.exists() {
            return Err(Error::other("Safe is not encrypted."));
        }
        let target_path = match self.decrypted_path.parent() {
            Some(x)=>x,
            None=>return Err(Error::other("Couldn't find safe directory."))
        };

        match decrypt_directory(&self.encrypted_path, target_path, password.as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err(Error::other("Failed to decrypt directory"))
        }

        remove_file(&self.encrypted_path)?;

        let mut file = File::create(&self.password_path)?;        
        file.write_all(password.as_bytes())?;

        Ok(())
    }
}