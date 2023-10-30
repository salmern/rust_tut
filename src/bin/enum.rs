



fn main() {
let numbre = 300;
match numbre{
     100..=800 => println!("The number is between 100 and 800"),
    1=>println!("this number is 1"),
    2=>println!("this number is 2"),
    3=>println!("this number is 3"),
    _=> println!("this number is sth else"),
}
let mut i =1;
while i<=5 {
    println!("{:?}", i);
    // i = i + 1;
    i+= 1;
//      if i ==5{
//         break;
// }

}
enum Direction{
    Red,
   Green,
   Blue,
}
let go = Direction::Green;
match go {
    Direction::Green=>println!("green"),
    Direction::Blue=>println!("blue"),
    Direction::Red=>println!("red"),
   
}

// println!("done!")
}
