enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String)
}

fn main(){

    let tickets = vec![
        Ticket::Backstage(100.00, "Salman".to_owned()),
        Ticket::Standard(80.00),
        Ticket::Vip(150.00, "Muhammad".to_owned())
    ];
    for ticket in tickets{
        match ticket {
            Ticket::Backstage(price, holder )=>println!("Backstage Ticket Holder:{} Price:{:?}",holder, price ),
            Ticket::Standard(price)=>println!(" Standard  Ticket Price:{:?}", price),
            Ticket::Vip(price,holder )=>println!(" Vip Ticket Holder:{} Price:{:?}", holder, price)
        }
    }
}