
struct Test{
    score:i32,
}
fn main(){
    let my_scores =vec![
        Test{score:10},
        Test{score:20},
        Test{score:30},
        Test{score:40},
        Test{score:50},
    ];
    for test in my_scores{
    println!("Score = {:?}", test.score);
   
    }
    println!("============================\n============================\n============================");

    let numbers=vec![10,20, 30, 40];
    for num in &numbers{
        match num{
            30 =>println!("thirty"),
            _=>println!("{:?}", num)
        }
        
    }
    println!("the length is {:?}", numbers.len());
}