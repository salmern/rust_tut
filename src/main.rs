mod my_modules;
// use my_modules::{add, display_result};
use std::io;

// mod display {
//     pub fn display_result(result: i32) {
//         println!("{:?}", result);
//     }
// }

fn main() {
 
    // match add(1, 90) {
    //     Ok(result) => display_result(result),
    //     Err(e) => panic!("Error: {}", e),
    // }

    let mut input = String::new();

    println!("Enter the first number:");
    
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let a: i32 = input.trim().parse().expect("Invalid input");
    

    input.clear();

    println!("Enter the second number:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
      // Replace any commas with an empty string
      let input = input.replace(",", "");
    let b: i32 = input.trim().parse().expect("Invalid nput");

    match my_modules::add(a, b) {
        Ok(result) => my_modules::display_result(result),
        Err(e) => println!("Error: {}", e),
    }
}


