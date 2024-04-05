

use clap::{ArgAction, Command};
use fernet::Fernet;
use std::fs;




fn main() {


    let cmd = Command::new("File Encryptor/Decryptor")
        .author("O.D")
        .version("1.0.0")
        .about("Encrypt and decrypt files using Fernet")
        .arg(
            clap::Arg::new("file")
                .short('f')
                .long("file")
                .action(ArgAction::Set)
            )
        .arg(
            clap::Arg::new("encrypt")
                .short('e')
                .long("encrypt")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("decrypt")
                .short('d')
                .long("decrypt")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("key")
                .short('k')
                .long("key")
                .action(ArgAction::Set)
        )
        .arg(
            clap::Arg::new("Encrypted file")
                .short('E')
                .long("Encrypted file")
                .action(ArgAction::Set)
        )
        .get_matches();



        if cmd.args_present() {
            
            if let Some(path) = cmd.get_one::<String>("file") {
                println!("Path: {}", path);
            
            if cmd.get_flag("encrypt") {

                let content = std::fs::read_to_string(path).expect("could not read file");
                let key = Fernet::generate_key();
                let fernet = Fernet::new(&key).unwrap();
                let token = fernet.encrypt(content.as_bytes());
                fs::write(path, token).expect("could not write file");

                println!("File encrypted");

                println!("Please save the key for decryption: {}", key);

            }

            if cmd.get_flag("decrypt") {

                if !cmd.get_flag("key"){
                    println!("Please provide a key for decryption");
                    return;
                }

                if !cmd.get_flag("Encrypted file") {
                    
                    println!("Please provide an encrypted file for decryption");

                    return;
                }

            let key = cmd.get_one::<String>("key").unwrap();

            let encrypted_file = cmd.get_one::<String>("Encrypted file").unwrap();

            let fernet = Fernet::new(&key).unwrap();
            
            let new_content = std::fs::read_to_string(encrypted_file).expect("could not read file");
            println!("File content: {}", new_content);
            let decryption = fernet.decrypt(&new_content).unwrap();
            let decrypted_content = String::from_utf8(decryption).unwrap();
            println!("Decrypted content: {}", decrypted_content);
            
            }
        }
    }
    


}
