#[derive(Debug, Clone, Copy)]
enum Position{
    Manager,
    Director,
    Worker
}
#[derive(Debug, Clone, Copy)]
struct Employee{
    position: Position,
    work_hour: i32,
}
fn print_empl(empl:Employee){
    println!("{:?}", empl);
}

fn main(){
 let emp  = Employee{
    position:Position::Manager,
    work_hour:40,

 };
 println!("{:?}", emp.position);
 println!("{:?}", emp);
 print_empl(emp);
}