

use clap::{ArgAction, Command};
use fernet::Fernet;
use std::fs;

use std::io::Write;


fn extract_file_extension(filename: &str) -> &str {

    let mut extension = "";
    for (i, c) in filename.chars().enumerate() {
        if c == '.' {
            extension = &filename[i..];
        }
    }

    extension


}




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
       
        .get_matches();



        if cmd.args_present() {
            
            if let Some(path) = cmd.get_one::<String>("file") {
            

                let extension = extract_file_extension(&path);

                if extension != ".txt" {
                    println!("Only .txt files are supported");
                    return;
                }

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

                    if let Some(_key) = cmd.get_one::<String>("key") {

                        let key = cmd.get_one::<String>("key").unwrap();

                        let encrypted_file = cmd.get_one::<String>("file").unwrap();

                        let fernet = Fernet::new(&key).unwrap();
                        
                        let new_content = std::fs::read_to_string(encrypted_file).expect("could not read file");
                        let decryption = fernet.decrypt(&new_content).unwrap();
                        let decrypted_content = String::from_utf8(decryption).unwrap();
                        println!("Decrypted content: {}", decrypted_content);

                        print!("Would you like to save the decrypted content to a file? (y/n): ");

                        std::io::stdout().flush().unwrap();
                        

                        let mut user_input = String::new();

                        std::io::stdin().read_line(&mut user_input).expect("Failed to read line");

                        if user_input.trim() == "y" {
                            print!("Please provide a name for the file: ");

                            std::io::stdout().flush().unwrap();

                            let mut file_name = String::new();
                            std::io::stdin().read_line(&mut file_name).expect("Failed to read line");
                            fs::write(file_name.trim(), decrypted_content).expect("could not write file");
                            println!("File saved");
                            return;
                        }

                        println!("Exiting...");

                        return;
                    }


                    println!("Please provide a key for decryption")


                }
        }
    }
    


}
