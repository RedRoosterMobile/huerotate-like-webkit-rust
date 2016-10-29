// read first arg
// read roatation
// manipulate
// write file to target locations from third arg
use std::env;
use std::os;

// [angle_deg] [in_image] [out_image]
fn main() {
    println!("Hello, world!");
    //let executable = '';

    // Prints each argument on a separate line
    for argument in env::args() {
        println!("{}", argument);
    }
    // Prints each argument on a separate line
    for argument in env::args_os() {
        println!("{:?}", argument);
    }
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
    }
}
