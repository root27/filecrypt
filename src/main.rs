

use clap::Parser;
use fernet::Fernet;
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};

#[derive(Parser,Debug)]
struct Cli {
    path: String,
}


fn check_key(key: &str) -> Vec<u8> {
    
    let decoded = base64::engine::general_purpose::URL_SAFE.decode(key).unwrap();

    if decoded.len() != 32 {

        let mut new_decoded = Vec::new();

        for i in 0..32 {
            new_decoded.push(decoded[i]);
        }

        return new_decoded;


        
    }

    return decoded;

}


fn main() {

    let arg = Cli::parse();


    let content = std::fs::read_to_string(&arg.path).expect("could not read file");

    println!("File content: {}", content);

    _ = content; 

    let  my_key = String::from("my_key");

    
    let base64_encoded=  base64::engine::general_purpose::URL_SAFE.encode(my_key.as_bytes());

    println!("Encoded Key is: {}", base64_encoded);

    let newkey = check_key(&base64_encoded);

    println!("New Key is: {:?}", newkey);
   

    // let fernet = Fernet::new(&newkey).unwrap();

    // _ = fernet
   


}
