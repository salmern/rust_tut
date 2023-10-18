mod my_modules;

// mod display {
//     pub fn display_result(result: i32) {
//         println!("{:?}", result);
//     }
// }

fn main() {
    match my_modules::add(1, 90) {
        Ok(result) => my_modules::display_result(result),
        Err(e) => println!("Error: {}", e),
    }
}


