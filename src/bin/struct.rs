



fn main() {
struct Shippingbox{
    /*
    depth: i32,
    width: i32,
    */
    height: i32,
}
let my_box = Shippingbox{
    // depth: 3,
    // width: 8,
    height: 9,

};
// let tall = my_box.height;
println!("my height is {:?}", my_box.height);

    enum Flavors{
        Sparkling,
        Sweet,
        Fruity,
    }
    struct Drink{
        flavor:Flavors,
        flavor_ouz:f64,
    }
    fn print_drink(drink: Drink){
        match drink.flavor {
            Flavors::Fruity=>println!("flavor: fruity"),
            Flavors::Sparkling=>println!("flavor: sparkling"),
            Flavors::Sweet=>println!("flavor: sweet"),  
        }
         println!("oz: {:?} ", drink.flavor_ouz)
    }

    let fruitee = Drink{
        flavor:Flavors::Fruity,
        flavor_ouz:700.9,

    };
     print_drink(fruitee);

     let sparkle = Drink{
        flavor:Flavors::Sparkling,
        flavor_ouz:800.9,

    };
     print_drink(sparkle);

     let sweetish = Drink{
        flavor:Flavors::Sweet,
        flavor_ouz:900.9,

    };
     print_drink(sweetish);
    
}

