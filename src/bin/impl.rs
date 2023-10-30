struct Temperature {
    degree_f: f32,
}

impl Temperature{
    fn boiling()-> Self{
        Self { degree_f: 89.9, }
    }
    fn freezing()-> Self{
        Self { degree_f: 989.9, }
    }
    fn show_temp(&self) {
        println!(" the temp is {:?}", self.degree_f);
        
    }
}
fn main() {
    let hot = Temperature {
         degree_f: 67.99 
        };
        let boil = Temperature::boiling();
        boil.show_temp();

        let freeze = Temperature::freezing();
        freeze.show_temp();


        
    // show_temp(hot);
    // Temperature::show_temp(hot);
    hot.show_temp();
}
