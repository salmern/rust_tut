
fn coordinates()->(i32, i32){
    (1, 1)
}


fn main() {
let (x,y)= coordinates();
if y > x{
    println!("y > x")
}else if y < x{
    println!("y < x")
}else {
    println!("y = 7");
}
    let coord =(2,3);
    println!("The value of coord is: {:?} {:?}",coord.0, coord.1); 

    let (x, y)= coord;
 println!("{} {}", x, y);
 let  (name, age)=("salman", 28);
 println!("print my name and age {} {}", name, age);
    
}

