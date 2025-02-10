//declare the constant values for faren 
const FREEZING_WATER : f64 = 32.0;

//create the converters
//faren -> cels
fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - FREEZING_WATER) * 5.0 / 9.0
} 

//cels to faren
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_WATER
}
fn main () {
    //made the inital temp this, but can change it if needed
    let mut temp_f: f64 = 50.0;
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{} deg faren {:.2} deg cels", temp_f, temp_c);

    //loop 
    for i in 1..=5{
        let next_f = temp_f + i as f64;
        let next_c = fahrenheit_to_celsius(next_f as f64);
        println!("{} deg faren {:.2} deg cels", next_f, next_c);
    }
}
