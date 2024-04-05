

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
        .get_matches();



        if cmd.args_present() {
            
            if let Some(path) = cmd.get_one::<String>("file") {
                println!("Path: {}", path);
            

            let content = std::fs::read_to_string(path).expect("could not read file");
            let key = Fernet::generate_key();
            let fernet = Fernet::new(&key).unwrap();
            let token = fernet.encrypt(content.as_bytes());
            fs::write(path, token).expect("could not write file");
            let new_content = std::fs::read_to_string("test.txt").expect("could not read file");
            println!("File content: {}", new_content);
            let decryption = fernet.decrypt(&new_content).unwrap();
            let decrypted_content = String::from_utf8(decryption).unwrap();
            println!("Decrypted content: {}", decrypted_content);
            }
        }
    


}
