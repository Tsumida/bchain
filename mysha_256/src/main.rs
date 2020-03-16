pub mod sha_256;
use sha_256::*;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("My SHA-256")
        .version("0.1.0")
        .author("Tsumida")
        .about("Use SHA-256 to generate digest")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .required(true)
            .value_name("INPUT")
            .takes_value(true))
        .get_matches();

    if let Some(inp) = matches.value_of("input"){
        let result = SHA256::new(inp.as_bytes()).cal_sha_256();
        println!("{}", result);
    }
    
}