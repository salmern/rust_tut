



fn main() {
let numbre = 300;
match numbre{
     100..=800 => println!("The number is between 100 and 800"),
    1=>println!("this number is 1"),
    2=>println!("this number is 2"),
    3=>println!("this number is 3"),
    _=> println!("this number is sth else"),
}
}