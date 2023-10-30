
/// a structure for the student fields
struct Student{
    name:String,
    locker: Option<i32>
}
fn main(){
let zainab = Student{
    name:"Zainab".to_owned(),
    locker:Some(8),
    // locker:None,
};
match zainab.locker{
    Some(num)=>println!("the number is {:?}", num),
    None=>println!("no locker assigned"),
}
match zainab.name{
    name=>println!("the name is {:?}", name),
    // None=>println!("no locker assigned"),
}
}