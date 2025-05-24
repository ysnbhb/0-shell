mod utils;

use utils::io::*;
use utils::parser::*;

fn main() {
    let input = read_line().unwrap();
    let tokens = parst_input(input);

    match tokens {
        Ok(value) => {
            println!("{:?}" , value)
        }
        Err(_) => println!("Incorrect input"),
    }
    // for token in tokens {
    //     println!("{}", token);
    // }
}
