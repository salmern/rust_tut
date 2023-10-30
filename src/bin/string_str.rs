struct LineItem{
    name: String,
    count: i32,
}
fn print_name(name:&str){
println!("name {}", name);
}

fn main(){
    let items = vec![
        // LineItem {name: "apple".to_string(), count: 5},
        LineItem{
            name: "orange".to_owned(),
            count:5
        },
        LineItem{
            name:String::from("banana"),
            count:50
        }
    ];
    for item in  items{
        print_name(&item.name);
        println!("count:{}\n", item.count);
    }
    

}