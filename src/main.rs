
use clap::Parser;
use ring::digest;

#[derive(Parser,Debug)]
struct Cli {
    path: String,
}


fn main() {

    let arg = Cli::parse();


    let content = std::fs::read_to_string(&arg.path).expect("could not read file");


    println!("File Content is: ");

    println!("{}", content);

    let bytesofcontent = content.as_bytes();

    let hash = digest::digest(&digest::SHA256, bytesofcontent);

    println!("Hash of the file is {:?}", hash);
    

}
